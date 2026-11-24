//! IBSI shape features of 3-D masks (``shape``) and 2-D masks (``shape2D``).
//!
//! Mirrors `mlxradiomics/features/shape.py` (412 lines at mlx-radiomics b58783c) and the
//! `_roi_shapes` dispatch of `workflows/roi.py`. Covers the 28 geometric `shape` features
//! (mesh volume and area, diameters, PCA axes, the IBSI morphology extensions AABB, AEE,
//! OMBB, MVEE and convex-hull densities) and the nine `shape2D` features, with the
//! `covariance_divisor` (`n-1` IBSI, `n` PyRadiomics) and `ellipsoid_area` (`ibsi-series`,
//! `exact`) conventions and single-slice `shape2D` through `force2d_axis`. The four
//! intensity-weighted morphology features live in [`crate::features::morphology`].
//!
//! Execution: CPU numerical work in float64. The marching mesh, covariance, hull, MVEE
//! and AEE steps are single-threaded; the all-pairs diameters and the OMBB candidate search
//! use rayon (see [`crate::features::geometry::diameters`] and
//! [`crate::features::geometry::boxes`]). There is no GPU path and no placement
//! choice to verify. Input contract: boolean masks with 2 or 3 axes in (z, y, x) order and
//! positive spacing per axis.
//!
//! The float32 binary-exponent scaling of the Python module is not ported: float64 holds
//! the dimensional measures directly, and a non-finite result is reported as
//! [`Error::Execution`].
//!
//! Status: done (all 28 geometric shape features and all nine shape2D features).

use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

use ndarray::{ArrayD, Axis};

use crate::config::settings::{CovarianceDivisor, EllipsoidArea, Family, Settings, features};
use crate::contracts::types::{Features, Prepared};
use crate::error::{Error, Result};
use crate::features::geometry::arithmetic::{checked_measure, validate_spacing};
use crate::features::geometry::boxes::{centroid, facet_normals, ombb_measures};
use crate::features::geometry::diameters::{
    diameter_maxima, diameter_maxima_with_hull, hull_vertices_2d,
};
use crate::features::geometry::ellipsoids::{
    aee_densities, ellipsoid_area_normalized, enclosing_densities, mvee_semi_axes,
};
use crate::features::geometry::hull::{ConvexHull, convex_hull, hull_measures};
use crate::features::geometry::mesh::MeshGeometryContext;
use crate::features::geometry::moments::{coordinate_rank, shape_eigenvalues};
use crate::features::morphology::INTENSITY_MORPHOLOGY;

const SHAPE_MESH: &[&str] = &[
    "MeshVolume",
    "SurfaceArea",
    "SurfaceVolumeRatio",
    "Sphericity",
];
const SHAPE2D_MESH: &[&str] = &[
    "MeshSurface",
    "Perimeter",
    "PerimeterSurfaceRatio",
    "Sphericity",
];
const AXIS_FEATURES: &[&str] = &[
    "MajorAxisLength",
    "MinorAxisLength",
    "LeastAxisLength",
    "Elongation",
    "Flatness",
];
const SHAPE_DIAMETERS: &[&str] = &[
    "Maximum3DDiameter",
    "Maximum2DDiameterSlice",
    "Maximum2DDiameterColumn",
    "Maximum2DDiameterRow",
];
const SHAPE2D_DIAMETERS: &[&str] = &["MaximumDiameter"];
const MORPHOLOGY_3D: &[&str] = &[
    "Compactness1",
    "Compactness2",
    "SphericalDisproportion",
    "Asphericity",
    "VolumeDensityAxisAlignedBoundingBox",
    "AreaDensityAxisAlignedBoundingBox",
    "VolumeDensityApproximateEnclosingEllipsoid",
    "AreaDensityApproximateEnclosingEllipsoid",
    "VolumeDensityOrientedMinimumBoundingBox",
    "AreaDensityOrientedMinimumBoundingBox",
    "VolumeDensityMinimumVolumeEnclosingEllipsoid",
    "AreaDensityMinimumVolumeEnclosingEllipsoid",
    "VolumeDensityConvexHull",
    "AreaDensityConvexHull",
];
const HULL_FEATURES: &[&str] = &["VolumeDensityConvexHull", "AreaDensityConvexHull"];
const OMBB_FEATURES: &[&str] = &[
    "VolumeDensityOrientedMinimumBoundingBox",
    "AreaDensityOrientedMinimumBoundingBox",
];
const MVEE_FEATURES: &[&str] = &[
    "VolumeDensityMinimumVolumeEnclosingEllipsoid",
    "AreaDensityMinimumVolumeEnclosingEllipsoid",
];
const AEE_FEATURES: &[&str] = &[
    "VolumeDensityApproximateEnclosingEllipsoid",
    "AreaDensityApproximateEnclosingEllipsoid",
];
const AABB_FEATURES: &[&str] = &[
    "VolumeDensityAxisAlignedBoundingBox",
    "AreaDensityAxisAlignedBoundingBox",
];

