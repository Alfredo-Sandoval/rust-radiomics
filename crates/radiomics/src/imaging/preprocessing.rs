//! Host preprocessing: normalization, resegmentation and gray-level discretization.
//!
//! Mirrors `mlxradiomics/imaging/preprocessing.py` (546 lines at mlx-radiomics b58783c).
//!
//! Execution: CPU, single-threaded. Orchestration (validation, mask gathering, scatter)
//! and numerical work (float64 normalisation and resegmentation statistics, exact binning)
//! both run on the calling thread; nothing runs on a GPU. These steps retain float64
//! source intensities. Fixed-width bins classify the exact binary64 values against a
//! zero-origin lattice, or an explicit `Settings::bin_anchor`, with exact rational floors
//! (`num-rational` over `num-bigint`, the counterpart of Python's `Fraction`): no floating
//! step drift and no epsilon ties. Small lattices use equivalent upward-rounded float64
//! thresholds; large lattices classify only the distinct sampled values. Sums and
//! standard deviations use NumPy's pairwise summation order so that results track
//! mlx-radiomics to the last bits.
//!
//! The internal `_integer_fixed_width_bins` fast path of mlx is not ported: it is an
//! optimisation that selects the same interval as the exact rational path (its docstring
//! states so) and only additionally records `integer_source_bounds`, which has no field in
//! [`Prepared`] yet. Results are identical; see the report for the shared-type request.
//!
//! Settings are checked with [`Settings::validate`](crate::config::settings::Settings::validate).
//!
//! Status: done (`round_intensities`, `validate_exact_float64`, `exact_float64`,
//! `normalize_values`, `prepare_sampled_input`, `prepare_derived_input`, `prepare`,
//! `discretize_prepared`, `directions`).

use crate::config::settings::{BinOrigin, ConstantRoiBin, Family, ResegmentMode, Settings};
use crate::contracts::types::{Prepared, PreparedInput};
use crate::error::{Error, Result};
use crate::imaging::io::{ArrayKind, HostArray};
use ndarray::{ArrayD, IxDyn};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use rayon::prelude::*;

fn invalid<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::InvalidInput(message.into()))
}

// ---------------------------------------------------------------------------------------
// NumPy-compatible float64 helpers
// ---------------------------------------------------------------------------------------

/// NumPy's pairwise summation (`pairwise_sum` of `loops.c.src`) over a contiguous slice.
pub fn pairwise_sum(values: &[f64]) -> f64 {
    let n = values.len();
    if n < 8 {
        let mut result = 0.0;
        for v in values {
            result += v;
        }
        result
    } else if n <= 128 {
        let mut r = [0.0; 8];
        r.copy_from_slice(&values[..8]);
        let mut i = 8;
        while i < n - (n % 8) {
            for (j, slot) in r.iter_mut().enumerate() {
                *slot += values[i + j];
            }
            i += 8;
        }
        let mut result = ((r[0] + r[1]) + (r[2] + r[3])) + ((r[4] + r[5]) + (r[6] + r[7]));
        while i < n {
            result += values[i];
            i += 1;
        }
        result
    } else {
        let mut half = n / 2;
        half -= half % 8;
        pairwise_sum(&values[..half]) + pairwise_sum(&values[half..])
    }
}

/// `np.mean` of a flat array.
pub fn numpy_mean(values: &[f64]) -> f64 {
    pairwise_sum(values) / values.len() as f64
}

/// `np.std(values, ddof=ddof)` of a flat array.
pub fn numpy_std(values: &[f64], ddof: usize) -> f64 {
    let mean = numpy_mean(values);
    let squares: Vec<f64> = values
        .iter()
        .map(|v| {
            let x = v - mean;
            x * x
        })
        .collect();
    (pairwise_sum(&squares) / (values.len() - ddof) as f64).sqrt()
}

/// C `frexp` exponent: `x = m * 2**e` with `0.5 <= |m| < 1` (zero gives 0).
pub fn frexp_exponent(x: f64) -> i32 {
    if x == 0.0 || !x.is_finite() {
        return 0;
    }
    let bits = x.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    if exponent == 0 {
        // Subnormal: normalise the mantissa.
        let mantissa = bits & ((1u64 << 52) - 1);
        let leading = mantissa.leading_zeros() as i32 - 11;
        -1022 - leading + 1
    } else {
        exponent - 1022
    }
}

