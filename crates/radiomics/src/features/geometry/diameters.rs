//! Pairwise diameter reductions on mesh vertices.
//!
//! Mirrors `mlxradiomics/features/geometry/diameters.py` (189 lines at mlx-radiomics
//! b58783c). [`diameter_maxima`] evaluates every unordered vertex pair exactly once, as in
//! the Metal kernel, with a rayon loop over the first vertex of each pair.
//! [`diameter_maxima_with_hull`] returns the same bits without the quadratic loop: the
//! largest distance is attained between convex-hull vertices, so only vertices that can
//! come within rounding of the hull-pair maximum are compared with all vertices, and plane
//! maxima are searched inside coordinate groups.
//!
//! Execution: CPU numerical work in float64, multi-threaded with rayon (maxima of squared
//! distances are reductions, so the result does not depend on thread count). Input
//! contract: flattened `(n, ndim)` grid-coordinate vertices (half-integers) and a spacing
//! per axis; the hull variant also takes the extreme vertex indices.
//!
//! Status: done.

use rayon::prelude::*;

/// Relative margin of the hull-vertex bound: far above the few-ulp rounding of a squared
/// distance (about 1e-15), far below any distance gap between distinct lattice pairs.
const PRUNE_MARGIN: f64 = 1e-12;

/// Plane groups up to this size are compared pair by pair.
const SMALL_GROUP: usize = 48;

/// Squared physical distance of two vertices, accumulated axis by axis in float64.
#[inline]
fn squared_distance(vertices: &[f64], spacing: &[f64], ndim: usize, a: usize, b: usize) -> f64 {
    let mut total = 0.0;
    for axis in 0..ndim {
        let difference = (vertices[a * ndim + axis] - vertices[b * ndim + axis]) * spacing[axis];
        total += difference * difference;
    }
    total
}

fn roots(squares: [f64; 4]) -> [f64; 4] {
    squares.map(f64::sqrt)
}

/// Overall and per-plane maximum boundary-vertex distances.
///
/// Returns `[axis 0, axis 1, axis 2, overall]`: entries 0..3 are the maxima over pairs that
/// share their axis-0, axis-1 and axis-2 coordinate (3-D only; zero in 2-D) and entry 3 is
/// the maximum over all pairs. A single vertex gives all zeros.
///
/// Every pair is evaluated, so this is quadratic in the vertex count; 3-D callers with a
/// convex hull use [`diameter_maxima_with_hull`], which returns the same values.
#[must_use]
pub fn diameter_maxima(vertices: &[f64], spacing: &[f64], ndim: usize) -> [f64; 4] {
    roots(exhaustive_squares(vertices, spacing, ndim))
}

/// The squared maxima of [`diameter_maxima`] over every vertex pair.
fn exhaustive_squares(vertices: &[f64], spacing: &[f64], ndim: usize) -> [f64; 4] {
    let count = vertices.len() / ndim;
    (0..count)
        .into_par_iter()
        .map(|t| {
            let mut maxima = [0.0f64; 4];
            for q in 0..t {
                let total = squared_distance(vertices, spacing, ndim, t, q);
                if total > maxima[3] {
                    maxima[3] = total;
                }
                if ndim == 3 {
                    for axis in 0..3 {
                        if vertices[t * ndim + axis] == vertices[q * ndim + axis]
                            && total > maxima[axis]
                        {
                            maxima[axis] = total;
                        }
                    }
                }
            }
            maxima
        })
        .reduce(
            || [0.0; 4],
            |mut a, b| {
                for k in 0..4 {
                    a[k] = a[k].max(b[k]);
                }
                a
            },
        )
}

/// Largest squared vertex distance, from the extreme points `hull_vertices` of the set.
///
/// The squared distance is convex in each endpoint, so the largest true distance over all
/// pairs is attained between extreme points. A vertex whose farthest extreme point is not
/// within [`PRUNE_MARGIN`] of the extreme-pair maximum cannot reach it even with rounding, so
/// it is skipped; the few that remain are compared with every vertex. Every compared pair
/// uses the expression of [`squared_distance`], so the result equals the exhaustive maximum
/// of those values exactly.
fn pruned_max_square(
    vertices: &[f64],
    spacing: &[f64],
    ndim: usize,
    hull_vertices: &[usize],
    parallel: bool,
) -> f64 {
    let count = vertices.len() / ndim;
    let distance = |a: usize, b: usize| squared_distance(vertices, spacing, ndim, a, b);
    // Extreme points as per-axis columns, so the farthest-point scan runs over contiguous
    // lanes; each distance accumulates over the axes exactly as in `squared_distance`.
    let columns: Vec<Vec<f64>> = (0..ndim)
        .map(|axis| {
            hull_vertices
                .iter()
                .map(|&h| vertices[h * ndim + axis])
                .collect()
        })
        .collect();
    let farthest = |t: usize| {
        let mut lanes = [0.0f64; 4];
        let at = |axis: usize| vertices[t * ndim + axis];
        let (full, tail) = (columns[0].len() / 4 * 4, columns[0].len());
        for start in (0..full).step_by(4) {
            for lane in 0..4 {
                let mut total = 0.0;
                for axis in 0..ndim {
                    let difference = (at(axis) - columns[axis][start + lane]) * spacing[axis];
                    total += difference * difference;
                }
                if total > lanes[lane] {
                    lanes[lane] = total;
                }
            }
        }
        let mut best = lanes.into_iter().fold(0.0, f64::max);
        for k in full..tail {
            let mut total = 0.0;
            for axis in 0..ndim {
                let difference = (at(axis) - columns[axis][k]) * spacing[axis];
                total += difference * difference;
            }
            best = best.max(total);
        }
        best
    };
    let hull_best = (0..hull_vertices.len())
        .map(|k| {
            let a = hull_vertices[k];
            hull_vertices[..k]
                .iter()
                .map(|&b| distance(a, b))
                .fold(0.0, f64::max)
        })
        .fold(0.0, f64::max);
    let threshold = hull_best * (1.0 - PRUNE_MARGIN);
    let survivors: Vec<usize> = if parallel {
        (0..count)
            .into_par_iter()
            .with_min_len(256)
            .filter(|&t| farthest(t) >= threshold)
            .collect()
    } else {
        (0..count).filter(|&t| farthest(t) >= threshold).collect()
    };
    let rest = survivors
        .iter()
        .map(|&t| (0..count).map(|q| distance(t, q)).fold(0.0, f64::max))
        .fold(0.0, f64::max);
    hull_best.max(rest)
}