fn any_of(requested: &HashSet<&str>, names: &[&str]) -> bool {
    names.iter().any(|name| requested.contains(name))
}

/// Bare names of the geometric features for a mask dimension, in catalog order.
#[must_use]
pub fn known_names(ndim: usize) -> Vec<&'static str> {
    let family = if ndim == 3 {
        Family::Shape
    } else {
        Family::Shape2D
    };
    features()
        .iter()
        .filter(|spec| spec.family == family && !INTENSITY_MORPHOLOGY.contains(&spec.name))
        .map(|spec| spec.name)
        .collect()
}

/// Dimensionless `(36 pi V^2)^(1/3) / A`.
fn sphericity_3d(volume: f64, area: f64) -> f64 {
    (36.0 * PI).cbrt() * volume.powf(2.0 / 3.0) / area
}

/// Dimensionless `2 sqrt(pi A) / P`.
fn sphericity_2d(surface: f64, perimeter: f64) -> f64 {
    2.0 * (PI * surface).sqrt() / perimeter
}

/// Compute the requested `shape` (3-D mask) or `shape2D` (2-D mask) features.
///
/// Only dependencies of the requested names are evaluated: count-only and axes-only
/// requests skip the marching mesh, mesh-only and diameter-only requests skip the
/// covariance spectrum, and the diameter search runs only for diameter features. With
/// `names = None` every geometric feature of the mask dimension's family is requested.
///
/// * Volume and area: MeshVolume / MeshSurface come from the marching mesh; VoxelVolume /
///   PixelSurface are voxel count times voxel volume (area). SurfaceArea (3-D) and
///   Perimeter (2-D) are the mesh boundary measure.
/// * Ratios: SurfaceVolumeRatio and PerimeterSurfaceRatio are boundary over enclosed
///   measure. Sphericity is `(36 pi V^2)^(1/3) / A` in 3-D and `2 sqrt(pi A) / P` in 2-D.
/// * Compactness 1 is `V / (sqrt(pi) A^(3/2))`; Compactness 2 is `36 pi V^2 / A^3`;
///   SphericalDisproportion is `A / (36 pi V^2)^(1/3)`; Asphericity is
///   `(A^3 / (36 pi V^2))^(1/3) - 1` (3-D only).
/// * Densities: AABB densities use the mask's axis-aligned bounding box, which is also the
///   mesh vertex bounding box. Convex-hull densities use the exact hull of the mesh
///   vertices. AEE volume and area use the PCA semi-axes; `EllipsoidArea::IbsiSeries`
///   (default) evaluates the AEE area with the series printed in the IBSI manual, which
///   reproduces the published area-density reference, and `Exact` uses the corrected
///   surface. OMBB uses the minimum-volume box among orientations with one face flush with
///   a hull face (an approximation of O'Rourke's exact optimum); equal minimum volumes
///   select the smallest area. MVEE uses Khachiyan barycentric coordinate ascent on the
///   hull vertices with the IBSI tolerance 0.001 and the exact ellipsoid area. All three
///   density pairs use the mesh volume and area.
/// * Diameters: the largest distance between boundary vertices. In 3-D,
///   Maximum2DDiameterSlice, Column and Row restrict to vertex pairs sharing their axis-0,
///   axis-1 and axis-2 coordinate respectively.
/// * Axis lengths: `4 * sqrt(eigenvalue)` of the ROI voxel-center covariance (spacing
///   applied). `CovarianceDivisor::NMinusOne` (default, IBSI) uses the sample divisor
///   `N - 1` and `N` the population divisor. A singleton has zero axes. AEE semi-axes use
///   the same divisor. Major and Minor use the largest and second-largest eigenvalues
///   (Least uses the smallest, 3-D only); Elongation is `sqrt(minor / major)` and Flatness
///   `sqrt(least / major)`.
///
/// Returns `shape_<Name>` or `shape2D_<Name>` keys in the order of `names` (or catalog
/// order for `None`).
///
/// # Errors
///
/// [`Error::InvalidInput`] if the spacing does not have one value per mask axis or is not
/// finite and positive, a name is not a shape feature for the mask dimension, the mask is
/// empty, or the mesh has no boundary vertices. [`Error::Execution`] if a dimensional
/// measure or diameter is not finite, the covariance spectrum is non-finite or below its
/// PSD bound, a requested positive axis is too ill-conditioned, the convex hull is
/// degenerate, or MVEE loses positive definiteness or exceeds its iteration cap.
pub fn shape_statistics<'a>(
    mask: &'a ArrayD<bool>,
    spacing: &[f64],
    names: Option<&[&str]>,
    covariance_divisor: CovarianceDivisor,
    ellipsoid_area: EllipsoidArea,
    mesh_context: Option<&mut MeshGeometryContext<'a>>,
) -> Result<Features> {
    let ndim = mask.ndim();
    if ndim != 2 && ndim != 3 {
        return Err(Error::InvalidInput(
            "shape features require a 2-D or 3-D mask".into(),
        ));
    }
    if spacing.len() != ndim {
        return Err(Error::InvalidInput(
            "spacing must contain one value per mask axis".into(),
        ));
    }
    let known = known_names(ndim);
    let order: Vec<&str> = match names {
        None => known.clone(),
        Some(names) => names.to_vec(),
    };
    let known_set: HashSet<&str> = known.iter().copied().collect();
    let requested: HashSet<&str> = order.iter().copied().collect();
    let mut unsupported: Vec<&str> = requested.difference(&known_set).copied().collect();
    if !unsupported.is_empty() {
        unsupported.sort_unstable();
        return Err(Error::InvalidInput(format!(
            "unsupported {ndim}D shape features: {}",
            unsupported.join(", ")
        )));
    }
    if requested.is_empty() {
        return Ok(Vec::new());
    }
    if !mask.iter().any(|&value| value) {
        return Err(Error::InvalidInput("shape mask is empty".into()));
    }
    validate_spacing(spacing)?;
    let mesh_features: Vec<&str> = if ndim == 3 {
        SHAPE_MESH.iter().chain(MORPHOLOGY_3D).copied().collect()
    } else {
        SHAPE2D_MESH.to_vec()
    };
    let diameter_features = if ndim == 3 {
        SHAPE_DIAMETERS
    } else {
        SHAPE2D_DIAMETERS
    };
    let needs_mesh = any_of(&requested, &mesh_features);
    let needs_diameter = any_of(&requested, diameter_features);
    let needs_axes = any_of(&requested, AXIS_FEATURES) || any_of(&requested, AEE_FEATURES);
    let needs_hull_geometry = any_of(&requested, HULL_FEATURES)
        || any_of(&requested, OMBB_FEATURES)
        || any_of(&requested, MVEE_FEATURES);
    let mut local_context;
    let context = match mesh_context {
        Some(context) => context,
        None => {
            local_context = MeshGeometryContext::new(mask, spacing);
            &mut local_context
        }
    };
    context.coordinates();
    if needs_mesh || needs_diameter {
        context.get(needs_diameter || needs_hull_geometry)?;
    }
    let (coordinates, geometry) = context.cached();
    let coordinates = coordinates.expect("coordinates were just computed");
    let geometry = if needs_mesh || needs_diameter {
        geometry
    } else {
        None
    };
    // The hull serves the hull, OMBB and MVEE features and bounds the 3-D diameter search.
    // A planar or collinear vertex set has no hull; only the hull features then fail.
    let grid: Vec<[f64; 3]> = match geometry {
        Some(geometry) if ndim == 3 && (needs_hull_geometry || needs_diameter) => geometry
            .vertices
            .chunks_exact(3)
            .map(|v| [v[0], v[1], v[2]])
            .collect(),
        _ => Vec::new(),
    };
    let hull: Option<Result<ConvexHull>> = (!grid.is_empty()).then(|| convex_hull(&grid));
    let mut values: HashMap<&str, f64> = HashMap::new();
    if requested.contains("VoxelVolume") || requested.contains("PixelSurface") {
        let physical = coordinates.len() as f64 * spacing.iter().product::<f64>();
        let name = if ndim == 2 {
            "PixelSurface"
        } else {
            "VoxelVolume"
        };
        values.insert(name, checked_measure(physical, name)?);
    }
    if let (true, Some(geometry)) = (needs_mesh, geometry) {
        if ndim == 2 {
            if requested.contains("MeshSurface") {
                values.insert(
                    "MeshSurface",
                    checked_measure(geometry.enclosed, "MeshSurface")?,
                );
            }
            if requested.contains("Perimeter") {
                values.insert(
                    "Perimeter",
                    checked_measure(geometry.boundary, "Perimeter")?,
                );
            }
            if requested.contains("PerimeterSurfaceRatio") {
                values.insert(
                    "PerimeterSurfaceRatio",
                    checked_measure(
                        geometry.boundary / geometry.enclosed,
                        "PerimeterSurfaceRatio",
                    )?,
                );
            }
            if requested.contains("Sphericity") {
                values.insert(
                    "Sphericity",
                    checked_measure(
                        sphericity_2d(geometry.enclosed, geometry.boundary),
                        "Sphericity",
                    )?,
                );
            }
        } else {
            if requested.contains("MeshVolume") {
                values.insert(
                    "MeshVolume",
                    checked_measure(geometry.enclosed, "MeshVolume")?,
                );
            }
            if requested.contains("SurfaceArea") {
                values.insert(
                    "SurfaceArea",
                    checked_measure(geometry.boundary, "SurfaceArea")?,
                );
            }
            if requested.contains("SurfaceVolumeRatio") {
                values.insert(
                    "SurfaceVolumeRatio",
                    checked_measure(geometry.boundary / geometry.enclosed, "SurfaceVolumeRatio")?,
                );
            }
            if any_of(
                &requested,
                &[
                    "Sphericity",
                    "Compactness1",
                    "Compactness2",
                    "SphericalDisproportion",
                    "Asphericity",
                ],
            ) {
                let sphericity = sphericity_3d(geometry.enclosed, geometry.boundary);
                if requested.contains("Sphericity") {
                    values.insert("Sphericity", checked_measure(sphericity, "Sphericity")?);
                }
                values.insert("Compactness1", sphericity.powf(1.5) / (6.0 * PI));
                values.insert("Compactness2", sphericity * sphericity * sphericity);
                values.insert("SphericalDisproportion", 1.0 / sphericity);
                values.insert("Asphericity", 1.0 / sphericity - 1.0);
            }
            if any_of(&requested, AABB_FEATURES) {
                let mut extents = [0.0f64; 3];
                for (axis, extent) in extents.iter_mut().enumerate() {
                    let (low, high) = coordinates
                        .rows()
                        .fold((i64::MAX, i64::MIN), |(lo, hi), row| {
                            (lo.min(row[axis]), hi.max(row[axis]))
                        });
                    *extent = (high - low + 1) as f64;
                }
                values.insert(
                    "VolumeDensityAxisAlignedBoundingBox",
                    geometry.grid_enclosed / (6.0 * extents.iter().product::<f64>()),
                );
                let physical: Vec<f64> = (0..3).map(|i| extents[i] * spacing[i]).collect();
                let aabb_area = 2.0
                    * (physical[0] * physical[1]
                        + physical[1] * physical[2]
                        + physical[2] * physical[0]);
                values.insert(
                    "AreaDensityAxisAlignedBoundingBox",
                    geometry.boundary / aabb_area,
                );
            }
            if needs_hull_geometry {
                let hull = match hull.as_ref().expect("hull was built for hull features") {
                    Ok(hull) => hull,
                    Err(error) => return Err(error.clone()),
                };
                if any_of(&requested, HULL_FEATURES) {
                    let (hull_area, hull_volume) = hull_measures(hull, spacing);
                    values.insert(
                        "VolumeDensityConvexHull",
                        geometry.grid_enclosed / (6.0 * hull_volume),
                    );
                    values.insert("AreaDensityConvexHull", geometry.boundary / hull_area);
                }
                if any_of(&requested, OMBB_FEATURES) || any_of(&requested, MVEE_FEATURES) {
                    let hull_points: Vec<[f64; 3]> = hull
                        .vertices
                        .iter()
                        .map(|&i| {
                            [
                                grid[i][0] * spacing[0],
                                grid[i][1] * spacing[1],
                                grid[i][2] * spacing[2],
                            ]
                        })
                        .collect();
                    if any_of(&requested, OMBB_FEATURES) {
                        let mean = centroid(&hull_points);
                        let centered: Vec<[f64; 3]> = hull_points
                            .iter()
                            .map(|p| [p[0] - mean[0], p[1] - mean[1], p[2] - mean[2]])
                            .collect();
                        let (volume, area) =
                            ombb_measures(&centered, &facet_normals(hull, spacing))?;
                        let (volume_density, area_density) =
                            enclosing_densities(geometry, volume, area);
                        values.insert("VolumeDensityOrientedMinimumBoundingBox", volume_density);
                        values.insert("AreaDensityOrientedMinimumBoundingBox", area_density);
                    }
                    if any_of(&requested, MVEE_FEATURES) {
                        let [a, b, c] = mvee_semi_axes(&hull_points)?;
                        let volume = (4.0 * PI / 3.0) * a * b * c;
                        let area =
                            if requested.contains("AreaDensityMinimumVolumeEnclosingEllipsoid") {
                                ellipsoid_area_normalized(a, b, c)
                            } else {
                                1.0
                            };
                        let (volume_density, area_density) =
                            enclosing_densities(geometry, volume, area);
                        values.insert(
                            "VolumeDensityMinimumVolumeEnclosingEllipsoid",
                            volume_density,
                        );
                        values.insert("AreaDensityMinimumVolumeEnclosingEllipsoid", area_density);
                    }
                }
            }
        }
    }
    if needs_axes {
        let rank = coordinate_rank(coordinates);
        let mut required: Vec<usize> = Vec::new();
        let aee = any_of(&requested, AEE_FEATURES);
        if aee || any_of(&requested, &["MajorAxisLength", "Elongation", "Flatness"]) {
            required.push(ndim - 1);
        }
        if aee || any_of(&requested, &["MinorAxisLength", "Elongation"]) {
            required.push(ndim - 2);
        }
        if aee || any_of(&requested, &["LeastAxisLength", "Flatness"]) {
            required.push(0);
        }
        required.sort_unstable();
        required.dedup();
        let mut eigen = shape_eigenvalues(coordinates, spacing, rank, &required)?;
        let count = coordinates.len();
        if covariance_divisor == CovarianceDivisor::NMinusOne && count > 1 {
            // IBSI reference PCA uses the sample covariance; rescale the population
            // spectrum so axes and AEE densities share that denominator.
            let factor = count as f64 / (count - 1) as f64;
            for value in &mut eigen {
                *value *= factor;
            }
        }
        let last = eigen[ndim - 1];
        let next = eigen[ndim - 2];
        values.insert("MajorAxisLength", 4.0 * last.sqrt());
        values.insert("MinorAxisLength", 4.0 * next.sqrt());
        values.insert("Elongation", (next / last).sqrt());
        if ndim == 3 {
            values.insert("LeastAxisLength", 4.0 * eigen[0].sqrt());
            values.insert("Flatness", (eigen[0] / last).sqrt());
        }
        if aee {
            let geometry = geometry.ok_or_else(|| {
                Error::Execution("approximate enclosing ellipsoid requires a mesh".into())
            })?;
            let (volume_density, area_density) = aee_densities(
                geometry,
                &eigen,
                requested.contains("AreaDensityApproximateEnclosingEllipsoid"),
                ellipsoid_area == EllipsoidArea::IbsiSeries,
            );
            values.insert("VolumeDensityApproximateEnclosingEllipsoid", volume_density);
            values.insert("AreaDensityApproximateEnclosingEllipsoid", area_density);
        }
    }
    if needs_diameter {
        let geometry = geometry.ok_or_else(|| {
            Error::Execution("diameter features require mesh boundary vertices".into())
        })?;
        let maxima = match (ndim, hull.as_ref()) {
            (3, Some(Ok(hull))) => {
                diameter_maxima_with_hull(&geometry.vertices, spacing, 3, &hull.vertices)
            }
            (2, _) => {
                let extremes = hull_vertices_2d(&geometry.vertices);
                diameter_maxima_with_hull(&geometry.vertices, spacing, 2, &extremes)
            }
            _ => diameter_maxima(&geometry.vertices, spacing, ndim),
        };
        if maxima.iter().any(|value| !value.is_finite()) {
            return Err(Error::Execution(
                "shape diameter exceeds the float64 range".into(),
            ));
        }
        if ndim == 2 {
            values.insert("MaximumDiameter", maxima[3]);
        } else {
            values.insert("Maximum3DDiameter", maxima[3]);
            values.insert("Maximum2DDiameterSlice", maxima[0]);
            values.insert("Maximum2DDiameterColumn", maxima[1]);
            values.insert("Maximum2DDiameterRow", maxima[2]);
        }
        if maxima[3] <= 0.0 {
            let name = if ndim == 2 {
                "MaximumDiameter"
            } else {
                "Maximum3DDiameter"
            };
            return Err(Error::Execution(format!(
                "{name} underflows the float64 range"
            )));
        }
    }
    let family = if ndim == 2 { "shape2D" } else { "shape" };
    let mut seen = HashSet::new();
    Ok(order
        .iter()
        .filter(|name| seen.insert(**name))
        .map(|name| {
            let value = values
                .get(name)
                .copied()
                .expect("every requested feature was computed");
            (format!("{family}_{name}"), value)
        })
        .collect())
}

