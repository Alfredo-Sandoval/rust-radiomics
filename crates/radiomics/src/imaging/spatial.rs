//! Image-file loading and spatial preprocessing (mask correction, resampling).
//!
//! Mirrors `mlxradiomics/imaging/spatial.py` (512 lines at mlx-radiomics b58783c).
//!
//! The grid and padding equations follow PyRadiomics' `resampleImage` by default. Spacing
//! arguments use NumPy array-axis order, while origin and direction keep ITK's physical
//! x/y/z order. An explicit opt-in `IbsiInterpolation` instead builds the IBSI
//! interpolation grid: a ceil-sized, centered grid that covers the source extent, float64
//! image interpolation (linear by default or cubic B-spline), nearest-integer intensity
//! rounding, and a linearly interpolated float32 binary ROI mask binarized at a
//! partial-volume threshold.
//!
//! Execution: CPU, float64; resampling, the centre-hull test and edge padding run one scan
//! line per rayon task, each voxel computed independently. mlx-radiomics delegates resampling to
//! SimpleITK (`ResampleImageFilter`); this port implements the ITK 5.4 semantics directly:
//! identity transform, output grid to physical point to continuous source index, inside
//! test `-0.5 <= index < size - 0.5` per axis, otherwise the default value 0 (or the
//! nearest-voxel extrapolator for the IBSI grid); nearest neighbour is `floor(index + 0.5)`;
//! linear is multilinear with neighbours clamped to the image; cubic B-spline uses ITK's
//! prefilter (`itk_spline_coefficients`) with mirror boundaries. Orchestration (grid
//! algebra, cropping) runs on the calling thread.
//! ITK's scaled direction matrix, scanline endpoints and interpolation order determine
//! which voxel or integer intensity is selected at a half-integer boundary.
//!
//! Status: done (`ImagePair`, `LoadedCase`, `same_geometry`, `roi_bounds`, `correct_mask`,
//! `resample`, `ibsi_grid`, `inside_source_centers`, `edge_padded`, `resample_ibsi`,
//! `load_image_pair`, `prepare_image_pair`).

use crate::config::settings::{MaskExtrapolation, Settings, SplineBoundary};
use crate::config::spatial::{IbsiInterpolation, ImageSettings, Interpolator};
use crate::error::{Error, Result};
use crate::imaging::io::{HostArray, Volume, read_image, read_mask};
use crate::imaging::preprocessing::{exact_float64, normalize_values, round_intensities};
use crate::imaging::spatial::inverse::physical_to_index;
use crate::numerics::spline::{evaluate_flat, itk_spline_coefficients};
use ndarray::{ArrayD, IxDyn};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use rayon::prelude::*;
use std::path::Path;

mod inverse;

/// Tolerance of a voxel-centre coordinate for [`inside_source_centers`].
pub const CENTER_HULL_TOLERANCE: f64 = 1e-6;

/// Spacing/origin tolerance of [`same_geometry`] in units of the smallest image spacing.
pub const GEOMETRY_COORDINATE_TOLERANCE: f64 = 1e-6;

/// Direction-cosine tolerance of [`same_geometry`].
pub const GEOMETRY_DIRECTION_TOLERANCE: f64 = 1e-6;

/// ROI corner tolerance: a corner may lie this far (in voxels) beyond the outer image
/// edges of the `[-0.5, n - 0.5]` extent (`0.501` and `0.499` of `_roi_bounds`).
pub const ROI_EDGE_TOLERANCE: f64 = 0.001;

/// Edge replication in voxels before cubic B-spline prefiltering (`_SPLINE_EDGE_PAD`).
pub const SPLINE_EDGE_PAD: usize = 32;

fn invalid<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::InvalidInput(message.into()))
}

/// Physical placement of the arrays that were processed (`ImageGeometryPayload`).
///
/// `shape` and `spacing` follow NumPy array-axis order. `origin` and `direction` keep the
/// ITK physical x/y/z order; `direction` is the flattened row-major matrix.
#[derive(Clone, Debug, PartialEq)]
pub struct ImageGeometry {
    /// Array shape.
    pub shape: Vec<usize>,
    /// Spacing in mm per array axis.
    pub spacing: Vec<f64>,
    /// Origin in mm, ITK order.
    pub origin: Vec<f64>,
    /// Flattened row-major direction matrix.
    pub direction: Vec<f64>,
}

/// Aligned volumes, with the source image grid retained by the file loader (`ImagePair`).
#[derive(Clone, Debug, PartialEq)]
pub struct ImagePair {
    /// Image volume.
    pub image: Volume,
    /// Mask volume on the same grid.
    pub mask: Volume,
    /// Source image grid, when loaded from files.
    pub source_geometry: Option<ImageGeometry>,
}

impl ImagePair {
    /// Construct a pair, rejecting an image and mask that differ in shape or geometry.
    ///
    /// # Errors
    /// [`Error::InvalidInput`] if shape, spacing, origin or direction differ.
    pub fn new(
        image: Volume,
        mask: Volume,
        source_geometry: Option<ImageGeometry>,
    ) -> Result<Self> {
        if image.array.shape() != mask.array.shape()
            || image.spacing != mask.spacing
            || image.origin != mask.origin
            || image.direction != mask.direction
        {
            return invalid("aligned image and mask must share geometry");
        }
        Ok(Self {
            image,
            mask,
            source_geometry,
        })
    }
}

// ---------------------------------------------------------------------------------------
// ITK-style image
// ---------------------------------------------------------------------------------------

/// The part of a `SimpleITK.Image` used here: pixels in NumPy order plus ITK-order
/// spacing, origin and row-major direction.
#[derive(Clone, Debug, PartialEq)]
pub struct ItkImage {
    /// Pixels, NumPy order (z, y, x).
    pub array: HostArray,
    /// Spacing, ITK x-first order.
    pub spacing: Vec<f64>,
    /// Origin, ITK order.
    pub origin: Vec<f64>,
    /// Row-major direction.
    pub direction: Vec<f64>,
}

impl ItkImage {
    /// Number of dimensions.
    pub fn dimension(&self) -> usize {
        self.array.ndim()
    }

    /// Size in ITK x-first order.
    pub fn size(&self) -> Vec<usize> {
        self.array.shape().iter().rev().copied().collect()
    }

