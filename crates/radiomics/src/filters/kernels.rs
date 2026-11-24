//! Spatial-filter primitives: boundary extension, window sums and valid convolutions.
//!
//! Mirrors `mlxradiomics/filters/_kernels.py` (94 lines at mlx-radiomics b58783c), which holds
//! the fused compensated window sums. This module additionally holds the CPU
//! building blocks that mlx-radiomics gets from `mx.pad`, `mx.conv2d` and `mx.conv3d`.
//! Status: done.
//!
//! Execution: CPU numerical work in float64 on flat row-major buffers. Window sums are
//! Neumaier-compensated per window (no prefix-sum subtraction), as the Metal kernel is.
//! Boundary extension, window sums and 1-D and dense valid convolutions run on rayon
//! worker threads, one output row (or, on the last axis, one lane) per task, so each output
//! sample is computed by one thread in a fixed order. Arrays of two axes are promoted to
//! three with a leading axis of length one, so one set of loops serves both.

use crate::config::filtering::FilterPadding;
use crate::error::{Error, Result};
use ndarray::{ArrayD, IxDyn};
use rayon::prelude::*;

/// Row-major copy of `array`; a memcpy when the array is already in standard layout.
pub(crate) fn copy_flat(array: &ArrayD<f64>) -> Vec<f64> {
    match array.as_slice() {
        Some(flat) => flat.to_vec(),
        None => array.iter().copied().collect(),
    }
}

/// Row-major buffer with three axes (a 2-D array has a leading axis of length one).
#[derive(Clone, Debug)]
pub(crate) struct Vol {
    /// Axis lengths.
    pub dims: [usize; 3],
    /// Row-major values.
    pub data: Vec<f64>,
}

impl Vol {
    /// Promote a 2-D or 3-D array.
    pub fn from_array(array: &ArrayD<f64>) -> Result<Self> {
        let ndim = array.ndim();
        if !(2..=3).contains(&ndim) {
            return Err(Error::InvalidInput(
                "filters require a 2-D or 3-D image".into(),
            ));
        }
        let mut dims = [1usize; 3];
        for (axis, &size) in array.shape().iter().enumerate() {
            dims[axis + 3 - ndim] = size;
        }
        Ok(Self {
            dims,
            data: copy_flat(array),
        })
    }

    /// Demote to an array with `ndim` axes.
    pub fn into_array(self, ndim: usize) -> ArrayD<f64> {
        let shape: Vec<usize> = self.dims[3 - ndim..].to_vec();
        ArrayD::from_shape_vec(IxDyn(&shape), self.data).expect("volume buffer matches its shape")
    }
}

/// Source index of extended position `i` on an axis of length `n`; `None` for the constant.
fn source_index(i: isize, n: usize, padding: FilterPadding) -> Option<usize> {
    let size = n as isize;
    match padding {
        FilterPadding::Constant => (0..size).contains(&i).then_some(i as usize),
        FilterPadding::Nearest => Some(i.clamp(0, size - 1) as usize),
        FilterPadding::Mirror => {
            let m = i.rem_euclid(2 * size);
            Some(if m < size { m } else { 2 * size - 1 - m } as usize)
        }
        FilterPadding::Periodic => Some(i.rem_euclid(size) as usize),
    }
}

/// Extend `vol` by `widths` (before, after) per axis with an IBSI boundary condition.
pub(crate) fn pad_vol(
    vol: &Vol,
    widths: [(usize, usize); 3],
    padding: FilterPadding,
    constant: f64,
) -> Vol {
    if widths.iter().all(|&(b, a)| b == 0 && a == 0) {
        return vol.clone();
    }
    let out_dims = [
        vol.dims[0] + widths[0].0 + widths[0].1,
        vol.dims[1] + widths[1].0 + widths[1].1,
        vol.dims[2] + widths[2].0 + widths[2].1,
    ];
    let maps: Vec<Vec<Option<usize>>> = (0..3)
        .map(|axis| {
            (0..out_dims[axis])
                .map(|i| {
                    source_index(
                        i as isize - widths[axis].0 as isize,
                        vol.dims[axis],
                        padding,
                    )
                })
                .collect()
        })
        .collect();
    let mut data = vec![constant; out_dims.iter().product()];
    let row_len = out_dims[2];
    let middle = widths[2].0..widths[2].0 + vol.dims[2];
    data.par_chunks_mut(row_len.max(1))
        .enumerate()
        .for_each(|(row, out)| {
            let (Some(a), Some(b)) = (maps[0][row / out_dims[1]], maps[1][row % out_dims[1]])
            else {
                return;
            };
            let base = (a * vol.dims[1] + b) * vol.dims[2];
            // The interior of the row is a straight copy; the borders follow the map.
            out[middle.clone()].copy_from_slice(&vol.data[base..base + vol.dims[2]]);
            for (ic, slot) in out.iter_mut().enumerate() {
                if !middle.contains(&ic)
                    && let Some(c) = maps[2][ic]
                {
                    *slot = vol.data[base + c];
                }
            }
        });
    Vol {
        dims: out_dims,
        data,
    }
}

