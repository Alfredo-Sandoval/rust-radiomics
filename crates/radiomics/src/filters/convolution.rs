//! Spatial mean, Laws, Gabor and separable-wavelet filters.
//!
//! Mirrors `mlxradiomics/filters/convolution.py` (752 lines at mlx-radiomics b58783c).
//! Status: done. Every function in the Python module is ported; the float32
//! overflow-guard machinery of the mean filter (`FLOAT32_SUM_LIMIT`, power-of-two
//! prescaling, overflow-safe orientation means) is not needed in float64 and is not ported.
//!
//! Execution: CPU numerical work in float64 (no Metal, no GPU). Window sums and the
//! separable passes run on rayon worker threads (see `kernels.rs`); orientations of Laws
//! and separable-wavelet filters are chains of those passes that share their common
//! prefixes and are pooled as they arrive; Gabor kernels, which span the image plane, are
//! applied by FFT convolution on rayon worker threads (`fft_conv.rs`). Input contract: 2-D
//! or 3-D [`ArrayD`] in NumPy (z, y, x) order. Laws and separable kernels apply as a
//! pad-once separable valid convolution, which equals the dense outer-product kernel of
//! the Python code up to float64 rounding.

use crate::config::filtering::{
    FilterPadding, GaborResponse, OrientationPooling, laws_taps, validate_laws_kernel,
};
use crate::error::{Error, Result};
use crate::filters::fft_conv::{ComplexKernel, valid_complex_responses};
use crate::filters::kernels::{Vol, pad_vol, valid_conv_axis, window_sum};
use ndarray::{ArrayD, IxDyn};
use rayon::prelude::*;

/// Right-angle rotations of a separable 3-D filter (manual Appendix A.2).
///
/// Each entry lists the source (0, 1, 2 for g1, g2, g3) and flip flag of the k1, k2, k3 axes.
const ROTATIONS_3D: [[(usize, bool); 3]; 24] = [
    [(0, false), (1, false), (2, false)],
    [(2, true), (1, false), (0, false)],
    [(0, true), (1, false), (2, true)],
    [(2, false), (1, false), (0, true)],
    [(1, false), (2, false), (0, false)],
    [(1, false), (2, true), (0, true)],
    [(1, false), (0, true), (2, false)],
    [(0, true), (1, true), (2, false)],
    [(1, true), (0, false), (2, false)],
    [(2, true), (0, true), (1, false)],
    [(2, true), (1, true), (0, true)],
    [(2, true), (0, false), (1, true)],
    [(1, true), (0, true), (2, true)],
    [(0, false), (1, true), (2, true)],
    [(1, false), (0, false), (2, true)],
    [(2, false), (0, true), (1, true)],
    [(2, false), (1, true), (0, false)],
    [(2, false), (0, false), (1, false)],
    [(0, true), (2, false), (1, false)],
    [(1, true), (2, false), (0, true)],
    [(0, false), (2, false), (1, true)],
    [(0, true), (2, true), (1, true)],
    [(1, true), (2, true), (0, false)],
    [(0, false), (2, true), (1, false)],
];

fn reversed(values: &[f64]) -> Vec<f64> {
    values.iter().rev().copied().collect()
}

/// Validated Laws 1-D kernels of `codes` in IBSI axis order (k1, k2, k3).
fn laws_vectors(codes: &str) -> Result<Vec<Vec<f64>>> {
    validate_laws_kernel(codes)?;
    (0..codes.len())
        .step_by(2)
        .map(|index| {
            laws_taps(&codes[index..index + 2])
                .ok_or_else(|| Error::InvalidInput("unknown Laws kernel".into()))
        })
        .collect()
}

/// Every right-angle rotation of the Laws kernel `codes`.
fn laws_orientations(codes: &str) -> Result<Vec<Vec<Vec<f64>>>> {
    Ok(right_angle_orientations(&laws_vectors(codes)?))
}