/// `ldexp(x, n)` with a single rounding (port of musl `scalbn`).
pub fn ldexp(x: f64, n: i32) -> f64 {
    let mut y = x;
    let mut n = n;
    let two_pow = |e: i32| f64::from_bits(((0x3ff + e) as u64) << 52);
    if n > 1023 {
        y *= two_pow(1023);
        n -= 1023;
        if n > 1023 {
            y *= two_pow(1023);
            n -= 1023;
            if n > 1023 {
                n = 1023;
            }
        }
    } else if n < -1022 {
        // Keep the final scaling below -53 so the subnormal range rounds once.
        y *= two_pow(-1022) * two_pow(53);
        n += 1022 - 53;
        if n < -1022 {
            y *= two_pow(-1022) * two_pow(53);
            n += 1022 - 53;
            if n < -1022 {
                n = -1022;
            }
        }
    }
    y * two_pow(n)
}

/// Scale `values` by the exact power of two that brings their peak into `[0.5, 1)`
/// (`_unit_scaled`).
fn unit_scaled(values: &[f64]) -> Vec<f64> {
    let peak = values.iter().fold(0.0f64, |acc, v| acc.max(v.abs()));
    let shift = if peak > 0.0 { -frexp_exponent(peak) } else { 0 };
    values.iter().map(|&v| ldexp(v, shift)).collect()
}

/// Python `repr(float)`, for messages that quote a float64.
pub fn py_float(x: f64) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf".into() } else { "-inf".into() };
    }
    if x == 0.0 {
        return if x.is_sign_negative() {
            "-0.0".into()
        } else {
            "0.0".into()
        };
    }
    let scientific = format!("{x:e}");
    let (mantissa, exponent) = scientific.split_once('e').expect("exponent form");
    let exponent: i32 = exponent.parse().expect("integer exponent");
    let (sign, mantissa) = mantissa
        .strip_prefix('-')
        .map_or(("", mantissa), |m| ("-", m));
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    if (-4..16).contains(&exponent) {
        if exponent >= 0 {
            let point = exponent as usize + 1;
            if digits.len() <= point {
                format!("{sign}{digits}{}.0", "0".repeat(point - digits.len()))
            } else {
                format!("{sign}{}.{}", &digits[..point], &digits[point..])
            }
        } else {
            format!("{sign}0.{}{digits}", "0".repeat((-exponent - 1) as usize))
        }
    } else {
        let body = if digits.len() == 1 {
            digits
        } else {
            format!("{}.{}", &digits[..1], &digits[1..])
        };
        format!(
            "{sign}{body}e{}{:02}",
            if exponent < 0 { '-' } else { '+' },
            exponent.abs()
        )
    }
}

// ---------------------------------------------------------------------------------------
// Exactness and conversion
// ---------------------------------------------------------------------------------------

/// Return a float64 copy of `values` rounded to the nearest integer, ties to even
/// (`round_intensities`).
///
/// # Errors
/// [`Error::InvalidInput`] if 64-bit integer values are not exactly representable in
/// float64; the check runs before conversion, so rounding cannot hide the loss.
pub fn round_intensities(values: &HostArray) -> Result<ArrayD<f64>> {
    let mut rounded = exact_float64(values)?;
    match rounded.as_slice_memory_order_mut() {
        Some(flat) => flat
            .par_iter_mut()
            .for_each(|v| *v = f64::round_ties_even(*v)),
        None => rounded.mapv_inplace(f64::round_ties_even),
    }
    Ok(rounded)
}

/// Refuse 64-bit integer values that conversion to float64 would round
/// (`validate_exact_float64`).
///
/// Integer magnitudes above `2**53` are exact in float64 only when their odd part (the
/// value with trailing zero bits removed) fits in 53 bits. Integer types of at most 32
/// bits are always exact.
///
/// # Errors
/// [`Error::InvalidInput`] if a 64-bit integer value is not exactly representable.
pub fn validate_exact_float64(image: &HostArray) -> Result<()> {
    let lossy = |magnitude: u64| {
        magnitude != 0 && magnitude / (magnitude & magnitude.wrapping_neg()) >= 1u64 << 53
    };
    let refused = match image {
        HostArray::U64(a) => a.iter().any(|&v| lossy(v)),
        HostArray::I64(a) => a.iter().any(|&v| lossy(v.unsigned_abs())),
        _ => false,
    };
    if refused {
        return invalid(
            "image contains 64-bit integer intensities that float64 cannot represent exactly; rescale or convert the image explicitly",
        );
    }
    Ok(())
}

