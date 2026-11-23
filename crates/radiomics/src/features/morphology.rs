//! Intensity-dependent IBSI morphological features.
//!
//! Mirrors `mlxradiomics/features/morphology.py` (495 lines at mlx-radiomics b58783c).
//!
//! The morphological family uses two masks. The intensity mask defines the voxels whose
//! intensities enter an intensity-weighted quantity; the morphological mask defines the
//! shape whose volume and geometric center are used. The IBSI manual distinguishes these
//! masks because resegmentation can remove high- or low-intensity voxels from the intensity
//! set while leaving the segmented shape intact.
//!
//! Execution: CPU numerical work in float64. The centre-of-mass shift and integrated
//! intensity are single-threaded compensated (Neumaier) sums. Moran's I and Geary's C
//! evaluate every unordered voxel pair once, grouped by x-run with the weights shared per
//! x offset ([`crate::features::geometry::pairwise`]; run totals are compensated and merged
//! in a fixed order so the result does not depend on the thread count); the quadratic pass runs only when selected
//! and is refused above [`MAX_PAIRWISE_VOXELS`] intensity-mask voxels. The float32 source
//! splitting and Metal pair kernel of the Python module are not needed in float64.
//! Input contract: image, intensity mask and morphology mask share one 3-D shape.
//!
//! Status: done (CentreOfMassShift, IntegratedIntensity, MoransI, GearysC).

use std::collections::HashSet;

use ndarray::ArrayD;

use crate::config::settings::Settings;
use crate::contracts::types::{Features, Prepared};
use crate::error::{Error, Result};
use crate::features::geometry::arithmetic::{NeumaierSum, compensated_sum};
use crate::features::geometry::mesh::{Coordinates, MeshGeometryContext};
use crate::features::geometry::pairwise::inverse_distance_sums;

/// The intensity-dependent morphology feature names (bare, without `shape_`).
pub const INTENSITY_MORPHOLOGY: [&str; 4] = [
    "CentreOfMassShift",
    "IntegratedIntensity",
    "MoransI",
    "GearysC",
];

/// Moran's I and Geary's C sum over every voxel pair; 2^20 voxels are about 5.5e11 distinct
/// pairs. Larger ROIs are refused before any feature work starts.
pub const MAX_PAIRWISE_VOXELS: usize = 1 << 20;

