//! Feature validity counts, mirroring `mlxradiomics/contracts/validity.py`.
//! Single-threaded CPU orchestration over evaluated samples.
//! Status: ported.

use serde::{Deserialize, Serialize};

/// Computed samples by category, plus the count of excluded background voxels.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeatureValidity {
    /// All finite samples, including zero.
    pub finite: usize,
    /// Subset of finite samples equal to positive or negative zero.
    pub valid_zero: usize,
    /// Undefined samples.
    pub nan: usize,
    /// Positively unbounded samples.
    pub positive_infinity: usize,
    /// Negatively unbounded samples.
    pub negative_infinity: usize,
    /// Uncomputed background voxels.
    pub excluded: usize,
}

/// Initialise counts before evaluating a feature.
pub fn empty_validity(excluded: usize) -> FeatureValidity {
    FeatureValidity {
        excluded,
        ..Default::default()
    }
}

/// Accumulate one batch of evaluated ROI or voxel values. Excluded samples stay separate.
pub fn accumulate_validity<'a, T: Copy + Into<f64> + 'a>(
    counts: &mut FeatureValidity,
    values: impl IntoIterator<Item = &'a T>,
) {
    for &value in values {
        let value = value.into();
        if value.is_finite() {
            counts.finite += 1;
            counts.valid_zero += usize::from(value == 0.0);
        } else if value.is_nan() {
            counts.nan += 1;
        } else if value.is_sign_positive() {
            counts.positive_infinity += 1;
        } else {
            counts.negative_infinity += 1;
        }
    }
}