/// The 4 (2-D) or 24 (3-D) right-angle rotations of a separable kernel.
///
/// `vectors` are 1-D kernels in IBSI axis order; each rotation permutes and reverses
/// them (manual Appendix A.2).
fn right_angle_orientations(vectors: &[Vec<f64>]) -> Vec<Vec<Vec<f64>>> {
    if vectors.len() == 2 {
        let (g1, g2) = (&vectors[0], &vectors[1]);
        return vec![
            vec![g1.clone(), g2.clone()],
            vec![reversed(g2), g1.clone()],
            vec![reversed(g1), reversed(g2)],
            vec![g2.clone(), reversed(g1)],
        ];
    }
    ROTATIONS_3D
        .iter()
        .map(|rotation| {
            rotation
                .iter()
                .map(|&(index, flip)| {
                    if flip {
                        reversed(&vectors[index])
                    } else {
                        vectors[index].clone()
                    }
                })
                .collect()
        })
        .collect()
}

/// Extend `array` by `widths` with an IBSI boundary condition (manual section 2.2).
///
/// `Mirror` repeats the edge sample (NumPy `symmetric`) and `Periodic` wraps by modular
/// indexing, so widths larger than the axis are supported.
pub fn pad_image(
    array: &ArrayD<f64>,
    widths: &[(usize, usize)],
    padding: FilterPadding,
    constant: f64,
) -> Result<ArrayD<f64>> {
    let ndim = array.ndim();
    if widths.len() != ndim {
        return Err(Error::InvalidInput(
            "padding widths need one pair per axis".into(),
        ));
    }
    let vol = Vol::from_array(array)?;
    let mut full = [(0, 0); 3];
    for (axis, &pair) in widths.iter().enumerate() {
        full[axis + 3 - ndim] = pair;
    }
    Ok(pad_vol(&vol, full, padding, constant).into_array(ndim))
}

fn native_widths(ndim: usize, sizes: &[usize]) -> [(usize, usize); 3] {
    let mut widths = [(0, 0); 3];
    for (axis, &size) in sizes.iter().enumerate() {
        widths[axis + 3 - ndim] = (size / 2, size / 2);
    }
    widths
}

/// One step of a separable filter chain: an optional axis extension, then a valid 1-D
/// convolution along `axis` (a 3-axis [`Vol`] axis).
#[derive(Clone, Debug)]
struct ChainStep {
    axis: usize,
    taps: Vec<f64>,
    /// Extend `axis` by `taps.len() / 2` on both sides first, with this constant for the
    /// `Constant` boundary; `None` convolves the buffer as it is.
    pad: Option<f64>,
}

impl ChainStep {
    fn same_as(&self, other: &Self) -> bool {
        self.axis == other.axis
            && self.pad.map(f64::to_bits) == other.pad.map(f64::to_bits)
            && self.taps.len() == other.taps.len()
            && self
                .taps
                .iter()
                .zip(&other.taps)
                .all(|(a, b)| a.to_bits() == b.to_bits())
    }

    fn order(&self, other: &Self) -> std::cmp::Ordering {
        let bits = |step: &Self| step.taps.iter().map(|t| t.to_bits()).collect::<Vec<_>>();
        self.axis
            .cmp(&other.axis)
            .then(self.pad.map(f64::to_bits).cmp(&other.pad.map(f64::to_bits)))
            .then_with(|| bits(self).cmp(&bits(other)))
    }
}

/// Run separable filter chains over one start volume, sharing common prefixes.
///
/// Chains are visited in lexicographic step order, so chains that begin with the same
/// steps reuse the intermediate volume of the previous chain; only a stack of volumes the
/// length of the longest chain is alive. `sink(index, response)` receives the response of
/// `chains[index]`, in the visiting order (not the input order). Each step is the same
/// pad-and-convolve of the unshared computation, so a response does not depend on the
/// sharing.
fn run_chains(
    start: &Vol,
    chains: &[Vec<ChainStep>],
    padding: FilterPadding,
    mut sink: impl FnMut(usize, &Vol),
) -> Result<()> {
    let mut order: Vec<usize> = (0..chains.len()).collect();
    order.sort_by(|&a, &b| {
        let (left, right) = (&chains[a], &chains[b]);
        for (x, y) in left.iter().zip(right) {
            let ordering = x.order(y);
            if ordering.is_ne() {
                return ordering;
            }
        }
        left.len().cmp(&right.len()).then(a.cmp(&b))
    });
    let mut stack: Vec<Vol> = Vec::new();
    let mut previous: Option<usize> = None;
    for &index in &order {
        let chain = &chains[index];
        let shared = previous.map_or(0, |p| {
            chains[p]
                .iter()
                .zip(chain)
                .take_while(|(a, b)| a.same_as(b))
                .count()
        });
        stack.truncate(shared);
        for step in &chain[shared..] {
            let input = stack.last().unwrap_or(start);
            let next = match step.pad {
                Some(constant) => {
                    let mut widths = [(0, 0); 3];
                    widths[step.axis] = (step.taps.len() / 2, step.taps.len() / 2);
                    valid_conv_axis(
                        &pad_vol(input, widths, padding, constant),
                        step.axis,
                        &step.taps,
                    )?
                }
                None => valid_conv_axis(input, step.axis, &step.taps)?,
            };
            stack.push(next);
        }
        sink(index, stack.last().unwrap_or(start));
        previous = Some(index);
    }
    Ok(())
}

