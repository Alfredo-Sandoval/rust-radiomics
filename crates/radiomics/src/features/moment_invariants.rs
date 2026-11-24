//! Three-dimensional moment invariants of the ROI shape and intensity.
//!
//! Mirrors `mlxradiomics/features/moment_invariants.py`.
//!
//! The family follows the ten moment invariants of PySERA 2.1.5
//! (`MomentInvariantsFeaturesExtractor`): five invariants of a density `f` over the voxel
//! centers, evaluated once for the shape (`f = 1` on the morphological mask) and once for
//! the intensity (`f = (I - min) / (mean - min)` on the intensity mask, so the density is
//! nonnegative with mean one). With the density's mass `m000`, centroid and central
//! moments `mu_pqr`, the scale-normalized moments are
//!
//! ```text
//! eta_pqr = mu_pqr / m000 ^ ((p + q + r) / 3 + 1)
//! ```
//!
//! and the invariants are, for the second-order tensor `A` and third-order tensor `T`,
//!
//! ```text
//! I1 = trace(A)                     F2 = sum(A_ij ^ 2)
//! I2 = second principal invariant   I3 = det(A)
//! F3 = sum(T_ijk ^ 2)
//! ```
//!
//! All five are unchanged by translation, rotation and isotropic scaling of the ROI; the
//! intensity set is also unchanged by an affine intensity transform with positive gain.
//!
//! Moments use physical voxel-center coordinates and the voxel volume, so anisotropic
//! spacing is honored. PySERA uses voxel indices; the two agree on an isotropic grid. Both
//! densities have mass `n * voxel_volume` for `n` mask voxels, which makes every `eta_pqr`
//! a density-weighted mean of products of the centered coordinates divided by
//! `(n * voxel_volume) ^ (1 / 3)`. Weighted means do not depend on the scale of the
//! weights, so the intensity density enters as `I - min` and its mean is never formed.
//!
//! Execution: single-threaded CPU numerical work in float64 with Neumaier-compensated
//! sums. The double-single Metal kernel of the Python module is not needed in float64.
//! Input contract: image, intensity mask and morphology mask share one 3-D shape.

use std::array::from_fn;
use std::collections::HashSet;

use ndarray::ArrayD;

use crate::config::settings::Settings;
use crate::contracts::types::{Features, Prepared};
use crate::error::{Error, Result};
use crate::features::geometry::arithmetic::NeumaierSum;
use crate::features::geometry::mesh::Coordinates;

/// The invariants of one density, in output order.
pub const INVARIANTS: [&str; 5] = ["I1", "F2", "I2", "I3", "F3"];

/// The moment-invariant feature names (bare, without `momentinvariant_`).
pub const MOMENT_INVARIANTS: [&str; 10] = [
    "IntensityI1",
    "IntensityF2",
    "IntensityI2",
    "IntensityI3",
    "IntensityF3",
    "ShapeI1",
    "ShapeF2",
    "ShapeI2",
    "ShapeI3",
    "ShapeF3",
];

/// Axis pairs of the second-order moments: eta200, eta020, eta002, eta110, eta101, eta011.
const SECOND: [[usize; 2]; 6] = [[0, 0], [1, 1], [2, 2], [0, 1], [0, 2], [1, 2]];
/// Axis triples of the third-order moments: eta300, eta030, eta003, eta210, eta201,
/// eta120, eta102, eta021, eta012, eta111.
const THIRD: [[usize; 3]; 10] = [
    [0, 0, 0],
    [1, 1, 1],
    [2, 2, 2],
    [0, 0, 1],
    [0, 0, 2],
    [0, 1, 1],
    [0, 2, 2],
    [1, 1, 2],
    [1, 2, 2],
    [0, 1, 2],
];

