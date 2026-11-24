//! Symmetric eigenspectra and GLCM MCC, mirroring `mlxradiomics/numerics/eigen.py`.
//! Status: ported. Single-threaded CPU numerical work in f64. The public batched spectra use
//! cyclic Jacobi; the GLCM MCC takes only the largest eigenvalue, from a Householder
//! tridiagonalization and implicit QL iteration.

use crate::Result;
use crate::numerics::sparse_counts::{Counts, Marginal};
use ndarray::Array2;

/// Dot product with four independent accumulators, so the loop vectorizes. The summation
/// order is fixed, so results are reproducible.
fn dot(x: &[f64], y: &[f64]) -> f64 {
    let mut acc = [0.; 4];
    let (xc, yc) = (x.chunks_exact(4), y.chunks_exact(4));
    let tail: f64 = xc
        .remainder()
        .iter()
        .zip(yc.remainder())
        .map(|(a, b)| a * b)
        .sum();
    for (a, b) in xc.zip(yc) {
        for lane in 0..4 {
            acc[lane] += a[lane] * b[lane];
        }
    }
    (acc[0] + acc[1]) + (acc[2] + acc[3]) + tail
}

/// Largest eigenvalue of a real symmetric matrix: Householder tridiagonalization (EISPACK
/// `tred1`, Wilkinson and Reinsch 1971) followed by bisection. Only the lower triangle of
/// the row-major `n x n` matrix `a` is read; `a` is destroyed.
fn largest_eigenvalue(a: &mut [f64], n: usize) -> f64 {
    let mut e = vec![0.; n];
    let mut u = vec![0.; n];
    for i in (1..n).rev() {
        let l = i - 1;
        let scale: f64 = a[i * n..=i * n + l].iter().map(|v| v.abs()).sum();
        if l == 0 || scale == 0. {
            e[i] = a[i * n + l];
            continue;
        }
        let mut h = 0.;
        for k in 0..=l {
            a[i * n + k] /= scale;
            h += a[i * n + k] * a[i * n + k];
        }
        let f = a[i * n + l];
        let g = if f >= 0. { -h.sqrt() } else { h.sqrt() };
        e[i] = scale * g;
        h -= f * g;
        a[i * n + l] = f - g;
        u[..=l].copy_from_slice(&a[i * n..=i * n + l]);
        // e = A u / h for the symmetric leading block stored in its lower triangle, one
        // contiguous row at a time: a dot for the row and an axpy for its mirror image.
        e[..=l].fill(0.);
        for r in 0..=l {
            let row = &a[r * n..=r * n + r];
            e[r] += dot(row, &u[..=r]);
            let ur = u[r];
            for (ec, &v) in e[..r].iter_mut().zip(&row[..r]) {
                *ec += v * ur;
            }
        }
        for v in e[..=l].iter_mut() {
            *v /= h;
        }
        let f = dot(&e[..=l], &u[..=l]);
        let hh = f / (h + h);
        for j in 0..=l {
            e[j] -= hh * u[j];
        }
        for j in 0..=l {
            let (uj, ej) = (u[j], e[j]);
            for (k, v) in a[j * n..=j * n + j].iter_mut().enumerate() {
                *v -= uj * e[k] + ej * u[k];
            }
        }
    }
    let d: Vec<f64> = (0..n).map(|i| a[i * n + i]).collect();
    largest_tridiagonal_eigenvalue(&d, &e)
}

/// The largest eigenvalue of the symmetric tridiagonal matrix with diagonal `d` and
/// sub-diagonal `e[1..]`, by bisection on the Sturm count (the sign changes of the pivots of
/// the shifted LDL^T recurrence). Bisection cannot fail to converge, and the result is
/// accurate to a few units in the last place of the matrix norm, which is the largest
/// eigenvalue itself for the positive semidefinite Gram matrices of `maximal_correlation`.
fn largest_tridiagonal_eigenvalue(d: &[f64], e: &[f64]) -> f64 {
    let n = d.len();
    let reach = |i: usize| {
        let below = if i > 0 { e[i].abs() } else { 0. };
        let above = if i + 1 < n { e[i + 1].abs() } else { 0. };
        below + above
    };
    let mut low = (0..n)
        .map(|i| d[i] - reach(i))
        .fold(f64::INFINITY, f64::min);
    let mut high = (0..n)
        .map(|i| d[i] + reach(i))
        .fold(f64::NEG_INFINITY, f64::max);
    let pivot_floor = f64::MIN_POSITIVE * e.iter().map(|v| v * v).fold(1., f64::max).max(1.);
    // Whether every eigenvalue lies below `x`: all n pivots of T - xI are negative.
    let all_below = |x: f64| {
        let mut pivot = d[0] - x;
        if pivot >= 0. {
            return false;
        }
        for i in 1..n {
            if pivot.abs() < pivot_floor {
                pivot = -pivot_floor;
            }
            pivot = d[i] - x - e[i] * e[i] / pivot;
            if pivot >= 0. {
                return false;
            }
        }
        true
    };
    for _ in 0..200 {
        if high - low <= 2. * f64::EPSILON * high.abs().max(low.abs()) {
            break;
        }
        let middle = low + 0.5 * (high - low);
        if all_below(middle) {
            high = middle;
        } else {
            low = middle;
        }
    }
    0.5 * (low + high)
}

/// The largest eigenvalue of the centered Gram matrix is the squared second
/// singular value of the standardized contingency table.
pub(crate) fn maximal_correlation(
    counts: &Counts,
    rows: &Marginal,
    columns: &Marginal,
    n: u64,
) -> Result<f64> {
    if rows.len().min(columns.len()) < 2 {
        return Ok(f64::NAN);
    }
    let (nr, nc) = (rows.len(), columns.len());
    // n < 2**31 (checked by the caller), so every product below is under 2**62.
    let standardized = |c: u64, ni: u64, nj: u64| {
        let expected = (ni * nj) as i64;
        ((c * n) as i64 - expected) as f64 / (n as f64 * (expected as f64).sqrt())
    };
    // Empty cells first, then the occupied cells from `counts` (sorted by row, column).
    let mut residual = Array2::zeros((nr, nc));
    for (r, ni) in rows.values().enumerate() {
        for (s, nj) in columns.values().enumerate() {
            residual[[r, s]] = standardized(0, *ni, *nj);
        }
    }
    let column_counts: Vec<u64> = columns.values().copied().collect();
    let mut row = 0;
    for (&(i, j), &c) in counts {
        while rows.keys()[row] != i {
            row += 1;
        }
        let s = columns
            .keys()
            .binary_search(&j)
            .expect("every occupied column has a marginal");
        residual[[row, s]] = standardized(c, rows.get(i).unwrap(), column_counts[s]);
    }
    let mut gram = if nr <= nc {
        residual.dot(&residual.t())
    } else {
        residual.t().dot(&residual)
    };
    let size = gram.nrows();
    let gram = gram
        .as_slice_mut()
        .expect("dot returns a standard-layout matrix");
    let largest = largest_eigenvalue(gram, size);
    Ok(largest.max(0.).sqrt())
}
