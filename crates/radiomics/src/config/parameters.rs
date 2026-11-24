//! PyRadiomics parameter files, mirroring `mlxradiomics/config/parameters.py`.
//! Single-threaded CPU orchestration.
//! Status: ported (wavelet-name lookup awaits W6).

use std::{fs, path::Path, str::FromStr};

use crate::config::settings::*;
use crate::config::spatial::{ImageSettings, Interpolator};
use crate::config::transforms::{ImageType, TransformSettings, validate_filter_shape};
use crate::config::validation::*;
use crate::error::{Error, Result};
use yaml_serde::Mapping;

/// Ordered YAML value used at the parameter boundary. Mapping keys are checked by context.
pub use yaml_serde::Value;

/// Effective settings for one extraction.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtractionParameters {
    /// Feature selection, discretisation and conventions.
    pub settings: Settings,
    /// Spatial preprocessing in NumPy axis order.
    pub image_settings: ImageSettings,
    /// Derived images in requested order.
    pub transforms: TransformSettings,
}

impl ExtractionParameters {
    /// Parse a PyRadiomics-style YAML parameter document for a 2-D or 3-D image.
    ///
    /// # Errors
    /// [`Error::InvalidInput`] for malformed YAML, unsupported sections or keys, or
    /// values that [`Settings::validate`] rejects for `ndim`.
    pub fn from_yaml(text: &str, ndim: usize) -> Result<Self> {
        let value = load_yaml(text)
            .map_err(|detail| Error::InvalidInput(format!("invalid params YAML: {detail}")))?;
        require_mapping(&value, "params")?;
        parse_params(&value, ndim)
    }

    /// Read and parse a YAML parameter file; see [`ExtractionParameters::from_yaml`].
    ///
    /// # Errors
    /// [`Error::InvalidInput`] if the file is missing, is not UTF-8 or fails validation;
    /// [`Error::Io`] if it cannot be read.
    pub fn from_file(path: impl AsRef<Path>, ndim: usize) -> Result<Self> {
        parse_params(&load_params(path)?, ndim)
    }
}

/// Load a UTF-8 YAML document and require a string-keyed top-level mapping.
pub fn load_params(path: impl AsRef<Path>) -> Result<Value> {
    let path = path.as_ref();
    if !path.is_file() {
        return invalid(format!("missing params file: {}", path.display()));
    }
    let bytes = fs::read(path)?;
    let text = std::str::from_utf8(&bytes).map_err(|e| {
        Error::InvalidInput(format!("invalid params UTF-8: {}: {e}", path.display()))
    })?;
    let value = load_yaml(text).map_err(|detail| {
        Error::InvalidInput(format!("invalid params YAML: {}: {detail}", path.display()))
    })?;
    require_mapping(&value, "params")?;
    Ok(value)
}

fn section<'a>(params: &'a Mapping, key: &str, empty: &'a Mapping) -> Result<&'a Mapping> {
    match params.get(key) {
        Some(value) => require_mapping(value, key),
        None => Ok(empty),
    }
}

// All callers have already checked the string-key contract.
fn entries(map: &Mapping) -> impl Iterator<Item = (&str, &Value)> {
    map.iter()
        .map(|(key, value)| (key.as_str().expect("validated string key"), value))
}

fn reject_unknown(map: &Mapping, allowed: &[&str], context: &str) -> Result<()> {
    let unknown: Vec<_> = entries(map)
        .map(|(key, _)| key)
        .filter(|k| !allowed.contains(k))
        .collect();
    if !unknown.is_empty() {
        return invalid(format!("{context}: {}", sorted_names(unknown)));
    }
    Ok(())
}

fn integer<T: TryFrom<i64>>(value: &Value, key: &str, range_message: &str) -> Result<T> {
    require_int(value, key)?
        .try_into()
        .map_err(|_| Error::InvalidInput(range_message.into()))
}

fn integers(value: &Value, key: &str, range_message: &str) -> Result<Vec<u32>> {
    require_int_tuple(value, key)?
        .into_iter()
        .map(|i| {
            i.try_into()
                .map_err(|_| Error::InvalidInput(range_message.into()))
        })
        .collect()
}

fn text_option<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    as_text(value).ok_or_else(|| Error::InvalidInput(format!("setting '{key}' must be a string")))
}

fn setting_choice<T: FromStr>(value: &Value, key: &str, choices: &str) -> Result<T> {
    as_text(value)
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| Error::InvalidInput(format!("setting '{key}' must be one of {choices}")))
}

fn text_field<'a>(value: &'a Value, context: &str) -> Result<&'a str> {
    as_text(value).ok_or_else(|| Error::InvalidInput(format!("{context} must be a string")))
}