/// The five invariants of one density over `coordinates`.
///
/// `weights` holds one nonnegative value per coordinate row, or `None` for the unit
/// density. All five are NaN when the weights sum to zero.
fn invariants(coordinates: &Coordinates, weights: Option<&[f64]>, spacing: &[f64]) -> [f64; 5] {
    let count = coordinates.len() as f64;
    // Offsets from the bounding-box corner are exact small integers, and an axis the ROI
    // does not extend along contributes exact zeros to every moment.
    let mut origin = [i64::MAX; 3];
    for row in coordinates.rows() {
        for axis in 0..3 {
            origin[axis] = origin[axis].min(row[axis]);
        }
    }
    // eta_pqr is a weighted mean of coordinate products after dividing each coordinate by
    // cbrt(count * voxel_volume). The geometric-mean spacing keeps the cube root in range
    // for spacings whose product is not representable.
    let geometric_mean = (spacing.iter().map(|value| value.ln()).sum::<f64>() / 3.0).exp();
    let scale: [f64; 3] = from_fn(|axis| spacing[axis] / geometric_mean / count.cbrt());
    let weight = |index: usize| weights.map_or(1.0, |values| values[index]);
    let position = |row: &[i64]| -> [f64; 3] {
        from_fn(|axis| (row[axis] - origin[axis]) as f64 * scale[axis])
    };

    let mut total = NeumaierSum::new();
    let mut first = [NeumaierSum::new(); 3];
    for (index, row) in coordinates.rows().enumerate() {
        let w = weight(index);
        let p = position(row);
        total.add(w);
        for axis in 0..3 {
            first[axis].add(w * p[axis]);
        }
    }
    let total = total.value();
    if total == 0.0 {
        return [f64::NAN; 5];
    }
    let center: [f64; 3] = from_fn(|axis| first[axis].value() / total);

    let mut second = [NeumaierSum::new(); 6];
    let mut third = [NeumaierSum::new(); 10];
    for (index, row) in coordinates.rows().enumerate() {
        let w = weight(index);
        let p = position(row);
        let x: [f64; 3] = from_fn(|axis| p[axis] - center[axis]);
        for (sum, [a, b]) in second.iter_mut().zip(SECOND) {
            sum.add(w * x[a] * x[b]);
        }
        for (sum, [a, b, c]) in third.iter_mut().zip(THIRD) {
            sum.add(w * x[a] * x[b] * x[c]);
        }
    }
    let [xx, yy, zz, xy, xz, yz] = second.map(|sum| sum.value() / total);
    let t = third.map(|sum| sum.value() / total);
    let off = xy * xy + xz * xz + yz * yz;
    let pure: f64 = t[..3].iter().map(|v| v * v).sum();
    let mixed: f64 = t[3..9].iter().map(|v| v * v).sum();
    [
        xx + yy + zz,
        xx * xx + yy * yy + zz * zz + 2.0 * off,
        xx * yy + xx * zz + yy * zz - off,
        xx * yy * zz + 2.0 * xy * xz * yz - zz * xy * xy - yy * xz * xz - xx * yz * yz,
        pure + 3.0 * mixed + 6.0 * t[9] * t[9],
    ]
}