/// Convert `image` to float64, refusing integer values that the conversion rounds
/// (`exact_float64`).
pub fn exact_float64(image: &HostArray) -> Result<ArrayD<f64>> {
    validate_exact_float64(image)?;
    Ok(image.to_f64_lossy())
}

/// Return a float64 copy of `image`, normalized over the whole image if requested
/// (`normalize_values`).
///
/// Normalization is `(x - mean) / std` over every voxel, not just the ROI, with the sample
/// standard deviation (ddof=1) as in ITK's normalize filter. If `remove_outliers` is set
/// the result is clipped to plus or minus that many standard deviations, then multiplied
/// by `normalize_scale`.
///
/// # Errors
/// [`Error::InvalidInput`] if normalization is requested and the image has fewer than two
/// voxels, non-finite values or zero variance, or if 64-bit integer intensities are not
/// exactly representable in float64.
pub fn normalize_values(image: &HostArray, settings: &Settings) -> Result<ArrayD<f64>> {
    let values = exact_float64(image)?;
    if !settings.normalize {
        return Ok(values);
    }
    let flat: Vec<f64> = values.iter().copied().collect();
    if flat.len() < 2 || !flat.iter().all(|v| v.is_finite()) {
        return invalid("normalization requires at least two finite image intensities");
    }
    // Center on an observed value before reducing, and rescale by exact powers of two so
    // that neither the subtraction nor the squared offsets overflow or flush to zero.
    let scaled = unit_scaled(&flat);
    let first = scaled[0];
    let offsets = unit_scaled(&scaled.iter().map(|v| v - first).collect::<Vec<_>>());
    let deviation = numpy_std(&offsets, 1);
    if deviation == 0.0 || !deviation.is_finite() {
        return invalid("normalization requires nonzero finite whole-image variance");
    }
    let mean = numpy_mean(&offsets);
    let mut normalised: Vec<f64> = offsets.iter().map(|v| (v - mean) / deviation).collect();
    if let Some(limit) = settings.remove_outliers {
        for v in &mut normalised {
            *v = v.clamp(-limit, limit);
        }
    }
    for v in &mut normalised {
        *v *= settings.normalize_scale;
    }
    Ok(ArrayD::from_shape_vec(IxDyn(values.shape()), normalised).expect("same element count"))
}

// ---------------------------------------------------------------------------------------
// Resegmentation
// ---------------------------------------------------------------------------------------

