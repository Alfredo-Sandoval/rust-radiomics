//! Sampled LoG and Simoncelli/Riesz frequency-domain filters.
//!
//! Mirrors `mlxradiomics/filters/spectral.py` (533 lines at mlx-radiomics b58783c).
//! Status: done. Unlike the Metal runtime, which disables Simoncelli and Riesz on
//! MLX 0.32.4 (FFT gate), this port runs them on a CPU FFT, so [`FFT_AVAILABLE`] is always
//! true and the gate error is never raised. Parity is established against the IBSI 2
//! reference maps and a NumPy-FFT oracle, not against mlx-radiomics goldens.
//!
//! Execution: CPU numerical work in float64. The FFT is `rustfft` (mixed-radix with
//! Bluestein fallback for awkward lengths), run on rayon worker threads one batch of lanes
//! per task; the sampled LoG convolution is the separable decomposition of
//! `convolution.rs`; spectrum products and the band/multiplier construction run per scan
//! line on rayon worker threads, each voxel computed independently of the thread count.
//! mlx-radiomics' float32 FFT overflow scaling (`_spectral_scale`) is not needed in float64
//! and is not ported. Input contract: 2-D or 3-D [`ArrayD`] in NumPy (z, y, x) order.

use crate::config::filtering::{FilterPadding, FourierGrid};
use crate::error::{Error, Result};
use crate::filters::convolution::{convolve_separable_native, pad_image};
use crate::filters::kernels::par_column_blocks;
use ndarray::{ArrayD, Dimension, IxDyn};
use rayon::prelude::*;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex64;
use std::f64::consts::PI;

/// Lanes transformed together by one task along a strided axis.
const FFT_COLUMNS: usize = 16;

/// Row-major N-d FFT over `axes`; the inverse is normalised by the transformed length.
///
/// Lanes are independent, so contiguous lanes are batched per task and strided lanes are
/// gathered in blocks of [`FFT_COLUMNS`]; every lane gets the same plan arithmetic as a
/// lone transform, so the result does not depend on the batching or the thread count.
pub(crate) fn fftn(data: &mut [Complex64], dims: &[usize], axes: &[usize], inverse: bool) {
    let mut planner = FftPlanner::<f64>::new();
    for &axis in axes {
        let length = dims[axis];
        let inner: usize = dims[axis + 1..].iter().product();
        let plan = if inverse {
            planner.plan_fft_inverse(length)
        } else {
            planner.plan_fft_forward(length)
        };
        let scale = 1.0 / length as f64;
        let scratch_len = plan.get_inplace_scratch_len();
        if inner == 1 {
            let lanes = (8192 / length).max(1);
            data.par_chunks_mut(length * lanes).for_each(|chunk| {
                let mut scratch = vec![Complex64::new(0.0, 0.0); scratch_len];
                plan.process_with_scratch(chunk, &mut scratch);
                if inverse {
                    chunk.iter_mut().for_each(|value| *value *= scale);
                }
            });
            continue;
        }
        par_column_blocks(data, length, inner, FFT_COLUMNS, |rows| {
            let width = rows[0].len();
            let mut lanes = vec![Complex64::new(0.0, 0.0); width * length];
            let mut scratch = vec![Complex64::new(0.0, 0.0); scratch_len];
            for (k, row) in rows.iter().enumerate() {
                for (j, &value) in row.iter().enumerate() {
                    lanes[j * length + k] = value;
                }
            }
            plan.process_with_scratch(&mut lanes, &mut scratch);
            if inverse {
                lanes.iter_mut().for_each(|value| *value *= scale);
            }
            for (k, row) in rows.iter_mut().enumerate() {
                for (j, slot) in row.iter_mut().enumerate() {
                    *slot = lanes[j * length + k];
                }
            }
        });
    }
}

/// `np.fft.fftfreq(n)` scaled by `scale`.
fn fftfreq(size: usize, scale: f64) -> Vec<f64> {
    let n = size as isize;
    (0..n)
        .map(|k| {
            let signed = if k < (n + 1) / 2 { k } else { k - n };
            signed as f64 / size as f64 * scale
        })
        .collect()
}

