//! Convert Python dataclass fields into typed Rust settings. The core validates values.
use crate::{invalid, map_error};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyInt, PyList, PyString, PyTuple};
use radiomics::settings::*;
use radiomics::{
    IbsiInterpolation, ImageSettings, ImageType, Interpolator, Settings, TransformSettings,
};

trait Field: Sized {
    fn read(value: &Bound<'_, PyAny>) -> Option<Self>;
}

macro_rules! scalar {
    ($ty:ty, $($python:ty)|+) => {
        impl Field for $ty {
            fn read(value: &Bound<'_, PyAny>) -> Option<Self> {
                if $(value.is_exact_instance_of::<$python>())||+ {
                    value.extract().ok()
                } else {
                    None
                }
            }
        }
    };
}

scalar!(bool, PyBool);
scalar!(u32, PyInt);
scalar!(usize, PyInt);
scalar!(i64, PyInt);
scalar!(f64, PyInt | PyFloat);
scalar!(String, PyString);

impl<T: Field> Field for Option<T> {
    fn read(value: &Bound<'_, PyAny>) -> Option<Self> {
        if value.is_none() {
            Some(None)
        } else {
            T::read(value).map(Some)
        }
    }
}

impl<T: Field> Field for Vec<T> {
    fn read(value: &Bound<'_, PyAny>) -> Option<Self> {
        if !value.is_instance_of::<PyTuple>() && !value.is_instance_of::<PyList>() {
            return None;
        }
        value
            .try_iter()
            .ok()?
            .map(|item| T::read(&item.ok()?))
            .collect()
    }
}

impl Field for (f64, f64) {
    fn read(value: &Bound<'_, PyAny>) -> Option<Self> {
        let [lo, hi]: [f64; 2] = Vec::read(value)?.try_into().ok()?;
        Some((lo, hi))
    }
}

fn field<T: Field>(obj: &Bound<'_, PyAny>, name: &str, message: &'static str) -> PyResult<T> {
    T::read(&obj.getattr(name)?).ok_or_else(|| invalid(message))
}

// Spatial spacing and threshold accept objects implementing Python's numeric protocol.
fn spatial_number<'py, T: FromPyObjectOwned<'py>>(
    obj: &Bound<'py, PyAny>,
    name: &str,
    message: &'static str,
) -> PyResult<T> {
    obj.getattr(name)?.extract().map_err(|_| invalid(message))
}

fn tuple<T: Field>(obj: &Bound<'_, PyAny>, name: &str, message: &'static str) -> PyResult<Vec<T>> {
    let value = obj.getattr(name)?;
    if !value.is_exact_instance_of::<PyTuple>() {
        return Err(invalid(message));
    }
    Vec::read(&value).ok_or_else(|| invalid(message))
}

// Typed spatial/filter choices reject bare strings, including strings with valid spellings.
macro_rules! typed_enum {
    ($ty:ty, $module:literal, $name:literal) => {
        impl Field for $ty {
            fn read(value: &Bound<'_, PyAny>) -> Option<Self> {
                let module = value
                    .py()
                    .import(concat!("rustrad.config.", $module))
                    .ok()?;
                if !value.is_exact_instance(&module.getattr($name).ok()?) {
                    return None;
                }
                value.extract::<String>().ok()?.parse().ok()
            }
        }
    };
}

typed_enum!(ImageType, "transforms", "ImageType");
typed_enum!(Interpolator, "spatial", "Interpolator");
typed_enum!(FilterPadding, "filtering", "FilterPadding");
typed_enum!(FourierGrid, "filtering", "FourierGrid");
typed_enum!(GaborResponse, "filtering", "GaborResponse");
typed_enum!(LbpMethod, "filtering", "LBPMethod");
typed_enum!(OrientationPooling, "filtering", "OrientationPooling");
typed_enum!(RieszBase, "filtering", "RieszBase");
typed_enum!(LogKernel, "filtering", "LogKernel");

macro_rules! convention {
    ($obj:ident, $name:ident, $ty:ty) => {{
        let value = $obj.getattr(stringify!($name))?;
        value
            .extract::<String>()
            .ok()
            .and_then(|text| text.parse().ok())
            .ok_or_else(|| {
                let choices = <$ty>::ALL
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                invalid(format!("{} must be one of {choices}", stringify!($name)))
            })?
    }};
}

fn aggregation<T: std::str::FromStr>(obj: &Bound<'_, PyAny>, family: &str) -> PyResult<T> {
    let value = obj.getattr(format!("{family}_aggregation"))?;
    value
        .extract::<String>()
        .ok()
        .and_then(|text| text.parse().ok())
        .ok_or_else(|| invalid(format!("unsupported {family} aggregation: {value}")))
}

pub(crate) fn settings(obj: &Bound<'_, PyAny>) -> PyResult<Settings> {
    let bin_count: Option<u32> = field(
        obj,
        "bin_count",
        "bin_count must be a positive integer below 2**31",
    )?;
    let width = obj.getattr("bin_width")?;
    let bin_width = match f64::read(&width) {
        Some(value) => value,
        None if bin_count.is_some() => Settings::default().bin_width,
        None => return Err(invalid("bin_width must be finite and positive")),
    };
    let resegment_mode = obj
        .getattr("resegment_mode")?
        .extract::<String>()
        .ok()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| invalid("resegment_mode must be absolute, relative, or sigma"))?;
    Ok(Settings {
        bin_count,
        bin_width,
        resegment_mode,
        gldm_alpha: field(
            obj,
            "gldm_alpha",
            "gldm_alpha must be a nonnegative integer",
        )?,
        label: field(obj, "label", "label must be a positive integer")?,
        voxel_array_shift: field(obj, "voxel_array_shift", "voxel_array_shift must be finite")?,
        distances: field(obj, "distances", "distances must contain positive integers")?,
        normalize: field(
            obj,
            "normalize",
            "normalize and resegment_shape must be booleans",
        )?,
        resegment_shape: field(
            obj,
            "resegment_shape",
            "normalize and resegment_shape must be booleans",
        )?,
        normalize_scale: field(
            obj,
            "normalize_scale",
            "normalize_scale must be finite and positive",
        )?,
        remove_outliers: field(
            obj,
            "remove_outliers",
            "remove_outliers requires normalization and a finite positive value",
        )?,
        resegment_range: field(
            obj,
            "resegment_range",
            "resegment_range must contain one or two finite thresholds",
        )?,
        force2d_axis: field(
            obj,
            "force2d_axis",
            "force2d_axis must be an integer indexing a NumPy image axis",
        )?,
        symmetrical_glcm: field(
            obj,
            "symmetrical_glcm",
            "masked_kernel and symmetrical_glcm must be booleans",
        )?,
        masked_kernel: field(
            obj,
            "masked_kernel",
            "masked_kernel and symmetrical_glcm must be booleans",
        )?,
        kernel_radius: field(
            obj,
            "kernel_radius",
            "kernel_radius must be a positive integer",
        )?,
        scratch_mib: field(obj, "scratch_mib", "scratch_mib must be a positive integer")?,
        max_gray_levels: field(
            obj,
            "max_gray_levels",
            "max_gray_levels must be a positive integer or None",
        )?,
        features: field(
            obj,
            "features",
            "features must be a nonempty unique selection",
        )?,
        bin_anchor: field(
            obj,
            "bin_anchor",
            "bin_anchor must be a finite number or None",
        )?,
        resegment_outlier_sigma: field(
            obj,
            "resegment_outlier_sigma",
            "resegment_outlier_sigma must be a finite positive number or None",
        )?,
        slice_axis: field(
            obj,
            "slice_axis",
            "slice_axis must be an integer indexing an image axis",
        )?,
        neighborhood_radius: field(
            obj,
            "neighborhood_radius",
            "neighborhood_radius must be a positive integer",
        )?,
        ivh_bin_width: field(
            obj,
            "ivh_bin_width",
            "ivh_bin_width must be finite and positive",
        )?,
        ivh_bin_count: field(
            obj,
            "ivh_bin_count",
            "ivh_bin_count must be a positive integer",
        )?,
        ivh_intensity_range: field(
            obj,
            "ivh_intensity_range",
            "ivh_intensity_range must be finite ordered lower/upper bounds",
        )?,
        covariance_divisor: convention!(obj, covariance_divisor, CovarianceDivisor),
        ellipsoid_area: convention!(obj, ellipsoid_area, EllipsoidArea),
        kurtosis: convention!(obj, kurtosis, Kurtosis),
        zero_variance_moments: convention!(obj, zero_variance_moments, ZeroVarianceMoments),
        ngtdm_singular_cases: convention!(obj, ngtdm_singular_cases, NgtdmSingularCases),
        neighborhood_support: convention!(obj, neighborhood_support, NeighborhoodSupport),
        bin_origin: convention!(obj, bin_origin, BinOrigin),
        constant_roi_bin: convention!(obj, constant_roi_bin, ConstantRoiBin),
        mask_extrapolation: convention!(obj, mask_extrapolation, MaskExtrapolation),
        spline_boundary: convention!(obj, spline_boundary, SplineBoundary),
        glcm_aggregation: aggregation(obj, "glcm")?,
        glrlm_aggregation: aggregation(obj, "glrlm")?,
        gldm_aggregation: aggregation(obj, "gldm")?,
        gldzm_aggregation: aggregation(obj, "gldzm")?,
        ngtdm_aggregation: aggregation(obj, "ngtdm")?,
        glszm_aggregation: aggregation(obj, "glszm")?,
    })
}

