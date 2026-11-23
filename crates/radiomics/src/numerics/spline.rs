//! Cubic B-spline interpolation (prefilter and point sampling), used by LBP3D and by the
//! file-route resampler.
//!
//! Mirrors `mlxradiomics/numerics/spline.py` (129 lines at mlx-radiomics b58783c).
//!
//! Execution: CPU numerical work in float64; the prefilter runs on rayon worker threads over
//! lines or column blocks, the rest on the caller's thread. The
//! Python source runs these two stages as float32 Metal kernels; this port has no GPU
//! path and states a float64 result (tests compare against float32 within the float32
//! rounding of the mlx kernels). Mirror (whole-sample, period `2 (n - 1)`) boundary
//! conditions, as in mlx and ITK.
//!
//! In addition to the exact prefilter of mlx (`spline_coefficients`), [`itk_spline_coefficients`]
//! reproduces ITK's `BSplineDecompositionImageFilter` (causal initial coefficient truncated
//! at the horizon `ceil(log(1e-10) / log(|z|))` terms), which `SimpleITK`'s `sitkBSpline`
//! resampler uses. The two agree to about 1e-10 relative.
//!
//! Status: done (`spline_coefficients`, `cubic_samples`, plus the ITK variants).

use ndarray::{ArrayD, IxDyn};
use rayon::prelude::*;

/// Pole `sqrt(3) - 2` of the cubic B-spline prefilter.
pub const POLE: f64 = -0.267_949_192_431_122_7;

/// ITK's `BSplineDecompositionImageFilter` default tolerance for the causal horizon.
pub const ITK_TOLERANCE: f64 = 1e-10;

/// Prefilter one line in place: gain, causal and anticausal recursion with mirror
/// boundaries.
///
/// `tolerance = None` evaluates the exact initial causal sum (mlx kernel, gain 6);
/// `Some(tol)` truncates it at `ceil(log(tol) / log(|z|))` terms when that is shorter than
/// the line, with ITK's gain `(1 - z)(1 - 1/z)`.
pub fn prefilter_line(line: &mut [f64], tolerance: Option<f64>) {
    let length = line.len();
    if length < 2 {
        return;
    }
    let z = POLE;
    let gain = if tolerance.is_some() {
        (1.0 - z) * (1.0 - 1.0 / z)
    } else {
        6.0
    };
    for value in line.iter_mut() {
        *value *= gain;
    }
    let horizon = tolerance.map_or(length, |tol| {
        (tol.ln() / z.abs().ln()).ceil().max(0.0) as usize
    });
    if horizon < length {
        let mut sum = line[0];
        let mut zn = z;
        for value in &line[1..horizon] {
            sum += zn * value;
            zn *= z;
        }
        line[0] = sum;
    } else {
        let zn = z.powi(length as i32 - 1);
        let mut sum = line[0] + zn * line[length - 1];
        // mlx: initial += z^k (c[k] + z^(n-1) c[n-1-k]); ITK folds the same terms.
        let mut zk = z;
        for k in 1..length - 1 {
            sum += zk * (line[k] + zn * line[length - 1 - k]);
            zk *= z;
        }
        line[0] = sum / (1.0 - zn * zn);
    }
    for k in 1..length {
        line[k] += z * line[k - 1];
    }
    line[length - 1] = (z / (z * z - 1.0)) * (z * line[length - 2] + line[length - 1]);
    for k in (0..length - 1).rev() {
        line[k] = z * (line[k + 1] - line[k]);
    }
}

/// [`prefilter_line`] applied to `rows.len()` lines at once, one per column of `rows`.
///
/// `rows[k]` holds sample `k` of every line, so each step is a vector operation over the
/// columns; per column the floating-point operations and their order equal
/// `prefilter_line`.
fn prefilter_rows(rows: &mut [&mut [f64]], tolerance: Option<f64>) {
    let length = rows.len();
    if length < 2 {
        return;
    }
    let z = POLE;
    let gain = if tolerance.is_some() {
        (1.0 - z) * (1.0 - 1.0 / z)
    } else {
        6.0
    };
    for row in rows.iter_mut() {
        for value in row.iter_mut() {
            *value *= gain;
        }
    }
    let horizon = tolerance.map_or(length, |tol| {
        (tol.ln() / z.abs().ln()).ceil().max(0.0) as usize
    });
    if horizon < length {
        let (head, tail) = rows.split_at_mut(1);
        let mut zn = z;
        for row in &tail[..horizon - 1] {
            for (sum, &value) in head[0].iter_mut().zip(row.iter()) {
                *sum += zn * value;
            }
            zn *= z;
        }
    } else {
        let zn = z.powi(length as i32 - 1);
        let width = rows[0].len();
        let mut sum: Vec<f64> = (0..width)
            .map(|i| rows[0][i] + zn * rows[length - 1][i])
            .collect();
        let mut zk = z;
        for k in 1..length - 1 {
            for (i, total) in sum.iter_mut().enumerate() {
                *total += zk * (rows[k][i] + zn * rows[length - 1 - k][i]);
            }
            zk *= z;
        }
        for (slot, total) in rows[0].iter_mut().zip(sum) {
            *slot = total / (1.0 - zn * zn);
        }
    }
    for k in 1..length {
        let (done, rest) = rows.split_at_mut(k);
        for (value, &previous) in rest[0].iter_mut().zip(done[k - 1].iter()) {
            *value += z * previous;
        }
    }
    let factor = z / (z * z - 1.0);
    {
        let (done, rest) = rows.split_at_mut(length - 1);
        for (value, &previous) in rest[0].iter_mut().zip(done[length - 2].iter()) {
            *value = factor * (z * previous + *value);
        }
    }
    for k in (0..length - 1).rev() {
        let (current, next) = rows.split_at_mut(k + 1);
        for (value, &following) in current[k].iter_mut().zip(next[0].iter()) {
            *value = z * (following - *value);
        }
    }
}