/// Endpoint-inclusive coordinates from -1 to 1 per axis, in centred array order.
///
/// This is the MIRP/IBSI 2 consensus sampling `(k - (N - 1) / 2) / ((N - 1) / 2)`. A
/// length-one axis has no well-defined endpoint-inclusive sampling.
fn centered_coordinates(shape: &[usize]) -> Result<Vec<Vec<f64>>> {
    if shape.iter().any(|&size| size < 2) {
        return Err(Error::InvalidInput(
            "the centered Fourier grid requires at least two samples per axis".into(),
        ));
    }
    Ok(shape
        .iter()
        .map(|&size| {
            let half = (size as f64 - 1.0) / 2.0;
            (0..size).map(|k| (k as f64 - half) / half).collect()
        })
        .collect())
}

/// Coordinates per axis for `grid` in the order the FFT uses (centred ones are shifted later).
fn grid_coordinates(shape: &[usize], grid: FourierGrid, dft_scale: f64) -> Result<Vec<Vec<f64>>> {
    match grid {
        FourierGrid::Dft => Ok(shape.iter().map(|&n| fftfreq(n, dft_scale)).collect()),
        FourierGrid::Centered => centered_coordinates(shape),
    }
}

/// Row-major array over `shape` whose voxel at `index` is `f(index)`; one scan line of the
/// last axis per rayon task.
fn fill_rows<T: Send + Clone + Default>(
    shape: &[usize],
    f: impl Fn(&[usize]) -> T + Sync,
) -> Vec<T> {
    let ndim = shape.len();
    let line = shape[ndim - 1];
    let mut out = vec![T::default(); shape.iter().product()];
    out.par_chunks_mut(line.max(1))
        .enumerate()
        .for_each(|(row, chunk)| {
            let mut index = [0usize; 3];
            let mut rest = row;
            for axis in (0..ndim - 1).rev() {
                index[axis] = rest % shape[axis];
                rest /= shape[axis];
            }
            for (j, slot) in chunk.iter_mut().enumerate() {
                index[ndim - 1] = j;
                *slot = f(&index[..ndim]);
            }
        });
    out
}

/// Squared norm of the coordinates at one grid index, accumulated axis by axis.
fn squared_at(coordinates: &[Vec<f64>], index: &[usize]) -> f64 {
    let mut squared = 0.0;
    for (axis, &i) in index.iter().enumerate() {
        let value = coordinates[axis][i];
        squared += value * value;
    }
    squared
}

/// `np.fft.ifftshift` of a row-major array: `out[i] = in[(i + n // 2) % n]` per axis.
fn ifftshift<T: Copy + Send + Sync + Default>(data: &[T], dims: &[usize]) -> Vec<T> {
    fill_rows(dims, |index| {
        let mut flat = 0;
        for (axis, &size) in dims.iter().enumerate() {
            flat = flat * size + (index[axis] + size / 2) % size;
        }
        data[flat]
    })
}

/// Real Simoncelli band-pass filter in DFT array order, row-major over `shape`.
fn simoncelli_band(shape: &[usize], level: u32, grid: FourierGrid) -> Result<Vec<f64>> {
    if level < 1 {
        return Err(Error::InvalidInput(
            "Simoncelli level must be a positive integer".into(),
        ));
    }
    let coordinates = match grid {
        FourierGrid::Dft => grid_coordinates(shape, grid, 2.0 * PI)?,
        FourierGrid::Centered => grid_coordinates(shape, grid, 1.0)?
            .into_iter()
            .map(|axis| axis.into_iter().map(|v| v * PI).collect())
            .collect(),
    };
    let cutoff = PI * 2f64.powi(1 - level as i32);
    let longest = shape.iter().copied().max().unwrap_or(1) as f64;
    let count: usize = shape.iter().product();
    // Every nonzero sample has norm >= pi / max(shape) on both grids and the lower edge
    // cutoff / 4 > 0 excludes DC, so a finer cutoff selects nothing.
    if cutoff < PI / longest {
        return Ok(vec![0.0; count]);
    }
    let lower = cutoff / 4.0;
    let band: Vec<f64> = fill_rows(shape, |index| {
        let norm = squared_at(&coordinates, index).sqrt();
        if norm >= lower && norm <= cutoff {
            (PI / 2.0 * (2.0 * norm / cutoff).log2()).cos()
        } else {
            0.0
        }
    });
    Ok(if grid == FourierGrid::Centered {
        ifftshift(&band, shape)
    } else {
        band
    })
}