#[pyfunction]
pub(crate) fn validate_settings(
    py: Python<'_>,
    obj: &Bound<'_, PyAny>,
    ndim: isize,
) -> PyResult<()> {
    let ndim = usize::try_from(ndim).unwrap_or(0);
    let settings = settings(obj)?;
    py.detach(|| settings.validate(ndim)).map_err(map_error)
}

// These constructors only marshal fields. Domain and cross-field checks use validate().
macro_rules! block {
    ($func:ident, $ty:ident { $($name:ident: $read:ident($msg:literal)),+ $(,)? }) => {
        fn $func(obj: &Bound<'_, PyAny>) -> PyResult<$ty> {
            Ok($ty { $($name: $read(obj, stringify!($name), $msg)?),+ })
        }
    };
}

block!(
    ibsi,
    IbsiInterpolation {
        spacing: spatial_number("IBSI spacing requires 2 or 3 finite nonnegative values"),
        round_intensities: field("round_intensities must be boolean"),
        mask_threshold: spatial_number("mask_threshold must be finite and in (0, 1]"),
        image_interpolator: field("image_interpolator must be an Interpolator value"),
    }
);

fn nested<'py>(
    obj: &Bound<'py, PyAny>,
    name: &str,
    module: &str,
    class: &str,
    message: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let value = obj.getattr(name)?;
    let expected = obj.py().import(module)?.getattr(class)?;
    if value.is_exact_instance(&expected) {
        Ok(value)
    } else {
        Err(invalid(message))
    }
}