/// Apply the same thresholds in ROI sample order and optionally retain samples
/// (`_resegment_samples`).
///
/// Background comparisons never affect the retained mask. Retained samples are a subset
/// of the vector validated here, so masked callers can reuse them for binning without
/// another image gather or finite scan.
fn resegment_samples(
    values: &ArrayD<f64>,
    roi: &ArrayD<bool>,
    settings: &Settings,
    keep_samples: bool,
) -> Result<(ArrayD<bool>, Option<Vec<f64>>)> {
    let Some(range) = &settings.resegment_range else {
        return Ok((roi.clone(), None));
    };
    let positions = true_positions(roi);
    let selected = gather_at(values, &positions);
    if !selected.iter().all(|v| v.is_finite()) {
        return invalid("resegmentation requires finite selected intensities");
    }
    let mut thresholds: Vec<f64> = range.clone();
    let mut centered: Option<Vec<f64>> = None;
    let mut spread = 0.0;
    let sigma_mode = settings.resegment_mode == ResegmentMode::Sigma;
    if sigma_mode || settings.resegment_outlier_sigma.is_some() {
        // Keep sigma boundaries in centered coordinates rather than rounding a mean back
        // onto the original scale before inclusive comparisons. Exact power-of-two
        // rescalings keep opposite extremes and squared offsets in range; they leave the
        // inclusive comparisons unchanged. Only ROI voxels are centred.
        let scaled = unit_scaled(&selected);
        let first = scaled[0];
        let offsets = unit_scaled(&scaled.iter().map(|v| v - first).collect::<Vec<_>>());
        let mean = numpy_mean(&offsets);
        centered = Some(offsets.iter().map(|v| v - mean).collect());
        spread = numpy_std(&offsets, 0);
    }
    let mut compared: &[f64] = &selected;
    match settings.resegment_mode {
        ResegmentMode::Relative => {
            let maximum = selected.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            for t in &mut thresholds {
                *t *= maximum;
            }
        }
        ResegmentMode::Sigma => {
            compared = centered.as_deref().expect("sigma mode centres the samples");
            for t in &mut thresholds {
                *t *= spread;
            }
        }
        ResegmentMode::Absolute => {}
    }
    thresholds.sort_by(f64::total_cmp);
    let low = thresholds[0];
    let high = (thresholds.len() == 2).then(|| thresholds[1]);
    let mut retained: Vec<bool> = compared.iter().map(|&v| v >= low).collect();
    if let Some(high) = high {
        for (flag, &v) in retained.iter_mut().zip(compared) {
            *flag &= v <= high;
        }
    }
    if let Some(sigma) = settings.resegment_outlier_sigma {
        let offsets = centered
            .as_deref()
            .expect("outlier sigma centres the samples");
        for (flag, &v) in retained.iter_mut().zip(offsets) {
            *flag &= v.abs() <= sigma * spread;
        }
    }
    if !retained.iter().any(|&f| f) {
        return invalid("resegmentation must retain at least one selected voxel");
    }
    let mut mask = vec![false; roi.len()];
    for (&position, &keep) in positions.iter().zip(&retained) {
        mask[position] = keep;
    }
    let kept = keep_samples.then(|| {
        selected
            .iter()
            .zip(&retained)
            .filter_map(|(&v, &f)| f.then_some(v))
            .collect()
    });
    Ok((
        ArrayD::from_shape_vec(IxDyn(roi.shape()), mask).expect("same element count"),
        kept,
    ))
}

// ---------------------------------------------------------------------------------------
// Exact rational binning
// ---------------------------------------------------------------------------------------

/// Exact `Fraction(float(x))` of a finite float64.
pub fn fraction_of_f64(x: f64) -> BigRational {
    let bits = x.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i64;
    let fraction_bits = bits & ((1u64 << 52) - 1);
    let (mantissa, exponent) = if exponent == 0 {
        (fraction_bits, -1074)
    } else {
        (fraction_bits | (1u64 << 52), exponent - 1075)
    };
    let mut numerator = BigInt::from(mantissa);
    if bits >> 63 == 1 {
        numerator = -numerator;
    }
    if exponent >= 0 {
        BigRational::from_integer(numerator << exponent as usize)
    } else {
        BigRational::new(numerator, BigInt::one() << (-exponent) as usize)
    }
}

/// Correctly rounded (nearest-even) `float(Fraction)`; `None` where Python raises
/// `OverflowError`.
pub fn fraction_to_f64(q: &BigRational) -> Option<f64> {
    if q.is_zero() {
        return Some(0.0);
    }
    let negative = q.is_negative();
    let numerator = q.numer().abs();
    let denominator = q.denom();
    let shift = 55 - (numerator.bits() as i64 - denominator.bits() as i64);
    let (scaled_n, scaled_d) = if shift >= 0 {
        (numerator << shift as usize, denominator.clone())
    } else {
        (numerator, denominator << (-shift) as usize)
    };
    let (quotient, remainder) = scaled_n.div_rem(&scaled_d);
    let sticky = !remainder.is_zero();
    let quotient = quotient.to_u64()?;
    let quotient_bits = 64 - i64::from(quotient.leading_zeros());
    let exponent = quotient_bits - 1 - shift;
    let precision = if exponent >= -1022 {
        53
    } else {
        53 - (-1022 - exponent)
    };
    if precision < 0 {
        return Some(if negative { -0.0 } else { 0.0 });
    }
    let drop = quotient_bits - precision;
    let low = quotient & ((1u64 << drop) - 1);
    let half = 1u64 << (drop - 1);
    let mut mantissa = quotient >> drop;
    if low > half || (low == half && (sticky || mantissa & 1 == 1)) {
        mantissa += 1;
    }
    let value = ldexp(mantissa as f64, (drop - shift) as i32);
    if value.is_infinite() {
        return None;
    }
    Some(if negative { -value } else { value })
}