fn enum_field<T: FromStr>(value: &Value, context: &str, class: &str) -> Result<T> {
    let name = text_field(value, context)?;
    name.parse()
        .map_err(|_| Error::InvalidInput(format!("{} is not a valid {class}", quoted(name))))
}

fn optional_integer<T: TryFrom<i64>>(value: &Value, key: &str, message: &str) -> Result<Option<T>> {
    if matches!(value, Value::Null) {
        Ok(None)
    } else {
        integer(value, key, message).map(Some)
    }
}

const ADVANCED: &[&str] = &[
    "resampledPixelSpacing",
    "interpolator",
    "padDistance",
    "correctMask",
    "lbp2DRadius",
    "lbp2DSamples",
    "lbp2DMethod",
    "lbp3DLevels",
    "lbp3DIcosphereRadius",
    "lbp3DIcosphereSubdivision",
    "gradientUseSpacing",
];

// Evaluate fields in the same order as the reference constructor.
macro_rules! read_fields {
    ($mapping:expr, |$v:ident, $k:ident| { $($key:literal => $update:expr),* $(,)? }) => {
        $(if let Some($v) = $mapping.get($key) { let $k = $key; $update; })*
    }
}

fn image_settings(setting: &Mapping, ndim: usize) -> Result<ImageSettings> {
    let mut image = ImageSettings::default();
    if let Some(value) = setting
        .get("resampledPixelSpacing")
        .filter(|v| !matches!(v, Value::Null))
    {
        let mut spacing = require_float_tuple(value, "resampledPixelSpacing")?;
        if spacing.len() != ndim {
            return invalid("resampledPixelSpacing must contain one value per image axis");
        }
        spacing.reverse();
        image.resampled_spacing = Some(spacing);
    }
    if let Some(value) = setting.get("interpolator") {
        image.interpolator = match as_text(value) {
            Some("sitkNearestNeighbor" | "nearest") => Interpolator::Nearest,
            Some("sitkLinear" | "linear") => Interpolator::Linear,
            Some("sitkBSpline" | "bspline") => Interpolator::Bspline,
            _ => {
                return invalid(
                    "setting 'interpolator' must be sitkNearestNeighbor, sitkLinear or sitkBSpline",
                );
            }
        };
    }
    read_fields!(setting, |value, key| {
        "padDistance" => image.pad_distance = integer(value, key, "pad_distance must be a nonnegative integer")?,
        "correctMask" => image.correct_mask = require_bool(value, key)?,
    });
    image.validate()?;
    Ok(image)
}

fn with_gradient_and_lbp(t: &mut TransformSettings, setting: &Mapping) -> Result<()> {
    read_fields!(setting, |value, key| {
        "gradientUseSpacing" => t.gradient.use_spacing = require_bool(value, key)?,
        "lbp2DRadius" => t.lbp2d.radius = require_float(value, key)?,
        "lbp2DSamples" => t.lbp2d.samples = integer(value, key, "lbp2d_samples must be an integer from 1 to 24")?,
        "lbp2DMethod" => t.lbp2d.method = setting_choice(value, key, "default, nri_uniform, ror, uniform, var")?,
        "lbp3DLevels" => t.lbp3d.levels = integer(value, key, "lbp3d_levels must be a positive integer")?,
        "lbp3DIcosphereRadius" => t.lbp3d.radius = require_float(value, key)?,
        "lbp3DIcosphereSubdivision" => t.lbp3d.subdivision = integer(value, key, "lbp3d_subdivision must be an integer from zero to four")?,
    });
    t.lbp2d.validate()?;
    t.lbp3d.validate()
}

fn parse_orders(value: &Value, key: &str) -> Result<Vec<Vec<u32>>> {
    let items = sequence(value).filter(|a| !a.is_empty()).ok_or_else(|| {
        Error::InvalidInput(format!("setting '{key}' must be a list of integer lists"))
    })?;
    const ORDER_DOMAIN: &str =
        "riesz_orders must contain nonempty tuples of nonnegative integers with a positive sum";
    items
        .iter()
        .map(|order| integers(order, key, ORDER_DOMAIN))
        .collect()
}

fn parse_bands(value: &Value, key: &str) -> Result<Vec<String>> {
    sequence(value)
        .and_then(|items| {
            items
                .iter()
                .map(|v| as_text(v).map(str::to_owned))
                .collect()
        })
        .ok_or_else(|| Error::InvalidInput(format!("{key} must be a list of band strings")))
}

