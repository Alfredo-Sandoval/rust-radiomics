//! Derived feature images (PyRadiomics image types) and their names.
//!
//! Mirrors `mlxradiomics/imaging/transforms.py` (771 lines at mlx-radiomics b58783c).
//! Status: done. Ported: square, square root, logarithm, exponential, gradient, the
//! undecimated PyRadiomics wavelet (periodic a trous, level and start level), recursive
//! and sampled LoG, mean, Laws (+energy, pooling), Gabor, separable wavelets, LBP2D,
//! LBP3D, Simoncelli and Riesz (with steering), the exact derived-image naming rules and
//! duplicate-name rejection. Not ported: the Python `window` crop and the
//! `source_residual`/`source_tail` float32 recovery arguments (the CPU path is float64
//! end to end); `_require_available_image_types`, because the CPU FFT has no MLX FFT gate.
//!
//! Execution: CPU numerical work in float64, no GPU. This module is orchestration plus
//! the wavelet, gradient and pointwise transforms, which run on rayon worker threads one
//! row per task; each filter's own module states its threading. Input contract: 2-D or 3-D [`ArrayD`] of finite
//! intensities in NumPy (z, y, x) order, spacing in mm per axis.
//!
//! `gabor_scale_pairs` and `validate_filter_shape` come from `config::transforms`.

use crate::config::filtering::{
    FilterPadding, FourierGrid, LogKernel, OrientationPooling, RieszBase,
};
use crate::config::transforms::{ImageType, TransformSettings};
use crate::config::transforms::{gabor_scale_pairs, validate_filter_shape};
use crate::config::wavelets::wavelet_filters;
use crate::error::{Error, Result};
use crate::filters::convolution::{
    gabor_responses, laws_energy, laws_response, mean_response, separable_wavelet_responses,
};
use crate::filters::kernels::copy_flat;
use crate::filters::lbp2d::lbp2d;
use crate::filters::lbp3d::{SourceDtype, lbp3d_images};
use crate::filters::recursive_gaussian::{laplacian_gaussian, validate_log_domain};
use crate::filters::riesz_steering::{align_riesz, riesz_multi_indices};
use crate::filters::spectral::{
    log_response, require_isotropic_spacing, riesz_log_responses, riesz_simoncelli_responses,
    simoncelli_response,
};
use ndarray::{ArrayD, IxDyn};
use rayon::prelude::*;
use std::collections::{BTreeSet, HashMap};

/// A derived image: the feature-key prefix and its float64 array.
pub type FeatureImage = (String, ArrayD<f64>);

/// Python `repr(float)`: the shortest round-trip decimal in Python's layout.
pub fn python_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".into();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.into();
    }
    if value == 0.0 {
        return if value.is_sign_negative() {
            "-0.0"
        } else {
            "0.0"
        }
        .into();
    }
    let scientific = format!("{:e}", value.abs());
    let (mantissa, exponent) = scientific.split_once('e').expect("exponent present");
    let exponent: i32 = exponent.parse().expect("integer exponent");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let sign = if value < 0.0 { "-" } else { "" };
    if (-4..16).contains(&exponent) {
        if exponent >= 0 {
            let split = exponent as usize + 1;
            let (whole, fraction) = if digits.len() > split {
                (digits[..split].to_string(), digits[split..].to_string())
            } else {
                (
                    format!("{digits}{}", "0".repeat(split - digits.len())),
                    "0".into(),
                )
            };
            format!("{sign}{whole}.{fraction}")
        } else {
            format!("{sign}0.{}{digits}", "0".repeat((-exponent - 1) as usize))
        }
    } else {
        let mantissa = if digits.len() > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            digits
        };
        let exponent_sign = if exponent < 0 { '-' } else { '+' };
        format!("{sign}{mantissa}e{exponent_sign}{:02}", exponent.abs())
    }
}

/// Lossless identifier tag for one binary64 parameter value.
///
/// `repr` is the shortest decimal that round-trips, so distinct parameters keep distinct
/// names. Whole floats drop the trailing `.0` (`s1mm`); a leading sign becomes `m`, the
/// decimal point `p`, and an exponent's `+` is dropped.
pub fn tag(value: f64) -> String {
    let mut text = python_float_repr(value);
    if let Some(stripped) = text.strip_suffix(".0") {
        text = stripped.to_string();
    }
    text.replace('-', "m").replace('.', "p").replace('+', "")
}

/// Name tag for a padding mode, including the constant value.
fn padding_tag(padding: FilterPadding, constant: f64) -> String {
    if padding == FilterPadding::Constant {
        format!("constant{}", tag(constant))
    } else {
        padding.as_str().to_string()
    }
}