/// Compute selected moment invariants of the ROI shape and intensity.
///
/// `image` is the intensity image, `intensity_mask` selects the voxels whose intensities
/// form the intensity density, and `morphology_mask` is the segmented shape that forms the
/// unit density. `names` are bare names from [`MOMENT_INVARIANTS`]. Returns dimensionless
/// `momentinvariant_<Name>` keys in the order of `names`. The intensity invariants are NaN
/// for a constant intensity ROI, whose density `(I - min) / (mean - min)` is undefined.
///
/// # Errors
///
/// [`Error::InvalidInput`] if shapes, dimensions, spacing or feature names are invalid, a
/// required mask is empty, or a selected intensity is not finite. [`Error::Execution`] if
/// the intensity range overflows float64.
pub fn moment_invariant_statistics(
    image: &ArrayD<f64>,
    intensity_mask: &ArrayD<bool>,
    morphology_mask: &ArrayD<bool>,
    spacing: &[f64],
    names: &[&str],
) -> Result<Features> {
    let requested: HashSet<&str> = names.iter().copied().collect();
    let mut unsupported: Vec<&str> = requested
        .iter()
        .copied()
        .filter(|name| !MOMENT_INVARIANTS.contains(name))
        .collect();
    if !unsupported.is_empty() {
        unsupported.sort_unstable();
        return Err(Error::InvalidInput(format!(
            "unsupported moment invariants: {}",
            unsupported.join(", ")
        )));
    }
    if requested.is_empty() {
        return Ok(Features::default());
    }
    if image.shape() != intensity_mask.shape() || image.shape() != morphology_mask.shape() {
        return Err(Error::InvalidInput(
            "image and both masks must have the same shape".into(),
        ));
    }
    if morphology_mask.ndim() != 3 {
        return Err(Error::InvalidInput(
            "moment invariants require three-dimensional masks".into(),
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
    let mut shape = None;
    if requested.iter().any(|name| name.starts_with("Shape")) {
        if !morphology_mask.iter().any(|&v| v) {
            return Err(Error::InvalidInput("morphology mask is empty".into()));
        }
        shape = Some(invariants(
            &Coordinates::from_mask(morphology_mask),
            None,
            spacing,
        ));
    }
    let mut intensity = None;
    if requested.iter().any(|name| name.starts_with("Intensity")) {
        let values: Vec<f64> = intensity_mask
            .indexed_iter()
            .filter(|(_, selected)| **selected)
            .map(|(index, _)| image[index])
            .collect();
        if values.is_empty() {
            return Err(Error::InvalidInput("intensity mask is empty".into()));
        }
        if values.iter().any(|v| !v.is_finite()) {
            return Err(Error::InvalidInput(
                "moment invariants require finite source intensities".into(),
            ));
        }
        let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
        let range = values.iter().copied().fold(f64::NEG_INFINITY, f64::max) - minimum;
        if !range.is_finite() {
            return Err(Error::Execution(
                "moment invariant intensity range exceeds the float64 range".into(),
            ));
        }
        intensity = Some(if range == 0.0 {
            [f64::NAN; 5]
        } else {
            // Weighted means ignore the weight scale; unit-range weights cannot overflow.
            let weights: Vec<f64> = values.iter().map(|v| (v - minimum) / range).collect();
            invariants(
                &Coordinates::from_mask(intensity_mask),
                Some(&weights),
                spacing,
            )
        });
    }
    let mut seen = HashSet::new();
    Ok(names
        .iter()
        .filter(|name| seen.insert(**name))
        .map(|name| {
            let (density, suffix) = match name.strip_prefix("Shape") {
                Some(suffix) => (shape, suffix),
                None => (intensity, &name["Intensity".len()..]),
            };
            let index = INVARIANTS
                .iter()
                .position(|invariant| *invariant == suffix)
                .expect("validated invariant name");
            let value = density.expect("every requested density was computed")[index];
            (format!("momentinvariant_{name}"), value)
        })
        .collect())
}

/// Engine entry point: compute the moment invariants of a prepared input.
///
/// The intensity mask is `prepared.input.roi`, the morphology mask
/// `prepared.input.shape_roi` and the intensities `prepared.input.image`. `names` are full
/// `momentinvariant_<Name>` keys; the output keeps their order.
///
/// # Errors
///
/// [`Error::InvalidInput`] for a name outside the family, plus everything
/// [`moment_invariant_statistics`] returns.
pub fn compute(
    prepared: &Prepared,
    spacing: &[f64],
    _settings: &Settings,
    names: &[&str],
) -> Result<Features> {
    let mut bare: Vec<&str> = Vec::with_capacity(names.len());
    for &name in names {
        match name.strip_prefix("momentinvariant_") {
            Some(stripped) => bare.push(stripped),
            None => {
                return Err(Error::InvalidInput(format!(
                    "unsupported moment invariants: {name}"
                )));
            }
        }
    }
    moment_invariant_statistics(
        &prepared.input.image,
        &prepared.input.roi,
        &prepared.input.shape_roi,
        spacing,
        &bare,
    )
}