fn thousands(value: usize) -> String {
    let digits = value.to_string();
    let mut out = String::new();
    for (index, ch) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

/// Refuse Moran's I or Geary's C when the intensity mask exceeds `limit` voxels.
fn require_pairwise_admission_with(
    features: &HashSet<&str>,
    voxels: usize,
    limit: usize,
) -> Result<()> {
    let mut selected: Vec<&str> = ["GearysC", "MoransI"]
        .into_iter()
        .filter(|name| features.contains(name))
        .collect();
    selected.sort_unstable();
    if !selected.is_empty() && voxels > limit {
        return Err(Error::InvalidInput(format!(
            "{} require quadratic work for {} ROI voxels, above the {}-voxel limit for \
             quadratic spatial autocorrelation; deselect them or use a smaller or coarser ROI",
            selected.join(" and "),
            thousands(voxels),
            thousands(limit),
        )));
    }
    Ok(())
}

/// Refuse Moran's I or Geary's C when the intensity mask exceeds the pair budget.
///
/// `features` are bare shape names; `voxels` counts the intensity mask after resegmentation.
///
/// # Errors
///
/// [`Error::InvalidInput`] naming the count and the limit.
pub fn require_pairwise_admission(features: &[&str], voxels: usize) -> Result<()> {
    require_pairwise_admission_with(
        &features.iter().copied().collect(),
        voxels,
        MAX_PAIRWISE_VOXELS,
    )
}

/// Distance between the geometric and the intensity-weighted centroids.
///
/// The geometric centroid is that of the morphological mask; the weighted centroid uses the
/// intensity mask with the (signed) intensities as weights. Constant weights on matching
/// masks give exactly zero shift. Sums are Neumaier-compensated.
fn centroid_shift(
    values: &[f64],
    intensity_coordinates: &Coordinates,
    morphology_coordinates: &Coordinates,
    spacing: &[f64],
) -> Result<f64> {
    let ndim = morphology_coordinates.ndim();
    let reference = morphology_coordinates.row(0).to_vec();
    let count = morphology_coordinates.len() as f64;
    let weight_scale = values.iter().fold(0.0f64, |acc, v| acc.max(v.abs()));
    if weight_scale == 0.0 {
        return Err(Error::Execution(
            "intensity-weighted centroid is undefined for zero intensity".into(),
        ));
    }
    // Constant nonzero weights cancel from the centroid ratio exactly.
    let constant = values.iter().all(|v| *v == values[0]);
    let weights: Vec<f64> = if constant {
        vec![1.0; values.len()]
    } else {
        values.iter().map(|v| v / weight_scale).collect()
    };
    let denominator = compensated_sum(&weights);
    if denominator == 0.0 {
        return Err(Error::Execution(
            "intensity morphology total intensity is too ill-conditioned to resolve".into(),
        ));
    }
    let mut shift = vec![0.0f64; ndim];
    for axis in 0..ndim {
        // Geometric centroid, exact from the integer coordinate sum.
        let sum: i128 = morphology_coordinates
            .rows()
            .map(|row| i128::from(row[axis] - reference[axis]))
            .sum();
        let geometric = sum as f64 / count;
        let mut numerator = NeumaierSum::new();
        for (row, weight) in intensity_coordinates.rows().zip(&weights) {
            numerator.add(weight * (row[axis] - reference[axis]) as f64);
        }
        shift[axis] = (numerator.value() / denominator - geometric) * spacing[axis];
    }
    let largest = shift.iter().fold(0.0f64, |acc, v| acc.max(v.abs()));
    if !largest.is_finite() {
        return Err(Error::Execution(
            "CentreOfMassShift exceeds the float64 range".into(),
        ));
    }
    if largest == 0.0 {
        return Ok(0.0);
    }
    Ok(largest
        * shift
            .iter()
            .map(|v| (v / largest).powi(2))
            .sum::<f64>()
            .sqrt())
}

/// Mesh volume times the mean intensity over the intensity mask.
fn integrated_intensity(values: &[f64], mesh_context: &mut MeshGeometryContext<'_>) -> Result<f64> {
    let enclosed = mesh_context.get(false)?.enclosed;
    let mean = compensated_sum(values) / values.len() as f64;
    let result = enclosed * mean;
    if !result.is_finite() {
        return Err(Error::Execution(
            "IntegratedIntensity exceeds the float64 range".into(),
        ));
    }
    if result == 0.0 && mean != 0.0 {
        return Err(Error::Execution(
            "IntegratedIntensity underflows the float64 range".into(),
        ));
    }
    Ok(result)
}

/// Moran's I and/or Geary's C with inverse-Euclidean-distance weights.
///
/// Every unordered pair is visited once ([`inverse_distance_sums`], rayon over x-runs of
/// voxels) and the totals are doubled to give the ordered-pair sums. NaN for a constant
/// intensity ROI or fewer than two intensity-mask voxels, where the weighted variance
/// denominator is zero.
fn spatial_autocorrelation(
    values: &[f64],
    coordinates: &Coordinates,
    spacing: &[f64],
    names: &HashSet<&str>,
) -> Vec<(&'static str, f64)> {
    let count = values.len();
    let nan = |names: &HashSet<&str>| {
        ["MoransI", "GearysC"]
            .into_iter()
            .filter(|name| names.contains(name))
            .map(|name| (name, f64::NAN))
            .collect::<Vec<_>>()
    };
    if count < 2 {
        return nan(names);
    }
    let reference = values[0];
    if values.iter().all(|v| *v == reference) {
        return nan(names);
    }
    let mean = compensated_sum(values) / count as f64;
    let centered: Vec<f64> = values.iter().map(|v| v - mean).collect();
    let denominator = compensated_sum(&centered.iter().map(|v| v * v).collect::<Vec<_>>());
    let totals = inverse_distance_sums(&centered, coordinates, spacing);
    // Doubling turns unordered-pair totals into ordered ones.
    let total_weight = 2.0 * totals[0];
    let moran_numerator = 2.0 * totals[1];
    let geary_numerator = 2.0 * totals[2];
    let mut out = Vec::new();
    if names.contains("MoransI") {
        out.push((
            "MoransI",
            if total_weight > 0.0 {
                (count as f64 / total_weight) * moran_numerator / denominator
            } else {
                f64::NAN
            },
        ));
    }
    if names.contains("GearysC") {
        out.push((
            "GearysC",
            if total_weight > 0.0 {
                ((count - 1) as f64 / (2.0 * total_weight)) * geary_numerator / denominator
            } else {
                f64::NAN
            },
        ));
    }
    out
}

fn morphology_statistics_with(
    image: &ArrayD<f64>,
    intensity_mask: &ArrayD<bool>,
    morphology_mask: &ArrayD<bool>,
    spacing: &[f64],
    names: &[&str],
    limit: usize,
) -> Result<Features> {
    let requested: HashSet<&str> = names.iter().copied().collect();
    let mut unsupported: Vec<&str> = requested
        .iter()
        .copied()
        .filter(|name| !INTENSITY_MORPHOLOGY.contains(name))
        .collect();
    if !unsupported.is_empty() {
        unsupported.sort_unstable();
        return Err(Error::InvalidInput(format!(
            "unsupported intensity morphology features: {}",
            unsupported.join(", ")
        )));
    }
    if requested.is_empty() {
        return Ok(Vec::new());
    }
    let voxels = intensity_mask.iter().filter(|&&v| v).count();
    require_pairwise_admission_with(&requested, voxels, limit)?;
    if image.shape() != intensity_mask.shape() || image.shape() != morphology_mask.shape() {
        return Err(Error::InvalidInput(
            "image and both masks must have the same shape".into(),
        ));
    }
    if morphology_mask.ndim() != 3 {
        return Err(Error::InvalidInput(
            "intensity morphology requires three-dimensional masks".into(),
        ));
    }
    if spacing.len() != 3 {
        return Err(Error::InvalidInput(
            "spacing must contain one value per mask axis".into(),
        ));
    }
    if spacing.iter().any(|v| !v.is_finite() || *v <= 0.0) {
        return Err(Error::InvalidInput(
            "spacing values must be finite and positive".into(),
        ));
    }
    if !morphology_mask.iter().any(|&v| v) {
        return Err(Error::InvalidInput("morphology mask is empty".into()));
    }
    if voxels == 0 {
        return Err(Error::InvalidInput("intensity mask is empty".into()));
    }
    let intensity_coordinates = Coordinates::from_mask(intensity_mask);
    let values: Vec<f64> = intensity_mask
        .indexed_iter()
        .filter(|(_, selected)| **selected)
        .map(|(index, _)| image[index])
        .collect();
    if values.iter().any(|v| !v.is_finite()) {
        return Err(Error::InvalidInput(
            "intensity morphology requires finite source intensities".into(),
        ));
    }
    let mut context = MeshGeometryContext::new(morphology_mask, spacing);
    let mut computed: Vec<(&str, f64)> = Vec::new();
    if requested.contains("CentreOfMassShift") {
        let shift = centroid_shift(
            &values,
            &intensity_coordinates,
            context.coordinates(),
            spacing,
        )?;
        computed.push(("CentreOfMassShift", shift));
    }
    if requested.contains("IntegratedIntensity") {
        computed.push((
            "IntegratedIntensity",
            integrated_intensity(&values, &mut context)?,
        ));
    }
    let pair_names: HashSet<&str> = requested
        .iter()
        .copied()
        .filter(|name| matches!(*name, "MoransI" | "GearysC"))
        .collect();
    if !pair_names.is_empty() {
        computed.extend(spatial_autocorrelation(
            &values,
            &intensity_coordinates,
            spacing,
            &pair_names,
        ));
    }
    let mut seen = HashSet::new();
    Ok(names
        .iter()
        .filter(|name| seen.insert(**name))
        .map(|name| {
            let value = computed
                .iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| *value)
                .expect("every requested feature was computed");
            (format!("shape_{name}"), value)
        })
        .collect())
}