/// Wavelet axis codes in output order (approximation first).
fn wavelet_codes(ndim: usize, axis2d: Option<usize>) -> Vec<String> {
    let mut codes = vec![String::new()];
    for axis in (0..ndim).rev() {
        if Some(axis) != axis2d {
            codes = codes
                .iter()
                .flat_map(|name| ["L", "H"].map(|suffix| format!("{name}{suffix}")))
                .collect();
        }
    }
    codes
}

fn wavelet_prefix(output_level: u32) -> String {
    if output_level == 1 {
        "wavelet".into()
    } else {
        format!("wavelet{output_level}")
    }
}

fn log_name(sigma: f64, options: &TransformSettings) -> String {
    if options.log.kernel == LogKernel::Recursive {
        return format!(
            "log-sigma-{}-mm-3D",
            python_float_repr(sigma).replace('.', "-")
        );
    }
    [
        "log-sampled".to_string(),
        format!("s{}mm", tag(sigma)),
        format!("c{}", tag(options.log.cutoff)),
        padding_tag(options.log.padding, options.log.constant),
        if options.log.planar { "2D" } else { "3D" }.to_string(),
    ]
    .join("-")
}

fn mean_name(options: &TransformSettings) -> String {
    let name = format!(
        "mean-M{}-{}",
        options.mean.support,
        padding_tag(options.mean.padding, options.mean.constant)
    );
    if options.mean.planar {
        format!("{name}-2D")
    } else {
        name
    }
}

fn laws_name(options: &TransformSettings) -> String {
    let mut parts = vec![
        "laws".to_string(),
        options.laws.kernel.clone(),
        padding_tag(options.laws.padding, options.laws.constant),
    ];
    if options.laws.pooling != OrientationPooling::None {
        parts.push("pool".into());
        parts.push(options.laws.pooling.as_str().into());
    }
    if let Some(distance) = options.laws.energy_distance {
        parts.push("energy".into());
        parts.push(format!("d{distance}"));
    }
    parts.join("-")
}

/// Gabor plane tag: empty in 2-D, else `planes` or `axis<k>`.
fn gabor_plane_tag(options: &TransformSettings, ndim: usize) -> String {
    if ndim == 2 {
        return String::new();
    }
    match options.gabor.slice_axis {
        None => "planes".into(),
        Some(axis) => format!("axis{axis}"),
    }
}

fn gabor_base_name(
    options: &TransformSettings,
    sigma: f64,
    wavelength: f64,
    thetas: &[f64],
) -> String {
    let mut parts = vec![
        "gabor".to_string(),
        format!("s{}mm", tag(sigma)),
        format!("l{}mm", tag(wavelength)),
        format!("g{}", tag(options.gabor.gamma)),
    ];
    if options.gabor.pooling == OrientationPooling::None {
        // One lossless angle tag per unpooled orientation map.
        parts.push(format!("t{}", tag(thetas[0])));
    } else {
        // Pooled maps carry only the orientation count (file-name length limit).
        parts.push(format!("o{}", thetas.len()));
    }
    parts.push(options.gabor.response.as_str().into());
    parts.push(padding_tag(options.gabor.padding, options.gabor.constant));
    if options.gabor.pooling != OrientationPooling::None {
        parts.push("pool".into());
        parts.push(options.gabor.pooling.as_str().into());
    }
    parts.join("-")
}

fn with_plane(base: String, plane: &str) -> String {
    if plane.is_empty() {
        base
    } else {
        format!("{base}-{plane}")
    }
}

fn gabor_names(options: &TransformSettings, ndim: usize) -> Result<Vec<String>> {
    let plane = gabor_plane_tag(options, ndim);
    let mut names = Vec::new();
    for (sigma, wavelength) in gabor_scale_pairs(options)? {
        if options.gabor.pooling == OrientationPooling::None {
            for &theta in &options.gabor.theta {
                names.push(with_plane(
                    gabor_base_name(options, sigma, wavelength, &[theta]),
                    &plane,
                ));
            }
        } else {
            names.push(with_plane(
                gabor_base_name(options, sigma, wavelength, &options.gabor.theta),
                &plane,
            ));
        }
    }
    Ok(names)
}

fn riesz_order_tag(order: &[u32]) -> String {
    format!(
        "l{}",
        order
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join("-")
    )
}