pub(crate) fn image_settings(obj: &Bound<'_, PyAny>) -> PyResult<ImageSettings> {
    let ibsi_interpolation = if obj.getattr("ibsi_interpolation")?.is_none() {
        None
    } else {
        Some(ibsi(&nested(
            obj,
            "ibsi_interpolation",
            "rustrad.config.spatial",
            "IbsiInterpolation",
            "ibsi_interpolation must be an IbsiInterpolation value",
        )?)?)
    };
    Ok(ImageSettings {
        resampled_spacing: spatial_number(
            obj,
            "resampled_spacing",
            "resampled_spacing requires 2 or 3 finite nonnegative values",
        )?,
        interpolator: field(
            obj,
            "interpolator",
            "interpolator must be an Interpolator value",
        )?,
        correct_mask: field(obj, "correct_mask", "correct_mask must be boolean")?,
        pad_distance: field(
            obj,
            "pad_distance",
            "pad_distance must be a nonnegative integer",
        )?,
        ibsi_interpolation,
    })
}

block!(
    wavelet,
    WaveletSettings {
        name: field("wavelet must be a string"),
        level: field("wavelet_level must be a positive integer"),
        start_level: field("wavelet_start_level must be a nonnegative integer"),
    }
);
block!(
    log,
    LogSettings {
        sigma: tuple("sigma values must be finite and positive"),
        kernel: field("log_kernel must be a typed LogKernel"),
        cutoff: field("log_cutoff must be finite and positive"),
        padding: field("log_padding must be a typed FilterPadding"),
        constant: field("log_constant must be a finite number"),
        planar: field("log_planar must be a boolean"),
    }
);
block!(
    lbp2d,
    Lbp2DSettings {
        samples: field("lbp2d_samples must be an integer from 1 to 24"),
        radius: field("LBP radii must be finite and positive"),
        method: field("lbp2d_method must be a typed LBPMethod"),
    }
);
block!(
    lbp3d,
    Lbp3DSettings {
        levels: field("lbp3d_levels must be a positive integer"),
        radius: field("LBP radii must be finite and positive"),
        subdivision: field("lbp3d_subdivision must be an integer from zero to four"),
    }
);
block!(
    gradient,
    GradientSettings {
        use_spacing: field("gradient_use_spacing must be boolean")
    }
);
block!(
    mean,
    MeanSettings {
        support: field("mean_support must be a positive odd integer"),
        padding: field("mean_padding must be a typed FilterPadding"),
        constant: field("mean_constant must be a finite number"),
        planar: field("mean_planar must be a boolean"),
    }
);
block!(
    laws,
    LawsSettings {
        kernel: field("laws_kernel must be a string"),
        padding: field("laws_padding must be a typed FilterPadding"),
        constant: field("laws_constant must be a finite number"),
        pooling: field("laws_pooling must be a typed OrientationPooling"),
        energy_distance: field("laws_energy_distance must be a nonnegative integer or None"),
    }
);
block!(
    gabor,
    GaborSettings {
        sigma: tuple("gabor_sigma must contain finite positive values"),
        wavelength: tuple("gabor_wavelength must contain finite positive values"),
        gamma: field("gabor_gamma must be finite and positive"),
        theta: tuple("gabor_theta must contain finite values"),
        response: field("gabor_response must be a typed GaborResponse"),
        pooling: field("gabor_pooling must be a typed OrientationPooling"),
        slice_axis: field("gabor_slice_axis must be None or an array axis from zero to two"),
        padding: field("gabor_padding must be a typed FilterPadding"),
        constant: field("gabor_constant must be a finite number"),
    }
);
block!(
    simoncelli,
    SimoncelliSettings {
        levels: tuple("simoncelli_levels must contain positive integers"),
        padding: field("simoncelli_padding must be a typed FilterPadding"),
        grid: field("simoncelli_grid must be a typed FourierGrid"),
        planar: field("simoncelli_planar must be a boolean"),
    }
);