/// Orientation pooling folded over response volumes as they arrive.
struct Pooled {
    pooling: OrientationPooling,
    total: Option<Vol>,
    count: usize,
}

impl Pooled {
    fn new(pooling: OrientationPooling) -> Self {
        Self {
            pooling,
            total: None,
            count: 0,
        }
    }

    fn add(&mut self, response: &Vol) {
        self.count += 1;
        let Some(total) = self.total.as_mut() else {
            self.total = Some(response.clone());
            return;
        };
        let fold: fn(f64, f64) -> f64 = match self.pooling {
            OrientationPooling::None => return,
            OrientationPooling::Average => |a, b| a + b,
            OrientationPooling::Max => f64::max,
            OrientationPooling::Min => f64::min,
        };
        total
            .data
            .par_iter_mut()
            .zip(&response.data)
            .for_each(|(a, &b)| *a = fold(*a, b));
    }

    fn finish(self, ndim: usize) -> ArrayD<f64> {
        let mut total = self.total.expect("pooling needs at least one response");
        if self.pooling == OrientationPooling::Average {
            let count = self.count as f64;
            total.data.par_iter_mut().for_each(|a| *a /= count);
        }
        total.into_array(ndim)
    }
}

/// Pad once, then convolve one odd 1-D kernel per native axis (`native[axis]`).
///
/// Equals the dense outer-product kernel on the once-padded image.
pub(crate) fn convolve_separable_native(
    image: &ArrayD<f64>,
    native: &[Vec<f64>],
    padding: FilterPadding,
    constant: f64,
) -> Result<ArrayD<f64>> {
    let ndim = image.ndim();
    let sizes: Vec<usize> = native.iter().map(Vec::len).collect();
    let vol = Vol::from_array(image)?;
    let mut result = pad_vol(&vol, native_widths(ndim, &sizes), padding, constant);
    for (axis, taps) in native.iter().enumerate() {
        result = valid_conv_axis(&result, axis + 3 - ndim, taps)?;
    }
    Ok(result.into_array(ndim))
}

/// Pool orientation response maps by `pooling`; `None` returns the first map.
fn pool(maps: &[ArrayD<f64>], pooling: OrientationPooling) -> Result<ArrayD<f64>> {
    let Some(first) = maps.first() else {
        return Err(Error::InvalidInput(
            "orientation pooling requires at least one response map".into(),
        ));
    };
    Ok(match pooling {
        OrientationPooling::None => first.clone(),
        OrientationPooling::Average => mean_maps(maps),
        OrientationPooling::Max => maps[1..].iter().fold(first.clone(), |mut acc, map| {
            acc.zip_mut_with(map, |a, &b| *a = a.max(b));
            acc
        }),
        OrientationPooling::Min => maps[1..].iter().fold(first.clone(), |mut acc, map| {
            acc.zip_mut_with(map, |a, &b| *a = a.min(b));
            acc
        }),
    })
}

/// Elementwise mean of response maps.
fn mean_maps(maps: &[ArrayD<f64>]) -> ArrayD<f64> {
    let mut total = maps[0].clone();
    for map in &maps[1..] {
        total += map;
    }
    total / maps.len() as f64
}