// Derive the allowed keys from the same arms that assign them.
macro_rules! overrides {
    ($map:expr, $name:expr, |$v:ident, $k:ident| { $($key:literal => $update:expr),+ $(,)? }) => {{
        reject_unknown($map, &[$($key),+], &format!("unsupported {} overrides", $name))?;
        for (field, $v) in entries($map) {
            let context = format!("imageType.{}.{field}", $name);
            let $k = context.as_str();
            match field { $($key => $update,)+ _ => unreachable!("validated override key") }
        }
    }}
}

const IMAGE_TYPES: &[(&str, ImageType)] = &[
    ("Original", ImageType::Original),
    ("Square", ImageType::Square),
    ("SquareRoot", ImageType::SquareRoot),
    ("Logarithm", ImageType::Logarithm),
    ("Exponential", ImageType::Exponential),
    ("Gradient", ImageType::Gradient),
    ("Wavelet", ImageType::Wavelet),
    ("LoG", ImageType::Log),
    ("LBP2D", ImageType::Lbp2D),
    ("LBP3D", ImageType::Lbp3D),
    ("Mean", ImageType::Mean),
    ("Laws", ImageType::Laws),
    ("Gabor", ImageType::Gabor),
    ("Simoncelli", ImageType::Simoncelli),
    ("Riesz", ImageType::Riesz),
    ("SeparableWavelet", ImageType::SeparableWavelet),
    ("NonseparableWavelet", ImageType::Simoncelli),
];

fn apply_overrides(
    t: &mut TransformSettings,
    kind: ImageType,
    name: &str,
    custom: &Mapping,
) -> Result<()> {
    match kind {
        ImageType::Wavelet => {
            reject_unknown(
                custom,
                &["wavelet", "level", "start_level"],
                "unsupported Wavelet overrides",
            )?;
            if let Some(value) = custom.get("wavelet") {
                t.wavelet.name = as_text(value)
                    .ok_or_else(|| {
                        Error::InvalidInput("Wavelet 'wavelet' must be a string".into())
                    })?
                    .to_owned();
            }
            read_fields!(custom, |value, key| {
                "level" => t.wavelet.level = integer(value, key, "wavelet_level must be a positive integer")?,
                "start_level" => t.wavelet.start_level = integer(value, key, "wavelet_start_level must be a nonnegative integer")?,
            });
            t.wavelet.validate()?;
        }
        ImageType::Log => {
            reject_unknown(
                custom,
                &["sigma", "kernel", "padding", "cutoff", "constant", "planar"],
                "unsupported LoG overrides",
            )?;
            let sigma = custom
                .get("sigma")
                .ok_or_else(|| Error::InvalidInput("LoG requires explicit sigma values".into()))?;
            t.log.sigma = require_float_tuple(sigma, "sigma")?;
            read_fields!(custom, |value, key| {
                "kernel" => t.log.kernel = enum_field(value, &format!("imageType.LoG.{key}"), "LogKernel")?,
                "padding" => t.log.padding = enum_field(value, &format!("imageType.LoG.{key}"), "FilterPadding")?,
                "cutoff" => t.log.cutoff = require_float(value, &format!("imageType.LoG.{key}"))?,
                "constant" => t.log.constant = require_float(value, &format!("imageType.LoG.{key}"))?,
                "planar" => t.log.planar = require_bool(value, &format!("imageType.LoG.{key}"))?,
            });
            t.log.validate()?;
        }
        ImageType::Mean => {
            overrides!(custom, name, |value, key| {
                "support" => t.mean.support = integer(value, key, "mean_support must be a positive odd integer")?,
                "padding" => t.mean.padding = enum_field(value, key, "FilterPadding")?,
                "constant" => t.mean.constant = require_float(value, key)?,
                "planar" => t.mean.planar = require_bool(value, key)?,
            });
            t.mean.validate()?;
        }
        ImageType::Laws => {
            overrides!(custom, name, |value, key| {
                "kernel" => t.laws.kernel = text_field(value, key)?.to_owned(),
                "padding" => t.laws.padding = enum_field(value, key, "FilterPadding")?,
                "constant" => t.laws.constant = require_float(value, key)?,
                "pooling" => t.laws.pooling = enum_field(value, key, "OrientationPooling")?,
                "energy_distance" => t.laws.energy_distance = optional_integer(value, key, "laws_energy_distance must be a nonnegative integer or None")?,
            });
            t.laws.validate()?;
        }
        ImageType::Gabor => {
            overrides!(custom, name, |value, key| {
                "sigma" => t.gabor.sigma = require_float_tuple(value, key)?,
                "wavelength" => t.gabor.wavelength = require_float_tuple(value, key)?,
                "gamma" => t.gabor.gamma = require_float(value, key)?,
                "theta" => t.gabor.theta = require_float_tuple(value, key)?,
                "response" => t.gabor.response = enum_field(value, key, "GaborResponse")?,
                "pooling" => t.gabor.pooling = enum_field(value, key, "OrientationPooling")?,
                "slice_axis" => t.gabor.slice_axis = optional_integer(value, key, "gabor_slice_axis must be None or an array axis from zero to two")?,
                "padding" => t.gabor.padding = enum_field(value, key, "FilterPadding")?,
                "constant" => t.gabor.constant = require_float(value, key)?,
            });
            t.gabor.validate()?;
        }
        ImageType::Simoncelli => {
            overrides!(custom, name, |value, key| {
                "levels" => t.simoncelli.levels = integers(value, key, "simoncelli_levels must contain positive integers")?,
                "padding" => t.simoncelli.padding = enum_field(value, key, "FilterPadding")?,
                "grid" => t.simoncelli.grid = enum_field(value, key, "FourierGrid")?,
                "planar" => t.simoncelli.planar = require_bool(value, key)?,
            });
            t.simoncelli.validate()?;
        }
        ImageType::Riesz => {
            overrides!(custom, name, |value, key| {
                "base" => t.riesz.base = enum_field(value, key, "RieszBase")?,
                "orders" => t.riesz.orders = parse_orders(value, key)?,
                "log_sigma" => t.riesz.log_sigma = require_float_tuple(value, key)?,
                "log_cutoff" => t.riesz.log_cutoff = require_float(value, key)?,
                "simoncelli_levels" => t.riesz.simoncelli_levels = integers(value, key, "riesz_simoncelli_levels must contain positive integers")?,
                "padding" => t.riesz.padding = enum_field(value, key, "FilterPadding")?,
                "constant" => t.riesz.constant = require_float(value, key)?,
                "grid" => t.riesz.grid = enum_field(value, key, "FourierGrid")?,
                "steering" => t.riesz.steering = require_bool(value, key)?,
                "tensor_sigma" => t.riesz.tensor_sigma = Some(require_float(value, key)?),
            });
            t.riesz.validate()?;
        }
        ImageType::SeparableWavelet => {
            overrides!(custom, name, |value, key| {
                "wavelet" => t.separable_wavelet.name = text_field(value, key)?.to_owned(),
                "bands" => t.separable_wavelet.bands = parse_bands(value, key)?,
                "level" => t.separable_wavelet.level = integer(value, key, "separable_wavelet_level must be a positive integer")?,
                "padding" => t.separable_wavelet.padding = enum_field(value, key, "FilterPadding")?,
                "constant" => t.separable_wavelet.constant = require_float(value, key)?,
                "pooling" => t.separable_wavelet.pooling = enum_field(value, key, "OrientationPooling")?,
                "planar" => t.separable_wavelet.planar = require_bool(value, key)?,
            });
            t.separable_wavelet.validate()?;
        }
        _ if !custom.is_empty() => {
            return invalid(format!("imageType.{name} overrides are unsupported"));
        }
        _ => {}
    }
    Ok(())
}