/// Columns per task of the row-vectorised prefilter.
const PREFILTER_COLUMNS: usize = 256;

fn prefilter_all(image: &ArrayD<f64>, tolerance: Option<f64>) -> ArrayD<f64> {
    let shape = image.shape().to_vec();
    let mut data: Vec<f64> = match image.as_slice() {
        Some(flat) => flat.to_vec(),
        None => image.iter().copied().collect(),
    };
    for axis in 0..shape.len() {
        let length = shape[axis];
        if length == 1 {
            continue;
        }
        let inner: usize = shape[axis + 1..].iter().product();
        if inner == 1 {
            data.par_chunks_mut(length)
                .for_each(|line| prefilter_line(line, tolerance));
            continue;
        }
        // Each outer block holds `length` rows of `inner` samples; tasks take column blocks.
        let mut tasks: Vec<Vec<&mut [f64]>> = Vec::new();
        for block in data.chunks_mut(length * inner) {
            let first = tasks.len();
            for row in block.chunks_mut(inner) {
                for (index, piece) in row.chunks_mut(PREFILTER_COLUMNS).enumerate() {
                    if tasks.len() <= first + index {
                        tasks.push(Vec::with_capacity(length));
                    }
                    tasks[first + index].push(piece);
                }
            }
        }
        tasks
            .par_iter_mut()
            .for_each(|rows| prefilter_rows(rows, tolerance));
    }
    ArrayD::from_shape_vec(IxDyn(&shape), data).expect("prefilter keeps the shape")
}

/// Cubic B-spline coefficients of an image for interpolation (`spline_coefficients`).
///
/// Applies the recursive prefilter (pole `sqrt(3) - 2`) along every axis longer than one
/// voxel, with mirror boundary conditions and the exact initial causal sum.
pub fn spline_coefficients(image: &ArrayD<f64>) -> ArrayD<f64> {
    prefilter_all(image, None)
}

/// Coefficients as ITK computes them (horizon-truncated causal initialisation,
/// tolerance [`ITK_TOLERANCE`]).
pub fn itk_spline_coefficients(image: &ArrayD<f64>) -> ArrayD<f64> {
    prefilter_all(image, Some(ITK_TOLERANCE))
}

/// Mirror a tap index into `[0, n)` with period `2 (n - 1)` (whole-sample reflection).
pub fn mirror_index(index: i64, n: usize) -> usize {
    let period = 2 * (n as i64 - 1);
    if period == 0 {
        return 0;
    }
    let q = ((index % period) + period) % period;
    (if q < n as i64 { q } else { period - q }) as usize
}

/// Cubic B-spline tap weights for the fractional part `x` of a coordinate, for the taps
/// at `floor(c) - 1 ..= floor(c) + 2`.
pub fn cubic_weights(x: f64) -> [f64; 4] {
    let y = 1.0 - x;
    let w0 = y * y * y / 6.0;
    let w1 = (x * x * (x - 2.0) * 3.0 + 4.0) / 6.0;
    let w2 = (y * y * (y - 2.0) * 3.0 + 4.0) / 6.0;
    [w0, w1, w2, 1.0 - w0 - w1 - w2]
}

/// Evaluate the spline over row-major coefficient storage at a continuous index
/// (array-axis order, 1 to 3 axes) with mirror boundaries.
///
/// No bounds test: the caller decides what lies outside `[0, n - 1]`.
pub fn evaluate_flat(data: &[f64], shape: &[usize], coordinate: &[f64]) -> f64 {
    let dims = shape.len();
    debug_assert!(dims <= 3 && coordinate.len() == dims);
    let mut taps = [[0usize; 4]; 3];
    let mut weights = [[0f64; 4]; 3];
    for axis in 0..dims {
        let c = coordinate[axis];
        let floor = c.floor();
        let left = floor as i64 - 1;
        if left >= 0 && left + 3 < shape[axis] as i64 {
            // All four taps lie inside the axis: no reflection.
            for k in 0..4 {
                taps[axis][k] = left as usize + k;
            }
        } else {
            for k in 0..4 {
                taps[axis][k] = mirror_index(left + k as i64, shape[axis]);
            }
        }
        weights[axis] = cubic_weights(c - floor);
    }
    let mut strides = [0usize; 3];
    let mut stride = 1;
    for axis in (0..dims).rev() {
        strides[axis] = stride;
        stride *= shape[axis];
    }
    if dims == 3 {
        // The 64 taps in the order of the generic loop below: the last axis varies fastest
        // and each weight is `((1 * w_last) * w_middle) * w_first`.
        let mut value = 0.0;
        for k0 in 0..4 {
            let base0 = taps[0][k0] * strides[0];
            for k1 in 0..4 {
                let base = base0 + taps[1][k1] * strides[1];
                for k2 in 0..4 {
                    let weight = weights[2][k2] * weights[1][k1] * weights[0][k0];
                    value += data[base + taps[2][k2]] * weight;
                }
            }
        }
        return value;
    }
    let mut value = 0.0;
    for combo in 0..4usize.pow(dims as u32) {
        let mut rest = combo;
        let mut weight = 1.0;
        let mut flat = 0usize;
        for axis in (0..dims).rev() {
            let k = rest % 4;
            rest /= 4;
            flat += taps[axis][k] * strides[axis];
            weight *= weights[axis][k];
        }
        value += data[flat] * weight;
    }
    value
}