/// Dilate wavelet taps for decomposition `level` and make the support odd.
///
/// Level `j` places `2**(j-1) - 1` zeros after every tap, including the last, and one
/// more zero is appended when the length is even, so the centre sits at `size // 2`.
fn atrous_taps(taps: &[f64], level: u32) -> Vec<f64> {
    let step = 1usize << (level - 1);
    let mut dilated = vec![0.0; taps.len() * step];
    for (index, &tap) in taps.iter().enumerate() {
        dilated[index * step] = tap;
    }
    if dilated.len().is_multiple_of(2) {
        dilated.push(0.0);
    }
    dilated
}

/// Undecimated separable wavelet bands (IBSI `25BO`, `CVCQ`).
///
/// `low` and `high` are the decomposition taps as PyWavelets lists them. Level `j`
/// filters the level `j - 1` all-low response with the a trous taps, applied along each
/// axis with `padding`. Each band is a string of `L`/`H` letters in IBSI axis order, so
/// `"LHL"` is low on k1, high on k2 and low on k3. With pooling, every right-angle
/// rotation of the separable kernel is applied through the whole level chain and the
/// rotated bands are pooled (manual section 4.5.2). With `planar` set, two-letter bands
/// filter every plane normal to native axis 0 of a 3-D image.
///
/// Returns one response map per band, in `bands` order.
#[allow(clippy::too_many_arguments)]
pub fn separable_wavelet_responses(
    image: &ArrayD<f64>,
    low: &[f64],
    high: &[f64],
    bands: &[String],
    level: u32,
    padding: FilterPadding,
    constant: f64,
    pooling: OrientationPooling,
    planar: bool,
) -> Result<Vec<ArrayD<f64>>> {
    if !(2..=3).contains(&image.ndim()) {
        return Err(Error::InvalidInput(
            "separable wavelets require a 2-D or 3-D image".into(),
        ));
    }
    if planar && image.ndim() != 3 {
        return Err(Error::InvalidInput(
            "planar separable wavelets require a 3-D image".into(),
        ));
    }
    if level < 1 {
        return Err(Error::InvalidInput(
            "separable wavelet level must be a positive integer".into(),
        ));
    }
    let axes = if planar { 2 } else { image.ndim() };
    if bands.is_empty()
        || bands
            .iter()
            .any(|band| band.chars().count() != axes || band.chars().any(|c| c != 'L' && c != 'H'))
    {
        return Err(Error::InvalidInput(
            "separable wavelet bands need one L or H per filtered axis".into(),
        ));
    }
    let taps = |letter: char| if letter == 'L' { low } else { high };
    let rotated = |code: &str, scale: u32, rotation: usize| -> Vec<Vec<f64>> {
        let vectors: Vec<Vec<f64>> = code
            .chars()
            .map(|letter| atrous_taps(taps(letter), scale))
            .collect();
        right_angle_orientations(&vectors).swap_remove(rotation)
    };
    let rotations = if pooling == OrientationPooling::None {
        1
    } else if axes == 2 {
        4
    } else {
        24
    };
    let all_low = "L".repeat(axes);
    let ndim = image.ndim();
    // One chain per (rotation, band): the level chain of the all-low approximation, then the
    // band's own taps. Chains of one rotation share the approximation steps, and rotations
    // that apply the same kernel to the same axis first share their first steps.
    let steps = |vectors: Vec<Vec<f64>>, mut extension: f64| -> (Vec<ChainStep>, f64) {
        let mut chain = Vec::with_capacity(vectors.len());
        for (index, taps) in vectors.into_iter().enumerate() {
            let sum: f64 = taps.iter().sum();
            chain.push(ChainStep {
                axis: 3 - 1 - index,
                taps,
                pad: Some(extension),
            });
            extension *= sum;
        }
        (chain, extension)
    };
    let mut chains: Vec<Vec<ChainStep>> = Vec::with_capacity(rotations * bands.len());
    for rotation in 0..rotations {
        let (mut approximation, mut extension) = (Vec::new(), constant);
        for scale in 1..level {
            let (chain, next) = steps(rotated(&all_low, scale, rotation), extension);
            approximation.extend(chain);
            extension = next;
        }
        for band in bands {
            let (chain, _) = steps(rotated(band, level, rotation), extension);
            let mut full = approximation.clone();
            full.extend(chain);
            chains.push(full);
        }
    }
    let start = Vol::from_array(image)?;
    let mut pools: Vec<Pooled> = bands.iter().map(|_| Pooled::new(pooling)).collect();
    run_chains(&start, &chains, padding, |index, response| {
        pools[index % bands.len()].add(response);
    })?;
    Ok(pools.into_iter().map(|pool| pool.finish(ndim)).collect())
}