fn to_array(data: Vec<f64>, shape: &[usize]) -> ArrayD<f64> {
    ArrayD::from_shape_vec(IxDyn(shape), data).expect("buffer matches shape")
}

fn spectrum_of(image: &ArrayD<f64>) -> Vec<Complex64> {
    let standard = image.as_standard_layout();
    let values = standard.as_slice().expect("standard layout is contiguous");
    values.par_iter().map(|&v| Complex64::new(v, 0.0)).collect()
}

/// Real parts of a spectrum as an array of `shape`.
fn real_parts(data: &[Complex64], shape: &[usize]) -> ArrayD<f64> {
    to_array(data.par_iter().map(|c| c.re).collect(), shape)
}

/// Radial Simoncelli band-pass response map (IBSI Eq. 4.8, `PRT7`).
///
/// The band-pass profile is evaluated on the full `N1 x ... x ND` Fourier grid with
/// `nu_B = pi` and cut off at `nu_B / 2**(level - 1)`; level one is the highest-frequency
/// band. The filter is a full-size Fourier Hadamard product, so the boundary condition is
/// periodic by construction (manual section 4.6.4). `Dft` is the manual's exact `2*pi/N`
/// grid; `Centered` reproduces the endpoint-inclusive convention of the published IBSI 2
/// consensus maps. With `planar`, each plane normal to axis 0 of a 3-D image is filtered
/// by its own 2-D FFT.
pub fn simoncelli_response(
    image: &ArrayD<f64>,
    level: u32,
    grid: FourierGrid,
    planar: bool,
) -> Result<ArrayD<f64>> {
    let ndim = image.ndim();
    if !(2..=3).contains(&ndim) {
        return Err(Error::InvalidInput(
            "Simoncelli filtering requires a 2-D or 3-D image".into(),
        ));
    }
    if planar && ndim != 3 {
        return Err(Error::InvalidInput(
            "planar Simoncelli filtering requires a 3-D image".into(),
        ));
    }
    let axes: Vec<usize> = if planar {
        vec![1, 2]
    } else {
        (0..ndim).collect()
    };
    let band_shape: Vec<usize> = axes.iter().map(|&a| image.shape()[a]).collect();
    let band = simoncelli_band(&band_shape, level, grid)?;
    let mut data = spectrum_of(image);
    let dims = image.shape().to_vec();
    fftn(&mut data, &dims, &axes, false);
    let plane = band.len();
    data.par_iter_mut().enumerate().for_each(|(index, value)| {
        *value *= band[index % plane];
    });
    fftn(&mut data, &dims, &axes, true);
    Ok(real_parts(&data, &dims))
}

/// `ln(n!)`.
fn log_factorial(n: u32) -> f64 {
    (2..=n).map(|k| (k as f64).ln()).sum()
}

