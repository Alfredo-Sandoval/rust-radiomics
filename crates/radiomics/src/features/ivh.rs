//! Intensity-volume histograms on an independently calibrated lattice.
//!
//! Mirrors `mlxradiomics/features/ivh.py` (v1.1.0, b58783c).
//! Status: all three discretizers and seven ROI features ported. Single-threaded
//! CPU numerical work using exact binary64 input ratios for bin assignment.

use ndarray::ArrayD;
use num_rational::BigRational;
use num_traits::{One, ToPrimitive};

use crate::config::settings::{ResegmentMode, Settings};
use crate::contracts::types::Features;
use crate::error::{Error, Result};
use crate::features::intensity_histogram::contiguous;

/// One-based IVH bins with their inclusive physical calibration.
#[derive(Debug)]
pub struct IvhLattice {
    /// Zero outside the sampling mask.
    pub bins: ArrayD<i32>,
    /// Physical intensity of bin one.
    pub origin: f64,
    /// Physical intensity increment between neighboring bins.
    pub step: f64,
    /// Number of levels, including unoccupied ones.
    pub levels: i32,
}

fn validate_settings(settings: &Settings) -> Result<()> {
    if settings
        .ivh_bin_width
        .is_some_and(|w| !w.is_finite() || w <= 0.)
    {
        return Err(Error::InvalidInput(
            "ivh_bin_width must be finite and positive".into(),
        ));
    }
    if settings.ivh_bin_count == Some(0) {
        return Err(Error::InvalidInput(
            "ivh_bin_count must be a positive integer".into(),
        ));
    }
    if settings.ivh_bin_width.is_some() && settings.ivh_bin_count.is_some() {
        return Err(Error::InvalidInput(
            "IVH accepts either bin width or bin count".into(),
        ));
    }
    if settings
        .ivh_intensity_range
        .is_some_and(|(a, b)| !a.is_finite() || !b.is_finite() || a > b)
    {
        return Err(Error::InvalidInput(
            "ivh_intensity_range must be finite ordered lower/upper bounds".into(),
        ));
    }
    if let Some(range) = &settings.resegment_range
        && (!(1..=2).contains(&range.len()) || range.iter().any(|v| !v.is_finite()))
    {
        return Err(Error::InvalidInput(
            "resegment_range must contain one or two finite thresholds".into(),
        ));
    }
    Ok(())
}

fn exact(value: f64) -> BigRational {
    // Called only for validated finite binary64 inputs.
    BigRational::from_float(value).unwrap()
}

fn lattice_limit() -> Error {
    Error::InvalidInput("IVH lattice exceeds scratch_mib or the supported index range".into())
}

// The first f64 on or above the exact boundary. A source equal to a
// downward-rounded edge still belongs in the preceding bin.
fn ceil_binary64(edge: &BigRational) -> f64 {
    let rounded = edge.to_f64().unwrap();
    if exact(rounded) < *edge {
        // Rust 1.85 does not yet have f64::next_up.
        if rounded == 0. {
            f64::from_bits(1)
        } else if rounded > 0. {
            f64::from_bits(rounded.to_bits() + 1)
        } else {
            f64::from_bits(rounded.to_bits() - 1)
        }
    } else {
        rounded
    }
}

fn assign_bins(
    lattice: &mut IvhLattice,
    image: &ArrayD<f64>,
    mask: &ArrayD<bool>,
    low: &BigRational,
    interval: &BigRational,
    samples: usize,
) {
    let levels = lattice.levels;
    // For a short lattice amortize exact edge construction over all samples.
    // For sparse, wide domains solve the same rational inequality per sample.
    let edges = (levels as usize <= samples).then(|| {
        let mut edge = low + interval;
        (1..levels)
            .map(|_| {
                let boundary = ceil_binary64(&edge);
                edge += interval;
                boundary
            })
            .collect::<Vec<_>>()
    });
    let (image, mask) = (contiguous(image), contiguous(mask));
    let bins = lattice
        .bins
        .as_slice_mut()
        .expect("a fresh lattice is contiguous");
    for ((bin, &value), &inside) in bins.iter_mut().zip(image.iter()).zip(mask.iter()) {
        if inside {
            *bin = if let Some(edges) = &edges {
                edges.partition_point(|edge| *edge <= value) as i32 + 1
            } else {
                let index = ((exact(value) - low) / interval).to_integer();
                index.to_i32().unwrap().min(levels - 1) + 1
            };
        }
    }
}