/// Engine entry point: compute `shape_*` and `shape2D_*` features of a prepared input.
///
/// The mask is `prepared.input.shape_roi`. `shape_*` names require a 3-D mask;
/// `shape2D_*` names require a 2-D mask, or a 3-D mask occupying exactly one slice along
/// `settings.force2d_axis`. The four intensity morphology names are computed by
/// [`crate::features::morphology::compute`]. The output keeps the order of `names`.
///
/// # Errors
///
/// [`Error::InvalidInput`] for an unknown or intensity-morphology name, a non-3-D mask with
/// `shape_*` names, `shape2D` without a 2-D mask or `force2d_axis`, or a `shape2D` ROI that
/// does not occupy exactly one slice; everything [`shape_statistics`] returns.
pub fn compute(
    prepared: &Prepared,
    spacing: &[f64],
    settings: &Settings,
    names: &[&str],
) -> Result<Features> {
    let mut shape: Vec<&str> = Vec::new();
    let mut shape2d: Vec<&str> = Vec::new();
    for &name in names {
        if let Some(bare) = name.strip_prefix("shape2D_") {
            shape2d.push(bare);
        } else if let Some(bare) = name.strip_prefix("shape_") {
            if INTENSITY_MORPHOLOGY.contains(&bare) {
                return Err(Error::InvalidInput(format!(
                    "{name} is an intensity morphology feature; use features::morphology::compute"
                )));
            }
            shape.push(bare);
        } else {
            return Err(Error::InvalidInput(format!(
                "unsupported shape feature: {name}"
            )));
        }
    }
    let mut computed: HashMap<String, f64> = HashMap::new();
    let mask = &prepared.input.shape_roi;
    if !shape.is_empty() {
        if mask.ndim() != 3 {
            return Err(Error::InvalidInput(
                "shape requires a three-dimensional mask".into(),
            ));
        }
        computed.extend(shape_statistics(
            mask,
            spacing,
            Some(&shape),
            settings.covariance_divisor,
            settings.ellipsoid_area,
            None,
        )?);
    }
    if !shape2d.is_empty() {
        let (plane, plane_spacing): (ArrayD<bool>, Vec<f64>) = if mask.ndim() == 3 {
            let Some(axis) = settings.force2d_axis else {
                return Err(Error::InvalidInput(
                    "shape2D requires 2D data or force2d_axis".into(),
                ));
            };
            if axis >= 3 {
                return Err(Error::InvalidInput(
                    "force2d_axis must be an integer indexing a NumPy image axis".into(),
                ));
            }
            let occupied: Vec<usize> = (0..mask.shape()[axis])
                .filter(|&index| mask.index_axis(Axis(axis), index).iter().any(|&v| v))
                .collect();
            if occupied.len() != 1 {
                return Err(Error::InvalidInput(
                    "shape2D ROI must occupy exactly one slice".into(),
                ));
            }
            (
                mask.index_axis(Axis(axis), occupied[0]).to_owned(),
                spacing
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != axis)
                    .map(|(_, s)| *s)
                    .collect(),
            )
        } else {
            (mask.clone(), spacing.to_vec())
        };
        computed.extend(shape_statistics(
            &plane,
            &plane_spacing,
            Some(&shape2d),
            settings.covariance_divisor,
            settings.ellipsoid_area,
            None,
        )?);
    }
    Ok(names
        .iter()
        .map(|&name| (name.to_string(), computed[name]))
        .collect())
}