/// One complex Riesz multiplier in DFT array order (IBSI Eq. 4.13).
///
/// For `L = sum(order)` the multiplier is
/// `(-j)**L * sqrt(L! / prod(l_i!)) * prod(nu_i**l_i) / ||nu||**L`, evaluated from the
/// normalised directions `d_i = nu_i / ||nu||`. `order` is in native array-axis order; the
/// public response functions accept IBSI `(k1, ..., kD)` order and reverse it. The
/// zero-frequency sample is zero for every `L >= 1` (on the centred grid, when every axis
/// has odd length its centre sample).
pub fn riesz_multiplier(
    shape: &[usize],
    order: &[u32],
    grid: FourierGrid,
) -> Result<ArrayD<Complex64>> {
    if !(2..=3).contains(&shape.len()) || order.len() != shape.len() {
        return Err(Error::InvalidInput(
            "a Riesz order needs one integer per 2-D or 3-D image axis".into(),
        ));
    }
    let length: u32 = order.iter().sum();
    if length < 1 {
        return Err(Error::InvalidInput(
            "Riesz orders must be nonnegative integers with a positive sum".into(),
        ));
    }
    let coordinates = grid_coordinates(shape, grid, 2.0)?;
    let log_coefficient =
        0.5 * (log_factorial(length) - order.iter().map(|&l| log_factorial(l)).sum::<f64>());
    let phase = match length % 4 {
        0 => Complex64::new(1.0, 0.0),
        1 => Complex64::new(0.0, -1.0),
        2 => Complex64::new(-1.0, 0.0),
        _ => Complex64::new(0.0, 1.0),
    };
    let values = fill_rows(shape, |index| {
        let sq = squared_at(&coordinates, index);
        if sq > 0.0 {
            let norm = sq.sqrt();
            let mut log_magnitude = log_coefficient;
            let mut sign = 1.0;
            for (axis, &power) in order.iter().enumerate() {
                if power == 0 {
                    continue;
                }
                let d = coordinates[axis][index[axis]] / norm;
                if d == 0.0 {
                    return Complex64::new(0.0, 0.0);
                }
                log_magnitude += power as f64 * d.abs().ln();
                if power % 2 == 1 && d < 0.0 {
                    sign = -sign;
                }
            }
            phase * (sign * log_magnitude.exp())
        } else {
            Complex64::new(0.0, 0.0)
        }
    });
    let values = if grid == FourierGrid::Centered {
        ifftshift(&values, shape)
    } else {
        values
    };
    Ok(ArrayD::from_shape_vec(IxDyn(shape), values).expect("buffer matches shape"))
}

/// Reverse an IBSI `(k1, ..., kD)` order into native array-axis order.
fn native_order(order: &[u32]) -> Vec<u32> {
    order.iter().rev().copied().collect()
}

/// Sampled IBSI 2 LoG kernel (manual Eq. 4.9) of `shape.len()` axes, dense.
///
/// `sigma` is in mm and converted once to voxel units with the common spacing. Anisotropic
/// spacing is rejected. The odd support per axis is `1 + 2*floor(cutoff*sigma_vox + 0.5)`.
pub fn log_kernel(
    shape: &[usize],
    spacing: &[f64],
    sigma: f64,
    cutoff: f64,
) -> Result<ArrayD<f64>> {
    let (size, sigma_voxel) = log_kernel_geometry(shape, spacing, sigma, cutoff)?;
    let ndim = shape.len();
    let center = (size as f64 - 1.0) / 2.0;
    let scale = -(1.0 / (sigma_voxel * sigma_voxel))
        * (1.0 / (2.0 * PI * sigma_voxel * sigma_voxel).sqrt()).powi(ndim as i32);
    let count = size.pow(ndim as u32);
    let mut values = Vec::with_capacity(count);
    let mut index = vec![0usize; ndim];
    for _ in 0..count {
        let squared: f64 = index
            .iter()
            .map(|&i| ((i as f64 - center) / sigma_voxel).powi(2))
            .sum();
        values.push(scale * (ndim as f64 - squared) * (-squared / 2.0).exp());
        for axis in (0..ndim).rev() {
            index[axis] += 1;
            if index[axis] < size {
                break;
            }
            index[axis] = 0;
        }
    }
    Ok(to_array(values, &vec![size; ndim]))
}

fn log_kernel_geometry(
    shape: &[usize],
    spacing: &[f64],
    sigma: f64,
    cutoff: f64,
) -> Result<(usize, f64)> {
    if !(2..=3).contains(&shape.len()) || spacing.len() != shape.len() {
        return Err(Error::InvalidInput(
            "sampled LoG requires one spacing value per 2-D or 3-D filter axis".into(),
        ));
    }
    require_isotropic_spacing(spacing)?;
    if !sigma.is_finite() || sigma <= 0.0 {
        return Err(Error::InvalidInput(
            "sampled LoG sigma must be finite and positive".into(),
        ));
    }
    if !cutoff.is_finite() || cutoff <= 0.0 {
        return Err(Error::InvalidInput(
            "sampled LoG cutoff must be finite and positive".into(),
        ));
    }
    let sigma_voxel = sigma / spacing[0];
    let size = 1 + 2 * (cutoff * sigma_voxel + 0.5).floor() as usize;
    Ok((size, sigma_voxel))
}