    /// `TransformContinuousIndexToPhysicalPoint`: `origin + D (spacing * index)`.
    pub fn continuous_index_to_physical(&self, index: &[f64]) -> Vec<f64> {
        let n = self.dimension();
        (0..n)
            .map(|row| {
                let mut sum = 0.0;
                for column in 0..n {
                    let step = self.direction[row * n + column] * self.spacing[column];
                    sum = step.mul_add(index[column], sum);
                }
                sum + self.origin[row]
            })
            .collect()
    }

    /// Transform a physical point through the inverse scaled direction matrix.
    pub fn physical_to_continuous_index(&self, point: &[f64]) -> Vec<f64> {
        let n = self.dimension();
        let inverse = physical_to_index(&self.direction, &self.spacing);
        inverse[..n]
            .iter()
            .map(|row| (0..n).fold(0.0, |sum, k| row[k].mul_add(point[k] - self.origin[k], sum)))
            .collect()
    }
}

/// Convert a [`Volume`] to an ITK-style image (`_itk`): spacing is reversed to x-first.
pub fn itk(volume: &Volume) -> ItkImage {
    ItkImage {
        array: volume.array.clone(),
        spacing: volume.spacing.iter().rev().copied().collect(),
        origin: volume.origin.clone(),
        direction: volume.direction.clone(),
    }
}

/// Convert an ITK-style image to a [`Volume`], reversing spacing into array-axis order
/// (`_volume`).
pub fn volume(image: &ItkImage) -> Result<Volume> {
    Volume::new(
        image.array.clone(),
        image.spacing.iter().rev().copied().collect(),
        image.origin.clone(),
        image.direction.clone(),
    )
}

/// Return whether image and mask share a grid within ITK's tolerances (`_same_geometry`).
///
/// Sizes must be equal; spacing and origin must agree within `1e-6` of the smallest image
/// spacing, as in ITK's geometry check, so the test stays in voxel units at any physical
/// scale; direction cosines must agree within `1e-6`.
pub fn same_geometry(image: &ItkImage, mask: &ItkImage) -> bool {
    let coordinate =
        GEOMETRY_COORDINATE_TOLERANCE * image.spacing.iter().copied().fold(f64::INFINITY, f64::min);
    let close = |a: &[f64], b: &[f64], tolerance: f64| {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() <= tolerance)
    };
    image.size() == mask.size()
        && close(&image.spacing, &mask.spacing, coordinate)
        && close(&image.origin, &mask.origin, coordinate)
        && close(
            &image.direction,
            &mask.direction,
            GEOMETRY_DIRECTION_TOLERANCE,
        )
}

/// Return the label's bounding box as `(lower, upper)` voxel indexes, upper exclusive
/// (`_roi_bounds`).
///
/// Both are in ITK x/y/z order. The box corners, mapped through physical space, must lie
/// inside the image extent (a tolerance of 0.001 voxel beyond its outer edges).
///
/// # Errors
/// [`Error::InvalidInput`] if the label is absent or the ROI extends outside the image.
pub fn roi_bounds(image: &ItkImage, mask: &ItkImage, label: i64) -> Result<(Vec<i64>, Vec<i64>)> {
    let n = mask.dimension();
    let hits = mask.array.eq_label(label);
    let shape = hits.shape().to_vec();
    // Per row of the last axis: the position of the row and the first and last hit in it.
    let standard = hits.as_standard_layout();
    let flat = standard.as_slice().expect("standard layout is contiguous");
    let row_len = shape[n - 1].max(1);
    let (low, high): (Vec<i64>, Vec<i64>) = flat
        .par_chunks(row_len)
        .enumerate()
        .fold(
            || (vec![i64::MAX; n], vec![i64::MIN; n]),
            |(mut low, mut high), (row, cells)| {
                if let Some(first) = cells.iter().position(|&hit| hit) {
                    let last = cells.iter().rposition(|&hit| hit).expect("a hit exists");
                    let mut rest = row;
                    // NumPy axes n - 2 down to 0 index the row; the last axis is the hit.
                    for axis in (0..n - 1).rev() {
                        let at = (rest % shape[axis]) as i64;
                        rest /= shape[axis];
                        low[axis] = low[axis].min(at);
                        high[axis] = high[axis].max(at + 1);
                    }
                    low[n - 1] = low[n - 1].min(first as i64);
                    high[n - 1] = high[n - 1].max(last as i64 + 1);
                }
                (low, high)
            },
        )
        .reduce(
            || (vec![i64::MAX; n], vec![i64::MIN; n]),
            |(mut low, mut high), (other_low, other_high)| {
                for axis in 0..n {
                    low[axis] = low[axis].min(other_low[axis]);
                    high[axis] = high[axis].max(other_high[axis]);
                }
                (low, high)
            },
        );
    let found = high[0] != i64::MIN;
    // ITK axis k is NumPy axis n - 1 - k.
    let lower: Vec<i64> = (0..n).map(|k| low[n - 1 - k]).collect();
    let upper: Vec<i64> = (0..n).map(|k| high[n - 1 - k]).collect();
    if !found {
        return invalid("selected label is absent from mask");
    }
    let size = image.size();
    for corner in 0..(1usize << n) {
        let index: Vec<f64> = (0..n)
            .map(|k| {
                if corner >> (n - 1 - k) & 1 == 1 {
                    upper[k] as f64 - 0.5
                } else {
                    lower[k] as f64 - 0.5
                }
            })
            .collect();
        let physical = mask.continuous_index_to_physical(&index);
        let in_image = image.physical_to_continuous_index(&physical);
        let outside = in_image.iter().zip(&size).any(|(&v, &s)| {
            v < -(0.5 + ROI_EDGE_TOLERANCE) || v > s as f64 - (0.5 - ROI_EDGE_TOLERANCE)
        });
        if outside {
            return invalid("mask ROI lies outside the physical image bounds");
        }
    }
    Ok((lower, upper))
}