/// Build the IVH lattice from source image values inside the supplied mask.
/// Absolute resegmentation bounds supply the domain when IVH bounds are absent.
pub fn prepare_ivh(
    image: &ArrayD<f64>,
    mask: &ArrayD<bool>,
    settings: &Settings,
) -> Result<IvhLattice> {
    validate_settings(settings)?;
    if image.shape() != mask.shape() || image.is_empty() || !(2..=3).contains(&image.ndim()) {
        return Err(Error::InvalidInput(
            "image and mask must share a nonempty 2D or 3D array shape".into(),
        ));
    }
    let (mut minimum, mut maximum, mut samples) = (f64::INFINITY, f64::NEG_INFINITY, 0);
    let mut integral = true;
    let (image_values, mask_values) = (contiguous(image), contiguous(mask));
    for (&value, &inside) in image_values.iter().zip(mask_values.iter()) {
        if inside {
            if !value.is_finite() {
                return Err(Error::InvalidInput(
                    "sampled image intensities must be finite".into(),
                ));
            }
            minimum = minimum.min(value);
            maximum = maximum.max(value);
            integral &= value.fract() == 0.;
            samples += 1;
        }
    }
    if samples == 0 {
        return Err(Error::InvalidInput(
            "selected label is absent from mask".into(),
        ));
    }
    let bounds = settings.ivh_intensity_range.or_else(|| {
        let range = settings.resegment_range.as_ref()?;
        let upper = range.get(1).copied().unwrap_or(maximum);
        (settings.resegment_mode == ResegmentMode::Absolute)
            .then(|| (range[0].min(upper), range[0].max(upper)))
    });
    let (low, high) = bounds.unwrap_or((minimum, maximum));
    if minimum < low || maximum > high {
        return Err(Error::InvalidInput(
            "IVH intensity range must contain every sampled intensity".into(),
        ));
    }
    let mut lattice = IvhLattice {
        bins: ArrayD::zeros(image.raw_dim()),
        origin: 1.,
        step: 1.,
        levels: 1,
    };
    let (start, interval) = if let Some(count) = settings.ivh_bin_count {
        if settings.ivh_intensity_range.is_some() {
            return Err(Error::InvalidInput(
                "fixed-count IVH uses the sampled range, not ivh_intensity_range".into(),
            ));
        }
        if minimum == maximum {
            lattice.bins = mask.mapv(i32::from);
            return Ok(lattice);
        }
        lattice.levels = i32::try_from(count).map_err(|_| lattice_limit())?;
        let start = exact(minimum);
        let interval = (exact(maximum) - &start) / BigRational::from_integer(count.into());
        (start, interval)
    } else if let Some(width) = settings.ivh_bin_width {
        let start = exact(low);
        let interval = exact(width);
        let span = (exact(high) - &start) / &interval;
        let levels = if bounds.is_some() {
            span.ceil().max(BigRational::one())
        } else {
            span.floor() + BigRational::one()
        };
        lattice.levels = levels.to_i32().ok_or_else(lattice_limit)?;
        lattice.origin = width.mul_add(0.5, low);
        lattice.step = width;
        (start, interval)
    } else {
        if !integral || low.fract() != 0. || high.fract() != 0. {
            return Err(Error::InvalidInput(
                "continuous IVH intensities require ivh_bin_width or ivh_bin_count".into(),
            ));
        }
        let levels = high - low + 1.;
        if levels > i32::MAX as f64 {
            return Err(Error::InvalidInput(
                "IVH lattice exceeds the supported int32 index range".into(),
            ));
        }
        lattice.origin = low;
        lattice.levels = levels as i32;
        let bins = lattice
            .bins
            .as_slice_mut()
            .expect("a fresh lattice is contiguous");
        for ((bin, &value), &inside) in bins
            .iter_mut()
            .zip(image_values.iter())
            .zip(mask_values.iter())
        {
            if inside {
                *bin = (value - low) as i32 + 1;
            }
        }
        return Ok(lattice);
    };
    if (lattice.levels as u64 + 1) * 32 > u64::from(settings.scratch_mib) * 1024 * 1024 {
        return Err(lattice_limit());
    }
    assign_bins(&mut lattice, image, mask, &start, &interval, samples);
    Ok(lattice)
}

