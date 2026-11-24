//! Exact coordinate covariance, rank and shape eigenvalues.
//!
//! Mirrors `mlxradiomics/features/geometry/moments.py` (297 lines at mlx-radiomics
//! b58783c). The double-single Metal covariance and Jacobi kernels are replaced by exact
//! integer moment sums (`i128`) rounded once to float64 and a float64 Jacobi spectrum, so
//! a thin but full-rank ROI keeps its small eigenvalue without compensated arithmetic.
//!
//! Execution: CPU numerical work, single-threaded. Input contract: integer voxel
//! coordinates from [`Coordinates`] and a positive spacing per axis.
//!
//! Status: done.

use crate::error::{Error, Result};
use crate::features::geometry::arithmetic::symmetric_eigen;
use crate::features::geometry::mesh::Coordinates;

/// Machine epsilon scale of the spectrum roundoff bound (float64).
const EPSILON: f64 = f64::EPSILON;

/// Exact affine rank of integer voxel centers (0 to `ndim`).
///
/// Differences, cross products and scalar triple products are evaluated in `i128`, so the
/// rank never depends on a floating tolerance.
#[must_use]
pub fn coordinate_rank(coordinates: &Coordinates) -> usize {
    let ndim = coordinates.ndim();
    let origin = coordinates.row(0);
    let vectors: Vec<[i128; 3]> = coordinates
        .rows()
        .map(|row| {
            let mut v = [0i128; 3];
            for axis in 0..ndim {
                v[axis] = i128::from(row[axis] - origin[axis]);
            }
            v
        })
        .collect();
    let Some(first) = vectors.iter().find(|v| **v != [0; 3]) else {
        return 0;
    };
    let cross = |a: &[i128; 3], b: &[i128; 3]| -> [i128; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    if ndim == 2 {
        let area = vectors
            .iter()
            .any(|v| v[0] * first[1] - v[1] * first[0] != 0);
        return if area { 2 } else { 1 };
    }
    let Some(normal) = vectors
        .iter()
        .map(|v| cross(v, first))
        .find(|n| *n != [0; 3])
    else {
        return 1;
    };
    let volume = vectors
        .iter()
        .any(|v| v[0] * normal[0] + v[1] * normal[1] + v[2] * normal[2] != 0);
    if volume { 3 } else { 2 }
}

/// Ascending covariance spectrum of the voxel centers in physical units.
///
/// The population covariance (divisor `N`) of `coordinates * spacing` is formed from exact
/// integer moment sums and diagonalised with Jacobi rotations. The exact integer `rank`
/// fixes null axes to zero. A required positive eigenvalue below the roundoff bound is
/// rejected instead of returning a false zero axis. Only axes in `required_axes` (indices
/// into the ascending spectrum) must exceed that bound.
///
/// # Errors
///
/// [`Error::Execution`] if the spectrum is non-finite, a null axis or negative value
/// exceeds the roundoff bound, or a required positive axis cannot be resolved.
pub fn shape_eigenvalues(
    coordinates: &Coordinates,
    spacing: &[f64],
    rank: usize,
    required_axes: &[usize],
) -> Result<Vec<f64>> {
    let ndim = coordinates.ndim();
    let count = coordinates.len();
    let origin = coordinates.row(0).to_vec();
    let mut sums = [0i128; 3];
    let mut products = [[0i128; 3]; 3];
    for row in coordinates.rows() {
        for i in 0..ndim {
            let a = i128::from(row[i] - origin[i]);
            sums[i] += a;
            for j in i..ndim {
                products[i][j] += a * i128::from(row[j] - origin[j]);
            }
        }
    }
    let n = count as i128;
    let mut covariance = vec![vec![0.0f64; ndim]; ndim];
    for i in 0..ndim {
        for j in i..ndim {
            // (N * sum(a_i a_j) - sum(a_i) sum(a_j)) / N^2, exact until the final rounding.
            let numerator = n * products[i][j] - sums[i] * sums[j];
            let value = numerator as f64 / (count as f64 * count as f64) * spacing[i] * spacing[j];
            covariance[i][j] = value;
            covariance[j][i] = value;
        }
    }
    let peak = covariance
        .iter()
        .flatten()
        .fold(0.0f64, |acc, value| acc.max(value.abs()));
    let mut eigen: Vec<f64> = if ndim == 3 {
        let matrix = [
            [covariance[0][0], covariance[0][1], covariance[0][2]],
            [covariance[1][0], covariance[1][1], covariance[1][2]],
            [covariance[2][0], covariance[2][1], covariance[2][2]],
        ];
        symmetric_eigen(matrix).0.to_vec()
    } else {
        let matrix = [
            [covariance[0][0], covariance[0][1]],
            [covariance[1][0], covariance[1][1]],
        ];
        symmetric_eigen(matrix).0.to_vec()
    };
    let tolerance = 8.0 * ndim as f64 * EPSILON * peak;
    let null_count = ndim - rank;
    if eigen.iter().any(|value| !value.is_finite()) {
        return Err(Error::Execution(
            "shape covariance spectrum is not finite".into(),
        ));
    }
    if required_axes
        .iter()
        .any(|&axis| axis >= null_count && eigen[axis] <= tolerance)
    {
        return Err(Error::Execution(
            "shape covariance is too ill-conditioned to resolve a requested axis".into(),
        ));
    }
    if eigen[..null_count]
        .iter()
        .any(|value| value.abs() > tolerance)
    {
        return Err(Error::Execution(
            "shape covariance null axis exceeds the compensated roundoff bound".into(),
        ));
    }
    for value in &mut eigen[..null_count] {
        *value = 0.0;
    }
    if eigen.iter().any(|value| *value < -tolerance) {
        return Err(Error::Execution(
            "shape covariance has an eigenvalue below its PSD roundoff bound".into(),
        ));
    }
    Ok(eigen.into_iter().map(|value| value.max(0.0)).collect())
}

#[cfg(test)]
mod tests {
    use crate::features::geometry::moments::*;
    use ndarray::{ArrayD, IxDyn};

    fn coordinates(points: &[[usize; 3]]) -> Coordinates {
        let mut mask = ArrayD::from_elem(IxDyn(&[8, 8, 8]), false);
        for p in points {
            mask[[p[0], p[1], p[2]]] = true;
        }
        Coordinates::from_mask(&mask)
    }

    #[test]
    fn rank_distinguishes_point_line_plane_volume() {
        assert_eq!(coordinate_rank(&coordinates(&[[1, 1, 1]])), 0);
        assert_eq!(coordinate_rank(&coordinates(&[[1, 1, 1], [2, 2, 2]])), 1);
        assert_eq!(
            coordinate_rank(&coordinates(&[[1, 1, 1], [2, 2, 2], [1, 2, 1]])),
            2
        );
        assert_eq!(
            coordinate_rank(&coordinates(&[[1, 1, 1], [2, 2, 2], [1, 2, 1], [1, 1, 2]])),
            3
        );
    }

    #[test]
    fn thin_axis_survives() {
        let points = coordinates(&[
            [0, 0, 0],
            [0, 0, 1],
            [1, 0, 0],
            [1, 0, 1],
            [0, 1, 0],
            [1, 1, 1],
        ]);
        let eigen = shape_eigenvalues(&points, &[1.0, 1.0, 1.0], 3, &[0, 1, 2]).unwrap();
        assert!(eigen[0] > 0.0);
    }
}
