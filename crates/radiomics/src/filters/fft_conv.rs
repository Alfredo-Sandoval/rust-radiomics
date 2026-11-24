//! FFT valid convolution of plane stacks with large complex 2-D kernels (the Gabor filter).
//!
//! Execution: CPU numerical work in float64 through `rustfft`, on rayon worker threads. The
//! spatial Gabor kernel spans the whole image plane (manual Eq. 4.5), so a direct valid
//! convolution costs `H * W * K0 * K1` multiply-adds per plane and kernel; the FFT route
//! costs a few `L0 * L1 * log(L0 * L1)` for the same result up to float64 rounding.
//! Input contract: a once-padded stack `(planes, H + K0 - 1, W + K1 - 1)` and odd
//! `(K0, K1)` kernels no larger than the padded planes. The circular convolution length
//! `L >= H + K0 - 1` on each plane axis keeps every valid output index free of wrap-around,
//! so the valid response `out[j] = c[j + K - 1]` is exact convolution. Each plane and each
//! (plane, kernel) pair is processed by one task with its own buffers, so the result does
//! not depend on the number of threads.

use crate::config::filtering::GaborResponse;
use crate::filters::kernels::Vol;
use rayon::prelude::*;
use rustfft::num_complex::Complex64;
use rustfft::{Fft, FftPlanner};
use std::sync::Arc;

/// Largest working set of stored plane spectra, in bytes.
const SPECTRUM_BUDGET: usize = 768 << 20;

/// One complex kernel with row-major `(k0, k1)` real and imaginary parts.
pub(crate) struct ComplexKernel<'a> {
    pub dims: (usize, usize),
    pub real: &'a [f64],
    pub imaginary: &'a [f64],
}

/// Smallest `2^a 3^b 5^c` that is at least `n`, a length every FFT plan handles quickly.
fn fast_length(n: usize) -> usize {
    let mut best = usize::MAX;
    let mut p2 = 1usize;
    while p2 < 2 * n {
        let mut p3 = p2;
        while p3 < 2 * n {
            let mut p5 = p3;
            while p5 < 2 * n {
                if p5 >= n {
                    best = best.min(p5);
                }
                p5 *= 5;
            }
            p3 *= 3;
        }
        p2 *= 2;
    }
    best
}

/// Plans for a 2-D transform of `(rows, cols)` that leaves its spectrum transposed.
///
/// The forward transform ends with the `(cols, rows)` layout and the inverse starts from
/// it, so one transpose per direction replaces strided column passes.
struct Plan2d {
    rows: usize,
    cols: usize,
    forward_row: Arc<dyn Fft<f64>>,
    forward_col: Arc<dyn Fft<f64>>,
    inverse_row: Arc<dyn Fft<f64>>,
    inverse_col: Arc<dyn Fft<f64>>,
}

/// Reusable per-task buffers of one transform.
struct Workspace {
    scratch: Vec<Complex64>,
    transposed: Vec<Complex64>,
}

impl Plan2d {
    fn new(rows: usize, cols: usize) -> Self {
        let mut planner = FftPlanner::<f64>::new();
        Self {
            rows,
            cols,
            forward_row: planner.plan_fft_forward(cols),
            forward_col: planner.plan_fft_forward(rows),
            inverse_row: planner.plan_fft_inverse(cols),
            inverse_col: planner.plan_fft_inverse(rows),
        }
    }

    fn workspace(&self) -> Workspace {
        let scratch = [
            &self.forward_row,
            &self.forward_col,
            &self.inverse_row,
            &self.inverse_col,
        ]
        .iter()
        .map(|plan| plan.get_inplace_scratch_len())
        .max()
        .unwrap_or(0);
        Workspace {
            scratch: vec![Complex64::default(); scratch],
            transposed: vec![Complex64::default(); self.rows * self.cols],
        }
    }

    /// Unnormalised forward transform; `data` is `(rows, cols)` row-major on entry and
    /// `(cols, rows)` row-major (transposed spectrum) on return.
    fn forward(&self, data: &mut [Complex64], work: &mut Workspace) {
        self.forward_row
            .process_with_scratch(data, &mut work.scratch);
        transpose(data, &mut work.transposed, self.rows, self.cols);
        self.forward_col
            .process_with_scratch(&mut work.transposed, &mut work.scratch);
        data.copy_from_slice(&work.transposed);
    }

    /// Unnormalised inverse transform from the transposed spectrum back to `(rows, cols)`.
    fn inverse(&self, data: &mut [Complex64], work: &mut Workspace) {
        self.inverse_col
            .process_with_scratch(data, &mut work.scratch);
        transpose(data, &mut work.transposed, self.cols, self.rows);
        self.inverse_row
            .process_with_scratch(&mut work.transposed, &mut work.scratch);
        data.copy_from_slice(&work.transposed);
    }
}