/// IBSI 2 sampled Laplacian-of-Gaussian response (manual Eq. 4.9, `L6PA`).
///
/// The kernel has odd support `1 + 2*floor(cutoff*sigma_vox + 0.5)` per filtered axis and
/// is applied by spatial convolution with `padding`. With `planar`, a 2-D kernel (`D = 2`
/// normalisation) filters every plane normal to axis 0. The kernel
/// `c (D - r^2) exp(-r^2/2)` equals `c sum_d (1 - x_d^2) g(x_d) prod_{e != d} g(x_e)`, so
/// it is applied as `D` separable passes (equal to the dense kernel up to rounding).
pub fn log_response(
    image: &ArrayD<f64>,
    spacing: &[f64],
    sigma: f64,
    cutoff: f64,
    planar: bool,
    padding: FilterPadding,
    constant: f64,
) -> Result<ArrayD<f64>> {
    let ndim = image.ndim();
    if !(2..=3).contains(&ndim) {
        return Err(Error::InvalidInput(
            "LoG filtering requires a 2-D or 3-D image".into(),
        ));
    }
    if spacing.len() != ndim {
        return Err(Error::InvalidInput(
            "LoG spacing must provide one value per image axis".into(),
        ));
    }
    if planar && ndim != 3 {
        return Err(Error::InvalidInput(
            "planar LoG filtering requires a 3-D image".into(),
        ));
    }
    let first = if planar { 1 } else { 0 };
    let filtered = ndim - first;
    let (size, sigma_voxel) =
        log_kernel_geometry(&image.shape()[first..], &spacing[first..], sigma, cutoff)?;
    let center = (size as f64 - 1.0) / 2.0;
    let scale = -(1.0 / (sigma_voxel * sigma_voxel))
        * (1.0 / (2.0 * PI * sigma_voxel * sigma_voxel).sqrt()).powi(filtered as i32);
    let gauss: Vec<f64> = (0..size)
        .map(|i| (-(((i as f64 - center) / sigma_voxel).powi(2)) / 2.0).exp())
        .collect();
    let curved: Vec<f64> = (0..size)
        .map(|i| {
            let x = (i as f64 - center) / sigma_voxel;
            (1.0 - x * x) * (-(x * x) / 2.0).exp()
        })
        .collect();
    let mut total: Option<ArrayD<f64>> = None;
    for term in 0..filtered {
        let mut native = vec![vec![1.0]; ndim];
        for axis in 0..filtered {
            native[first + axis] = if axis == term {
                curved.clone()
            } else {
                gauss.clone()
            };
        }
        let part = convolve_separable_native(image, &native, padding, constant)?;
        total = Some(match total {
            None => part,
            Some(mut sum) => {
                sum += &part;
                sum
            }
        });
    }
    Ok(total.expect("at least two filtered axes") * scale)
}