fn transforms(params: &Mapping) -> Result<TransformSettings> {
    let mut t = TransformSettings::default();
    let Some(raw) = params
        .get("imageType")
        .filter(|v| !matches!(v, Value::Null))
    else {
        return Ok(t);
    };
    let image_types = require_mapping(raw, "imageType")?;
    if image_types.is_empty() {
        return invalid("imageType must enable at least one image type");
    }
    t.image_types.clear();
    let empty = Mapping::new();
    let mut overrides = std::collections::HashMap::new();
    for (name, raw) in entries(image_types) {
        let (_, kind) = IMAGE_TYPES
            .iter()
            .find(|(n, _)| name == *n)
            .ok_or_else(|| Error::InvalidInput(format!("unsupported image type: {name}")))?;
        let custom = if matches!(raw, Value::Null) {
            &empty
        } else {
            require_mapping(raw, &format!("imageType.{name}"))?
        };
        t.image_types.push(*kind);
        overrides.insert(*kind, custom);
    }
    for kind in t.image_types.clone() {
        let name = IMAGE_TYPES.iter().find(|(_, k)| kind == *k).unwrap().0;
        apply_overrides(&mut t, kind, name, overrides[&kind])?;
    }
    t.validate()?;
    Ok(t)
}

/// Parse all three parameter sections and validate them for a 2-D or 3-D image.
pub fn parse_params(params: &Value, ndim: usize) -> Result<ExtractionParameters> {
    if !matches!(ndim, 2 | 3) {
        return invalid("parameter files require a 2D or 3D image");
    }
    let params = require_mapping(params, "params")?;
    reject_unknown(
        params,
        &["setting", "featureClass", "imageType"],
        "unsupported params sections",
    )?;
    let empty = Mapping::new();
    let setting = section(params, "setting", &empty)?;
    let mut transforms = transforms(params)?;
    with_gradient_and_lbp(&mut transforms, setting)?;
    if ndim != 3 && transforms.image_types.contains(&ImageType::Lbp3D) {
        return invalid("LBP3D requires a 3D image");
    }
    for &kind in &transforms.image_types {
        validate_filter_shape(ndim, &transforms, kind)?;
    }
    let settings = feature_settings(setting, params.get("featureClass"), None, true)?;
    settings.validate(ndim)?;
    Ok(ExtractionParameters {
        settings,
        image_settings: image_settings(setting, ndim)?,
        transforms,
    })
}