fn orders(obj: &Bound<'_, PyAny>, name: &str, message: &'static str) -> PyResult<Vec<Vec<u32>>> {
    let value = obj.getattr(name)?;
    if !value.is_exact_instance_of::<PyTuple>() {
        return Err(invalid(message));
    }
    value
        .try_iter()?
        .map(|item| {
            let item = item?;
            if !item.is_exact_instance_of::<PyTuple>() {
                return Err(invalid(message));
            }
            Vec::read(&item).ok_or_else(|| invalid(message))
        })
        .collect()
}
block!(
    riesz,
    RieszSettings {
        base: field("riesz_base must be a typed RieszBase"),
        orders: orders(
            "riesz_orders must contain nonempty tuples of nonnegative integers with a positive sum"
        ),
        log_sigma: tuple("riesz_log_sigma must contain finite positive values"),
        log_cutoff: field("riesz_log_cutoff must be finite and positive"),
        simoncelli_levels: tuple("riesz_simoncelli_levels must contain positive integers"),
        padding: field("riesz_padding must be a typed FilterPadding"),
        constant: field("riesz_constant must be a finite number"),
        grid: field("riesz_grid must be a typed FourierGrid"),
        steering: field("riesz_steering must be a boolean"),
        tensor_sigma: field("riesz_tensor_sigma must be finite and positive"),
    }
);
block!(
    separable_wavelet,
    SeparableWaveletSettings {
        name: field("separable_wavelet must be a string"),
        bands: tuple("separable_wavelet_bands must contain L/H band codes"),
        level: field("separable_wavelet_level must be a positive integer"),
        padding: field("separable_wavelet_padding must be a typed FilterPadding"),
        constant: field("separable_wavelet_constant must be a finite number"),
        pooling: field("separable_wavelet_pooling must be a typed OrientationPooling"),
        planar: field("separable_wavelet_planar must be a boolean"),
    }
);