/// Mean filter over an isotropic `support`-voxel cube (IBSI `S60F`).
///
/// The kernel is `1 / M**D` over the full cube (`D = 2` when `planar`, which filters each
/// axis-0 plane of a 3-D image and neither averages nor pads axis 0). The cube is
/// separable: the padded image is summed with unit weights, one compensated axis pass at
/// a time, and divided once by `M**D`.
pub fn mean_response(
    image: &ArrayD<f64>,
    support: usize,
    padding: FilterPadding,
    constant: f64,
    planar: bool,
) -> Result<ArrayD<f64>> {
    let ndim = image.ndim();
    if !(2..=3).contains(&ndim) {
        return Err(Error::InvalidInput(
            "filters require a 2-D or 3-D image".into(),
        ));
    }
    if planar && ndim != 3 {
        return Err(Error::InvalidInput(
            "planar mean filtering requires a 3-D image".into(),
        ));
    }
    if support < 1 || support.is_multiple_of(2) {
        return Err(Error::InvalidInput(
            "mean filter support must be a positive odd integer".into(),
        ));
    }
    let half = support / 2;
    let filtered: Vec<usize> = if planar {
        vec![1, 2]
    } else {
        (3 - ndim..3).collect()
    };
    let mut widths = [(0, 0); 3];
    for &axis in &filtered {
        widths[axis] = (half, half);
    }
    let mut result = pad_vol(&Vol::from_array(image)?, widths, padding, constant);
    for &axis in &filtered {
        result = window_sum(&result, axis, support)?;
    }
    let full = (support as f64).powi(filtered.len() as i32);
    result.data.par_iter_mut().for_each(|value| *value /= full);
    Ok(result.into_array(ndim))
}

/// Filter an image with a separable Laws kernel (IBSI `JTXT`).
///
/// A two-name kernel (for example `"E5L5"`) is applied to each `(k2, k1)` plane when the
/// image is 3-D; a three-name kernel is a 3-D kernel. Orientation pooling uses the
/// right-angle rotations of manual Appendix A (four in 2-D, twenty-four in 3-D).
pub fn laws_response(
    image: &ArrayD<f64>,
    codes: &str,
    padding: FilterPadding,
    constant: f64,
    pooling: OrientationPooling,
) -> Result<ArrayD<f64>> {
    let vectors = laws_vectors(codes)?;
    let ndim = image.ndim();
    if !(2..=3).contains(&ndim) {
        return Err(Error::InvalidInput(
            "filters require a 2-D or 3-D image".into(),
        ));
    }
    if vectors.len() == 3 && ndim != 3 {
        return Err(Error::InvalidInput(
            "a three-axis Laws kernel requires a 3-D image".into(),
        ));
    }
    let orientations = if pooling == OrientationPooling::None {
        vec![vectors.clone()]
    } else {
        laws_orientations(codes)?
    };
    // IBSI axis k_(i+1) is native axis ndim - 1 - i; a planar kernel leaves the rest.
    let natives: Vec<Vec<Vec<f64>>> = orientations
        .iter()
        .map(|orientation| {
            let mut native = vec![vec![1.0]; ndim];
            for (index, vector) in orientation.iter().enumerate() {
                native[ndim - 1 - index] = vector.clone();
            }
            native
        })
        .collect();
    let vol = Vol::from_array(image)?;
    let mut pooled = Pooled::new(pooling);
    // Orientations whose kernels have the same lengths share one padded image.
    let mut groups: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
    for (index, native) in natives.iter().enumerate() {
        let sizes: Vec<usize> = native.iter().map(Vec::len).collect();
        match groups.iter_mut().find(|(known, _)| *known == sizes) {
            Some((_, members)) => members.push(index),
            None => groups.push((sizes, vec![index])),
        }
    }
    for (sizes, members) in &groups {
        let padded = pad_vol(&vol, native_widths(ndim, sizes), padding, constant);
        let chains: Vec<Vec<ChainStep>> = members
            .iter()
            .map(|&index| {
                natives[index]
                    .iter()
                    .enumerate()
                    .filter(|(_, taps)| !(taps.len() == 1 && taps[0] == 1.0))
                    .map(|(axis, taps)| ChainStep {
                        axis: axis + 3 - ndim,
                        taps: taps.clone(),
                        pad: None,
                    })
                    .collect()
            })
            .collect();
        run_chains(&padded, &chains, padding, |_, response| {
            pooled.add(response);
        })?;
    }
    Ok(pooled.finish(ndim))
}