/// Overlay feature settings and an explicit feature selection on an optional base.
/// Values outside the existing enum or integer field types fail at conversion.
/// Dimension-dependent checks run later in [`Settings::validate`].
pub fn settings_from_params(params: &Value, base: Option<&Settings>) -> Result<Settings> {
    let params = require_mapping(params, "params")?;
    let empty = Mapping::new();
    feature_settings(
        section(params, "setting", &empty)?,
        params.get("featureClass"),
        base,
        false,
    )
}

fn selection(value: &Value) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for (family, selection) in entries(require_mapping(value, "featureClass")?) {
        let list = sequence(selection).ok_or_else(|| {
            Error::InvalidInput(format!(
                "featureClass '{family}' must list feature names explicitly for this runner"
            ))
        })?;
        for name in list {
            let name = as_text(name).ok_or_else(|| {
                Error::InvalidInput(format!("featureClass '{family}' entries must be strings"))
            })?;
            names.push(format!("{family}_{name}"));
        }
    }
    if names.is_empty() {
        return invalid("featureClass must select at least one feature");
    }
    Ok(names)
}

fn parse_aggregation<T: FromStr>(value: &Value, key: &str, family: &str) -> Result<T> {
    let text = text_option(value, key)?;
    text.parse()
        .map_err(|_| Error::InvalidInput(format!("unsupported {family} aggregation: {text}")))
}

