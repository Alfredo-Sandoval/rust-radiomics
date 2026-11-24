//! Oriented bounding boxes from convex-hull face-flush orientations.
//!
//! Mirrors `mlxradiomics/features/geometry/boxes.py` (209 lines at mlx-radiomics
//! b58783c). The candidate set is the same: every box with one face flush with a hull
//! facet, whose second axis is parallel to a silhouette edge of the hull projected along
//! that facet's normal. For a fixed normal `u` the box height along `u` is constant, so the
//! minimum-volume candidate is the minimum-area rectangle of the projected hull, and by
//! Freeman and Shapira one of its sides is collinear with an edge of the projected 2-D
//! hull. The Python module enumerates silhouette edges of the 3-D hull and projects every
//! vertex per candidate on Metal; this module takes the edges of the 2-D hull of the
//! projection instead, which are the projections of the same silhouette edges, and
//! evaluates each rectangle exactly over the hull vertices. As in mlx-radiomics this is the
//! face-flush approximation of O'Rourke's exact optimum, not the exact algorithm.
//!
//! Equal minimum volumes are resolved to the smallest surface area, using the same
//! first-order rounding bound as the Python module (with float64 unit roundoff).
//!
//! Execution: CPU numerical work in float64, multi-threaded with rayon over the facet
//! normals (the result is the minimum over all candidates and does not depend on thread
//! count). Input contract: centered physical hull vertices and the hull of the same points.
//!
//! Status: done.

use rayon::prelude::*;

use crate::error::{Error, Result};
use crate::features::geometry::arithmetic::{cross_product, dot3, norm3};
use crate::features::geometry::hull::ConvexHull;

/// Unit roundoff of float64 for the volume tie bound.
const UNIT: f64 = 1.0 / (1u64 << 53) as f64;

/// Unit outward facet normals of a hull in physical space.
///
/// Hull planes are stored in grid space; under a diagonal spacing scale a plane normal maps
/// by the inverse spacing.
#[must_use]
pub fn facet_normals(hull: &ConvexHull, spacing: &[f64]) -> Vec<[f64; 3]> {
    hull.planes
        .iter()
        .map(|plane| {
            let n = [
                plane[0] as f64 / spacing[0],
                plane[1] as f64 / spacing[1],
                plane[2] as f64 / spacing[2],
            ];
            let length = norm3(n);
            [n[0] / length, n[1] / length, n[2] / length]
        })
        .collect()
}

/// Bound on the rounding error of a box volume from its extents.
///
/// With unit roundoff `UNIT` and point radius `radius`, each projection is within
/// `10 * UNIT * radius + UNIT * extent` and the two volume products add
/// `2 * UNIT * volume`; twice this first-order bound covers the dropped terms.
fn volume_error(x: f64, y: f64, z: f64, radius: f64) -> f64 {
    2.0 * UNIT * (10.0 * radius * (y * z + z * x + x * y) + 5.0 * x * y * z)
}