/// Elementwise absolute value on rayon worker threads.
fn absolute(array: &ArrayD<f64>) -> ArrayD<f64> {
    let standard = array.as_standard_layout();
    let flat = standard.as_slice().expect("standard layout is contiguous");
    let values: Vec<f64> = flat.par_iter().map(|v| v.abs()).collect();
    ArrayD::from_shape_vec(IxDyn(array.shape()), values).expect("same shape")
}

/// Laws texture energy image (IBSI `PQSD`).
///
/// The absolute response is averaged over a `(2 * distance + 1)` cube, or over each
/// `(k2, k1)` plane when the response came from a two-axis kernel. `Constant` padding
/// extends the absolute response by `|constant|` times the product of the kernel's tap
/// sums.
pub fn laws_energy(
    response: &ArrayD<f64>,
    codes: &str,
    distance: i64,
    padding: FilterPadding,
    constant: f64,
) -> Result<ArrayD<f64>> {
    if distance < 0 {
        return Err(Error::InvalidInput(
            "Laws energy distance must be nonnegative".into(),
        ));
    }
    let vectors = laws_vectors(codes)?;
    let extension = (constant
        * vectors
            .iter()
            .map(|vector| vector.iter().sum::<f64>())
            .product::<f64>())
    .abs();
    mean_response(
        &absolute(response),
        (2 * distance + 1) as usize,
        padding,
        extension,
        vectors.len() == 2 && response.ndim() == 3,
    )
}

/// One complex 2-D Gabor kernel as separate real and imaginary parts.
#[derive(Clone, Debug, PartialEq)]
pub struct GaborKernel {
    /// Real part, odd `(k2, k1)` dimensions.
    pub real: ArrayD<f64>,
    /// Imaginary part, same shape.
    pub imaginary: ArrayD<f64>,
}

/// Sample one complex 2-D Gabor kernel (IBSI Eq. 4.3).
///
/// The kernel spans the plane: each odd support is the plane size, or the plane size plus
/// one when that size is even (manual Eq. 4.5). `spacing` is the physical spacing of the
/// first and last plane axes in mm; `sigma` and `wavelength` are in mm. `theta` is
/// clockwise from the k1 (last) axis. The Gaussian envelope is centred at `M // 2`.
pub fn gabor_kernel(
    plane_shape: (usize, usize),
    spacing: (f64, f64),
    sigma: f64,
    wavelength: f64,
    gamma: f64,
    theta: f64,
) -> GaborKernel {
    let size0 = plane_shape.0 + (plane_shape.0 + 1) % 2;
    let size1 = plane_shape.1 + (plane_shape.1 + 1) % 2;
    let mut real = Vec::with_capacity(size0 * size1);
    let mut imaginary = Vec::with_capacity(size0 * size1);
    let (sin_t, cos_t) = theta.sin_cos();
    for i2 in 0..size0 {
        let grid2 = (i2 as f64 - (size0 / 2) as f64) * spacing.0;
        for i1 in 0..size1 {
            let grid1 = (i1 as f64 - (size1 / 2) as f64) * spacing.1;
            let radial = cos_t * grid1 + sin_t * grid2;
            let orthoradial = sin_t * grid1 - cos_t * grid2;
            let envelope = (-(radial * radial + gamma * gamma * orthoradial * orthoradial)
                / (2.0 * sigma * sigma))
                .exp();
            let phase = 2.0 * std::f64::consts::PI * radial / wavelength;
            real.push(envelope * phase.cos());
            imaginary.push(envelope * phase.sin());
        }
    }
    let shape = [size0, size1];
    GaborKernel {
        real: ArrayD::from_shape_vec(IxDyn(&shape), real).expect("kernel shape"),
        imaginary: ArrayD::from_shape_vec(IxDyn(&shape), imaginary).expect("kernel shape"),
    }
}