/// Reject integer labels outside the exact float64 range used by ITK interpolation
/// (`_require_interpolable_labels`).
pub fn require_interpolable_labels(mask: &ItkImage) -> Result<()> {
    if mask.array.itemsize() <= 4 {
        return Ok(());
    }
    if let Some((low, high)) = mask.array.integer_range() {
        let limit = 1i128 << 53;
        if low < -limit || high > limit {
            return invalid(
                "spatial interpolation requires labels in exact integer range [-2**53, 2**53]",
            );
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------
// Resampling core (ITK ResampleImageFilter with an identity transform)
// ---------------------------------------------------------------------------------------

/// Output grid in ITK order.
#[derive(Clone, Debug)]
struct Grid {
    size: Vec<usize>,
    spacing: Vec<f64>,
    origin: Vec<f64>,
    direction: Vec<f64>,
}

impl Grid {
    fn of(image: &ItkImage) -> Self {
        Self {
            size: image.size(),
            spacing: image.spacing.clone(),
            origin: image.origin.clone(),
            direction: image.direction.clone(),
        }
    }

    fn numpy_shape(&self) -> Vec<usize> {
        self.size.iter().rev().copied().collect()
    }

    fn voxel_count(&self) -> usize {
        self.size.iter().product()
    }
}

/// Pixel-to-physical steps for the output and physical-to-pixel steps for the input.
/// ITK computes an integer output index's physical point starting at its origin.
struct IndexMap<'a> {
    n: usize,
    grid_origin: &'a [f64],
    source_origin: &'a [f64],
    forward: [[f64; 3]; 3],
    inverse: [[f64; 3]; 3],
}

impl<'a> IndexMap<'a> {
    fn new(grid: &'a Grid, source: &'a ItkImage) -> Self {
        let n = grid.size.len();
        let mut forward = [[0.0; 3]; 3];
        for (r, row) in forward[..n].iter_mut().enumerate() {
            for (c, entry) in row[..n].iter_mut().enumerate() {
                *entry = grid.direction[r * n + c] * grid.spacing[c];
            }
        }
        Self {
            n,
            grid_origin: &grid.origin,
            source_origin: &source.origin,
            forward,
            inverse: physical_to_index(&source.direction, &source.spacing),
        }
    }

    fn map(&self, index: &[f64; 3]) -> [f64; 3] {
        let n = self.n;
        let mut relative = [0.0; 3];
        for (r, value) in relative[..n].iter_mut().enumerate() {
            let point = (0..n).fold(self.grid_origin[r], |p, c| {
                self.forward[r][c].mul_add(index[c], p)
            });
            *value = point - self.source_origin[r];
        }
        let mut result = [0.0; 3];
        for (r, value) in result[..n].iter_mut().enumerate() {
            *value = (0..n).fold(0.0, |sum, c| self.inverse[r][c].mul_add(relative[c], sum));
        }
        result
    }
}

/// Evaluate `f` at the continuous source index of every output voxel, in NumPy C order,
/// one scan line (ITK axis 0, the fastest NumPy axis) per rayon task.
fn map_output<T: Send + Default + Clone>(
    source: &ItkImage,
    grid: &Grid,
    f: impl Fn(&[f64]) -> T + Sync,
) -> Vec<T> {
    let n = grid.size.len();
    let mapper = IndexMap::new(grid, source);
    let line_len = grid.size[0];
    let mut out = vec![T::default(); grid.voxel_count()];
    out.par_chunks_mut(line_len.max(1))
        .enumerate()
        .for_each(|(line, chunk)| {
            // ITK axis 1 varies fastest among the outer axes; the highest axis is slowest.
            let mut index = [0.0; 3];
            let mut rest = line;
            for k in 1..n {
                index[k] = (rest % grid.size[k]) as f64;
                rest /= grid.size[k];
            }
            // ITK interpolates each scanline between index zero and its exclusive end.
            // Reusing the two endpoints also keeps the index independent of Rayon scheduling.
            let start = mapper.map(&index);
            index[0] = line_len as f64;
            let end = mapper.map(&index);
            let delta: [f64; 3] = std::array::from_fn(|k| end[k] - start[k]);
            for (x, slot) in chunk.iter_mut().enumerate() {
                let fraction = x as f64 / line_len as f64;
                let point: [f64; 3] = std::array::from_fn(|k| fraction.mul_add(delta[k], start[k]));
                *slot = f(&point[..n]);
            }
        });
    out
}

fn inside_buffer(index: &[f64], size: &[usize]) -> bool {
    index
        .iter()
        .zip(size)
        .all(|(&v, &s)| v >= -0.5 && v < s as f64 - 0.5)
}

/// ITK `RoundHalfIntegerUp` clamped to the image (nearest-neighbour extrapolation).
fn nearest_flat(index: &[f64], size: &[usize]) -> usize {
    let mut flat = 0usize;
    let mut stride = 1usize;
    for (k, &v) in index.iter().enumerate() {
        let rounded = ((v + 0.5).floor() as i64).clamp(0, size[k] as i64 - 1) as usize;
        flat += rounded * stride;
        stride *= size[k];
    }
    flat
}

/// ITK `LinearInterpolateImageFunction` at a continuous index inside the buffer.
fn linear_at(data: &[f64], size: &[usize], index: &[f64]) -> f64 {
    let mut base = 0;
    let mut stride = 1;
    let mut step = [0; 3];
    let mut weight = [0.0; 3];
    for (axis, (&x, &length)) in index.iter().zip(size).enumerate() {
        let lower = x.floor().max(0.0) as usize;
        base += lower * stride;
        if x > lower as f64 && lower + 1 < length {
            step[axis] = stride;
            weight[axis] = x - lower as f64;
        }
        stride *= length;
    }

    // ITK blends along x, then y and then z. Skipping zero-weight neighbours
    // also preserves an exact sample when another corner is non-finite.
    let row = |i: usize| {
        let left = data[i];
        if step[0] == 0 {
            left
        } else {
            weight[0].mul_add(data[i + step[0]] - left, left)
        }
    };
    let plane = |i: usize| {
        let lower = row(i);
        if step[1] == 0 {
            lower
        } else {
            weight[1].mul_add(row(i + step[1]) - lower, lower)
        }
    };
    let front = plane(base);
    if step[2] == 0 {
        front
    } else {
        weight[2].mul_add(plane(base + step[2]) - front, front)
    }
}

/// How output pixels are computed from the source.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kernel {
    Nearest,
    Linear,
    Bspline,
}

impl From<Interpolator> for Kernel {
    fn from(value: Interpolator) -> Self {
        match value {
            Interpolator::Nearest => Self::Nearest,
            Interpolator::Linear => Self::Linear,
            Interpolator::Bspline => Self::Bspline,
        }
    }
}

/// Bounding box `(lower, upper)` of the samples that are not exactly zero, in ITK order
/// (upper exclusive); `None` when every sample is zero.
fn nonzero_box(data: &[f64], numpy_shape: &[usize]) -> Option<(Vec<usize>, Vec<usize>)> {
    let n = numpy_shape.len();
    let row_len = numpy_shape[n - 1].max(1);
    let (low, high) = data
        .par_chunks(row_len)
        .enumerate()
        .fold(
            || (vec![usize::MAX; n], vec![0usize; n]),
            |(mut low, mut high), (row, cells)| {
                if let Some(first) = cells.iter().position(|&v| v != 0.0) {
                    let last = cells.iter().rposition(|&v| v != 0.0).expect("a sample");
                    let mut rest = row;
                    for axis in (0..n - 1).rev() {
                        let at = rest % numpy_shape[axis];
                        rest /= numpy_shape[axis];
                        low[axis] = low[axis].min(at);
                        high[axis] = high[axis].max(at + 1);
                    }
                    low[n - 1] = low[n - 1].min(first);
                    high[n - 1] = high[n - 1].max(last + 1);
                }
                (low, high)
            },
        )
        .reduce(
            || (vec![usize::MAX; n], vec![0usize; n]),
            |(mut low, mut high), (other_low, other_high)| {
                for axis in 0..n {
                    low[axis] = low[axis].min(other_low[axis]);
                    high[axis] = high[axis].max(other_high[axis]);
                }
                (low, high)
            },
        );
    (high[0] != 0).then(|| {
        (
            (0..n).map(|k| low[n - 1 - k]).collect(),
            (0..n).map(|k| high[n - 1 - k]).collect(),
        )
    })
}

/// Resample float64 values onto `grid`; outside the source buffer use the nearest source
/// voxel (`extrapolate`) or 0.
///
/// Linear interpolation returns exactly `+0` where all the source voxels it reads are
/// zero, so it skips the kernel outside the box of nonzero samples (a binary ROI mask is
/// zero almost everywhere).
fn resample_f64(
    source: &ItkImage,
    values: &ArrayD<f64>,
    grid: &Grid,
    kernel: Kernel,
    extrapolate: bool,
) -> ArrayD<f64> {
    let size = source.size();
    let standard = values.as_standard_layout();
    let data = standard.as_slice().expect("standard layout is contiguous");
    let coefficients = (kernel == Kernel::Bspline).then(|| itk_spline_coefficients(values));
    let coefficient_standard = coefficients.as_ref().map(ArrayD::as_standard_layout);
    let numpy_shape = values.shape().to_vec();
    let support = (kernel == Kernel::Linear).then(|| nonzero_box(data, &numpy_shape));
    let out: Vec<f64> = map_output(source, grid, |index| {
        if !inside_buffer(index, &size) {
            return if extrapolate {
                data[nearest_flat(index, &size)]
            } else {
                0.0
            };
        }
        match kernel {
            Kernel::Nearest => data[nearest_flat(index, &size)],
            Kernel::Linear => {
                let silent = match support.as_ref().expect("computed for linear") {
                    None => true,
                    Some((lower, upper)) => index.iter().enumerate().any(|(k, &v)| {
                        let base = v.floor() as i64;
                        let touched_low = base.max(0);
                        let touched_high = (base + 1).min(size[k] as i64 - 1);
                        touched_high < lower[k] as i64 || touched_low >= upper[k] as i64
                    }),
                };
                if silent {
                    0.0
                } else {
                    linear_at(data, &size, index)
                }
            }
            Kernel::Bspline => {
                let mut coordinate = [0.0; 3];
                let n = index.len();
                for (slot, &value) in coordinate.iter_mut().zip(index.iter().rev()) {
                    *slot = value;
                }
                evaluate_flat(
                    coefficient_standard
                        .as_ref()
                        .expect("computed for B-spline")
                        .as_slice()
                        .expect("standard layout is contiguous"),
                    &numpy_shape,
                    &coordinate[..n],
                )
            }
        }
    });
    ArrayD::from_shape_vec(IxDyn(&grid.numpy_shape()), out).expect("grid voxel count")
}

/// Nearest-neighbour resampling that keeps the pixel type (0 outside the buffer).
fn resample_nearest_typed(source: &ItkImage, grid: &Grid) -> HostArray {
    let size = source.size();
    let mapped: Vec<Option<usize>> = map_output(source, grid, |index| {
        inside_buffer(index, &size).then(|| nearest_flat(index, &size))
    });
    source.array.gather(&grid.numpy_shape(), &mapped)
}

fn with_grid(array: HostArray, grid: &Grid) -> ItkImage {
    ItkImage {
        array,
        spacing: grid.spacing.clone(),
        origin: grid.origin.clone(),
        direction: grid.direction.clone(),
    }
}

/// `RegionOfInterest`: crop to `[index, index + size)` (ITK order), moving the origin.
fn region_of_interest(image: &ItkImage, size: &[i64], index: &[i64]) -> ItkImage {
    let n = image.dimension();
    let lower: Vec<usize> = (0..n).map(|axis| index[n - 1 - axis] as usize).collect();
    let upper: Vec<usize> = (0..n)
        .map(|axis| (index[n - 1 - axis] + size[n - 1 - axis]) as usize)
        .collect();
    let corner: Vec<f64> = index.iter().map(|&v| v as f64).collect();
    ItkImage {
        array: image.array.crop(&lower, &upper),
        spacing: image.spacing.clone(),
        origin: image.continuous_index_to_physical(&corner),
        direction: image.direction.clone(),
    }
}

/// Correct the mask by nearest neighbor, preserving exact integers when already aligned
/// (`_correct_mask`).
pub fn correct_mask(image: &ItkImage, mask: &ItkImage) -> Result<ItkImage> {
    if same_geometry(image, mask) {
        let mut aligned = mask.clone();
        aligned.spacing.clone_from(&image.spacing);
        aligned.origin.clone_from(&image.origin);
        aligned.direction.clone_from(&image.direction);
        return Ok(aligned);
    }
    require_interpolable_labels(mask)?;
    let grid = Grid::of(image);
    Ok(with_grid(resample_nearest_typed(mask, &grid), &grid))
}

fn all_close(a: &[f64], b: &[f64]) -> bool {
    // np.allclose(a, b): |a - b| <= atol + rtol * |b|.
    a.iter()
        .zip(b)
        .all(|(x, y)| (x - y).abs() <= 1e-8 + 1e-5 * y.abs())
}

/// Resample image and mask to `settings.resampled_spacing` and crop to the ROI
/// (`_resample`).
///
/// The grid is aligned to the mask origin and cropped to the ROI bounding box plus
/// `pad_distance` (PyRadiomics `resampleImage`). If the spacing already matches the image,
/// a misaligned mask is corrected and both are cropped; an aligned mask needs no
/// interpolation. That branch uses the image's direction; the general resampling branch
/// uses the mask's direction. Image intensities are interpolated in float64 regardless of
/// the source storage type, except in the same-grid crop, which keeps the source type;
/// masks keep their label type and use nearest neighbour.
pub fn resample(
    image: &ItkImage,
    mask: &ItkImage,
    settings: &ImageSettings,
    label: i64,
) -> Result<(ItkImage, ItkImage)> {
    let n = image.dimension();
    let Some(requested) = &settings.resampled_spacing else {
        return invalid("resampled_spacing must match the image dimension");
    };
    if requested.len() != n {
        return invalid("resampled_spacing must match the image dimension");
    }
    let (lower, upper) = roi_bounds(image, mask, label)?;
    let source_spacing = mask.spacing.clone();
    let spacing: Vec<f64> = (0..n)
        .map(|k| {
            let wanted = requested[n - 1 - k];
            if wanted == 0.0 || upper[k] - lower[k] == 1 {
                source_spacing[k]
            } else {
                wanted
            }
        })
        .collect();
    let pad = settings.pad_distance as i64;
    if all_close(&image.spacing, &spacing) {
        let corrected = correct_mask(image, mask)?;
        let (lower, upper) = roi_bounds(image, &corrected, label)?;
        let size = image.size();
        let low: Vec<i64> = lower.iter().map(|&v| (v - pad).max(0)).collect();
        let high: Vec<i64> = upper
            .iter()
            .zip(&size)
            .map(|(&v, &s)| (v + pad).min(s as i64))
            .collect();
        let region: Vec<i64> = low.iter().zip(&high).map(|(l, h)| h - l).collect();
        return Ok((
            region_of_interest(image, &region, &low),
            region_of_interest(&corrected, &region, &low),
        ));
    }
    require_interpolable_labels(mask)?;
    let ratio: Vec<f64> = (0..n).map(|k| source_spacing[k] / spacing[k]).collect();
    let mask_size = mask.size();
    let mut size = vec![0usize; n];
    let mut origin_index = vec![0.0; n];
    for k in 0..n {
        let new_lower = (((lower[k] as f64 - 0.5) * ratio[k] - pad as f64).floor()).max(0.0);
        let new_upper = (((upper[k] as f64 - 0.5) * ratio[k] + pad as f64).ceil())
            .min((mask_size[k] as f64 * ratio[k]).ceil() - 1.0);
        let extent = new_upper - new_lower + 1.0;
        if extent < 1.0 {
            return invalid("resampled grid is empty");
        }
        size[k] = extent as usize;
        origin_index[k] =
            0.5 * (spacing[k] - source_spacing[k]) / source_spacing[k] + new_lower / ratio[k];
    }
    let grid = Grid {
        size,
        spacing,
        origin: mask.continuous_index_to_physical(&origin_index),
        direction: mask.direction.clone(),
    };
    let values = exact_float64(&image.array)?;
    let resampled = resample_f64(image, &values, &grid, settings.interpolator.into(), false);
    let image_out = with_grid(HostArray::F64(resampled), &grid);
    let mask_out = with_grid(resample_nearest_typed(mask, &grid), &grid);
    Ok((image_out, mask_out))
}

// ---------------------------------------------------------------------------------------
// IBSI centred-grid interpolation
// ---------------------------------------------------------------------------------------

/// Return the centered IBSI grid as ITK-order size, spacing and world origin
/// (`_ibsi_grid`).
///
/// `spacing` follows NumPy array-axis order; a zero entry keeps the source spacing. The
/// per-axis size is `ceil(n * s / s_b)` on the exact supplied binary64 spacing ratios. The
/// origin is shifted so the grids share a physical center, with the index-space offset
/// rotated by the image direction:
/// `origin_b = origin_a + D @ ((s_a (n_a - 1) - s_b (n_b - 1)) / 2)`.
pub fn ibsi_grid(image: &ItkImage, spacing: &[f64]) -> (Vec<usize>, Vec<f64>, Vec<f64>) {
    let n = image.dimension();
    let size = image.size();
    let source = &image.spacing;
    let target: Vec<f64> = (0..n)
        .map(|k| {
            let wanted = spacing[n - 1 - k];
            if wanted == 0.0 || wanted == source[k] {
                source[k]
            } else {
                wanted
            }
        })
        .collect();
    // Round only the final rational extent. Floating multiplication/division can move an
    // exact integer above its boundary; an epsilon would erase real fractional cells.
    let shape: Vec<usize> = (0..n)
        .map(|k| {
            let ratio = BigRational::from_integer(BigInt::from(size[k]))
                * exact_fraction(source[k])
                / exact_fraction(target[k]);
            ratio
                .ceil()
                .to_integer()
                .to_usize()
                .expect("grid size fits usize")
        })
        .collect();
    let offset: Vec<f64> = (0..n)
        .map(|k| (source[k] * (size[k] as f64 - 1.0) - target[k] * (shape[k] as f64 - 1.0)) / 2.0)
        .collect();
    let origin: Vec<f64> = (0..n)
        .map(|row| {
            let mut sum = 0.0;
            for column in 0..n {
                sum += image.direction[row * n + column] * offset[column];
            }
            image.origin[row] + sum
        })
        .collect();
    (shape, target, origin)
}

fn exact_fraction(x: f64) -> BigRational {
    crate::imaging::preprocessing::fraction_of_f64(x)
}

/// Flag output voxels whose centers lie within the source voxel-center hull per axis
/// (`_inside_source_centers`).
///
/// The output grid is given in ITK order (`shape`, `spacing`, `origin` and flattened
/// `direction`); the result is a NumPy-order boolean array. A center is inside when its
/// continuous source index is in `[0, n - 1]` on every axis, up to a tolerance of `1e-6`
/// voxel that absorbs binary64 coordinate roundoff.
pub fn inside_source_centers(
    source: &ItkImage,
    shape: &[usize],
    spacing: &[f64],
    origin: &[f64],
    direction: &[f64],
) -> ArrayD<bool> {
    let n = shape.len();
    let source_direction = &source.direction;
    let source_spacing = &source.spacing;
    // Continuous source index = linear @ output_index + offset, exact for any pair of grids.
    let mut linear = vec![0.0; n * n];
    for row in 0..n {
        for column in 0..n {
            let mut product = 0.0;
            for k in 0..n {
                product += source_direction[k * n + row] * direction[k * n + column];
            }
            linear[row * n + column] = product * spacing[column] / source_spacing[row];
        }
    }
    let offset: Vec<f64> = (0..n)
        .map(|row| {
            let mut sum = 0.0;
            for k in 0..n {
                sum += source_direction[k * n + row] * (origin[k] - source.origin[k]);
            }
            sum / source_spacing[row]
        })
        .collect();
    let last: Vec<f64> = source.size().iter().map(|&s| s as f64 - 1.0).collect();
    let numpy_shape: Vec<usize> = shape.iter().rev().copied().collect();
    let mut inside = vec![false; shape.iter().product()];
    // NumPy C order: the highest ITK axis is slowest; x (ITK axis 0) is fastest.
    inside
        .par_chunks_mut(shape[0].max(1))
        .enumerate()
        .for_each(|(line, chunk)| {
            let mut counters = [0usize; 3];
            let mut rest = line;
            for k in 1..n {
                counters[k] = rest % shape[k];
                rest /= shape[k];
            }
            for (x, slot) in chunk.iter_mut().enumerate() {
                counters[0] = x;
                let mut valid = true;
                for row in 0..n {
                    // coordinate = offset + linear[row, n-1] * slab, then the remaining columns.
                    let mut coordinate =
                        offset[row] + linear[row * n + n - 1] * counters[n - 1] as f64;
                    for column in 0..n - 1 {
                        coordinate += linear[row * n + column] * counters[column] as f64;
                    }
                    valid &= coordinate >= -CENTER_HULL_TOLERANCE
                        && coordinate <= last[row] + CENTER_HULL_TOLERANCE;
                }
                *slot = valid;
            }
        });
    ArrayD::from_shape_vec(IxDyn(&numpy_shape), inside).expect("grid voxel count")
}

/// Return `image` with edge-replicated voxels added on every side, grid unchanged
/// (`_edge_padded`).
///
/// ITK prefilters a cubic B-spline with a mirror boundary. Replicating the edge voxels
/// first reproduces the "nearest" boundary of `scipy.ndimage.map_coordinates`; the mirror
/// condition's influence decays by roughly 0.27 per voxel, so 32 voxels leave it below
/// 1e-18 at the original border.
pub fn edge_padded(image: &ItkImage) -> Result<ItkImage> {
    let n = image.dimension();
    let pad = SPLINE_EDGE_PAD;
    let HostArray::F64(values) = &image.array else {
        return invalid("edge padding requires float64 intensities");
    };
    let shape = values.shape().to_vec();
    let padded_shape: Vec<usize> = shape.iter().map(|s| s + 2 * pad).collect();
    let maps: Vec<Vec<usize>> = (0..n)
        .map(|axis| {
            (0..padded_shape[axis])
                .map(|p| p.saturating_sub(pad).min(shape[axis] - 1))
                .collect()
        })
        .collect();
    let mut strides = vec![1usize; n];
    for axis in (0..n.saturating_sub(1)).rev() {
        strides[axis] = strides[axis + 1] * shape[axis + 1];
    }
    let source = values.as_standard_layout();
    let source = source.as_slice().expect("standard layout is contiguous");
    let total: usize = padded_shape.iter().product();
    let mut data = vec![0.0; total];
    let last = padded_shape[n - 1];
    data.par_chunks_mut(last.max(1))
        .enumerate()
        .for_each(|(line, chunk)| {
            let mut base = 0usize;
            let mut rest = line;
            for axis in (0..n - 1).rev() {
                base += maps[axis][rest % padded_shape[axis]] * strides[axis];
                rest /= padded_shape[axis];
            }
            for (slot, &source_index) in chunk.iter_mut().zip(&maps[n - 1]) {
                *slot = source[base + source_index * strides[n - 1]];
            }
        });
    let padded = ArrayD::from_shape_vec(IxDyn(&padded_shape), data).expect("padded voxel count");
    let shift: Vec<f64> = (0..n)
        .map(|row| {
            let mut sum = 0.0;
            for column in 0..n {
                sum += image.direction[row * n + column] * (pad as f64 * image.spacing[column]);
            }
            sum
        })
        .collect();
    Ok(ItkImage {
        array: HostArray::F64(padded),
        spacing: image.spacing.clone(),
        origin: image
            .origin
            .iter()
            .zip(&shift)
            .map(|(o, s)| o - s)
            .collect(),
        direction: image.direction.clone(),
    })
}

fn labelled_like(like: &HostArray, selected: &ArrayD<bool>, label: i64) -> Result<HostArray> {
    macro_rules! build {
        ($variant:ident, $ty:ty) => {{
            let value = <$ty>::try_from(label).map_err(|_| {
                Error::InvalidInput("label does not fit the mask pixel type".into())
            })?;
            HostArray::$variant(selected.mapv(|hit| if hit { value } else { 0 }))
        }};
    }
    Ok(match like {
        HostArray::Bool(_) => HostArray::Bool(selected.mapv(|hit| hit && label != 0)),
        HostArray::U8(_) => build!(U8, u8),
        HostArray::I8(_) => build!(I8, i8),
        HostArray::U16(_) => build!(U16, u16),
        HostArray::I16(_) => build!(I16, i16),
        HostArray::U32(_) => build!(U32, u32),
        HostArray::I32(_) => build!(I32, i32),
        HostArray::U64(_) => build!(U64, u64),
        HostArray::I64(_) => build!(I64, i64),
        HostArray::F32(_) | HostArray::F64(_) => {
            return invalid("mask must contain integer labels");
        }
    })
}

/// Interpolate image and mask onto the centered IBSI grid (`_resample_ibsi`).
///
/// Intensities are rounded to the nearest integer before interpolation, as the reference
/// data instructions require of converted NIfTI arrays, and rounded again afterwards, so
/// the interpolated result is integer-valued as well. The intensity image uses
/// `settings.image_interpolator` in float64; `Bspline` is the cubic B-spline (order 3)
/// with ITK's internal mirror boundary conditions. The requested label is selected before
/// interpolation, so other labels never blend into its occupancy. Occupancy is
/// interpolated linearly to float32, independently of the image interpolator, before
/// thresholding to exactly `label` or 0. Output voxel centers beyond the outermost source
/// voxel centers along any axis lie outside the sampled domain. `MaskExtrapolation::Zero`
/// leaves them out of the mask (MIRP registers masks with constant zero fill);
/// `Nearest` copies the nearest source occupancy, which grows an ROI at the image border.
/// The intensity image always uses the nearest source intensity.
pub fn resample_ibsi(
    image: &ItkImage,
    mask: &ItkImage,
    settings: &IbsiInterpolation,
    label: i64,
    mask_extrapolation: MaskExtrapolation,
    spline_boundary: SplineBoundary,
) -> Result<(ItkImage, ItkImage)> {
    let n = image.dimension();
    if settings.spacing.len() != n {
        return invalid("IBSI spacing must match the image dimension");
    }
    require_interpolable_labels(mask)?;
    let (shape, spacing, origin) = ibsi_grid(image, &settings.spacing);
    let grid = Grid {
        size: shape.clone(),
        spacing: spacing.clone(),
        origin: origin.clone(),
        direction: image.direction.clone(),
    };
    let kernel = match settings.image_interpolator {
        Interpolator::Linear => Kernel::Linear,
        Interpolator::Bspline => Kernel::Bspline,
        Interpolator::Nearest => return invalid("image_interpolator must be linear or bspline"),
    };
    let mut source_values = if settings.round_intensities {
        round_intensities(&image.array)?
    } else {
        exact_float64(&image.array)?
    };
    let mut cast_source = ItkImage {
        array: HostArray::F64(source_values.clone()),
        spacing: image.spacing.clone(),
        origin: image.origin.clone(),
        direction: image.direction.clone(),
    };
    if spline_boundary == SplineBoundary::Nearest && kernel == Kernel::Bspline {
        cast_source = edge_padded(&cast_source)?;
        if let HostArray::F64(values) = &cast_source.array {
            source_values = values.clone();
        }
    }
    let resampled_image = resample_f64(&cast_source, &source_values, &grid, kernel, true);
    // Interpolate occupancy, not label magnitude: the partial-volume threshold is a
    // fraction of one voxel regardless of the selected positive integer label. Occupancy
    // is stored in float32 (IBSI recommends full 32-bit precision for ROI interpolation)
    // before thresholding, so double-precision roundoff cannot exclude a half-covered
    // voxel as 0.4999999999999999. The mask is always linearly interpolated.
    let occupancy = mask
        .array
        .eq_label(label)
        .mapv(|hit| f64::from(u8::from(hit)));
    let occupancy_image = ItkImage {
        array: HostArray::F32(occupancy.mapv(|v| v as f32)),
        spacing: mask.spacing.clone(),
        origin: mask.origin.clone(),
        direction: mask.direction.clone(),
    };
    let interpolated_mask =
        resample_f64(&occupancy_image, &occupancy, &grid, Kernel::Linear, true).mapv(|v| v as f32);
    let mut values = resampled_image;
    if settings.round_intensities {
        values.mapv_inplace(f64::round_ties_even);
    }
    // Compare in the occupancy's float32 representation: a float64 threshold would
    // exclude an exactly 0.7-covered voxel stored as float32 0.699999988.
    let threshold = settings.mask_threshold as f32;
    let mut selected = interpolated_mask.mapv(|v| v >= threshold);
    if mask_extrapolation == MaskExtrapolation::Zero {
        let inside = inside_source_centers(mask, &shape, &spacing, &origin, &image.direction);
        selected.zip_mut_with(&inside, |s, &i| *s &= i);
    }
    let result_mask = labelled_like(&mask.array, &selected, label)?;
    Ok((
        with_grid(HostArray::F64(values), &grid),
        with_grid(result_mask, &grid),
    ))
}

// ---------------------------------------------------------------------------------------
// Pipeline
// ---------------------------------------------------------------------------------------

/// Reject invalid spatial options (`ImageSettings.__post_init__`).
fn validate_image_settings(settings: &ImageSettings) -> Result<()> {
    if let Some(spacing) = &settings.resampled_spacing
        && (!(2..=3).contains(&spacing.len()) || spacing.iter().any(|v| !v.is_finite() || *v < 0.0))
    {
        return invalid("resampled_spacing requires 2 or 3 finite nonnegative values");
    }
    if let Some(ibsi) = &settings.ibsi_interpolation {
        if !(2..=3).contains(&ibsi.spacing.len())
            || ibsi.spacing.iter().any(|v| !v.is_finite() || *v < 0.0)
        {
            return invalid("IBSI spacing requires 2 or 3 finite nonnegative values");
        }
        if !ibsi.mask_threshold.is_finite()
            || ibsi.mask_threshold <= 0.0
            || ibsi.mask_threshold > 1.0
        {
            return invalid("mask_threshold must be finite and in (0, 1]");
        }
        if ibsi.image_interpolator == Interpolator::Nearest {
            return invalid("image_interpolator must be linear or bspline");
        }
        if settings.resampled_spacing.is_some() {
            return invalid("ibsi_interpolation and resampled_spacing are mutually exclusive");
        }
        if settings.correct_mask {
            return invalid("ibsi_interpolation interpolates the mask; correct_mask must be False");
        }
    }
    Ok(())
}

/// Read an image and mask and bring them onto one grid (`load_image_pair`).
///
/// Normalization (if `settings.normalize`) is applied to the full image before any
/// resampling. Then, if `image_settings.ibsi_interpolation` is set, both are interpolated
/// onto the centered IBSI grid; otherwise, if `image_settings.resampled_spacing` is set,
/// both are resampled and cropped around the ROI; otherwise a geometry mismatch is
/// corrected only when `image_settings.correct_mask` is true. Interpolated intensities are
/// float64; mask labels keep their source integer type.
///
/// # Errors
/// [`Error::InvalidInput`] if a file is missing, dimensions or geometry disagree without
/// correction, the label is absent or lies outside the image, or preprocessing removes the
/// label. Spatial interpolation rejects masks with labels outside `[-2**53, 2**53]`; IBSI
/// interpolation selects binary occupancy for the requested label and refuses
/// normalization with `round_intensities = true`. 64-bit integer intensities must be
/// exactly representable in float64.
pub fn load_image_pair(
    image_path: &Path,
    mask_path: &Path,
    settings: Option<&Settings>,
    image_settings: Option<&ImageSettings>,
) -> Result<ImagePair> {
    let source = read_image(image_path)?;
    let labels = read_mask(mask_path)?;
    prepare_image_pair(&source, &labels, settings, image_settings)
}

/// Prepare one label from validated source volumes without modifying them
/// (`prepare_image_pair`).
///
/// Every call independently normalizes, corrects/resamples and crops using its label
/// settings, exactly as [`load_image_pair`] does after reading the files.
pub fn prepare_image_pair(
    source: &Volume,
    labels: &Volume,
    settings: Option<&Settings>,
    image_settings: Option<&ImageSettings>,
) -> Result<ImagePair> {
    let default_settings = Settings::default();
    let default_image_settings = ImageSettings::default();
    let options = settings.unwrap_or(&default_settings);
    let spatial = image_settings.unwrap_or(&default_image_settings);
    options.validate(source.array.ndim())?;
    validate_image_settings(spatial)?;
    // Only 64-bit integers can lose source identity when converted to float64. Validate
    // them before resampling/rounding.
    if source.array.is_wide_integer() {
        exact_float64(&source.array)?;
    }
    if options.normalize
        && spatial
            .ibsi_interpolation
            .as_ref()
            .is_some_and(|i| i.round_intensities)
    {
        // Normalization runs first; integer rounding would collapse z-scores to a handful
        // of levels and silently change every intensity feature.
        return invalid(
            "normalize=True cannot be combined with IBSI interpolation rounding; set IbsiInterpolation(round_intensities=False)",
        );
    }
    let mut image = itk(source);
    let mut mask = itk(labels);
    if image.dimension() != mask.dimension() {
        return invalid("image and mask dimensions must match");
    }
    roi_bounds(&image, &mask, options.label)?;
    if options.normalize {
        image.array = HostArray::F64(normalize_values(&source.array, options)?);
    }
    if let Some(ibsi) = &spatial.ibsi_interpolation {
        (image, mask) = resample_ibsi(
            &image,
            &mask,
            ibsi,
            options.label,
            options.mask_extrapolation,
            options.spline_boundary,
        )?;
    } else if spatial.resampled_spacing.is_some() {
        (image, mask) = resample(&image, &mask, spatial, options.label)?;
    } else if !same_geometry(&image, &mask) {
        if !spatial.correct_mask {
            return invalid("geometry mismatch; explicitly enable correct_mask or resampling");
        }
        mask = correct_mask(&image, &mask)?;
    } else {
        mask.spacing.clone_from(&image.spacing);
        mask.origin.clone_from(&image.origin);
        mask.direction.clone_from(&image.direction);
    }
    if !mask.array.eq_label(options.label).iter().any(|&hit| hit) {
        return invalid("spatial preprocessing removed the selected label");
    }
    let geometry = ImageGeometry {
        shape: source.array.shape().to_vec(),
        spacing: source.spacing.clone(),
        origin: source.origin.clone(),
        direction: source.direction.clone(),
    };
    ImagePair::new(volume(&image)?, volume(&mask)?, Some(geometry))
}

#[cfg(test)]
mod tests {
    use crate::imaging::io::{HostArray, Volume};
    use crate::imaging::spatial::prepare_image_pair;
    use crate::{IbsiInterpolation, ImageSettings, Interpolator, Settings};
    use ndarray::{ArrayD, IxDyn};

    const SHAPE: [usize; 3] = [9, 22, 25];

    fn volume(array: HostArray) -> Volume {
        Volume::new(
            array,
            vec![2.0, 1.2, 0.9],
            vec![0.0; 3],
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        )
        .unwrap()
    }

    /// Bits of the prepared image and the labels of the pair.
    fn prepared(image_settings: &ImageSettings) -> (Vec<u64>, Vec<u8>) {
        let image = ArrayD::from_shape_fn(IxDyn(&SHAPE), |i| {
            ((i[0] * 31 + i[1] * 17 + i[2] * 7) % 97) as f64 + 0.25 * (i[2] as f64).sin()
        });
        let mask = ArrayD::from_shape_fn(IxDyn(&SHAPE), |i| {
            u8::from((2..7).contains(&i[0]) && (4..18).contains(&i[1]) && (5..20).contains(&i[2]))
        });
        let pair = prepare_image_pair(
            &volume(HostArray::F64(image.mapv(f64::round))),
            &volume(HostArray::U8(mask)),
            Some(&Settings::default()),
            Some(image_settings),
        )
        .unwrap();
        let HostArray::F64(values) = &pair.image.array else {
            panic!("float64 image");
        };
        let HostArray::U8(labels) = &pair.mask.array else {
            panic!("u8 mask");
        };
        (
            values.iter().map(|v| v.to_bits()).collect(),
            labels.iter().copied().collect(),
        )
    }

    #[test]
    fn interpolation_and_resampling_are_bit_identical_across_thread_counts() {
        let one = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap();
        for interpolator in [Interpolator::Linear, Interpolator::Bspline] {
            let mut ibsi = IbsiInterpolation::new(vec![1.0, 1.0, 1.0]);
            ibsi.image_interpolator = interpolator;
            let cases = [
                ImageSettings {
                    ibsi_interpolation: Some(ibsi),
                    ..ImageSettings::default()
                },
                ImageSettings {
                    resampled_spacing: Some(vec![1.5, 1.0, 1.0]),
                    interpolator,
                    ..ImageSettings::default()
                },
            ];
            for (index, case) in cases.iter().enumerate() {
                assert_eq!(
                    one.install(|| prepared(case)),
                    prepared(case),
                    "{interpolator:?} case {index}"
                );
            }
        }
    }
}