/// `output[c][r] = input[r][c]` for a `(rows, cols)` input, in cache blocks.
fn transpose(input: &[Complex64], output: &mut [Complex64], rows: usize, cols: usize) {
    const BLOCK: usize = 32;
    for r0 in (0..rows).step_by(BLOCK) {
        for c0 in (0..cols).step_by(BLOCK) {
            for r in r0..(r0 + BLOCK).min(rows) {
                for c in c0..(c0 + BLOCK).min(cols) {
                    output[c * rows + r] = input[r * cols + c];
                }
            }
        }
    }
}

/// Valid convolution of every plane of `padded` with every complex kernel.
///
/// Returns, per kernel, a stack `(planes, H, W)` with `H = padded.rows - K0 + 1` holding the
/// modulus, real part or imaginary part of the complex response.
pub(crate) fn valid_complex_responses(
    padded: &Vol,
    kernels: &[ComplexKernel<'_>],
    response: GaborResponse,
) -> Vec<Vol> {
    let [planes, rows, cols] = padded.dims;
    let (k0, k1) = kernels.first().map_or((1, 1), |k| k.dims);
    let (out_rows, out_cols) = (rows + 1 - k0, cols + 1 - k1);
    let (length0, length1) = (fast_length(rows), fast_length(cols));
    let plan = Plan2d::new(length0, length1);
    let size = length0 * length1;
    let scale = 1.0 / size as f64;

    // Spectra of the kernels, in the transposed layout the forward transform leaves.
    let spectra: Vec<Vec<Complex64>> = kernels
        .par_iter()
        .map_init(
            || plan.workspace(),
            |work, kernel| {
                let mut data = vec![Complex64::default(); size];
                for a in 0..k0 {
                    for b in 0..k1 {
                        let index = a * kernel.dims.1 + b;
                        data[a * length1 + b] =
                            Complex64::new(kernel.real[index], kernel.imaginary[index]);
                    }
                }
                plan.forward(&mut data, work);
                data
            },
        )
        .collect();

    let plane_len = rows * cols;
    let out_len = out_rows * out_cols;
    let mut outputs: Vec<Vec<f64>> = kernels
        .iter()
        .map(|_| vec![0.0; planes * out_len])
        .collect();
    let group = (SPECTRUM_BUDGET / (size * 16))
        .max(rayon::current_num_threads())
        .clamp(1, planes.max(1));
    for first in (0..planes).step_by(group) {
        let last = (first + group).min(planes);
        let transforms: Vec<Vec<Complex64>> = (first..last)
            .into_par_iter()
            .map_init(
                || plan.workspace(),
                |work, p| {
                    let mut data = vec![Complex64::default(); size];
                    for r in 0..rows {
                        let source = &padded.data[p * plane_len + r * cols..][..cols];
                        for (slot, &value) in data[r * length1..][..cols].iter_mut().zip(source) {
                            *slot = Complex64::new(value, 0.0);
                        }
                    }
                    plan.forward(&mut data, work);
                    data
                },
            )
            .collect();
        let count = last - first;
        let tiles: Vec<Vec<f64>> = (0..count * kernels.len())
            .into_par_iter()
            .map_init(
                || plan.workspace(),
                |work, job| {
                    let (plane, kernel) = (job / kernels.len(), job % kernels.len());
                    let mut data: Vec<Complex64> = transforms[plane]
                        .iter()
                        .zip(&spectra[kernel])
                        .map(|(&x, &k)| x * k)
                        .collect();
                    plan.inverse(&mut data, work);
                    let mut tile = Vec::with_capacity(out_len);
                    for r in 0..out_rows {
                        let line = &data[(r + k0 - 1) * length1 + (k1 - 1)..][..out_cols];
                        tile.extend(line.iter().map(|c| {
                            let (re, im) = (c.re * scale, c.im * scale);
                            match response {
                                GaborResponse::Modulus => re.hypot(im),
                                GaborResponse::Real => re,
                                GaborResponse::Imaginary => im,
                            }
                        }));
                    }
                    tile
                },
            )
            .collect();
        for (job, tile) in tiles.into_iter().enumerate() {
            let (plane, kernel) = (job / kernels.len(), job % kernels.len());
            outputs[kernel][(first + plane) * out_len..][..out_len].copy_from_slice(&tile);
        }
    }
    outputs
        .into_iter()
        .map(|data| Vol {
            dims: [planes, out_rows, out_cols],
            data,
        })
        .collect()
}