fn feature_settings(
    setting: &Mapping,
    feature_class: Option<&Value>,
    base: Option<&Settings>,
    advanced: bool,
) -> Result<Settings> {
    const KNOWN: &[&str] = &[
        "binWidth",
        "binCount",
        "gldm_a",
        "normalize",
        "normalizeScale",
        "removeOutliers",
        "resegmentRange",
        "resegmentMode",
        "resegmentShape",
        "label",
        "distances",
        "symmetricalGLCM",
        "maskedKernel",
        "kernelRadius",
        "voxelArrayShift",
        "force2D",
        "force2Ddimension",
        "binAnchor",
        "resegmentOutlierSigma",
        "covarianceDivisor",
        "ellipsoidArea",
        "kurtosis",
        "zeroVarianceMoments",
        "ngtdmSingularCases",
        "neighborhoodSupport",
        "binOrigin",
        "constantRoiBin",
        "maskExtrapolation",
        "splineBoundary",
        "sliceAxis",
        "neighborhoodRadius",
        "glcmAggregation",
        "glrlmAggregation",
        "glszmAggregation",
        "gldzmAggregation",
        "ngtdmAggregation",
        "gldmAggregation",
        "ivhBinWidth",
        "ivhBinCount",
        "ivhIntensityRange",
    ];
    let unknown: Vec<_> = entries(setting)
        .map(|(key, _)| key)
        .filter(|key| !(KNOWN.contains(key) || advanced && ADVANCED.contains(key)))
        .collect();
    if !unknown.is_empty() {
        return invalid(format!(
            "unsupported setting keys for this runner: {}",
            sorted_names(unknown)
        ));
    }
    let mut s = base.cloned().unwrap_or_default();
    read_fields!(setting, |value, key| {
        "binWidth" => s.bin_width = require_float(value, key)?,
        "binCount" => s.bin_count = Some(integer(value, key, "bin_count must be a positive integer below 2**31")?),
        "gldm_a" => s.gldm_alpha = integer(value, key, "gldm_alpha must be a nonnegative integer")?,
        "normalize" => s.normalize = require_bool(value, key)?,
        "normalizeScale" => s.normalize_scale = require_float(value, key)?,
        "removeOutliers" => s.remove_outliers = Some(require_float(value, key)?),
        "resegmentRange" => s.resegment_range = Some(require_float_tuple(value, key)?),
        "resegmentMode" => s.resegment_mode = setting_choice(value, key, "absolute, relative, sigma")?,
        "resegmentShape" => s.resegment_shape = require_bool(value, key)?,
        "label" => s.label = require_int(value, key)?,
        "distances" => s.distances = integers(value, key, "distances must contain positive integers")?,
        "symmetricalGLCM" => s.symmetrical_glcm = require_bool(value, key)?,
        "maskedKernel" => s.masked_kernel = require_bool(value, key)?,
        "kernelRadius" => s.kernel_radius = integer(value, key, "kernel_radius must be a positive integer")?,
        "voxelArrayShift" => s.voxel_array_shift = require_float(value, key)?,
    });
    if let Some(raw) = setting.get("force2D") {
        s.force2d_axis = if require_bool(raw, "force2D")? {
            Some(match setting.get("force2Ddimension") {
                Some(axis) => integer(
                    axis,
                    "force2Ddimension",
                    "force2d_axis must be an integer indexing a NumPy image axis",
                )?,
                None => 0,
            })
        } else {
            None
        };
    }
    if let Some(value) = feature_class.filter(|v| !matches!(v, Value::Null)) {
        s.features = selection(value)?;
    }
    if let Some(value) = setting.get("ivhIntensityRange") {
        let bounds = require_float_tuple(value, "ivhIntensityRange")?;
        let [lo, hi] = bounds.as_slice() else {
            return invalid("ivhIntensityRange must contain two bounds");
        };
        s.ivh_intensity_range = Some((*lo, *hi));
    }
    read_fields!(setting, |value, key| {
        "binAnchor" => s.bin_anchor = Some(require_float(value, key)?),
        "resegmentOutlierSigma" => s.resegment_outlier_sigma = Some(require_float(value, key)?),
    });
    macro_rules! convention {
        ($key:literal, $field:ident, $ty:ty) => {
            if let Some(raw) = setting.get($key) {
                s.$field = text_option(raw, $key)?.parse().map_err(|_| {
                    let choices: Vec<_> = <$ty>::ALL.iter().map(|v| v.as_str()).collect();
                    Error::InvalidInput(format!(
                        "{} must be one of {}",
                        stringify!($field),
                        choices.join(", ")
                    ))
                })?;
            }
        };
    }
    convention!("covarianceDivisor", covariance_divisor, CovarianceDivisor);
    convention!("ellipsoidArea", ellipsoid_area, EllipsoidArea);
    convention!("kurtosis", kurtosis, Kurtosis);
    convention!(
        "zeroVarianceMoments",
        zero_variance_moments,
        ZeroVarianceMoments
    );
    convention!(
        "ngtdmSingularCases",
        ngtdm_singular_cases,
        NgtdmSingularCases
    );
    convention!(
        "neighborhoodSupport",
        neighborhood_support,
        NeighborhoodSupport
    );
    convention!("binOrigin", bin_origin, BinOrigin);
    convention!("constantRoiBin", constant_roi_bin, ConstantRoiBin);
    convention!("maskExtrapolation", mask_extrapolation, MaskExtrapolation);
    convention!("splineBoundary", spline_boundary, SplineBoundary);
    read_fields!(setting, |value, key| {
        "sliceAxis" => s.slice_axis = integer(value, key, "slice_axis must be an integer indexing an image axis")?,
        "neighborhoodRadius" => s.neighborhood_radius = integer(value, key, "neighborhood_radius must be a positive integer")?,
        "glcmAggregation" => s.glcm_aggregation = parse_aggregation(value, key, "glcm")?,
        "glrlmAggregation" => s.glrlm_aggregation = parse_aggregation(value, key, "glrlm")?,
        "glszmAggregation" => s.glszm_aggregation = parse_aggregation(value, key, "glszm")?,
        "gldzmAggregation" => s.gldzm_aggregation = parse_aggregation(value, key, "gldzm")?,
        "ngtdmAggregation" => s.ngtdm_aggregation = parse_aggregation(value, key, "ngtdm")?,
        "gldmAggregation" => s.gldm_aggregation = parse_aggregation(value, key, "gldm")?,
        "ivhBinWidth" => s.ivh_bin_width = Some(require_float(value, key)?),
        "ivhBinCount" => s.ivh_bin_count = Some(integer(value, key, "ivh_bin_count must be a positive integer")?),
    });
    Ok(s)
}

