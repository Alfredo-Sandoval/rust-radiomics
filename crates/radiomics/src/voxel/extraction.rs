//! Voxel scheduling and evaluated feature batches.
//!
//! Mirrors `mlxradiomics/voxel/extraction.py` at mlx-radiomics v1.1.0.
//! Status: ported on the CPU for all 147 voxel features (firstorder, glcm, glrlm,
//! glszm, gldzm, gldm, ngtdm, intensityhistogram, ivh); no stage returns
//! `NotImplemented`.
//!
//! CPU numerical work in float64 (first order: the float32-compatible values of
//! [`crate::features::first_order::compute_maps`], widened). Threading: rayon parallelises over the
//! centres of each batch; results are collected in centre order, so output does not
//! depend on the thread count. First-order columns use the existing reusable
//! first-order kernel in each parallel block.
//!
//! Semantics follow mlx. Every ROI voxel is the centre of a window of half-width
//! `kernel_radius` (0 extent along `force2d_axis`). Window positions outside the image
//! or outside `sampling_mask` (the ROI, or the whole image when preprocessing chose an
//! unmasked kernel) are invalid. Discretisation is never repeated per window: all
//! windows read the global gray-level lattice `Prepared::bins` with the global
//! `gray_levels`, and IVH reads its own global lattice. Each window has a fixed shape `2r+1` per axis, so
//! directions and distances are limited by the window, not by the image. GLDZM uses
//! the window of `shape_roi` as its morphological mask when `masked_kernel` is set and
//! the sampling window otherwise. Maps are 0 outside the ROI and may hold NaN or
//! infinities inside it.
//!
//! Admission accounts for each selected family's local working set and retained pair
//! indices. Mixed first-order and GLCM selections can cache a group of float32
//! first-order columns across delivery batches within the scratch budget. Resident
//! inputs and final maps are accounted for separately. Small native windows use
//! reusable integer matrices and bit sets, and larger windows use the ROI algorithms.

use crate::config::settings::{Mode, list_features};
use crate::features::first_order::FirstOrderMapKernel;
use crate::features::intensity_histogram;
use crate::features::ivh::{IvhLattice, ivh_statistics, prepare_ivh};
use crate::voxel::budget::{Schedule, schedule};
use crate::voxel::gather::NeighborhoodPlan;
use crate::voxel::glcm::local_glcm;
use crate::voxel::texture::local_texture;
use crate::voxel::window_glcm::{PairCounts, WindowGlcm};
use crate::voxel::window_texture::WindowTexture;
use crate::{Error, Prepared, PreparedInput, Result, Settings};
use ndarray::{Array2, ArrayD, ArrayView2, Axis, IxDyn, s};
use rayon::prelude::*;

/// Batch size and working-set estimate of one extraction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoxelMemory {
    /// Centres evaluated and delivered together.
    pub batch_size: usize,
    /// Estimated bytes of one batch's working set.
    pub scratch_estimate_bytes: usize,
}

/// Receives each evaluated batch: int32 centres `(batch, ndim)` and one column per
/// selected feature, in `settings.features` order.
pub type VoxelSink<'a> = &'a mut dyn FnMut(&Array2<i32>, &[Vec<f64>]) -> Result<()>;

struct Plan {
    first: Vec<usize>,
    glcm: Vec<usize>,
    texture: Vec<usize>,
    histogram: Vec<usize>,
    ivh: Vec<usize>,
    gldzm: bool,
    local_shape: Vec<usize>,
    offsets: Array2<i32>,
    schedule: Schedule,
}

fn invalid(message: &str) -> Error {
    Error::InvalidInput(message.into())
}