/// Name of one Riesz component; `scale` is the LoG sigma (mm) or the Simoncelli level.
fn riesz_name(options: &TransformSettings, order: &[u32], scale: f64) -> String {
    let mut parts = vec!["riesz".to_string(), options.riesz.base.as_str().to_string()];
    if options.riesz.base == RieszBase::Log {
        parts.push(format!("s{}mm", tag(scale)));
        parts.push(format!("c{}", tag(options.riesz.log_cutoff)));
    } else {
        parts.push(format!("L{}", scale as i64));
    }
    parts.push(riesz_order_tag(order));
    if options.riesz.base == RieszBase::Log {
        parts.push(padding_tag(options.riesz.padding, options.riesz.constant));
    }
    if options.riesz.grid != FourierGrid::Dft {
        parts.push(options.riesz.grid.as_str().into());
    }
    if let Some(sigma) = options.riesz.tensor_sigma {
        parts.push("aligned".into());
        parts.push(format!("t{}mm", tag(sigma)));
    }
    parts.join("-")
}

/// Riesz orders to filter: the requested ones, or the steering banks.
///
/// Steering mixes every component of one total order, so it needs the complete order-`L`
/// bank of each requested `L` plus the first-order bank that builds the structure tensor.
fn riesz_computed_orders(options: &TransformSettings, ndim: usize) -> Result<Vec<Vec<u32>>> {
    if !options.riesz.steering {
        return Ok(options.riesz.orders.clone());
    }
    let mut orders = riesz_multi_indices(ndim, 1)?;
    let lengths: BTreeSet<u32> = options
        .riesz
        .orders
        .iter()
        .map(|o| o.iter().sum())
        .collect();
    for length in lengths {
        orders.extend(riesz_multi_indices(ndim, length)?);
    }
    let mut seen = BTreeSet::new();
    Ok(orders
        .into_iter()
        .filter(|o| seen.insert(o.clone()))
        .collect())
}

/// Steer the computed banks if requested and return maps in `riesz_orders` order.
fn riesz_requested(
    options: &TransformSettings,
    computed: &[Vec<u32>],
    responses: Vec<ArrayD<f64>>,
    spacing: &[f64],
) -> Result<Vec<ArrayD<f64>>> {
    let Some(tensor_sigma) = options.riesz.tensor_sigma else {
        return Ok(responses);
    };
    let maps: HashMap<&Vec<u32>, &ArrayD<f64>> = computed.iter().zip(responses.iter()).collect();
    let ndim = spacing.len();
    let first_order: Vec<ArrayD<f64>> = riesz_multi_indices(ndim, 1)?
        .iter()
        .map(|order| maps[order].clone())
        .collect();
    let mut aligned: HashMap<Vec<u32>, ArrayD<f64>> = HashMap::new();
    let lengths: BTreeSet<u32> = options
        .riesz
        .orders
        .iter()
        .map(|o| o.iter().sum())
        .collect();
    for length in lengths {
        let bank = riesz_multi_indices(ndim, length)?;
        let components: Vec<ArrayD<f64>> = bank.iter().map(|order| maps[order].clone()).collect();
        let steered = align_riesz(
            &components,
            &first_order,
            length,
            spacing,
            tensor_sigma,
            options.riesz.padding,
        )?;
        aligned.extend(bank.into_iter().zip(steered));
    }
    Ok(options
        .riesz
        .orders
        .iter()
        .map(|o| aligned[o].clone())
        .collect())
}

fn separable_wavelet_name(options: &TransformSettings, band: &str) -> String {
    let w = &options.separable_wavelet;
    let mut parts = vec![
        "separablewavelet".to_string(),
        w.name.clone(),
        format!("L{}", w.level),
        band.to_string(),
        padding_tag(w.padding, w.constant),
    ];
    if w.pooling != OrientationPooling::None {
        parts.push("pool".into());
        parts.push(w.pooling.as_str().into());
    }
    if w.planar {
        parts.push("2D".into());
    }
    parts.join("-")
}

fn simoncelli_name(options: &TransformSettings, level: u32) -> String {
    let mut name = format!("simoncelli-L{level}");
    if options.simoncelli.grid != FourierGrid::Dft {
        name = format!("{name}-{}", options.simoncelli.grid.as_str());
    }
    if options.simoncelli.planar {
        format!("{name}-2D")
    } else {
        name
    }
}

/// Reject repeated derived-image names before any image is produced.
fn require_unique_names(names: Vec<String>) -> Result<Vec<String>> {
    let mut seen = BTreeSet::new();
    let mut repeated = BTreeSet::new();
    for name in &names {
        if !seen.insert(name.clone()) {
            repeated.insert(name.clone());
        }
    }
    if repeated.is_empty() {
        Ok(names)
    } else {
        Err(Error::InvalidInput(format!(
            "derived image names must be unique; repeated: {}",
            repeated.into_iter().collect::<Vec<_>>().join(", ")
        )))
    }
}