/// Run `f` in parallel on column blocks of a row-major `(outer, length, inner)` buffer.
///
/// A task receives the `length` row pieces of one outer index restricted to at most
/// `columns` consecutive inner positions, so strided lanes can be processed without
/// aliasing mutable access.
pub(crate) fn par_column_blocks<T: Send>(
    data: &mut [T],
    length: usize,
    inner: usize,
    columns: usize,
    f: impl Fn(&mut [&mut [T]]) + Sync,
) {
    let mut tasks: Vec<Vec<&mut [T]>> = Vec::new();
    for block in data.chunks_mut((length * inner).max(1)) {
        let first = tasks.len();
        for row in block.chunks_mut(inner.max(1)) {
            for (index, piece) in row.chunks_mut(columns.max(1)).enumerate() {
                if tasks.len() <= first + index {
                    tasks.push(Vec::with_capacity(length));
                }
                tasks[first + index].push(piece);
            }
        }
    }
    tasks.par_iter_mut().for_each(|rows| f(rows));
}

fn axis_layout(dims: [usize; 3], axis: usize) -> (usize, usize, usize) {
    let outer: usize = dims[..axis].iter().product();
    let inner: usize = dims[axis + 1..].iter().product();
    (outer, dims[axis], inner)
}

/// Valid forward box sum of `support` samples along `axis`, compensated per window.
///
/// The result is shorter by `support - 1` on `axis`. Mirrors `window_pair_sum`.
pub(crate) fn window_sum(vol: &Vol, axis: usize, support: usize) -> Result<Vol> {
    if axis >= 3 {
        return Err(Error::InvalidInput(
            "window axis is outside the input dimensions".into(),
        ));
    }
    if support < 1 || support > vol.dims[axis] {
        return Err(Error::InvalidInput(
            "window support must fit the input axis".into(),
        ));
    }
    if support == 1 {
        return Ok(vol.clone());
    }
    let (outer, length, inner) = axis_layout(vol.dims, axis);
    let out_len = length - (support - 1);
    let mut dims = vol.dims;
    dims[axis] = out_len;
    let mut data = vec![0.0; outer * out_len * inner];
    // A task is one output row of `inner` samples, or, on the last axis (`inner == 1`),
    // one whole lane; `step` is the source distance between window taps and `width` the
    // number of outputs of the task, so a task reads `support` equally long source slices.
    let (task, step) = if inner == 1 {
        (out_len, 1)
    } else {
        (inner, inner)
    };
    data.par_chunks_mut(task.max(1)).enumerate().for_each_init(
        || vec![0.0; task],
        |residual, (index, out)| {
            let base = if inner == 1 {
                index * length
            } else {
                let (o, j) = (index / out_len, index % out_len);
                (o * length + j) * inner
            };
            let width = out.len();
            let residual = &mut residual[..width];
            residual.fill(0.0);
            out.copy_from_slice(&vol.data[base..base + width]);
            for t in 1..support {
                let terms = &vol.data[base + t * step..base + t * step + width];
                for ((total, rest), &term) in out.iter_mut().zip(residual.iter_mut()).zip(terms) {
                    let added = *total + term;
                    *rest += if total.abs() >= term.abs() {
                        (*total - added) + term
                    } else {
                        (term - added) + *total
                    };
                    *total = added;
                }
            }
            for (total, &rest) in out.iter_mut().zip(residual.iter()) {
                *total += rest;
            }
        },
    );
    Ok(Vol { dims, data })
}

/// Valid true convolution of one 1-D kernel along `axis`.
///
/// `out[j] = sum_t taps[t] * in[j + n - 1 - t]`, shorter by `n - 1` on `axis`.
pub(crate) fn valid_conv_axis(vol: &Vol, axis: usize, taps: &[f64]) -> Result<Vol> {
    let n = taps.len();
    if n == 0 || n > vol.dims[axis] {
        return Err(Error::InvalidInput(
            "convolution kernel must fit the padded axis".into(),
        ));
    }
    if n == 1 && taps[0] == 1.0 {
        return Ok(vol.clone());
    }
    let (outer, length, inner) = axis_layout(vol.dims, axis);
    let out_len = length - (n - 1);
    let mut dims = vol.dims;
    dims[axis] = out_len;
    let mut data = vec![0.0; outer * out_len * inner];
    // One task per output row, or per whole lane on the last axis, as in `window_sum`.
    let (task, step) = if inner == 1 {
        (out_len, 1)
    } else {
        (inner, inner)
    };
    data.par_chunks_mut(task.max(1))
        .enumerate()
        .for_each(|(index, out)| {
            let base = if inner == 1 {
                index * length
            } else {
                let (o, j) = (index / out_len, index % out_len);
                (o * length + j) * inner
            };
            let width = out.len();
            for (t, &tap) in taps.iter().enumerate() {
                let src = base + (n - 1 - t) * step;
                for (value, &x) in out.iter_mut().zip(&vol.data[src..src + width]) {
                    *value += tap * x;
                }
            }
        });
    Ok(Vol { dims, data })
}
