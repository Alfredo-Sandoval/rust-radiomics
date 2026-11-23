//! Geometry arithmetic: spacing validation, compensated sums, 3-vector helpers and the
//! symmetric Jacobi eigensolver shared by the shape modules.
//!
//! Mirrors `mlxradiomics/features/geometry/arithmetic.py` (171 lines at mlx-radiomics
//! b58783c). The Python module scales float32 mesh measures by binary exponents and
//! carries double-single Metal primitives; none of that is needed in float64, so
//! `SpacingParts`, `scale_pow2` and the double-single header are not ported. Their roles
//! are filled by Neumaier-compensated sums ([`NeumaierSum`]) and plain float64 products.
//!
//! Execution: CPU numerical work, single-threaded. Input contract: float64 values and
//! spacings in NumPy axis order (z, y, x).
//!
//! Status: done.

use crate::error::{Error, Result};

/// Validate a shape spacing (`spacing_parts` checks in mlx-radiomics).
///
/// # Errors
///
/// [`Error::InvalidInput`] if the spacing does not have two or three values, or a value is
/// not finite and positive.
pub fn validate_spacing(spacing: &[f64]) -> Result<()> {
    if !(spacing.len() == 2 || spacing.len() == 3) {
        return Err(Error::InvalidInput(
            "shape spacing must have two or three values".into(),
        ));
    }
    if spacing
        .iter()
        .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err(Error::InvalidInput(
            "shape spacing values must be finite and positive".into(),
        ));
    }
    Ok(())
}

/// Cross product of two 3-vectors.
#[must_use]
pub fn cross_product(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Dot product of two 3-vectors.
#[must_use]
pub fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Euclidean norm of a 3-vector, scaled by its largest component so squares cannot
/// overflow or underflow.
#[must_use]
pub fn norm3(a: [f64; 3]) -> f64 {
    let largest = a[0].abs().max(a[1].abs()).max(a[2].abs());
    if largest == 0.0 || !largest.is_finite() {
        return largest;
    }
    let (x, y, z) = (a[0] / largest, a[1] / largest, a[2] / largest);
    largest * (x * x + y * y + z * z).sqrt()
}

/// Neumaier compensated running sum.
#[derive(Clone, Copy, Debug, Default)]
pub struct NeumaierSum {
    sum: f64,
    compensation: f64,
}

impl NeumaierSum {
    /// An empty sum.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add one term.
    pub fn add(&mut self, value: f64) {
        let total = self.sum + value;
        if self.sum.abs() >= value.abs() {
            self.compensation += (self.sum - total) + value;
        } else {
            self.compensation += (value - total) + self.sum;
        }
        self.sum = total;
    }

    /// The compensated total.
    #[must_use]
    pub fn value(&self) -> f64 {
        self.sum + self.compensation
    }
}

/// Compensated sum of a slice.
#[must_use]
pub fn compensated_sum(values: &[f64]) -> f64 {
    let mut total = NeumaierSum::new();
    for value in values {
        total.add(*value);
    }
    total.value()
}

/// Return a dimensional measure, rejecting a non-finite result.
///
/// # Errors
///
/// [`Error::Execution`] if `value` overflowed to infinity or is NaN, or underflowed to
/// zero (a non-empty mask with positive spacing never has a zero boundary or enclosed
/// measure).
pub fn checked_measure(value: f64, name: &str) -> Result<f64> {
    if !value.is_finite() {
        return Err(Error::Execution(format!(
            "{name} exceeds the float64 range"
        )));
    }
    if value == 0.0 {
        return Err(Error::Execution(format!(
            "{name} underflows the float64 range"
        )));
    }
    Ok(value)
}

/// Eigen-decomposition of a real symmetric `N x N` matrix by cyclic Jacobi rotations.
///
/// Returns the eigenvalues in ascending order and the matching eigenvectors as the columns
/// of the second value (`matrix[row][column]`). Jacobi keeps small eigenvalues of graded
/// matrices to high relative accuracy, which the thin-shape axes rely on.
#[must_use]
pub fn symmetric_eigen<const N: usize>(matrix: [[f64; N]; N]) -> ([f64; N], [[f64; N]; N]) {
    let mut a = matrix;
    let mut v = [[0.0; N]; N];
    for (i, row) in v.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    for _ in 0..100 {
        let mut off = 0.0;
        let mut diag = 0.0;
        for i in 0..N {
            diag += a[i][i] * a[i][i];
            for j in (i + 1)..N {
                off += a[i][j] * a[i][j];
            }
        }
        if off == 0.0 || off <= 1e-34 * diag {
            break;
        }
        for p in 0..N {
            for q in (p + 1)..N {
                if a[p][q] == 0.0 {
                    continue;
                }
                let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let t = if theta == 0.0 { 1.0 } else { t };
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for k in 0..N {
                    let akp = a[k][p];
                    let akq = a[k][q];
                    a[k][p] = c * akp - s * akq;
                    a[k][q] = s * akp + c * akq;
                }
                for k in 0..N {
                    let apk = a[p][k];
                    let aqk = a[q][k];
                    a[p][k] = c * apk - s * aqk;
                    a[q][k] = s * apk + c * aqk;
                }
                for row in v.iter_mut() {
                    let vkp = row[p];
                    let vkq = row[q];
                    row[p] = c * vkp - s * vkq;
                    row[q] = s * vkp + c * vkq;
                }
            }
        }
    }
    let mut order: Vec<usize> = (0..N).collect();
    order.sort_by(|&i, &j| a[i][i].total_cmp(&a[j][j]));
    let mut values = [0.0; N];
    let mut vectors = [[0.0; N]; N];
    for (rank, &index) in order.iter().enumerate() {
        values[rank] = a[index][index];
        for row in 0..N {
            vectors[row][rank] = v[row][index];
        }
    }
    (values, vectors)
}

#[cfg(test)]
mod tests {
    use crate::features::geometry::arithmetic::*;

    #[test]
    fn jacobi_recovers_a_known_spectrum() {
        let (values, _) = symmetric_eigen([[2.0, 1.0, 0.0], [1.0, 2.0, 0.0], [0.0, 0.0, 5.0]]);
        assert!((values[0] - 1.0).abs() < 1e-14);
        assert!((values[1] - 3.0).abs() < 1e-14);
        assert!((values[2] - 5.0).abs() < 1e-14);
    }

    #[test]
    fn neumaier_keeps_cancelling_terms() {
        assert_eq!(compensated_sum(&[1.0, 1e100, 1.0, -1e100]), 2.0);
    }
}