/// Riesz transforms of the IBSI 2 LoG kernel (manual Eq. 4.13).
///
/// The compact spatial kernel is zero-padded to the padded image size and transformed with
/// the image: pad each axis by `kernel_size // 2` with `padding`, multiply the spectra,
/// inverse transform, and crop the valid region with offset `2 * (kernel_size // 2)`.
/// `orders` are in IBSI `(k1, ..., kD)` order (reversed once before the multiplier is
/// built). The real part is taken by definition, as in MIRP and the reference construction.
#[allow(clippy::too_many_arguments)] // mirrors the Python signature
pub fn riesz_log_responses(
    image: &ArrayD<f64>,
    spacing: &[f64],
    sigma: f64,
    cutoff: f64,
    orders: &[Vec<u32>],
    grid: FourierGrid,
    padding: FilterPadding,
    constant: f64,
) -> Result<Vec<ArrayD<f64>>> {
    let ndim = image.ndim();
    if !(2..=3).contains(&ndim) {
        return Err(Error::InvalidInput(
            "Riesz LoG filtering requires a 2-D or 3-D image".into(),
        ));
    }
    if orders.is_empty() {
        return Err(Error::InvalidInput(
            "Riesz filtering requires at least one order tuple".into(),
        ));
    }
    let kernel = log_kernel(image.shape(), spacing, sigma, cutoff)?;
    let half: Vec<usize> = kernel.shape().iter().map(|&s| s / 2).collect();
    let widths: Vec<(usize, usize)> = half.iter().map(|&h| (h, h)).collect();
    let padded = pad_image(image, &widths, padding, constant)?;
    let dims = padded.shape().to_vec();
    let axes: Vec<usize> = (0..ndim).collect();
    let mut spectrum = spectrum_of(&padded);
    fftn(&mut spectrum, &dims, &axes, false);
    // Kernel zero-padded at the end of every axis.
    let mut weights = vec![Complex64::new(0.0, 0.0); spectrum.len()];
    for (index, &value) in kernel.indexed_iter() {
        let mut flat = 0;
        for axis in 0..ndim {
            flat = flat * dims[axis] + index.slice()[axis];
        }
        weights[flat] = Complex64::new(value, 0.0);
    }
    fftn(&mut weights, &dims, &axes, false);
    let product: Vec<Complex64> = spectrum
        .par_iter()
        .zip(&weights)
        .map(|(a, b)| a * b)
        .collect();
    let mut results = Vec::with_capacity(orders.len());
    for order in orders {
        let multiplier = riesz_multiplier(&dims, &native_order(order), grid)?;
        let multiplier = multiplier.as_standard_layout();
        let mut data: Vec<Complex64> = product
            .par_iter()
            .zip(
                multiplier
                    .as_slice()
                    .expect("standard layout is contiguous"),
            )
            .map(|(a, b)| a * b)
            .collect();
        fftn(&mut data, &dims, &axes, true);
        let full = real_parts(&data, &dims);
        let mut view = full.view();
        for axis in 0..ndim {
            view.slice_axis_inplace(
                ndarray::Axis(axis),
                ndarray::Slice::from(2 * half[axis]..2 * half[axis] + image.shape()[axis]),
            );
        }
        results.push(view.as_standard_layout().into_owned());
    }
    Ok(results)
}

/// Riesz transforms of a radial Simoncelli B-map (manual section 4.7).
///
/// The base filter is the full-image band-pass of [`simoncelli_response`], so the boundary
/// condition is periodic by construction. `orders` are in IBSI `(k1, ..., kD)` order.
pub fn riesz_simoncelli_responses(
    image: &ArrayD<f64>,
    level: u32,
    orders: &[Vec<u32>],
    grid: FourierGrid,
) -> Result<Vec<ArrayD<f64>>> {
    let ndim = image.ndim();
    if !(2..=3).contains(&ndim) {
        return Err(Error::InvalidInput(
            "Riesz Simoncelli filtering requires a 2-D or 3-D image".into(),
        ));
    }
    if orders.is_empty() {
        return Err(Error::InvalidInput(
            "Riesz filtering requires at least one order tuple".into(),
        ));
    }
    let dims = image.shape().to_vec();
    let band = simoncelli_band(&dims, level, grid)?;
    let axes: Vec<usize> = (0..ndim).collect();
    let mut spectrum = spectrum_of(image);
    fftn(&mut spectrum, &dims, &axes, false);
    let mut results = Vec::with_capacity(orders.len());
    for order in orders {
        let multiplier = riesz_multiplier(&dims, &native_order(order), grid)?;
        let multiplier = multiplier.as_standard_layout();
        let mut data: Vec<Complex64> = spectrum
            .par_iter()
            .zip(&band)
            .zip(
                multiplier
                    .as_slice()
                    .expect("standard layout is contiguous"),
            )
            .map(|((s, b), m)| s * *b * m)
            .collect();
        fftn(&mut data, &dims, &axes, true);
        results.push(real_parts(&data, &dims));
    }
    Ok(results)
}

/// Reject spacing the sampled LoG cannot use: it has one voxel-unit sigma.
pub fn require_isotropic_spacing(spacing: &[f64]) -> Result<()> {
    if spacing.iter().any(|v| !v.is_finite() || *v <= 0.0) {
        return Err(Error::InvalidInput(
            "sampled LoG spacing must be finite and positive".into(),
        ));
    }
    // math.isclose(value, spacing[0], rel_tol=1e-6): abs_tol 0, scaled by the larger magnitude.
    if spacing[1..]
        .iter()
        .any(|&v| (v - spacing[0]).abs() > 1e-6 * v.abs().max(spacing[0].abs()))
    {
        return Err(Error::InvalidInput(
            "sampled LoG requires isotropic voxel spacing over the filtered axes".into(),
        ));
    }
    Ok(())
}