/// The derived-image prefixes [`feature_images`] will yield, without computing.
///
/// Order and spelling match the image names exactly, which lets callers size streaming
/// storage before extraction starts.
///
/// Errors (`ValueError` in Python): inconsistent shape or spacing, a 2-D axis outside the
/// image, LBP3D for non-3-D data, a filter's axis count not matching the image
/// dimensionality, a LoG grid or sigma the filter rejects, or two selected
/// configurations generating the same name.
pub fn feature_image_names(
    shape: &[usize],
    spacing: &[f64],
    options: &TransformSettings,
    force2d_axis: Option<usize>,
) -> Result<Vec<String>> {
    let ndim = shape.len();
    if !(2..=3).contains(&ndim) || spacing.len() != ndim {
        return Err(Error::InvalidInput(
            "feature image names require aligned 2D/3D spacing".into(),
        ));
    }
    if force2d_axis.is_some_and(|axis| axis >= ndim) {
        return Err(Error::InvalidInput(
            "force2d_axis must index a NumPy image axis".into(),
        ));
    }
    let mut names: Vec<String> = Vec::new();
    for &kind in &options.image_types {
        match kind {
            ImageType::Lbp3D => {
                if ndim != 3 {
                    return Err(Error::InvalidInput("LBP3D requires a 3D image".into()));
                }
                names.extend((1..=options.lbp3d.levels).map(|level| format!("lbp-3D-m{level}")));
                names.push("lbp-3D-k".into());
            }
            ImageType::Wavelet => {
                let codes = wavelet_codes(ndim, force2d_axis);
                for level in 1..=options.wavelet.level {
                    let prefix = wavelet_prefix(level);
                    names.extend(codes[1..].iter().map(|code| format!("{prefix}-{code}")));
                }
                // Details at every level and only the final approximation.
                names.push(format!(
                    "{}-{}",
                    wavelet_prefix(options.wavelet.level),
                    codes[0]
                ));
            }
            ImageType::Log => {
                if options.log.planar && ndim != 3 {
                    return Err(Error::InvalidInput(
                        "planar LoG filtering requires a 3-D image".into(),
                    ));
                }
                // Fail before the Original image or any earlier derived image is extracted.
                if options.log.kernel == LogKernel::Sampled {
                    require_isotropic_spacing(if options.log.planar {
                        &spacing[1..]
                    } else {
                        spacing
                    })?;
                } else {
                    for &sigma in &options.log.sigma {
                        validate_log_domain(shape, spacing, sigma)?;
                    }
                }
                names.extend(options.log.sigma.iter().map(|&s| log_name(s, options)));
            }
            ImageType::Mean => {
                if options.mean.planar && ndim != 3 {
                    return Err(Error::InvalidInput(
                        "planar mean filtering requires a 3-D image".into(),
                    ));
                }
                names.push(mean_name(options));
            }
            ImageType::Laws => {
                validate_filter_shape(ndim, options, kind)?;
                names.push(laws_name(options));
            }
            ImageType::Gabor => {
                validate_filter_shape(ndim, options, kind)?;
                names.extend(gabor_names(options, ndim)?);
            }
            ImageType::Simoncelli => {
                if options.simoncelli.planar && ndim != 3 {
                    return Err(Error::InvalidInput(
                        "planar Simoncelli filtering requires a 3-D image".into(),
                    ));
                }
                names.extend(
                    options
                        .simoncelli
                        .levels
                        .iter()
                        .map(|&l| simoncelli_name(options, l)),
                );
            }
            ImageType::SeparableWavelet => {
                validate_filter_shape(ndim, options, kind)?;
                names.extend(
                    options
                        .separable_wavelet
                        .bands
                        .iter()
                        .map(|band| separable_wavelet_name(options, band)),
                );
            }
            ImageType::Riesz => {
                validate_filter_shape(ndim, options, kind)?;
                if options.riesz.base == RieszBase::Log {
                    require_isotropic_spacing(spacing)?;
                    for &sigma in &options.riesz.log_sigma {
                        for order in &options.riesz.orders {
                            names.push(riesz_name(options, order, sigma));
                        }
                    }
                } else {
                    for &level in &options.riesz.simoncelli_levels {
                        for order in &options.riesz.orders {
                            names.push(riesz_name(options, order, f64::from(level)));
                        }
                    }
                }
            }
            _ => names.push(kind.as_str().to_string()),
        }
    }
    require_unique_names(names)
}