/// Next representable float64 above `x` (`np.nextafter(x, inf)`).
fn next_up(x: f64) -> f64 {
    if x.is_nan() || x == f64::INFINITY {
        return x;
    }
    if x == 0.0 {
        return f64::from_bits(1);
    }
    let bits = x.to_bits();
    f64::from_bits(if x > 0.0 { bits + 1 } else { bits - 1 })
}

fn check_gray_levels(levels: &BigInt, settings: &Settings) -> Result<()> {
    if *levels < BigInt::one() || *levels > BigInt::from(i32::MAX) {
        return invalid("gray-level range exceeds int32 indexing capacity");
    }
    if let Some(limit) = settings.max_gray_levels
        && *levels > BigInt::from(limit)
    {
        return invalid("gray-level range exceeds the explicit max_gray_levels limit");
    }
    Ok(())
}

/// Classify exact binary64 samples with bounded host lattice storage (`_lattice_bins`).
///
/// A bounded threshold table keeps ordinary image preprocessing vectorised. Larger
/// lattices classify only distinct sampled values by integer-ratio floor division. The
/// threshold table is merely a representation: rounded coincident edges and an edge
/// outside binary64 cannot change the exact mathematical classification.
fn lattice_bins(
    selected: &[f64],
    origin: &BigRational,
    width: &BigRational,
    first: &BigInt,
    levels: usize,
    closed_last: bool,
) -> Result<Vec<i32>> {
    if levels <= 65536 {
        let exact: Vec<BigRational> = (0..=levels)
            .map(|offset| origin + BigRational::from_integer(first + BigInt::from(offset)) * width)
            .collect();
        let rounded: Option<Vec<f64>> = exact.iter().map(fraction_to_f64).collect();
        if let Some(mut edges) = rounded {
            for (edge, target) in edges.iter_mut().zip(&exact) {
                if fraction_of_f64(*edge) < *target {
                    *edge = next_up(*edge);
                }
            }
            // Repeated upward-rounded thresholds correctly skip intervals containing no
            // representable source value; the position keeps the original indices.
            return Ok(selected
                .par_iter()
                .map(|&v| {
                    let mut bin = edges.partition_point(|&e| e <= v);
                    if closed_last {
                        bin = bin.min(levels);
                    }
                    bin as i32
                })
                .collect());
        }
    }
    let mut order: Vec<usize> = (0..selected.len()).collect();
    order.sort_by(|&a, &b| selected[a].total_cmp(&selected[b]));
    let (anchor_n, anchor_d) = (origin.numer(), origin.denom());
    let (width_n, width_d) = (width.numer(), width.denom());
    let mut bins = vec![0i32; selected.len()];
    let mut previous: Option<(u64, i32)> = None;
    for &position in &order {
        let value = selected[position];
        // Equal values (including -0.0 and 0.0, which classify identically) share a bin.
        let key = if value == 0.0 { 0 } else { value.to_bits() };
        if let Some((last_key, bin)) = previous
            && last_key == key
        {
            bins[position] = bin;
            continue;
        }
        let ratio = fraction_of_f64(value);
        let (numerator, denominator) = (ratio.numer(), ratio.denom());
        let mut index = ((numerator * anchor_d - anchor_n * denominator) * width_d)
            .div_floor(&(denominator * anchor_d * width_n))
            - first
            + BigInt::one();
        if closed_last {
            index = index.min(BigInt::from(levels));
        }
        let bin = index.to_i32().ok_or_else(|| {
            Error::InvalidInput("gray-level range exceeds int32 indexing capacity".into())
        })?;
        bins[position] = bin;
        previous = Some((key, bin));
    }
    Ok(bins)
}

fn floor_to_bigint(q: &BigRational) -> BigInt {
    q.floor().to_integer()
}