// Saphyr handles YAML syntax, locations and anchor expansion. Keep scalar style until
// resolution because PyYAML safe_load applies its legacy schema only to plain scalars.
// The ordered Serde value is shared with callers that construct parameters directly.
fn load_yaml(text: &str) -> std::result::Result<Value, String> {
    use saphyr::{MarkedYaml, YamlLoader};
    let mut loader = YamlLoader::<MarkedYaml>::default();
    loader.early_parse(false);
    loader.allow_duplicate_keys(true);
    let mut parser = saphyr_parser::Parser::new_from_str(text);
    parser
        .load(&mut loader, true)
        .map_err(|err| yaml_location(&err, text))?;
    if let Some(err) = loader.error() {
        return Err(yaml_location(err, text));
    }
    let mut docs = loader.into_documents().into_iter();
    let first = docs.next();
    if docs.next().is_some() {
        return Err("expected a single document in the stream".into());
    }
    match first {
        Some(node) => resolve_node(node),
        None => Ok(Value::Null),
    }
}

fn yaml_location(error: &saphyr::ScanError, source: &str) -> String {
    let mark = error.marker();
    let end_line = source.bytes().filter(|&b| b == b'\n').count() + 1;
    // The scanner may synthesize a final newline on an unterminated line.
    let (line, column) = if mark.line() > end_line {
        (
            end_line,
            source.rsplit('\n').next().unwrap_or("").chars().count() + 1,
        )
    } else {
        (mark.line(), mark.col() + 1)
    };
    format!("{} at line {line}, column {column}", error.info())
}

fn resolve_node(node: saphyr::MarkedYaml<'_>) -> std::result::Result<Value, String> {
    use saphyr::{ScalarStyle, YamlData as Yaml};
    match node.data {
        Yaml::Representation(s, style, tag) => {
            if let Some(tag) = tag {
                if !tag.is_yaml_core_schema() {
                    return Err(format!(
                        "could not determine a constructor for the tag '{tag}'"
                    ));
                }
                return match tag.suffix.as_str() {
                    "str" => Ok(Value::String(s.into_owned())),
                    "timestamp" | "binary" => Ok(typed_scalar(&tag.suffix, &s)),
                    "null" => Ok(Value::Null),
                    "bool" => yaml_bool(&s.to_ascii_lowercase())
                        .map(Value::Bool)
                        .ok_or_else(|| format!("invalid boolean: {s}")),
                    "float" => yaml_float(&s)
                        .map(Value::from)
                        .ok_or_else(|| format!("invalid float: {s}")),
                    "int" => yaml_integer(&s).ok_or_else(|| format!("invalid integer: {s}")),
                    _ => Err(format!("unsupported parameter value tag: {tag}")),
                };
            }
            if style == ScalarStyle::Plain {
                resolve_plain(&s)
            } else {
                Ok(Value::String(s.into_owned()))
            }
        }
        Yaml::Sequence(seq) => seq
            .into_iter()
            .map(resolve_node)
            .collect::<std::result::Result<_, _>>()
            .map(Value::Sequence),
        Yaml::Mapping(entries) => {
            let mut merged = Mapping::new();
            let mut explicit = Vec::new();
            for (key, raw) in entries {
                let value = resolve_node(raw)?;
                let is_merge = matches!(&key.data, Yaml::Representation(s, ScalarStyle::Plain, None) if s == "<<")
                    || matches!(&key.data, Yaml::Representation(_, _, Some(tag)) if tag.is_yaml_core_schema() && tag.suffix == "merge");
                if is_merge {
                    let maps = match value {
                        Value::Sequence(maps) => maps,
                        map => vec![map],
                    };
                    // Earlier maps override later ones. Explicit keys override all merges.
                    for map in maps.into_iter().rev() {
                        let Value::Mapping(map) = map else {
                            return Err("expected a mapping for merging".into());
                        };
                        merged.extend(map);
                    }
                } else {
                    explicit.push((key.span.start.index(), resolve_node(key)?, value));
                }
            }
            // PyYAML retains a duplicate key's first position and last value. Saphyr
            // iterates overwritten entries at their last position but keeps the first key span.
            let mut positions: Vec<_> = explicit.iter().map(|(pos, key, _)| (*pos, key)).collect();
            positions.sort_unstable_by_key(|(pos, _)| *pos);
            for (_, key) in positions {
                merged.entry(key.clone()).or_insert(Value::Null);
            }
            for (_, key, value) in explicit {
                merged.insert(key, value);
            }
            Ok(Value::Mapping(merged))
        }
        Yaml::Tagged(tag, _) => Err(format!(
            "could not determine a constructor for the tag '{tag}'"
        )),
        _ => Err("invalid YAML node".into()),
    }
}