/// Convolve a stack of planes with several complex Gabor kernels and keep one component.
///
/// The planes are padded once and convolved by FFT (see `fft_conv.rs`); the result of
/// kernel `i` is the valid convolution of every plane with `kernels[i]`.
fn gabor_convolutions(
    planes: &ArrayD<f64>,
    kernels: &[GaborKernel],
    padding: FilterPadding,
    constant: f64,
    response: GaborResponse,
) -> Result<Vec<ArrayD<f64>>> {
    if planes.ndim() != 3 {
        return Err(Error::InvalidInput(
            "plane convolution needs a 3-D stack and a 2-D kernel".into(),
        ));
    }
    let Some(first) = kernels.first() else {
        return Ok(Vec::new());
    };
    if first.real.ndim() != 2 {
        return Err(Error::InvalidInput(
            "plane convolution needs a 3-D stack and a 2-D kernel".into(),
        ));
    }
    let vol = Vol::from_array(planes)?;
    let (k0, k1) = (first.real.shape()[0], first.real.shape()[1]);
    let mut widths = [(0, 0); 3];
    widths[1] = (k0 / 2, k0 / 2);
    widths[2] = (k1 / 2, k1 / 2);
    let padded = pad_vol(&vol, widths, padding, constant);
    let real: Vec<Vec<f64>> = kernels
        .iter()
        .map(|k| k.real.iter().copied().collect())
        .collect();
    let imaginary: Vec<Vec<f64>> = kernels
        .iter()
        .map(|k| k.imaginary.iter().copied().collect())
        .collect();
    let views: Vec<ComplexKernel<'_>> = real
        .iter()
        .zip(&imaginary)
        .map(|(re, im)| ComplexKernel {
            dims: (k0, k1),
            real: re,
            imaginary: im,
        })
        .collect();
    Ok(valid_complex_responses(&padded, &views, response)
        .into_iter()
        .map(|vol| vol.into_array(3))
        .collect())
}

/// Distinct kernel orientations of `thetas`, and for each requested orientation the index
/// into them and whether its response is the conjugate of that one.
///
/// Angles equal modulo `2 pi` share a kernel; angles that differ by an odd multiple of `pi`
/// share the conjugate kernel. Both are matched within `1e-12` of a turn fraction, which
/// absorbs the rounding of angle grids such as `k * pi / 8`.
fn gabor_orientation_pairs(thetas: &[f64]) -> (Vec<f64>, Vec<(usize, bool)>) {
    let mut unique: Vec<f64> = Vec::new();
    let mut source = Vec::with_capacity(thetas.len());
    for &theta in thetas {
        let related = unique.iter().enumerate().find_map(|(index, &known)| {
            let turns = (theta - known) / std::f64::consts::PI;
            let nearest = turns.round();
            ((turns - nearest).abs() < 1e-12).then_some((index, nearest.rem_euclid(2.0) == 1.0))
        });
        match related {
            Some(pair) => source.push(pair),
            None => {
                unique.push(theta);
                source.push((unique.len() - 1, false));
            }
        }
    }
    (unique, source)
}

/// Move `source` to position `destination`, returning a standard-layout copy.
pub(crate) fn move_axis(array: &ArrayD<f64>, source: usize, destination: usize) -> ArrayD<f64> {
    let mut order: Vec<usize> = (0..array.ndim()).filter(|&a| a != source).collect();
    order.insert(destination, source);
    array
        .view()
        .permuted_axes(IxDyn(&order))
        .as_standard_layout()
        .into_owned()
}