/// Build exact indices while preserving the requested origin and interval width
/// (`_exact_bins`).
fn exact_bins(selected: &[f64], settings: &Settings) -> Result<Vec<i32>> {
    let lowest = selected.iter().copied().fold(f64::INFINITY, f64::min);
    let highest = selected.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let (minimum, maximum) = (fraction_of_f64(lowest), fraction_of_f64(highest));
    if let Some(count) = settings.bin_count {
        // Evaluate floor(B*(x-min)/(max-min)) over the supplied binary64 values. A
        // nearest-rounded 1/3 boundary would incorrectly promote float64(1/3).
        check_gray_levels(&BigInt::from(count), settings)?;
        let width = (&maximum - &minimum) / BigRational::from_integer(BigInt::from(count));
        return lattice_bins(
            selected,
            &minimum,
            &width,
            &BigInt::zero(),
            count as usize,
            true,
        );
    }
    let width = fraction_of_f64(settings.bin_width);
    // IBSI fixed bin size takes X_gl,min from the lower bound of a user-set resegmentation
    // range, so gray-level indices are comparable across samples; a data-driven ROI
    // minimum shifts every index-weighted texture feature per sample. Relative and sigma
    // ranges are data-driven and keep the sampled minimum.
    let resegment_anchor = settings.bin_anchor.is_none()
        && settings.bin_origin == BinOrigin::ResegmentOrMinimum
        && settings.resegment_mode == ResegmentMode::Absolute
        && settings
            .resegment_range
            .as_ref()
            .is_some_and(|r| !r.is_empty());
    let anchor = if resegment_anchor {
        let range = settings.resegment_range.as_ref().expect("checked above");
        fraction_of_f64(range.iter().copied().fold(f64::INFINITY, f64::min))
    } else if settings.bin_anchor.is_none() && settings.bin_origin == BinOrigin::ResegmentOrMinimum
    {
        minimum.clone()
    } else {
        fraction_of_f64(settings.bin_anchor.unwrap_or(0.0))
    };
    if let Some(explicit) = settings.bin_anchor
        && minimum < anchor
    {
        return invalid(format!(
            "bin_anchor {} is above the selected minimum {}",
            py_float(explicit),
            py_float(lowest)
        ));
    }
    if resegment_anchor && minimum < anchor {
        // Only unmasked voxel kernels sample below the range. Extending the lattice down
        // to those samples would renumber every ROI gray level by the background minimum,
        // silently abandoning the fixed anchor; refuse them as an explicit anchor does.
        return invalid(format!(
            "sampled minimum {} lies below the resegmentation lower bound {} that anchors IBSI fixed-bin-size discretization; use masked_kernel=True or a resegment_range that covers the sampled image",
            py_float(lowest),
            py_float(fraction_to_f64(&anchor).unwrap_or(f64::NAN)),
        ));
    }
    let first = if settings.bin_anchor.is_none() && settings.bin_origin == BinOrigin::ZeroLattice {
        floor_to_bigint(&(&minimum / &width))
    } else {
        BigInt::zero()
    };
    let last = floor_to_bigint(&((&maximum - &anchor) / &width));
    let levels = &last - &first + BigInt::one();
    check_gray_levels(&levels, settings)?;
    lattice_bins(
        selected,
        &anchor,
        &width,
        &first,
        levels.to_usize().expect("levels checked against i32 range"),
        false,
    )
}

// ---------------------------------------------------------------------------------------
// Settings validation (subset of `Settings.validate` that preprocessing relies on)
// ---------------------------------------------------------------------------------------

// ---------------------------------------------------------------------------------------
// Preparation
// ---------------------------------------------------------------------------------------

/// Validate inputs, then normalize and resegment, keeping the gathered samples
/// (`prepare_sampled_input`).
///
/// `image` is real-valued (int, uint or float), 2-D or 3-D, NumPy axis order; `mask` is
/// bool or integer of the same shape; `settings.label` selects the ROI. `voxel` is true for
/// voxel mode, which needs at least two ROI voxels and, with `masked_kernel = false`,
/// samples the whole image. Returns the prepared masks and image and the float64 samples
/// used for binning.
///
/// # Errors
/// [`Error::InvalidInput`] if shapes or dtypes are unsupported, the settings are invalid,
/// the label is absent, resegmentation or normalization fails, or sampled intensities are
/// non-finite.
pub fn prepare_sampled_input(
    image: &HostArray,
    mask: &HostArray,
    settings: &Settings,
    voxel: bool,
) -> Result<(PreparedInput, Vec<f64>)> {
    let ndim = image.ndim();
    if !(ndim == 2 || ndim == 3) || image.shape() != mask.shape() || image.shape().contains(&0) {
        return invalid("image and mask must share a nonempty 2D or 3D array shape");
    }
    if image.kind() == ArrayKind::Bool || mask.kind() == ArrayKind::Float {
        return invalid("image must be real numeric and mask must contain integer labels");
    }
    settings.validate(ndim)?;
    let values = normalize_values(image, settings)?;
    let roi = mask.eq_label(settings.label);
    if !roi.iter().any(|&r| r) {
        return invalid("selected label is absent from mask");
    }
    let original_roi = roi.clone();
    let unmasked = voxel && !settings.masked_kernel;
    let (roi, retained_samples) = resegment_samples(&values, &roi, settings, !unmasked)?;
    let shape_roi = if settings.resegment_shape {
        roi.clone()
    } else {
        original_roi
    };
    if voxel && roi.iter().filter(|&&r| r).count() < 2 {
        return invalid("voxel extraction requires at least two selected voxels");
    }
    let sampling = if unmasked {
        ArrayD::from_elem(IxDyn(roi.shape()), true)
    } else {
        roi.clone()
    };
    let selected = match retained_samples {
        Some(samples) => samples,
        None => {
            let gathered = gather(&values, &sampling);
            if !gathered.iter().all(|v| v.is_finite()) {
                return invalid("sampled image intensities must be finite");
            }
            gathered
        }
    };
    Ok((
        PreparedInput {
            image: values,
            roi,
            shape_roi,
            sampling_mask: sampling,
        },
        selected,
    ))
}