/// Compute selected intensity-dependent morphological features.
///
/// `image` is the intensity image, `intensity_mask` selects the voxels whose intensities
/// enter weighted quantities and spatial autocorrelation, and `morphology_mask` the
/// segmented shape used for mesh volume and the geometric centroid. `names` are bare names
/// from [`INTENSITY_MORPHOLOGY`]. Returns `shape_<Name>` keys in the order of `names`.
/// Moran's I and Geary's C are NaN for a constant intensity ROI or fewer than two
/// intensity-mask voxels.
///
/// # Errors
///
/// [`Error::InvalidInput`] if shapes, dimensions, spacing or feature names are invalid, a
/// required mask is empty, or Moran's I or Geary's C is requested for more than
/// [`MAX_PAIRWISE_VOXELS`] intensity-mask voxels. [`Error::Execution`] if the
/// intensity-weighted centroid is undefined or a result is not finite.
pub fn morphology_statistics(
    image: &ArrayD<f64>,
    intensity_mask: &ArrayD<bool>,
    morphology_mask: &ArrayD<bool>,
    spacing: &[f64],
    names: &[&str],
) -> Result<Features> {
    morphology_statistics_with(
        image,
        intensity_mask,
        morphology_mask,
        spacing,
        names,
        MAX_PAIRWISE_VOXELS,
    )
}

/// Engine entry point: compute the intensity-morphology features of a prepared input.
///
/// The intensity mask is `prepared.input.roi`, the morphology mask
/// `prepared.input.shape_roi` and the intensities `prepared.input.image`. `names` are full
/// keys (`shape_CentreOfMassShift`, `shape_IntegratedIntensity`, `shape_MoransI`,
/// `shape_GearysC`); the output keeps their order.
///
/// # Errors
///
/// [`Error::InvalidInput`] for a name that is not one of the four keys, plus everything
/// [`morphology_statistics`] returns.
pub fn compute(
    prepared: &Prepared,
    spacing: &[f64],
    _settings: &Settings,
    names: &[&str],
) -> Result<Features> {
    let mut bare: Vec<&str> = Vec::with_capacity(names.len());
    for &name in names {
        match name.strip_prefix("shape_") {
            Some(stripped) if INTENSITY_MORPHOLOGY.contains(&stripped) => bare.push(stripped),
            _ => {
                return Err(Error::InvalidInput(format!(
                    "unsupported intensity morphology features: {name}"
                )));
            }
        }
    }
    morphology_statistics(
        &prepared.input.image,
        &prepared.input.roi,
        &prepared.input.shape_roi,
        spacing,
        &bare,
    )
}