/// Convex hull of 2-D points by Andrew's monotone chain, counter-clockwise.
fn hull_2d(mut points: Vec<[f64; 2]>) -> Vec<[f64; 2]> {
    points.sort_by(|a, b| a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])));
    points.dedup();
    if points.len() < 3 {
        return points;
    }
    let turn = |o: [f64; 2], a: [f64; 2], b: [f64; 2]| {
        (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    };
    let mut lower: Vec<[f64; 2]> = Vec::new();
    for &p in &points {
        while lower.len() >= 2 && turn(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper: Vec<[f64; 2]> = Vec::new();
    for &p in points.iter().rev() {
        while upper.len() >= 2 && turn(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

/// Candidate boxes `(volume, area, volume error)` flush with the facet of unit normal `u`.
fn candidates_for_normal(points: &[[f64; 3]], u: [f64; 3], radius: f64) -> Vec<(f64, f64, f64)> {
    // Orthonormal basis (e1, e2) of the facet plane.
    let smallest = (0..3)
        .min_by(|&a, &b| u[a].abs().total_cmp(&u[b].abs()))
        .expect("three axes");
    let mut axis = [0.0; 3];
    axis[smallest] = 1.0;
    let e1 = cross_product(u, axis);
    let length = norm3(e1);
    let e1 = [e1[0] / length, e1[1] / length, e1[2] / length];
    let e2 = cross_product(u, e1);
    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
    let mut planar = Vec::with_capacity(points.len());
    for point in points {
        let height = dot3(*point, u);
        low = low.min(height);
        high = high.max(height);
        planar.push([dot3(*point, e1), dot3(*point, e2)]);
    }
    let height = high - low;
    let outline = hull_2d(planar);
    let mut boxes: Vec<(f64, f64, f64)> = Vec::new();
    let count = outline.len();
    for k in 0..count {
        let a = outline[k];
        let b = outline[(k + 1) % count];
        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
        let edge = dx.hypot(dy);
        if edge <= 0.0 {
            continue;
        }
        let (vx, vy) = (dx / edge, dy / edge);
        let (mut vlo, mut vhi) = (f64::INFINITY, f64::NEG_INFINITY);
        let (mut wlo, mut whi) = (f64::INFINITY, f64::NEG_INFINITY);
        for p in &outline {
            let along = p[0] * vx + p[1] * vy;
            let across = -p[0] * vy + p[1] * vx;
            vlo = vlo.min(along);
            vhi = vhi.max(along);
            wlo = wlo.min(across);
            whi = whi.max(across);
        }
        let (x, y, z) = (height, vhi - vlo, whi - wlo);
        boxes.push((
            x * y * z,
            2.0 * (x * y + y * z + z * x),
            volume_error(x, y, z, radius),
        ));
    }
    // Keep only boxes that can still be within rounding of this normal's minimum.
    let bound = boxes
        .iter()
        .map(|(volume, _, error)| volume + error)
        .fold(f64::INFINITY, f64::min);
    boxes.retain(|(volume, _, error)| volume - error <= bound);
    boxes
}

/// Minimum-volume face-flush box volume and its surface area.
///
/// `points` are the centered physical hull vertices and `normals` the unit physical facet
/// normals ([`facet_normals`]). Boxes of equal volume (a full box mask is enclosed equally
/// by its axis-aligned box and by tilted boxes) resolve to the smallest surface area.
///
/// # Errors
///
/// [`Error::Execution`] if there is no candidate orientation, a box is not finite, or the
/// minimum volume is not positive.
pub fn ombb_measures(points: &[[f64; 3]], normals: &[[f64; 3]]) -> Result<(f64, f64)> {
    let radius = points.iter().map(|p| norm3(*p)).fold(0.0f64, f64::max);
    let all: Vec<(f64, f64, f64)> = normals
        .par_iter()
        .flat_map_iter(|&u| candidates_for_normal(points, u, radius))
        .collect();
    if all.is_empty() {
        return Err(Error::Execution(
            "convex hull has no face-flush box orientations".into(),
        ));
    }
    if all
        .iter()
        .any(|(volume, area, _)| !volume.is_finite() || !area.is_finite())
    {
        return Err(Error::Execution(
            "oriented minimum bounding box is not finite".into(),
        ));
    }
    let bound = all
        .iter()
        .map(|(volume, _, error)| volume + error)
        .fold(f64::INFINITY, f64::min);
    let best_volume = all
        .iter()
        .map(|(volume, _, _)| *volume)
        .fold(f64::INFINITY, f64::min);
    let best_area = all
        .iter()
        .filter(|(volume, _, error)| volume - error <= bound)
        .map(|(_, area, _)| *area)
        .fold(f64::INFINITY, f64::min);
    if best_volume <= 0.0 {
        return Err(Error::Execution(
            "oriented minimum bounding box volume underflows float64".into(),
        ));
    }
    Ok((best_volume, best_area))
}

/// Mean of a point set.
#[must_use]
pub fn centroid(points: &[[f64; 3]]) -> [f64; 3] {
    let n = points.len() as f64;
    let mut sum = [0.0; 3];
    for p in points {
        for axis in 0..3 {
            sum[axis] += p[axis];
        }
    }
    [sum[0] / n, sum[1] / n, sum[2] / n]
}

#[cfg(test)]
mod tests {
    use crate::features::geometry::boxes::*;

    #[test]
    fn rotated_box_recovers_the_true_box() {
        // A 1 x 2 x 3 box rotated about a skew axis; its face normals are the rotated axes.
        let (c, s) = (0.7f64.cos(), 0.7f64.sin());
        let rotation = [
            [c, -s, 0.0],
            [s * 0.6, c * 0.6, 0.8],
            [-s * 0.8, -c * 0.8, 0.6],
        ];
        let rotate = |p: [f64; 3]| {
            let mut out = [0.0; 3];
            for i in 0..3 {
                out[i] = rotation[i][0] * p[0] + rotation[i][1] * p[1] + rotation[i][2] * p[2];
            }
            out
        };
        let mut points = Vec::new();
        for x in [-0.5, 0.5] {
            for y in [-1.0, 1.0] {
                for z in [-1.5, 1.5] {
                    points.push(rotate([x, y, z]));
                }
            }
        }
        let normals = [
            rotate([1.0, 0.0, 0.0]),
            rotate([0.0, 1.0, 0.0]),
            rotate([0.0, 0.0, 1.0]),
        ];
        let (volume, area) = ombb_measures(&points, &normals).unwrap();
        assert!((volume - 6.0).abs() < 1e-12, "{volume}");
        assert!((area - 22.0).abs() < 1e-12, "{area}");
    }
}