/// Row-major flat positions of the true voxels of `mask`, in increasing order.
fn true_positions(mask: &ArrayD<bool>) -> Vec<usize> {
    const BLOCK: usize = 1 << 16;
    let standard = mask.as_standard_layout();
    let flat = standard.as_slice().expect("standard layout is contiguous");
    flat.par_chunks(BLOCK)
        .enumerate()
        .flat_map_iter(|(block, chunk)| {
            chunk
                .iter()
                .enumerate()
                .filter(|&(_, &hit)| hit)
                .map(move |(offset, _)| block * BLOCK + offset)
        })
        .collect()
}

/// `values` at flat `positions` (row-major), in order.
fn gather_at(values: &ArrayD<f64>, positions: &[usize]) -> Vec<f64> {
    let standard = values.as_standard_layout();
    let flat = standard.as_slice().expect("standard layout is contiguous");
    positions
        .par_iter()
        .map(|&position| flat[position])
        .collect()
}

fn gather(values: &ArrayD<f64>, mask: &ArrayD<bool>) -> Vec<f64> {
    gather_at(values, &true_positions(mask))
}

/// Validate derived pixels while reusing the original, already selected masks
/// (`prepare_derived_input`).
///
/// The pipeline disables normalization and resegmentation for derived images. The original
/// preparation therefore supplies the intensity, sampling and morphology masks, but each
/// derived image still needs its own exact conversion and sampled-value finite check
/// before its independent discretization.
pub fn prepare_derived_input(
    image: &HostArray,
    original: &PreparedInput,
) -> Result<(PreparedInput, Vec<f64>)> {
    if image.shape() != original.image.shape() {
        return invalid("image and mask must share a nonempty 2D or 3D array shape");
    }
    if image.kind() == ArrayKind::Bool {
        return invalid("image must be real numeric and mask must contain integer labels");
    }
    let values = exact_float64(image)?;
    let selected = gather(&values, &original.sampling_mask);
    if !selected.iter().all(|v| v.is_finite()) {
        return invalid("sampled image intensities must be finite");
    }
    Ok((
        PreparedInput {
            image: values,
            roi: original.roi.clone(),
            shape_roi: original.shape_roi.clone(),
            sampling_mask: original.sampling_mask.clone(),
        },
        selected,
    ))
}

/// Prepare, validate and discretize the sampled voxels into gray-level bins (`prepare`).
///
/// Binning uses `settings.bin_count` if set, otherwise `settings.bin_width`. With a fixed
/// width, an explicit `settings.bin_anchor` fixes the first edge and gray level 1 at the
/// anchor; `None` follows `settings.bin_origin`: the absolute resegmentation lower bound
/// (else the sampled minimum) for `ResegmentOrMinimum`, or a zero-origin lattice,
/// renumbered from the minimum bin, for `ZeroLattice`. Nonconstant fixed-count bins use
/// exact source-value thresholds and include the maximum in the final bin. A constant ROI
/// uses bin 1 for `ConstantRoiBin::First` and the midpoint bin `bin_count / 2 + 1` for
/// `Middle`. When only shape features are selected, binning is skipped (`bins = None`, one
/// gray level). The same applies when selected intensity features read only source
/// intensities (first-order features except Entropy and Uniformity).
///
/// # Errors
/// As [`prepare_sampled_input`], or if the gray-level count exceeds
/// `settings.max_gray_levels` or the sampled values lie below `bin_anchor` or below the
/// absolute resegmentation bound that anchors IBSI fixed-width bins (possible only for
/// unmasked voxel kernels).
pub fn prepare(
    image: &HostArray,
    mask: &HostArray,
    settings: &Settings,
    voxel: bool,
) -> Result<Prepared> {
    let (data, selected) = prepare_sampled_input(image, mask, settings, voxel)?;
    discretize_prepared(&data, &selected, settings)
}