fn plan(
    data: &Prepared,
    settings: &Settings,
    spacing: &[f64],
    roi_centers: Option<usize>,
) -> Result<Plan> {
    let input = &data.input;
    let shape = input.image.shape();
    let ndim = shape.len();
    if !(2..=3).contains(&ndim)
        || input.roi.shape() != shape
        || input.sampling_mask.shape() != shape
        || input.shape_roi.shape() != shape
        || data.bins.as_ref().is_some_and(|b| b.shape() != shape)
    {
        return Err(invalid(
            "image and masks must share a nonempty 2D or 3D array shape",
        ));
    }
    if spacing.len() != ndim || spacing.iter().any(|&v| !v.is_finite() || v <= 0.) {
        return Err(invalid(
            "spacing must contain one finite positive value per array axis",
        ));
    }
    if settings.kernel_radius == 0 || settings.scratch_mib == 0 {
        return Err(invalid(
            "kernel_radius and scratch_mib must be positive integers",
        ));
    }
    if settings.force2d_axis.is_some_and(|a| a >= ndim) {
        return Err(invalid(
            "force2d_axis must be an integer indexing a NumPy image axis",
        ));
    }
    let roi_centers = roi_centers.unwrap_or_else(|| match input.roi.as_slice_memory_order() {
        Some(mask) => mask
            .chunks(256)
            .filter(|block| block.contains(&true))
            .flatten()
            .filter(|&&v| v)
            .count(),
        None => input.roi.iter().filter(|&&v| v).count(),
    });
    if roi_centers < 2 {
        return Err(invalid(
            "voxel extraction requires at least two selected voxels",
        ));
    }
    let valid_names = list_features(&[], Mode::Voxel, Some(ndim as u8))?;
    let mut groups: [Vec<usize>; 5] = Default::default();
    let mut gldzm = false;
    for (i, key) in settings.features.iter().enumerate() {
        if settings.features[..i].contains(key) {
            return Err(Error::InvalidInput(format!(
                "duplicate feature name: {key}"
            )));
        }
        if !valid_names.contains(key) {
            return Err(Error::InvalidInput(format!(
                "feature {key} is not available in voxel mode for {ndim}D input"
            )));
        }
        let family = key.split_once('_').map_or("", |(f, _)| f);
        gldzm |= family == "gldzm";
        let slot = match family {
            "firstorder" => 0,
            "glcm" => 1,
            "glrlm" | "glszm" | "gldzm" | "gldm" | "ngtdm" => 2,
            "intensityhistogram" => 3,
            "ivh" => 4,
            _ => unreachable!("voxel catalog is limited to these families"),
        };
        groups[slot].push(i);
    }
    if settings.features.is_empty() {
        return Err(invalid("select at least one voxel feature"));
    }
    let [first, glcm, texture, histogram, ivh] = groups;
    if !(glcm.is_empty() && texture.is_empty() && histogram.is_empty()) && data.bins.is_none() {
        return Err(invalid(
            "texture and intensity-histogram maps require discretized bins",
        ));
    }
    let nonfinite = match (input.image.as_slice(), input.sampling_mask.as_slice()) {
        (Some(image), Some(mask)) => {
            image
                .chunks(256)
                .zip(mask.chunks(256))
                .any(|(values, selected)| {
                    selected.contains(&true)
                        && values
                            .iter()
                            .zip(selected)
                            .any(|(v, &m)| m && !v.is_finite())
                })
        }
        _ => input
            .image
            .iter()
            .zip(&input.sampling_mask)
            .any(|(v, &m)| m && !v.is_finite()),
    };
    if nonfinite {
        return Err(invalid("sampled image intensities must be finite"));
    }
    let schedule = schedule(settings, ndim, data.gray_levels, roi_centers)?;
    let r = settings.kernel_radius as i32;
    let axes: Vec<Vec<i32>> = (0..ndim)
        .map(|a| {
            if settings.force2d_axis == Some(a) {
                vec![0]
            } else {
                (-r..=r).collect()
            }
        })
        .collect();
    let local_shape: Vec<usize> = axes.iter().map(Vec::len).collect();
    let kernel = local_shape.iter().product::<usize>();
    // C order over the kernel axes, as `itertools.product`.
    let mut offsets = Array2::<i32>::zeros((kernel, ndim));
    for k in 0..kernel {
        let mut rest = k;
        for a in (0..ndim).rev() {
            offsets[[k, a]] = axes[a][rest % local_shape[a]];
            rest /= local_shape[a];
        }
    }
    Ok(Plan {
        first,
        glcm,
        texture,
        histogram,
        ivh,
        gldzm,
        local_shape,
        offsets,
        schedule,
    })
}