/// Row-major buffer with explicit dimensions for the a trous wavelet.
struct Grid {
    dims: Vec<usize>,
    data: Vec<f64>,
}

impl Grid {
    fn layout(&self, axis: usize) -> (usize, usize, usize) {
        (
            self.dims[..axis].iter().product(),
            self.dims[axis],
            self.dims[axis + 1..].iter().product(),
        )
    }

    /// Periodic extension of `axis` to `length` by modular indexing.
    fn extend_periodic(&self, axis: usize, length: usize) -> Grid {
        let (outer, size, inner) = self.layout(axis);
        let mut data = Vec::with_capacity(outer * length * inner);
        for o in 0..outer {
            for j in 0..length {
                let base = (o * size + j % size) * inner;
                data.extend_from_slice(&self.data[base..base + inner]);
            }
        }
        let mut dims = self.dims.clone();
        dims[axis] = length;
        Grid { dims, data }
    }

    /// Periodic convolution along `axis` with taps `dilation` voxels apart.
    ///
    /// The stationary (a trous) wavelet convolution, with the phase of PyWavelets `swtn`
    /// with `norm=False`: `out[i] = sum_k c[k] * in[(i + (n // 2 - k) * dilation) mod size]`.
    fn periodic_filter<const ROUND_TO_F32: bool>(
        &self,
        coefficients: &[f64],
        axis: usize,
        dilation: usize,
    ) -> Grid {
        let (_, size, inner) = self.layout(axis);
        let half = (coefficients.len() / 2) as isize;
        let mut data = vec![0.0; self.data.len()];
        let tap = |value: &mut f64, coefficient: f64, sample: f64| {
            if ROUND_TO_F32 {
                // MLX rounds both the tap product and the running sum.
                *value = f64::from(*value as f32 + coefficient as f32 * sample as f32);
            } else {
                *value += coefficient * sample;
            }
        };
        if inner == 1 {
            // Along the last axis a task is one lane; each tap adds the lane shifted
            // circularly by `shift`, in two contiguous runs.
            data.par_chunks_mut(size)
                .zip(self.data.par_chunks(size))
                .for_each(|(out, lane)| {
                    for (k, &coefficient) in coefficients.iter().enumerate() {
                        let shift = ((half - k as isize) * dilation as isize)
                            .rem_euclid(size as isize) as usize;
                        let (head, tail) = out.split_at_mut(size - shift);
                        for (value, &sample) in head.iter_mut().zip(&lane[shift..]) {
                            tap(value, coefficient, sample);
                        }
                        for (value, &sample) in tail.iter_mut().zip(&lane[..shift]) {
                            tap(value, coefficient, sample);
                        }
                    }
                });
        } else {
            // Every output row (one position along `axis`) sums its taps in tap order.
            data.par_chunks_mut(inner)
                .enumerate()
                .for_each(|(row, out)| {
                    let (o, j) = (row / size, row % size);
                    for (k, &coefficient) in coefficients.iter().enumerate() {
                        let shift = (half - k as isize) * dilation as isize;
                        let source = (j as isize + shift).rem_euclid(size as isize) as usize;
                        let src = &self.data[(o * size + source) * inner..][..inner];
                        for (value, &sample) in out.iter_mut().zip(src) {
                            tap(value, coefficient, sample);
                        }
                    }
                });
        }
        Grid {
            dims: self.dims.clone(),
            data,
        }
    }

    fn crop(&self, shape: &[usize]) -> ArrayD<f64> {
        let mut view = ndarray::ArrayViewD::from_shape(IxDyn(&self.dims), &self.data)
            .expect("grid buffer matches dims");
        for (axis, &size) in shape.iter().enumerate() {
            view.slice_axis_inplace(ndarray::Axis(axis), ndarray::Slice::from(0..size));
        }
        view.as_standard_layout().into_owned()
    }
}