/// Evaluate full `ivh_*` keys for a voxel set on an already calibrated lattice.
/// An empty set gives NaN for all outputs. A single occupied level has zero area.
pub fn ivh_statistics(
    lattice: &IvhLattice,
    valid: &ArrayD<bool>,
    names: &[&str],
) -> Result<Features> {
    let IvhLattice {
        bins,
        origin,
        step,
        levels,
    } = lattice;
    let levels = *levels;
    if valid.shape() != bins.shape() || levels < 1 {
        return Err(Error::InvalidInput(
            "IVH requires a positive lattice and matching bin and mask shapes".into(),
        ));
    }
    if origin.abs().max(step.abs() * (levels - 1) as f64) > f32::MAX as f64 {
        return Err(Error::InvalidInput(
            "IVH calibrated lattice exceeds the float32 range".into(),
        ));
    }
    let need_ranks = names.iter().any(|name| name.starts_with("ivh_Intensity"));
    let threshold = |q: u64| 1 + (q * (levels - 1) as u64).div_ceil(100) as i32;
    let (threshold10, threshold90) = (threshold(10), threshold(90));
    let (mut v10, mut v90, mut count) = (0usize, 0usize, 0usize);
    let (mut least, mut greatest, mut twice_area) = (levels, 0, 0u128);
    let mut ranks = Vec::new();
    let (bin_values, valid_values) = (contiguous(bins), contiguous(valid));
    for (&bin, &inside) in bin_values.iter().zip(valid_values.iter()) {
        if inside {
            if !(1..=levels).contains(&bin) {
                return Err(Error::InvalidInput(
                    "ROI bin outside the IVH lattice".into(),
                ));
            }
            count += 1;
            v10 += usize::from(bin >= threshold10);
            v90 += usize::from(bin >= threshold90);
            least = least.min(bin);
            greatest = greatest.max(bin);
            // Each sample contributes one trapezoid half-step short of its bin,
            // except in the last bin where survival extends to the endpoint.
            twice_area += 2 * bin as u128 - if bin == levels { 2 } else { 1 };
            if need_ranks {
                ranks.push(bin);
            }
        }
    }
    let defined = levels > 1 && count > 0;
    // The first admissible intensity is one bin above the last excluded sample.
    // Its zero-based calibration index equals that sample's one-based bin.
    let mut index_at_volume = |q: u128| {
        if !defined || !need_ranks {
            return None;
        }
        let allowed = (count as u128 * q / 100) as usize;
        let index = *ranks.select_nth_unstable(count - allowed - 1).1;
        (index < levels).then_some(index)
    };
    let (i10, i90) = (index_at_volume(10), index_at_volume(90));
    let intensity =
        |index: Option<i32>| index.map_or(f64::NAN, |i| step.mul_add(i as f64, *origin));
    let volume = |n| {
        if defined {
            n as f64 / count as f64
        } else {
            f64::NAN
        }
    };
    names
        .iter()
        .map(|&key| {
            let name = key.strip_prefix("ivh_");
            let value = match name {
                Some("VolumeAtIntensityFraction10") => volume(v10),
                Some("VolumeAtIntensityFraction90") => volume(v90),
                Some("IntensityAtVolumeFraction10") => intensity(i10),
                Some("IntensityAtVolumeFraction90") => intensity(i90),
                Some("VolumeFractionDifference") => volume(v10 - v90),
                Some("IntensityFractionDifference") => i10
                    .zip(i90)
                    .map_or(f64::NAN, |(a, b)| (a - b) as f64 * step),
                Some("AreaUnderIVHCurve") => {
                    if count == 0 {
                        f64::NAN
                    } else if least == greatest {
                        0.
                    } else {
                        twice_area as f64 / (2 * (levels - 1) as u128 * count as u128) as f64
                    }
                }
                _ => {
                    return Err(Error::InvalidInput(format!(
                        "unknown IVH statistics: {}",
                        name.unwrap_or(key)
                    )));
                }
            };
            Ok((key.to_owned(), value))
        })
        .collect()
}