pub(crate) fn extract_scheduled(
    data: &Prepared,
    settings: &Settings,
    spacing: &[f64],
    ivh: Option<&IvhLattice>,
    roi_centers: Option<usize>,
    sink: VoxelSink<'_>,
) -> Result<Schedule> {
    let plan = plan(data, settings, spacing, roi_centers)?;
    let input = &data.input;
    let shape = input.image.shape().to_vec();
    let features = &settings.features;
    let select = |indices: &[usize]| -> Vec<&str> {
        indices.iter().map(|&i| features[i].as_str()).collect()
    };
    let (glcm_names, texture_names) = (select(&plan.glcm), select(&plan.texture));
    let (histogram_names, ivh_names) = (select(&plan.histogram), select(&plan.ivh));

    let built;
    let ivh = if ivh_names.is_empty() {
        None
    } else if let Some(lattice) = ivh {
        if lattice.bins.shape() != shape.as_slice() {
            return Err(invalid("IVH lattice must match the image shape"));
        }
        Some(lattice)
    } else {
        built = prepare_ivh(&input.image, &input.sampling_mask, settings)?;
        Some(&built)
    };

    let local = IxDyn(&plan.local_shape);
    let origin = ivh.map(|l| (l.origin, l.step, l.levels));
    let first_names = select(&plan.first);
    let has_windows = !glcm_names.is_empty()
        || !texture_names.is_empty()
        || !histogram_names.is_empty()
        || ivh.is_some();
    let fast_glcm = WindowGlcm::new(&plan.local_shape, data.gray_levels, settings, &plan.glcm);
    let fast_texture =
        WindowTexture::new(&plan.local_shape, data.gray_levels, settings, &plan.texture);
    let grid = NeighborhoodPlan::new(&shape)?;
    let sampling = input.sampling_mask.as_standard_layout();
    let bins = data
        .bins
        .as_ref()
        .filter(|_| {
            !glcm_names.is_empty() || !texture_names.is_empty() || !histogram_names.is_empty()
        })
        .map(|b| b.as_standard_layout());
    let ivh_bins = ivh.map(|l| l.bins.as_standard_layout());
    let morphology =
        (plan.gldzm && settings.masked_kernel).then(|| input.shape_roi.as_standard_layout());

    let roi = input.roi.as_standard_layout();
    let ndim = shape.len();
    let stride = shape[ndim - 1];
    let plane = stride * shape[ndim - 2];
    let mut positions = roi
        .as_slice()
        .unwrap()
        .chunks(256)
        .enumerate()
        .filter(|(_, block)| block.contains(&true))
        .flat_map(|(k, block)| {
            block
                .iter()
                .enumerate()
                .filter_map(move |(i, &v)| v.then_some(k * 256 + i))
        });
    let batch = plan.schedule.memory.batch_size;
    let compact_textures = fast_glcm.is_some()
        && (texture_names.is_empty() || fast_texture.is_some())
        && histogram_names.is_empty()
        && ivh.is_none();
    let threads = rayon::current_num_threads();
    let kernel = plan.offsets.nrows();
    // Both compact kernels cap the lattice at 128 levels and the window at 125 sites.
    // Their count, eigenvalue and run/zone tables fit in one MiB per worker;
    // another MiB covers shared direction and entropy tables. Rows reserve gathered
    // indices, coordinates, feature vectors and the per-column Vec headers.
    // General-purpose matrices retain the reference's single-batch schedule.
    let wave_rows = if compact_textures {
        let limit = plan.schedule.memory.scratch_estimate_bytes;
        let available = limit
            .saturating_sub(
                plan.schedule
                    .first
                    .map_or(0, |g| g.scratch_bytes.max(g.cache_bytes)),
            )
            .saturating_sub((threads + 1).saturating_mul(1 << 20));
        let rows = (available / (256 + 16 * kernel + 64 * features.len())).min(threads * 256);
        batch.max(rows / batch * batch)
    } else {
        batch
    };
    let compute_block = |centers: ArrayView2<'_, i32>, grouped_first: bool| -> Result<Vec<f64>> {
        let rows = centers.nrows();
        let mut columns = vec![0.; features.len() * rows];
        if !grouped_first && !first_names.is_empty() {
            let first = FirstOrderMapKernel::new(data, spacing, settings, &first_names)?
                .compute_batch(centers)?;
            for (&slot, column) in plan.first.iter().zip(first) {
                columns[slot * rows..(slot + 1) * rows].copy_from_slice(&column);
            }
        }
        if !has_windows {
            return Ok(columns);
        }
        let index = grid.index(centers, plan.offsets.view());
        let sampling = sampling.as_slice().unwrap();
        let bins = bins.as_ref().map(|b| b.as_slice().unwrap());
        let morphology = morphology.as_ref().map(|b| b.as_slice().unwrap());
        let ivh_bins = ivh_bins.as_ref().map(|b| b.as_slice().unwrap());
        let mut window = Prepared {
            input: PreparedInput {
                image: ArrayD::zeros(local.clone()),
                roi: ArrayD::from_elem(local.clone(), false),
                sampling_mask: ArrayD::from_elem(local.clone(), false),
                shape_roi: ArrayD::from_elem(local.clone(), false),
            },
            bins: bins.map(|_| ArrayD::zeros(local.clone())),
            gray_levels: data.gray_levels,
        };
        let mut lattice = origin.map(|(origin, step, levels)| IvhLattice {
            bins: ArrayD::zeros(local.clone()),
            origin,
            step,
            levels,
        });
        let mut counts = fast_glcm
            .as_ref()
            .map(|_| PairCounts::new(data.gray_levels as usize));
        let mut sizes = fast_texture.as_ref().map(WindowTexture::scratch);
        let mut values = vec![f64::NAN; features.len()];
        for row in 0..rows {
            values.fill(f64::NAN);
            let valid = window.input.roi.as_slice_mut().unwrap();
            let morph = window.input.shape_roi.as_slice_mut().unwrap();
            let mut window_bins = window.bins.as_mut().map(|b| b.as_slice_mut().unwrap());
            let mut window_ivh = lattice.as_mut().map(|l| l.bins.as_slice_mut().unwrap());
            let positions = index.flat.row(row);
            let inside = index.inside.row(row);
            for (k, (&i, &in_bounds)) in positions.iter().zip(inside).enumerate() {
                let i = i as usize;
                let sampled = in_bounds && sampling[i];
                valid[k] = sampled;
                morph[k] = in_bounds && morphology.map_or(sampling[i], |mask| mask[i]);
                if let (Some(levels), Some(out)) = (bins, &mut window_bins) {
                    let b = if sampled { levels[i] } else { 0 };
                    if sampled && (fast_glcm.is_some() || fast_texture.is_some()) && b <= 0 {
                        return Err(invalid(
                            "bins must be 1..Ng inside sampling_mask and 0 outside",
                        ));
                    }
                    out[k] = b;
                }
                if let (Some(levels), Some(out)) = (ivh_bins, &mut window_ivh) {
                    out[k] = if sampled { levels[i] } else { 0 };
                }
            }
            window.input.sampling_mask.assign(&window.input.roi);
            if let (Some(kernel), Some(counts)) = (&fast_glcm, &mut counts) {
                kernel.evaluate(
                    window.bins.as_ref().unwrap().as_slice().unwrap(),
                    counts,
                    &mut values,
                )?;
            }
            if let (Some(kernel), Some(matrix)) = (&fast_texture, &mut sizes) {
                kernel.evaluate(
                    window.bins.as_ref().unwrap().as_slice().unwrap(),
                    window.input.shape_roi.as_slice().unwrap(),
                    matrix,
                    &mut values,
                )?;
            }
            let mut place = |slots: &[usize], found: crate::Features| {
                for (&slot, (_, value)) in slots.iter().zip(found) {
                    values[slot] = value;
                }
            };
            if !glcm_names.is_empty() && fast_glcm.is_none() {
                place(
                    &plan.glcm,
                    local_glcm(&window, spacing, settings, &glcm_names)?,
                );
            }
            if !texture_names.is_empty() && fast_texture.is_none() {
                place(
                    &plan.texture,
                    local_texture(&window, spacing, settings, &texture_names)?,
                );
            }
            if !histogram_names.is_empty() {
                place(
                    &plan.histogram,
                    intensity_histogram::compute(&window, spacing, settings, &histogram_names)?,
                );
            }
            if let Some(lattice) = &lattice {
                place(
                    &plan.ivh,
                    ivh_statistics(lattice, &window.input.roi, &ivh_names)?,
                );
            }
            for group in [&plan.glcm, &plan.texture, &plan.histogram, &plan.ivh] {
                for &slot in group {
                    columns[slot * rows + row] = values[slot];
                }
            }
        }
        Ok(columns)
    };
    let group_size = plan.schedule.first.map_or(wave_rows, |group| {
        group.rows.max(wave_rows / group.rows * group.rows)
    });
    let first_kernel = plan
        .schedule
        .first
        .map(|_| FirstOrderMapKernel::new(data, spacing, settings, &first_names))
        .transpose()?;
    let mut cached = vec![Vec::<f32>::new(); plan.first.len()];
    loop {
        let mut coords = Vec::new();
        for i in positions.by_ref().take(group_size) {
            if ndim == 3 {
                coords.push((i / plane) as i32);
            }
            coords.push((i % plane / stride) as i32);
            coords.push((i % stride) as i32);
        }
        if coords.is_empty() {
            break;
        }
        let centers = Array2::from_shape_vec((coords.len() / ndim, ndim), coords).unwrap();
        for (wave_index, wave) in centers.axis_chunks_iter(Axis(0), wave_rows).enumerate() {
            let block_size = wave.nrows().div_ceil(threads * 3).clamp(1, 64);
            let ranges: Vec<_> = (0..wave.nrows())
                .step_by(batch)
                .flat_map(|start| {
                    let end = (start + batch).min(wave.nrows());
                    (start..end)
                        .step_by(block_size)
                        .map(move |lo| lo..(lo + block_size).min(end))
                })
                .collect();
            let results: Vec<_> = ranges
                .par_iter()
                .map(|range| {
                    compute_block(wave.slice(s![range.clone(), ..]), first_kernel.is_some())
                })
                .collect();
            let mut jobs = ranges.iter().zip(results);
            for (n, chunk) in wave.axis_chunks_iter(Axis(0), batch).enumerate() {
                if let (Some(first), Some(schedule)) = (&first_kernel, plan.schedule.first) {
                    let at = wave_index * wave_rows + n * batch;
                    let start = at % schedule.rows;
                    if start == 0 {
                        let group =
                            centers.slice(s![at..(at + schedule.rows).min(centers.nrows()), ..]);
                        cache_first_order(first, group, &mut cached)?;
                    }
                }
                let mut columns: Vec<_> = (0..features.len())
                    .map(|_| Vec::with_capacity(chunk.nrows()))
                    .collect();
                for (range, block) in jobs.by_ref().take(chunk.nrows().div_ceil(block_size)) {
                    for (column, source) in columns.iter_mut().zip(block?.chunks_exact(range.len()))
                    {
                        column.extend_from_slice(source);
                    }
                }
                if let Some(schedule) = plan.schedule.first {
                    let start = (wave_index * wave_rows + n * batch) % schedule.rows;
                    for (&slot, values) in plan.first.iter().zip(&cached) {
                        columns[slot].clear();
                        columns[slot].extend(
                            values[start..start + chunk.nrows()]
                                .iter()
                                .copied()
                                .map(f64::from),
                        );
                    }
                }
                if chunk.nrows() == centers.nrows() {
                    sink(&centers, &columns)?;
                } else {
                    sink(&chunk.to_owned(), &columns)?;
                }
            }
        }
    }
    Ok(plan.schedule)
}

/// Keep the float32 columns for one group until each delivery batch has consumed them.
fn cache_first_order(
    kernel: &FirstOrderMapKernel<'_>,
    centers: ArrayView2<'_, i32>,
    columns: &mut [Vec<f32>],
) -> Result<()> {
    for column in columns.iter_mut() {
        column.clear();
        column.reserve(centers.nrows());
    }
    let jobs: Vec<_> = (0..centers.nrows().div_ceil(64))
        .into_par_iter()
        .map(|block| {
            let start = block * 64;
            kernel
                .clone()
                .compute_batch(centers.slice(s![start..(start + 64).min(centers.nrows()), ..]))
        })
        .collect();
    for job in jobs {
        for (column, values) in columns.iter_mut().zip(job?) {
            column.extend(values.into_iter().map(|v| v as f32));
        }
    }
    Ok(())
}