/// Stationary-wavelet detail images and the final approximation.
///
/// Each name is `wavelet-<codes>` for level 1 or `wavelet<level>-<codes>` for deeper
/// levels, with one code per filtered axis: `L` (low-pass) or `H` (high-pass), listed from
/// the last array axis to the first. With `axis2d` set that axis is not filtered and has no
/// code. The last image is the all-`L` approximation of the final level. Level `l` uses
/// filter dilation `2**(l-1)`, the standard a trous transform with unnormalised wavelet
/// filters; filtered axes are periodically extended to a multiple of
/// `2**(start_level + levels)` and cropped to the original grid. `start_level` discards
/// that many initial decompositions but keeps their approximation as input to the returned
/// levels.
fn wavelets<const ROUND_TO_F32: bool>(
    image: &ArrayD<f64>,
    options: &TransformSettings,
    axis2d: Option<usize>,
) -> Result<Vec<FeatureImage>> {
    let wavelet = wavelet_filters(&options.wavelet.name)?;
    let axes: Vec<usize> = (0..image.ndim())
        .rev()
        .filter(|&a| Some(a) != axis2d)
        .collect();
    let total_levels = options.wavelet.start_level + options.wavelet.level;
    let multiple = 1usize << total_levels;
    let shape = image.shape().to_vec();
    let mut approximation = Grid {
        dims: shape.clone(),
        data: copy_flat(image),
    };
    for &axis in &axes {
        let length = approximation.dims[axis];
        let padded = length.div_ceil(multiple) * multiple;
        if padded != length {
            approximation = approximation.extend_periodic(axis, padded);
        }
    }
    let mut outputs = Vec::new();
    for level in 0..total_levels {
        let mut decompositions: Vec<(String, Grid)> = vec![(
            String::new(),
            Grid {
                dims: approximation.dims.clone(),
                data: approximation.data.clone(),
            },
        )];
        for &axis in &axes {
            decompositions = decompositions
                .iter()
                .flat_map(|(name, values)| {
                    [("L", &wavelet.low), ("H", &wavelet.high)].map(|(suffix, taps)| {
                        (
                            format!("{name}{suffix}"),
                            values.periodic_filter::<ROUND_TO_F32>(taps, axis, 1 << level),
                        )
                    })
                })
                .collect();
        }
        let position = decompositions
            .iter()
            .position(|(name, _)| *name == "L".repeat(axes.len()))
            .expect("approximation code present");
        approximation = decompositions.remove(position).1;
        if level < options.wavelet.start_level {
            continue;
        }
        let prefix = wavelet_prefix(level - options.wavelet.start_level + 1);
        for (name, values) in decompositions {
            outputs.push((format!("{prefix}-{name}"), values.crop(&shape)));
        }
    }
    outputs.push((
        format!(
            "{}-{}",
            wavelet_prefix(options.wavelet.level),
            "L".repeat(axes.len())
        ),
        approximation.crop(&shape),
    ));
    Ok(outputs)
}

/// Gradient magnitude from central differences `(x[i+1] - x[i-1]) / 2` per axis.
///
/// Edge voxels are replicated at the boundary, so edge differences span one step but are
/// still halved. Each component divides by the voxel spacing when `use_spacing` is set. The
/// magnitude divides every component by the largest one before squaring and restores it
/// after the square root, as in mlx-radiomics.
fn gradient(image: &ArrayD<f64>, spacing: &[f64], use_spacing: bool) -> ArrayD<f64> {
    let ndim = image.ndim();
    let mut dims = [1usize; 3];
    for (axis, &size) in image.shape().iter().enumerate() {
        dims[axis + 3 - ndim] = size;
    }
    let mut step = [1.0; 3];
    if use_spacing {
        step[3 - ndim..].copy_from_slice(spacing);
    }
    let standard = image.as_standard_layout();
    let flat = standard.as_slice().expect("standard layout is contiguous");
    let [_, ny, nx] = dims;
    let mut out = vec![0.0; flat.len()];
    // One row of the last axis per task; each voxel reads its central-difference neighbours.
    out.par_chunks_mut(nx.max(1))
        .enumerate()
        .for_each(|(row, line)| {
            let (z, y) = (row / ny, row % ny);
            let base = row * nx;
            let up = base - (y - y.saturating_sub(1)) * nx;
            let down = base + ((y + 1).min(ny - 1) - y) * nx;
            let front = base - (z - z.saturating_sub(1)) * ny * nx;
            let back = base + ((z + 1).min(dims[0] - 1) - z) * ny * nx;
            for (x, slot) in line.iter_mut().enumerate() {
                let left = base + x.saturating_sub(1);
                let right = base + (x + 1).min(nx - 1);
                let components = [
                    (flat[back + x] - flat[front + x]) * 0.5 / step[0],
                    (flat[down + x] - flat[up + x]) * 0.5 / step[1],
                    (flat[right] - flat[left]) * 0.5 / step[2],
                ];
                // Only the filtered axes (the trailing `ndim`) contribute.
                let components = &components[3 - ndim..];
                let largest = components.iter().fold(0.0_f64, |l, c| l.max(c.abs()));
                let mut squared = 0.0;
                for &c in components {
                    let divisor = if largest > 0.0 && largest.is_finite() {
                        largest
                    } else {
                        1.0
                    };
                    squared += (c / divisor).powi(2);
                }
                *slot = largest * squared.sqrt();
            }
        });
    ArrayD::from_shape_vec(IxDyn(image.shape()), out).expect("shape")
}

