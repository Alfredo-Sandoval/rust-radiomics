//! 2-D local binary patterns (PyRadiomics `lbp-2D`).
//!
//! Mirrors `mlxradiomics/filters/lbp2d.py` (249 lines at mlx-radiomics b58783c).
//! Status: done, all five methods (`default`, `ror`, `uniform`, `nri_uniform`, `var`).
//!
//! Execution: CPU numerical work in float64, one image row per rayon task. The Metal kernel's exact
//! float32 expansion arithmetic (`source_residual`, `source_tail`) exists to recover the
//! float64 source sign; here the source already is float64, so those arguments and the
//! `[2**-90, 2**90]` source-magnitude guard are not ported. The categorical bit keeps the
//! same exactness contract in float64: sampling coordinates are declared to five decimals,
//! bilinear weights are the exact integers over `100000**2`, and the bit is the sign of
//! `sum_i w_i (sample_i - center)`, so equal samples tie exactly and set the bit.
//! Input contract: a 2-D array, or a 3-D array processed as slices perpendicular to `axis`.

use crate::config::filtering::LbpMethod;
use crate::error::{Error, Result};
use crate::filters::convolution::move_axis;
use ndarray::{ArrayD, Axis};
use rayon::prelude::*;
use std::f64::consts::PI;

/// Declared ring geometry of one sample: integer offsets, fractions and exact weights.
struct RingPoint {
    row_floor: i64,
    col_floor: i64,
    row_fraction: f64,
    col_fraction: f64,
    /// Integer weights of the corners `(0,0), (0,1), (1,0), (1,1)` over `100000**2`.
    weights: [f64; 4],
}

fn round5(value: f64) -> f64 {
    (value * 1e5).round() / 1e5
}

fn ring_points(samples: u32, radius: f64) -> Vec<RingPoint> {
    (0..samples)
        .map(|k| {
            let angle = 2.0 * PI * f64::from(k) / f64::from(samples);
            let row = round5(-radius * angle.sin());
            let col = round5(radius * angle.cos());
            let (row_floor, col_floor) = (row.floor(), col.floor());
            let (row_fraction, col_fraction) = (row - row_floor, col - col_floor);
            let ry = (row_fraction * 100_000.0).round();
            let rx = (col_fraction * 100_000.0).round();
            RingPoint {
                row_floor: row_floor as i64,
                col_floor: col_floor as i64,
                row_fraction,
                col_fraction,
                weights: [
                    (100_000.0 - ry) * (100_000.0 - rx),
                    (100_000.0 - ry) * rx,
                    ry * (100_000.0 - rx),
                    ry * rx,
                ],
            }
        })
        .collect()
}

/// Map the raw bit `code` to the output value for `method`.
fn method_value(method: LbpMethod, samples: u32, code: u32, bits: &[bool], ring: &[f64]) -> f64 {
    let n = samples as usize;
    let ones = bits.iter().filter(|&&b| b).count();
    let changes = bits.windows(2).filter(|w| w[0] != w[1]).count();
    let first_one = bits.iter().position(|&b| b);
    let first_zero = bits.iter().position(|&b| !b);
    match method {
        LbpMethod::Default => f64::from(code),
        LbpMethod::Uniform => {
            if changes <= 2 {
                ones as f64
            } else {
                (n + 1) as f64
            }
        }
        LbpMethod::NonrotationUniform => {
            let top = n * (n - 1);
            if changes > 2 {
                (top + 2) as f64
            } else if ones == 0 {
                0.0
            } else if ones == n {
                (top + 1) as f64
            } else {
                let first_one = first_one.expect("ones > 0") as i64;
                let first_zero = first_zero.expect("ones < n") as i64;
                let tail = if first_one == 0 {
                    ones as i64 - first_zero
                } else {
                    n as i64 - first_one
                };
                (1 + (ones - 1) * n) as f64 + tail as f64
            }
        }
        LbpMethod::RotationInvariant => {
            let mut code = code;
            let mut value = code;
            for _ in 1..samples {
                code = (code >> 1) | ((code & 1) << (samples - 1));
                value = value.min(code);
            }
            f64::from(value)
        }
        LbpMethod::Variance => {
            let deltas: Vec<f64> = ring.iter().map(|r| r - ring[0]).collect();
            let mean = deltas.iter().sum::<f64>() / n as f64;
            let value = deltas.iter().map(|d| (d - mean) * (d - mean)).sum::<f64>() / n as f64;
            if value == 0.0 { f64::NAN } else { value }
        }
    }
}