/// Vertices of a plane group projected onto the other two axes, in axis order.
fn project(vertices: &[f64], group: &[usize], axis: usize) -> Vec<f64> {
    let mut points = Vec::with_capacity(group.len() * 2);
    for &index in group {
        for other in (0..3).filter(|&other| other != axis) {
            points.push(vertices[index * 3 + other]);
        }
    }
    points
}

/// The values of [`diameter_maxima`] for 2-D or 3-D vertices, without the quadratic pair loop.
///
/// `hull_vertices` are indices of the extreme points of the vertex set (the vertices of its
/// convex hull). The overall maximum comes from `pruned_max_square`. A plane maximum is the
/// largest distance among vertices sharing the coordinate: they are grouped by it and each
/// group is searched in the plane of its other two axes (the shared axis adds an exact zero
/// to every squared distance). Each compared pair uses the same float64 expression as
/// [`diameter_maxima`], so the result is bit-identical to it.
#[must_use]
pub fn diameter_maxima_with_hull(
    vertices: &[f64],
    spacing: &[f64],
    ndim: usize,
    hull_vertices: &[usize],
) -> [f64; 4] {
    let count = vertices.len() / ndim;
    let mut squares = [0.0f64; 4];
    if ndim == 3 {
        for axis in 0..3 {
            // Mesh vertices are half-integers: twice a coordinate is its group key.
            let keys: Option<Vec<usize>> = (0..count)
                .map(|i| {
                    let doubled = vertices[i * 3 + axis] * 2.0;
                    (doubled >= 0.0 && doubled == doubled.round()).then_some(doubled as usize)
                })
                .collect();
            let Some(keys) = keys else {
                return roots(exhaustive_squares(vertices, spacing, ndim));
            };
            let mut groups: Vec<Vec<usize>> =
                vec![Vec::new(); keys.iter().max().map_or(0, |&k| k + 1)];
            for (index, &key) in keys.iter().enumerate() {
                groups[key].push(index);
            }
            let other: Vec<f64> = (0..3).filter(|&o| o != axis).map(|o| spacing[o]).collect();
            squares[axis] = groups
                .par_iter()
                .filter(|group| group.len() > 1)
                .map(|group| {
                    let points = project(vertices, group, axis);
                    if group.len() <= SMALL_GROUP {
                        let n = group.len();
                        return (0..n)
                            .map(|a| {
                                (0..a)
                                    .map(|b| squared_distance(&points, &other, 2, a, b))
                                    .fold(0.0, f64::max)
                            })
                            .fold(0.0, f64::max);
                    }
                    let extremes = hull_vertices_2d(&points);
                    pruned_max_square(&points, &other, 2, &extremes, false)
                })
                .reduce(|| 0.0, f64::max);
        }
    }
    squares[3] = pruned_max_square(vertices, spacing, ndim, hull_vertices, count > 4096);
    roots(squares)
}

/// Indices of the strict convex-hull vertices of 2-D points (collinear points excluded),
/// by Andrew's monotone chain. Coordinates are half-integers, so every turn test is exact.
#[must_use]
pub fn hull_vertices_2d(vertices: &[f64]) -> Vec<usize> {
    let count = vertices.len() / 2;
    let point = |i: usize| (vertices[2 * i], vertices[2 * i + 1]);
    let mut order: Vec<usize> = (0..count).collect();
    order.sort_by(|&a, &b| {
        point(a)
            .partial_cmp(&point(b))
            .expect("mesh vertices are finite")
    });
    order.dedup_by(|a, b| point(*a) == point(*b));
    if order.len() < 3 {
        return order;
    }
    let turn = |o: usize, a: usize, b: usize| {
        let (o, a, b) = (point(o), point(a), point(b));
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    };
    let chain = |points: &mut dyn Iterator<Item = usize>| {
        let mut chain: Vec<usize> = Vec::new();
        for p in points {
            while chain.len() >= 2 && turn(chain[chain.len() - 2], chain[chain.len() - 1], p) <= 0.0
            {
                chain.pop();
            }
            chain.push(p);
        }
        chain.pop();
        chain
    };
    let mut hull = chain(&mut order.iter().copied());
    hull.extend(chain(&mut order.iter().rev().copied()));
    hull
}

#[cfg(test)]
mod tests {
    use crate::features::geometry::diameters::*;

    #[test]
    fn unit_square_diameter_is_the_diagonal() {
        let vertices = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let maxima = diameter_maxima(&vertices, &[1.0, 1.0], 2);
        assert!((maxima[3] - 2f64.sqrt()).abs() < 1e-15);
    }

    #[test]
    fn planes_only_pair_vertices_sharing_the_coordinate() {
        let vertices = [0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 5.0, 0.0, 0.0];
        let maxima = diameter_maxima(&vertices, &[1.0, 1.0, 1.0], 3);
        assert_eq!(maxima[0], 3.0);
        assert_eq!(maxima[1], 5.0);
        assert!((maxima[3] - 34f64.sqrt()).abs() < 1e-15);
    }
}