fn yaml_bool(text: &str) -> Option<bool> {
    match text {
        "true" | "True" | "TRUE" | "yes" | "Yes" | "YES" | "on" | "On" | "ON" => Some(true),
        "false" | "False" | "FALSE" | "no" | "No" | "NO" | "off" | "Off" | "OFF" => Some(false),
        _ => None,
    }
}

// These safe_load types have no corresponding radiomics setting. Preserve the tag
// so typed accessors reject them, including in mapping keys and feature lists.
fn typed_scalar(kind: &str, text: &str) -> Value {
    use yaml_serde::value::{Tag, TaggedValue};
    Value::Tagged(Box::new(TaggedValue {
        tag: Tag::new(kind),
        value: Value::String(text.into()),
    }))
}

fn resolve_plain(text: &str) -> std::result::Result<Value, String> {
    use regex::Regex;
    use std::sync::LazyLock;
    // Keep YAML 1.1 integer bases and sexagesimal groups distinct from decimal
    // fractions. Plain exponents without a decimal point remain strings.
    static INTEGER: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?x) ^ [+-]? (?:
            0 | [1-9][0-9_]* | 0[0-7_]+ | 0b[01_]+ | 0x[0-9a-fA-F_]+
            | [1-9][0-9_]*(?::[0-5]?[0-9])+
        ) $",
        )
        .unwrap()
    });
    static FLOAT: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?x) ^ (?:
            (?: [+-]?[0-9][0-9_]*\.[0-9_]* | \.[0-9][0-9_]* ) (?:[eE][+-][0-9]+)?
            | [+-]?[0-9][0-9_]*(?::[0-5]?[0-9])+\.[0-9_]*
            | [+-]?\.(?:inf|Inf|INF) | \.(?:nan|NaN|NAN)
        ) $",
        )
        .unwrap()
    });
    static TIMESTAMP: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?x) ^ [0-9]{4}-
            (?:[0-9]{2}-[0-9]{2} | [0-9]{1,2}-[0-9]{1,2}(?:[Tt]|[[:blank:]]+)
            [0-9]{1,2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]*)?
            (?:[[:blank:]]*(?:Z|[+-][0-9]{1,2}(?::[0-9]{2})?))?) $
        ",
        )
        .unwrap()
    });
    if matches!(text, "" | "~" | "null" | "Null" | "NULL") {
        return Ok(Value::Null);
    }
    if let Some(b) = yaml_bool(text) {
        return Ok(Value::Bool(b));
    }
    if INTEGER.is_match(text) {
        return yaml_integer(text)
            .ok_or_else(|| format!("integer outside supported range: {text}"));
    }
    if FLOAT.is_match(text) {
        return yaml_float(text)
            .map(Value::from)
            .ok_or_else(|| format!("invalid float: {text}"));
    }
    if TIMESTAMP.is_match(text) {
        return Ok(typed_scalar("timestamp", text));
    }
    Ok(Value::String(text.into()))
}

fn yaml_integer(text: &str) -> Option<Value> {
    let cleaned = text.replace('_', "");
    let unsigned = cleaned.strip_prefix(['+', '-']).unwrap_or(&cleaned);
    let magnitude = if let Some(s) = unsigned.strip_prefix("0x") {
        u64::from_str_radix(s, 16).ok()?
    } else if let Some(s) = unsigned.strip_prefix("0b") {
        u64::from_str_radix(s, 2).ok()?
    } else if unsigned.contains(':') {
        unsigned.split(':').try_fold(0u64, |n, part| {
            n.checked_mul(60)?.checked_add(part.parse::<u64>().ok()?)
        })?
    } else {
        u64::from_str_radix(unsigned, if unsigned.starts_with('0') { 8 } else { 10 }).ok()?
    };
    if cleaned.starts_with('-') {
        Some(Value::from(i64::try_from(-i128::from(magnitude)).ok()?))
    } else {
        Some(Value::from(magnitude))
    }
}

fn yaml_float(text: &str) -> Option<f64> {
    let cleaned = text.replace('_', "").to_ascii_lowercase();
    match cleaned.as_str() {
        ".nan" => Some(f64::NAN),
        ".inf" | "+.inf" => Some(f64::INFINITY),
        "-.inf" => Some(f64::NEG_INFINITY),
        _ if cleaned.contains(':') => {
            let magnitude = cleaned
                .trim_start_matches(['+', '-'])
                .split(':')
                .try_fold(0.0, |n, part| Some(n * 60.0 + part.parse::<f64>().ok()?))?;
            Some(if cleaned.starts_with('-') {
                -magnitude
            } else {
                magnitude
            })
        }
        _ => cleaned.parse().ok(),
    }
}