/// Codes of one row of `slice`; `bits` and `ring` are caller-owned scratch of one ring.
#[allow(clippy::too_many_arguments)]
fn lbp_row(
    slice: &[f64],
    height: usize,
    width: usize,
    row: usize,
    points: &[RingPoint],
    samples: u32,
    method: LbpMethod,
    bits: &mut [bool],
    ring: &mut [f64],
    out: &mut [f64],
) {
    for col in 0..width {
        let center = slice[row * width + col];
        let mut code = 0u32;
        for (k, point) in points.iter().enumerate() {
            let mut corner = [0.0; 4];
            for a in 0..2i64 {
                for b in 0..2i64 {
                    let yy = row as i64 + point.row_floor + a;
                    let xx = col as i64 + point.col_floor + b;
                    let sample =
                        if (0..height as i64).contains(&yy) && (0..width as i64).contains(&xx) {
                            slice[yy as usize * width + xx as usize]
                        } else {
                            0.0
                        };
                    corner[(a * 2 + b) as usize] = sample - center;
                }
            }
            // Exact integer-weight sign test; ties set the bit.
            let exact: f64 = point.weights.iter().zip(&corner).map(|(w, c)| w * c).sum();
            bits[k] = exact >= 0.0;
            code |= u32::from(bits[k]) << k;
            // Nested lerps of corner differences keep a uniform ring exactly constant.
            let top = corner[0] + point.col_fraction * (corner[1] - corner[0]);
            let bottom = corner[2] + point.col_fraction * (corner[3] - corner[2]);
            ring[k] = top + point.row_fraction * (bottom - top);
        }
        out[col] = method_value(method, samples, code, bits, ring);
    }
}

/// Compute a 2-D local binary pattern image, slice by slice.
///
/// For each voxel, `samples` points on a circle of `radius` voxels (spacing is not used)
/// are read by bilinear interpolation, with samples outside the slice taken as 0. A bit is
/// set where the sample is at least the centre value, and the bits are coded by `method`.
///
/// Args: `image` is 2-D, or 3-D processed as slices perpendicular to `axis` (ignored for
/// 2-D); `samples` is the number of points on the circle (1-24).
///
/// Returns an array of the image's shape; `var` is NaN where the ring variance is zero.
pub fn lbp2d(
    image: &ArrayD<f64>,
    samples: u32,
    radius: f64,
    method: LbpMethod,
    axis: usize,
) -> Result<ArrayD<f64>> {
    if !(1..=24).contains(&samples) {
        return Err(Error::InvalidInput(
            "LBP2D samples must be between 1 and 24".into(),
        ));
    }
    if !(2..=3).contains(&image.ndim()) {
        return Err(Error::InvalidInput(
            "LBP2D requires a 2-D or 3-D image".into(),
        ));
    }
    if image.ndim() == 3 && axis >= 3 {
        return Err(Error::InvalidInput(
            "force2d_axis must index a NumPy image axis".into(),
        ));
    }
    if image
        .as_standard_layout()
        .as_slice()
        .expect("standard layout is contiguous")
        .par_iter()
        .any(|v| !v.is_finite())
    {
        return Err(Error::InvalidInput(
            "LBP2D source sign is unresolved outside the supported nonzero source-part magnitude range [2**-90, 2**90]"
                .into(),
        ));
    }
    let points = ring_points(samples, radius);
    let stack = if image.ndim() == 2 {
        image
            .view()
            .insert_axis(Axis(0))
            .as_standard_layout()
            .into_owned()
    } else {
        move_axis(image, axis, 0)
    };
    let (height, width) = (stack.shape()[1], stack.shape()[2]);
    let data = stack.as_slice().expect("standard layout");
    let mut output = vec![0.0; data.len()];
    let ring_len = points.len();
    // Every voxel depends only on its own slice, so rows are independent tasks.
    output
        .par_chunks_mut(width.max(1))
        .enumerate()
        .for_each_init(
            || (vec![false; ring_len], vec![0.0; ring_len]),
            |(bits, ring), (index, out)| {
                let (slice, row) = (index / height, index % height);
                lbp_row(
                    &data[slice * height * width..(slice + 1) * height * width],
                    height,
                    width,
                    row,
                    &points,
                    samples,
                    method,
                    bits,
                    ring,
                    out,
                );
            },
        );

    let result = ArrayD::from_shape_vec(stack.raw_dim(), output).expect("shape");
    Ok(if image.ndim() == 2 {
        result
            .index_axis_move(Axis(0), 0)
            .as_standard_layout()
            .into_owned()
    } else {
        move_axis(&result, 0, axis)
    })
}