pub(crate) fn transforms(obj: &Bound<'_, PyAny>) -> PyResult<TransformSettings> {
    macro_rules! child {
        ($name:ident, $class:literal) => {
            $name(&nested(
                obj,
                stringify!($name),
                "rustrad.config.transforms",
                $class,
                concat!(stringify!($name), " must be ", $class),
            )?)?
        };
    }
    Ok(TransformSettings {
        image_types: field(
            obj,
            "image_types",
            "image_types must contain typed ImageType entries",
        )?,
        scratch_mib: field(obj, "scratch_mib", "scratch_mib must be a positive integer")?,
        wavelet: child!(wavelet, "WaveletSettings"),
        log: child!(log, "LoGSettings"),
        lbp2d: child!(lbp2d, "LBP2DSettings"),
        lbp3d: child!(lbp3d, "LBP3DSettings"),
        gradient: child!(gradient, "GradientSettings"),
        mean: child!(mean, "MeanSettings"),
        laws: child!(laws, "LawsSettings"),
        gabor: child!(gabor, "GaborSettings"),
        simoncelli: child!(simoncelli, "SimoncelliSettings"),
        riesz: child!(riesz, "RieszSettings"),
        separable_wavelet: child!(separable_wavelet, "SeparableWaveletSettings"),
    })
}

#[pyfunction]
pub(crate) fn validate_block(py: Python<'_>, class: &str, obj: &Bound<'_, PyAny>) -> PyResult<()> {
    macro_rules! validate {
        ($constructor:ident) => {{
            let block = $constructor(obj)?;
            py.detach(|| block.validate()).map_err(map_error)
        }};
    }
    match class {
        "IbsiInterpolation" => validate!(ibsi),
        "ImageSettings" => validate!(image_settings),
        "TransformSettings" => validate!(transforms),
        "WaveletSettings" => validate!(wavelet),
        "LoGSettings" => validate!(log),
        "LBP2DSettings" => validate!(lbp2d),
        "LBP3DSettings" => validate!(lbp3d),
        "GradientSettings" => gradient(obj).map(|_| ()),
        "MeanSettings" => validate!(mean),
        "LawsSettings" => validate!(laws),
        "GaborSettings" => validate!(gabor),
        "SimoncelliSettings" => validate!(simoncelli),
        "RieszSettings" => validate!(riesz),
        "SeparableWaveletSettings" => validate!(separable_wavelet),
        _ => Err(invalid(format!("unknown settings class: {class}"))),
    }
}