/// Gabor response maps for one scale and a set of orientations.
///
/// A 2-D image is filtered directly. For a 3-D image `slice_axis` selects the array axis
/// normal to the filtered planes; `None` filters all three orthogonal plane families and
/// averages those responses (manual section 4.5). Within a plane, k1 is the last and k2
/// the first remaining array axis. Orientation pooling is applied per plane family before
/// that average. With pooling `None` one response per orientation is returned, otherwise
/// a single pooled response.
#[allow(clippy::too_many_arguments)]
pub fn gabor_responses(
    image: &ArrayD<f64>,
    spacing: &[f64],
    sigma: f64,
    wavelength: f64,
    gamma: f64,
    thetas: &[f64],
    response: GaborResponse,
    slice_axis: Option<usize>,
    padding: FilterPadding,
    constant: f64,
    pooling: OrientationPooling,
) -> Result<Vec<ArrayD<f64>>> {
    if thetas.is_empty() {
        return Err(Error::InvalidInput(
            "Gabor filtering requires at least one orientation".into(),
        ));
    }
    let ndim = image.ndim();
    if !(2..=3).contains(&ndim) {
        return Err(Error::InvalidInput(
            "Gabor filtering requires a 2-D or 3-D image".into(),
        ));
    }
    if spacing.len() != ndim {
        return Err(Error::InvalidInput(
            "Gabor spacing must provide one value per image axis".into(),
        ));
    }
    if ndim == 2 && slice_axis.is_some() {
        return Err(Error::InvalidInput(
            "slice_axis is only valid for a 3-D Gabor image".into(),
        ));
    }
    if slice_axis.is_some_and(|axis| axis >= ndim) {
        return Err(Error::InvalidInput(
            "slice_axis must index a NumPy image axis".into(),
        ));
    }
    let axes: Vec<usize> = match slice_axis {
        Some(axis) => vec![axis],
        None if ndim == 2 => vec![0],
        None => vec![0, 1, 2],
    };
    let mut families: Vec<Vec<ArrayD<f64>>> = Vec::new();
    for &axis in &axes {
        let (planes, plane_shape, plane_spacing) = if ndim == 2 {
            let stack = image
                .view()
                .insert_axis(ndarray::Axis(0))
                .as_standard_layout()
                .into_owned();
            (
                (stack),
                (image.shape()[0], image.shape()[1]),
                (spacing[0], spacing[1]),
            )
        } else {
            let moved = move_axis(image, axis, 0);
            let remaining: Vec<usize> = (0..ndim).filter(|&a| a != axis).collect();
            let shape = (moved.shape()[1], moved.shape()[2]);
            (moved, shape, (spacing[remaining[0]], spacing[remaining[1]]))
        };
        // A real image convolved with the kernel of `theta + pi`, the complex conjugate of
        // the kernel of `theta`, gives the conjugate response: the same modulus and real
        // part, and the negated imaginary part. Only one of each such pair is convolved.
        let (unique, source) = gabor_orientation_pairs(thetas);
        let kernels: Vec<GaborKernel> = unique
            .par_iter()
            .map(|&theta| gabor_kernel(plane_shape, plane_spacing, sigma, wavelength, gamma, theta))
            .collect();
        let computed: Vec<ArrayD<f64>> =
            gabor_convolutions(&planes, &kernels, padding, constant, response)?
                .into_iter()
                .map(|values| {
                    if ndim == 2 {
                        values
                            .index_axis_move(ndarray::Axis(0), 0)
                            .as_standard_layout()
                            .into_owned()
                    } else {
                        move_axis(&values, 0, axis)
                    }
                })
                .collect();
        let components: Vec<ArrayD<f64>> = source
            .iter()
            .map(|&(index, conjugate)| {
                if conjugate && response == GaborResponse::Imaginary {
                    computed[index].mapv(|v| -v)
                } else {
                    computed[index].clone()
                }
            })
            .collect();
        families.push(if pooling == OrientationPooling::None {
            components
        } else {
            vec![pool(&components, pooling)?]
        });
    }
    if pooling == OrientationPooling::None {
        Ok((0..thetas.len())
            .map(|index| {
                let maps: Vec<ArrayD<f64>> = families
                    .iter()
                    .map(|family| family[index].clone())
                    .collect();
                mean_maps(&maps)
            })
            .collect())
    } else {
        let maps: Vec<ArrayD<f64>> = families.iter().map(|family| family[0].clone()).collect();
        Ok(vec![mean_maps(&maps)])
    }
}