/// Elementwise `f` of every sample, evaluated on rayon worker threads.
fn par_map(image: &ArrayD<f64>, f: impl Fn(f64) -> f64 + Sync) -> ArrayD<f64> {
    let standard = image.as_standard_layout();
    let flat = standard.as_slice().expect("standard layout is contiguous");
    let out: Vec<f64> = flat.par_iter().map(|&x| f(x)).collect();
    ArrayD::from_shape_vec(IxDyn(image.shape()), out).expect("shape")
}

/// Extraction uses float32 wavelet taps and sums to match the MLX feature images.
/// The standalone float64 transform keeps the full precision of its inputs.
pub(crate) fn compute_feature_images<const F32_WAVELET: bool>(
    image: &ArrayD<f64>,
    mask: Option<&ArrayD<bool>>,
    spacing: &[f64],
    options: &TransformSettings,
    force2d_axis: Option<usize>,
    source_dtype: SourceDtype,
) -> Result<Vec<FeatureImage>> {
    if !(2..=3).contains(&image.ndim()) || spacing.len() != image.ndim() {
        return Err(Error::InvalidInput(
            "feature images require aligned 2D/3D spacing".into(),
        ));
    }
    // Validate the complete name set before the first transform runs.
    feature_image_names(image.shape(), spacing, options, force2d_axis)?;
    let standard = image.as_standard_layout();
    let flat = standard.as_slice().expect("standard layout is contiguous");
    if flat.par_iter().any(|v| !v.is_finite()) {
        return Err(Error::InvalidInput(
            "feature image intensities must be finite".into(),
        ));
    }
    let ndim = image.ndim();
    let maximum = flat
        .par_iter()
        .fold(|| 0.0_f64, |m, v| m.max(v.abs()))
        .reduce(|| 0.0, f64::max);
    let mut outputs: Vec<FeatureImage> = Vec::new();
    for &kind in &options.image_types {
        match kind {
            ImageType::Lbp2D => {
                let values = lbp2d(
                    image,
                    options.lbp2d.samples,
                    options.lbp2d.radius,
                    options.lbp2d.method,
                    force2d_axis.unwrap_or(0),
                )?;
                outputs.push((kind.as_str().into(), values));
            }
            ImageType::Lbp3D => {
                let Some(mask) = mask else {
                    return Err(Error::InvalidInput(
                        "LBP3D requires an explicit label mask".into(),
                    ));
                };
                outputs.extend(lbp3d_images(image, mask, &options.lbp3d, source_dtype)?);
            }
            ImageType::Wavelet => {
                outputs.extend(wavelets::<F32_WAVELET>(image, options, force2d_axis)?);
            }
            ImageType::Log => {
                for &sigma in &options.log.sigma {
                    let values = if options.log.kernel == LogKernel::Sampled {
                        log_response(
                            image,
                            spacing,
                            sigma,
                            options.log.cutoff,
                            options.log.planar,
                            options.log.padding,
                            options.log.constant,
                        )?
                    } else {
                        laplacian_gaussian(image, spacing, sigma)?
                    };
                    outputs.push((log_name(sigma, options), values));
                }
            }
            ImageType::Mean => outputs.push((
                mean_name(options),
                mean_response(
                    image,
                    options.mean.support as usize,
                    options.mean.padding,
                    options.mean.constant,
                    options.mean.planar,
                )?,
            )),
            ImageType::Laws => {
                let mut values = laws_response(
                    image,
                    &options.laws.kernel,
                    options.laws.padding,
                    options.laws.constant,
                    options.laws.pooling,
                )?;
                if let Some(distance) = options.laws.energy_distance {
                    values = laws_energy(
                        &values,
                        &options.laws.kernel,
                        i64::from(distance),
                        options.laws.padding,
                        options.laws.constant,
                    )?;
                }
                outputs.push((laws_name(options), values));
            }
            ImageType::Gabor => {
                for (sigma, wavelength) in gabor_scale_pairs(options)? {
                    let responses = gabor_responses(
                        image,
                        spacing,
                        sigma,
                        wavelength,
                        options.gabor.gamma,
                        &options.gabor.theta,
                        options.gabor.response,
                        options.gabor.slice_axis,
                        options.gabor.padding,
                        options.gabor.constant,
                        options.gabor.pooling,
                    )?;
                    let plane = gabor_plane_tag(options, ndim);
                    for (index, values) in responses.into_iter().enumerate() {
                        let thetas: Vec<f64> = if options.gabor.pooling != OrientationPooling::None
                        {
                            options.gabor.theta.clone()
                        } else {
                            vec![options.gabor.theta[index]]
                        };
                        outputs.push((
                            with_plane(
                                gabor_base_name(options, sigma, wavelength, &thetas),
                                &plane,
                            ),
                            values,
                        ));
                    }
                }
            }
            ImageType::SeparableWavelet => {
                let wavelet = wavelet_filters(&options.separable_wavelet.name)?;
                let responses = separable_wavelet_responses(
                    image,
                    &wavelet.low,
                    &wavelet.high,
                    &options.separable_wavelet.bands,
                    options.separable_wavelet.level,
                    options.separable_wavelet.padding,
                    options.separable_wavelet.constant,
                    options.separable_wavelet.pooling,
                    options.separable_wavelet.planar,
                )?;
                for (band, values) in options.separable_wavelet.bands.iter().zip(responses) {
                    outputs.push((separable_wavelet_name(options, band), values));
                }
            }
            ImageType::Simoncelli => {
                for &level in &options.simoncelli.levels {
                    outputs.push((
                        simoncelli_name(options, level),
                        simoncelli_response(
                            image,
                            level,
                            options.simoncelli.grid,
                            options.simoncelli.planar,
                        )?,
                    ));
                }
            }
            ImageType::Riesz => {
                let computed = riesz_computed_orders(options, ndim)?;
                if options.riesz.base == RieszBase::Log {
                    for &sigma in &options.riesz.log_sigma {
                        let responses = riesz_requested(
                            options,
                            &computed,
                            riesz_log_responses(
                                image,
                                spacing,
                                sigma,
                                options.riesz.log_cutoff,
                                &computed,
                                options.riesz.grid,
                                options.riesz.padding,
                                options.riesz.constant,
                            )?,
                            spacing,
                        )?;
                        for (order, values) in options.riesz.orders.iter().zip(responses) {
                            outputs.push((riesz_name(options, order, sigma), values));
                        }
                    }
                } else {
                    for &level in &options.riesz.simoncelli_levels {
                        let responses = riesz_requested(
                            options,
                            &computed,
                            riesz_simoncelli_responses(
                                image,
                                level,
                                &computed,
                                options.riesz.grid,
                            )?,
                            spacing,
                        )?;
                        for (order, values) in options.riesz.orders.iter().zip(responses) {
                            outputs.push((riesz_name(options, order, f64::from(level)), values));
                        }
                    }
                }
            }
            ImageType::Original
            | ImageType::Square
            | ImageType::SquareRoot
            | ImageType::Logarithm
            | ImageType::Exponential
            | ImageType::Gradient => {
                if matches!(
                    kind,
                    ImageType::Square | ImageType::Logarithm | ImageType::Exponential
                ) && maximum == 0.0
                {
                    return Err(Error::InvalidInput(format!(
                        "the {} transform is undefined for all-zero images",
                        kind.as_str()
                    )));
                }
                let values = match kind {
                    ImageType::Original => image.clone(),
                    ImageType::Square => par_map(image, |x| (x / maximum.sqrt()).powi(2)),
                    ImageType::SquareRoot => {
                        par_map(image, |x| x.signum() * x.abs().sqrt() * maximum.sqrt())
                    }
                    ImageType::Logarithm => {
                        let scale = maximum / maximum.ln_1p();
                        par_map(image, |x| {
                            if x == 0.0 {
                                0.0
                            } else {
                                x.signum() * x.abs().ln_1p() * scale
                            }
                        })
                    }
                    ImageType::Exponential => par_map(image, |x| maximum.powf(x / maximum)),
                    _ => gradient(image, spacing, options.gradient.use_spacing),
                };
                outputs.push((kind.as_str().into(), values));
            }
        }
    }
    Ok(outputs)
}

/// Derived images of the standalone float64 transform, for the IBSI 2 reference maps.
#[cfg(test)]
fn feature_images(
    image: &ArrayD<f64>,
    mask: Option<&ArrayD<bool>>,
    spacing: &[f64],
    options: &TransformSettings,
    force2d_axis: Option<usize>,
) -> Result<Vec<FeatureImage>> {
    compute_feature_images::<false>(
        image,
        mask,
        spacing,
        options,
        force2d_axis,
        SourceDtype::Float,
    )
}

#[cfg(test)]
mod ibsi2_maps;
