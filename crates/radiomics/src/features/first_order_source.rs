//! CPU admission checks for the float32 feature-report contract.
//!
//! Mirrors `mlxradiomics/features/first_order_source.py`. Rust keeps the original
//! float64 samples for arithmetic. These checks preserve the reference errors
//! for selections that would lose a meaningful source difference.

use crate::features::first_order::FirstOrderFeature as F;
use crate::numerics::precision::pair_representation_error;
use crate::{Error, Result};

const MIN_NORMAL: f64 = f32::MIN_POSITIVE as f64;
const PAIR_RESOLUTION: f64 = 1.0 / (1 << 20) as f64;

fn is_flushed(value: f64) -> bool {
    value != 0. && value.abs() < MIN_NORMAL
}

pub(super) fn uses_shift(f: F) -> bool {
    matches!(f, F::Energy | F::TotalEnergy | F::RootMeanSquared)
}

fn uses_spread(f: F) -> bool {
    matches!(
        f,
        F::CoefficientOfVariation
            | F::InterquartileRange
            | F::Kurtosis
            | F::MeanAbsoluteDeviation
            | F::MedianAbsoluteDeviation
            | F::QuartileCoefficientOfDispersion
            | F::Range
            | F::RobustMeanAbsoluteDeviation
            | F::Skewness
            | F::Variance
    )
}

pub(super) fn check_samples(samples: &[f64], selected: &[F], shift: f64) -> Result<()> {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    let mut any_flushed = false;
    let mut gap = 0.0_f64;
    let spread = selected.iter().copied().any(uses_spread);
    let shifted = selected.iter().copied().any(uses_shift) && (shift as f32).is_finite();
    for &value in samples {
        if !value.is_finite() {
            return Err(Error::InvalidInput(
                "sampled image intensities must be finite".into(),
            ));
        }
        minimum = minimum.min(value);
        maximum = maximum.max(value);
        any_flushed |= is_flushed(value);
        if spread || shifted {
            gap = gap.max(pair_representation_error(value));
        }
    }
    if any_flushed
        && selected.iter().any(|f| match f {
            F::Minimum => is_flushed(minimum),
            F::Maximum => is_flushed(maximum),
            _ => true,
        })
    {
        let mut names: Vec<_> = selected.iter().map(|feature| feature.name()).collect();
        names.sort_unstable();
        return Err(Error::Execution(format!(
            "first-order intensities are unresolved for requested features {names:?}: \
             nonzero float64 sources below 2**-126 are flushed to zero by float32 arithmetic on Metal"
        )));
    }
    if spread && gap > 0. && minimum != maximum {
        // Center locally before taking the deviation so that high-offset samples
        // that differ by a few float64 ulps retain their scale.
        let mean_offset = samples.iter().map(|v| v - minimum).sum::<f64>() / samples.len() as f64;
        let variance = samples
            .iter()
            .map(|v| ((v - minimum) - mean_offset).powi(2))
            .sum::<f64>()
            / samples.len() as f64;
        if gap > PAIR_RESOLUTION * variance.sqrt() {
            return Err(Error::Execution(
                "first-order source spread is unresolved at float32 pair precision".into(),
            ));
        }
    }
    if shifted {
        let amplitude = (maximum + shift).abs().max((minimum + shift).abs());
        if amplitude != 0. && gap + pair_representation_error(shift) > PAIR_RESOLUTION * amplitude {
            return Err(Error::Execution(
                "first-order shifted energy is unresolved at float32 pair precision".into(),
            ));
        }
    }
    Ok(())
}