/// Discretize validated samples without repeating source or mask preparation
/// (`discretize_prepared`).
///
/// Only selections that read bins construct a lattice. Raw intensity statistics, shape,
/// moment invariants, local intensity and IVH leave `bins = None` with the unused-lattice sentinel
/// `gray_levels = 1`; IVH constructs its separate lattice later.
pub fn discretize_prepared(
    data: &PreparedInput,
    selected: &[f64],
    settings: &Settings,
) -> Result<Prepared> {
    let assign = settings.features.iter().any(|name| {
        !(Family::is_morphology(name)
            || name.starts_with("localintensity_")
            || name.starts_with("ivh_")
            || name.starts_with("firstorder_"))
            || name == "firstorder_Entropy"
            || name == "firstorder_Uniformity"
    });
    if !assign {
        return Ok(Prepared {
            input: data.clone(),
            bins: None,
            gray_levels: 1,
        });
    }
    let lowest = selected.iter().copied().fold(f64::INFINITY, f64::min);
    let highest = selected.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let digitized: Vec<i32>;
    if let Some(count) = settings.bin_count.filter(|_| lowest == highest) {
        let level = match settings.constant_roi_bin {
            ConstantRoiBin::First => 1,
            ConstantRoiBin::Middle => count / 2 + 1,
        };
        check_gray_levels(&BigInt::from(level), settings)?;
        digitized = vec![level as i32; selected.len()];
    } else {
        digitized = exact_bins(selected, settings)?;
    }
    let levels = BigInt::from(digitized.iter().copied().max().unwrap_or(0));
    check_gray_levels(&levels, settings)?;
    let mut bins = vec![0i32; data.sampling_mask.len()];
    for (&position, &bin) in true_positions(&data.sampling_mask).iter().zip(&digitized) {
        bins[position] = bin;
    }
    Ok(Prepared {
        input: data.clone(),
        bins: Some(
            ArrayD::from_shape_vec(IxDyn(data.image.shape()), bins).expect("same element count"),
        ),
        gray_levels: levels.to_u32().expect("levels checked against i32 range"),
    })
}

/// Return the voxel offsets that define neighbor pairs, in array-axis order
/// (`directions`).
///
/// An offset is kept if its Chebyshev (maximum-norm) length is in `settings.distances`, its
/// first nonzero component is positive (one of each opposite pair), it fits inside `shape`,
/// and, with `force2d_axis` set, it has no component along that axis. Offsets are listed
/// from +radius down to -radius per axis, the order of PyRadiomics' angle list.
pub fn directions(shape: &[usize], settings: &Settings) -> Vec<Vec<i64>> {
    let radius = i64::from(settings.distances.iter().copied().max().unwrap_or(0));
    let side = (2 * radius + 1) as usize;
    let total = side.pow(shape.len() as u32);
    let mut result = Vec::new();
    for index in 0..total {
        let mut rest = index;
        let mut offset = vec![0i64; shape.len()];
        for axis in (0..shape.len()).rev() {
            offset[axis] = radius - (rest % side) as i64;
            rest /= side;
        }
        let length = offset.iter().map(|v| v.unsigned_abs()).max().unwrap_or(0);
        let positive = offset.iter().find(|&&v| v != 0).is_some_and(|&v| v > 0);
        if settings.distances.iter().any(|&d| u64::from(d) == length)
            && positive
            && offset
                .iter()
                .zip(shape)
                .all(|(&v, &s)| (v.unsigned_abs() as usize) < s)
            && settings.force2d_axis.is_none_or(|axis| offset[axis] == 0)
        {
            result.push(offset);
        }
    }
    result
}
