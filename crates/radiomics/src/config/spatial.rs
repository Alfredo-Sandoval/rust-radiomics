//! Host-only physical-grid and interpolation settings.
//!
//! Mirrors `mlxradiomics/config/spatial.py` (118 lines at mlx-radiomics b58783c).
//! Single-threaded CPU orchestration.
//! Status: settings and validation ported.
//! All spacings are in NumPy axis order (z, y, x).

string_enum! {
    /// Image interpolator for resampling.
    Interpolator { Nearest => "nearest", Linear => "linear", Bspline => "bspline" }
    default Bspline
}

/// IBSI reference-grid interpolation: centred grid of `ceil(n * s / t)` voxels per axis.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct IbsiInterpolation {
    /// Target spacing per NumPy axis; 0 keeps the source spacing on that axis.
    pub spacing: Vec<f64>,
    /// Round interpolated intensities to the nearest integer, ties to even.
    pub round_intensities: bool,
    /// Partial-volume threshold for the interpolated mask, in (0, 1].
    pub mask_threshold: f64,
    /// Image interpolator; only `Linear` or `Bspline`.
    pub image_interpolator: Interpolator,
}

impl IbsiInterpolation {
    /// IBSI interpolation to `spacing` with the mlx-radiomics defaults.
    #[must_use]
    pub fn new(spacing: Vec<f64>) -> Self {
        Self {
            spacing,
            round_intensities: true,
            mask_threshold: 0.5,
            image_interpolator: Interpolator::Linear,
        }
    }
}

/// File-route spatial preprocessing.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct ImageSettings {
    /// PyRadiomics-style resampling target per NumPy axis; 0 keeps the source spacing.
    pub resampled_spacing: Option<Vec<f64>>,
    /// Image interpolator for `resampled_spacing`.
    pub interpolator: Interpolator,
    /// Resample a mismatched mask onto the image grid with nearest neighbour.
    pub correct_mask: bool,
    /// Voxels kept around the ROI bounding box after resampling.
    pub pad_distance: usize,
    /// IBSI reference-grid interpolation; exclusive with `resampled_spacing` and `correct_mask`.
    pub ibsi_interpolation: Option<IbsiInterpolation>,
}

impl Default for ImageSettings {
    fn default() -> Self {
        Self {
            resampled_spacing: None,
            interpolator: Interpolator::Bspline,
            correct_mask: false,
            pad_distance: 5,
            ibsi_interpolation: None,
        }
    }
}

use crate::config::validation::invalid;
use crate::error::Result;

fn valid_spacing(spacing: &[f64]) -> bool {
    matches!(spacing.len(), 2 | 3) && spacing.iter().all(|n| n.is_finite() && *n >= 0.0)
}

impl IbsiInterpolation {
    /// Check the target grid, mask occupancy threshold and intensity interpolator.
    pub fn validate(&self) -> Result<()> {
        if !valid_spacing(&self.spacing) {
            return invalid("IBSI spacing requires 2 or 3 finite nonnegative values");
        }
        if !(self.mask_threshold > 0.0 && self.mask_threshold <= 1.0) {
            return invalid("mask_threshold must be finite and in (0, 1]");
        }
        if self.image_interpolator == Interpolator::Nearest {
            return invalid("image_interpolator must be linear or bspline");
        }
        Ok(())
    }
}

impl ImageSettings {
    /// Check spatial settings including any nested IBSI interpolation request.
    pub fn validate(&self) -> Result<()> {
        if self
            .resampled_spacing
            .as_ref()
            .is_some_and(|s| !valid_spacing(s))
        {
            return invalid("resampled_spacing requires 2 or 3 finite nonnegative values");
        }
        if let Some(ibsi) = &self.ibsi_interpolation {
            ibsi.validate()?;
            if self.resampled_spacing.is_some() {
                return invalid("ibsi_interpolation and resampled_spacing are mutually exclusive");
            }
            if self.correct_mask {
                return invalid(
                    "ibsi_interpolation interpolates the mask; correct_mask must be False",
                );
            }
        }
        Ok(())
    }
}
