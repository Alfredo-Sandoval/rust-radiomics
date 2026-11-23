//! IBSI matrix aggregation, mirroring `mlxradiomics/features/aggregation.py`.
//! Status: ported. CPU orchestration over domain views, integer counts and feature means.
//! Slices, directions and per-matrix statistics run on the Rayon pool when the lattice is
//! small; means are accumulated in fixed slice and direction order, so results do not
//! depend on the thread count.

use crate::config::settings::{
    DirectionalAggregation as Mode, Family, MatrixAggregation, Settings,
};
use crate::features::matrices::{
    DENSE_LEVELS, Domain, Scratch, TextureMatrix, matrices, matrices_into, table_cells, task_bytes,
};
use crate::features::{distributions, glcm};
use crate::{Error, Result};
use rayon::prelude::*;

pub(crate) fn resolved_aggregation(
    family: Family,
    settings: &Settings,
    ndim: usize,
) -> Result<Mode> {
    let mode = match family {
        Family::Glcm => settings.glcm_aggregation,
        Family::Glrlm => settings.glrlm_aggregation,
        _ => {
            let choice = match family {
                Family::Glszm => settings.glszm_aggregation,
                Family::Gldzm => settings.gldzm_aggregation,
                Family::Gldm => settings.gldm_aggregation,
                Family::Ngtdm => settings.ngtdm_aggregation,
                _ => unreachable!("selection restricts texture families"),
            };
            match choice {
                MatrixAggregation::Native => Mode::Native,
                MatrixAggregation::Average2D => Mode::Average2D,
                MatrixAggregation::Merge25D => Mode::Merge25D,
                MatrixAggregation::ThreeD => Mode::Average3D,
            }
        }
    };
    if matches!(mode, Mode::Average3D | Mode::Merge3D)
        && (ndim != 3 || settings.force2d_axis.is_some())
    {
        return Err(Error::InvalidInput(
            "3d aggregation requires 3D input without force2d_axis".into(),
        ));
    }
    Ok(match mode {
        Mode::Native if ndim == 2 => Mode::Average2D,
        Mode::Native if settings.force2d_axis.is_none() => Mode::Average3D,
        Mode::Native if matches!(family, Family::Glcm | Family::Glrlm) => Mode::DirectionMerge25D,
        Mode::Native => Mode::Merge25D,
        other => other,
    })
}

/// One feature's average over active matrices. MCC counts only defined spectra.
#[derive(Default, Clone)]
struct Mean {
    sum: f64,
    error: f64,
    count: u64,
}

impl Mean {
    fn add(&mut self, x: f64) {
        let total = self.sum + x;
        if total.is_finite() {
            let back = total - self.sum;
            self.error += (self.sum - (total - back)) + (x - back);
        }
        self.sum = total;
        self.count += 1;
    }
    fn value(self) -> f64 {
        (self.sum + self.error) / self.count as f64
    }
}

/// Statistics of one matrix, or `None` when it is empty.
fn evaluate(
    matrix: &TextureMatrix,
    ng: u32,
    settings: &Settings,
    family: Family,
    names: &[&str],
) -> Result<Option<Vec<f64>>> {
    if matrix.counts.is_empty() {
        return Ok(None);
    }
    Ok(Some(match family {
        Family::Glcm => glcm::glcm_statistics(&matrix.counts, ng, names, settings.scratch_mib)?,
        Family::Ngtdm => {
            distributions::ngtdm_statistics(matrix, names, settings.ngtdm_singular_cases)?
        }
        _ => distributions::size_statistics(matrix, names)?,
    }))
}

pub(super) fn aggregated_texture_statistics(
    domain: Domain<'_>,
    ng: u32,
    settings: &Settings,
    family: Family,
    names: &[&str],
    explicit: bool,
    budget: usize,
) -> Result<Vec<f64>> {
    let mut means = vec![Mean::default(); names.len()];
    let mut add = |values: Option<Vec<f64>>| {
        for ((mean, name), value) in means
            .iter_mut()
            .zip(names)
            .zip(values.into_iter().flatten())
        {
            if *name != "MCC" || !value.is_nan() {
                mean.add(value);
            }
        }
    };
    let eval = |matrix: &TextureMatrix| evaluate(matrix, ng, settings, family, names);
    // Native performs all counting in one grid, with force2D applied to offsets.
    let mode = if explicit {
        resolved_aggregation(family, settings, domain.ndim)?
    } else {
        Mode::Native
    };
    let planar =
        domain.ndim == 3 && !matches!(mode, Mode::Native | Mode::Average3D | Mode::Merge3D);
    let axis = settings.force2d_axis.unwrap_or(settings.slice_axis);
    let slices = if planar { domain.shape()[axis] } else { 1 };
    let first = if planar {
        domain.plane(axis, 0)
    } else {
        domain
    };
    let force = if planar { None } else { settings.force2d_axis };
    // Count tables live at once: every task of a wave holds a table and the matrices of its
    // slice. Zone families are sparse and unbounded; lattices above DENSE_LEVELS run serially.
    let (cells, directions) = table_cells(first, ng, family, settings, force);
    let voxels = first.bins.len();
    let task = task_bytes(cells, voxels).max(1);
    let slots = if cells == 0 {
        usize::MAX
    } else if ng > DENSE_LEVELS {
        1
    } else {
        (budget / task).max(1)
    };
    // Merging modes fold each direction into one accumulator per slice as it is produced.
    let streaming = matches!(mode, Mode::SliceMerge2D | Mode::Merge25D | Mode::Merge3D);
    let slices_at_once = if cells == 0 {
        slices
    } else if streaming {
        // One merged map per slice: up to every cell of the table, at most all voxel pairs.
        let map = 40 * cells.min(2 * voxels * directions.max(1));
        (budget / (task + map)).clamp(1, slices)
    } else {
        (slots / directions.max(1)).clamp(1, slices)
    };
    let width = if cells == 0 {
        usize::MAX
    } else {
        (slots / slices_at_once).max(1)
    };
    let scratch = Scratch {
        budget,
        slots: if cells == 0 {
            1
        } else {
            width.min(directions.max(1))
        },
    };
    let evaluate_all = |matrices: &[TextureMatrix]| -> Result<Vec<Option<Vec<f64>>>> {
        let mut values = Vec::with_capacity(matrices.len());
        for group in matrices.chunks(if ng <= DENSE_LEVELS { width.max(1) } else { 1 }) {
            if group.len() > 1 {
                values.extend(group.par_iter().map(eval).collect::<Result<Vec<_>>>()?);
            } else {
                for matrix in group {
                    values.push(eval(matrix)?);
                }
            }
        }
        Ok(values)
    };
    // Each slice is independent; results are folded in slice and direction order.
    let slice_matrices = |index: usize| -> Result<Vec<TextureMatrix>> {
        let (local, force_axis) = if planar {
            (domain.plane(axis, index), None)
        } else {
            (domain, settings.force2d_axis)
        };
        matrices(local, ng, family, settings, force_axis, scratch)
    };
    let waves = (0..slices).step_by(slices_at_once.max(1));
    let run_wave = |start: usize,
                    work: &(dyn Fn(usize) -> Result<Vec<Option<Vec<f64>>>> + Sync)|
     -> Result<Vec<Vec<Option<Vec<f64>>>>> {
        let range = start..(start + slices_at_once).min(slices);
        if range.len() > 1 {
            range.into_par_iter().map(work).collect()
        } else {
            range.map(work).collect()
        }
    };
    let collect_wave = |start: usize| -> Result<Vec<Vec<TextureMatrix>>> {
        let range = start..(start + slices_at_once).min(slices);
        if range.len() > 1 {
            range.into_par_iter().map(slice_matrices).collect()
        } else {
            range.map(slice_matrices).collect()
        }
    };
    let slice_merged = |index: usize| -> Result<TextureMatrix> {
        let (local, force_axis) = if planar {
            (domain.plane(axis, index), None)
        } else {
            (domain, settings.force2d_axis)
        };
        let mut merged = TextureMatrix::default();
        matrices_into(local, ng, family, settings, force_axis, scratch, |m| {
            merged.merge(m)
        })?;
        Ok(merged)
    };
    let merged_wave = |start: usize| -> Result<Vec<TextureMatrix>> {
        let range = start..(start + slices_at_once).min(slices);
        if range.len() > 1 {
            range.into_par_iter().map(slice_merged).collect()
        } else {
            range.map(slice_merged).collect()
        }
    };
    match mode {
        Mode::Native | Mode::Average2D | Mode::Average3D => {
            for start in waves {
                let done = run_wave(start, &|index| evaluate_all(&slice_matrices(index)?))?;
                done.into_iter().flatten().for_each(&mut add);
            }
        }
        Mode::SliceMerge2D => {
            for start in waves {
                let done = run_wave(start, &|index| Ok(vec![eval(&slice_merged(index)?)?]))?;
                done.into_iter().flatten().for_each(&mut add);
            }
        }
        Mode::DirectionMerge25D => {
            let mut by_direction: Vec<TextureMatrix> = Vec::new();
            let mut fold = |direction: usize, matrix: TextureMatrix| {
                if direction == by_direction.len() {
                    by_direction.push(matrix);
                } else {
                    by_direction[direction].merge(matrix);
                }
            };
            if slices_at_once == 1 {
                // One map per direction for the whole extraction, no per-slice copy.
                for index in 0..slices {
                    let (local, force_axis) = if planar {
                        (domain.plane(axis, index), None)
                    } else {
                        (domain, settings.force2d_axis)
                    };
                    let mut direction = 0;
                    matrices_into(local, ng, family, settings, force_axis, scratch, |m| {
                        fold(direction, m);
                        direction += 1;
                    })?;
                }
            } else {
                for start in waves {
                    for slice in collect_wave(start)? {
                        for (direction, matrix) in slice.into_iter().enumerate() {
                            fold(direction, matrix);
                        }
                    }
                }
            }
            evaluate_all(&by_direction)?.into_iter().for_each(&mut add);
        }
        Mode::Merge25D | Mode::Merge3D => {
            let mut merged = TextureMatrix::default();
            if slices_at_once == 1 {
                // One accumulator for the whole extraction; no per-slice copy.
                for index in 0..slices {
                    let (local, force_axis) = if planar {
                        (domain.plane(axis, index), None)
                    } else {
                        (domain, settings.force2d_axis)
                    };
                    matrices_into(local, ng, family, settings, force_axis, scratch, |m| {
                        merged.merge(m)
                    })?;
                }
            } else {
                for start in waves {
                    for part in merged_wave(start)? {
                        merged.merge(part);
                    }
                }
            }
            add(eval(&merged)?);
        }
    }
    Ok(means.into_iter().map(Mean::value).collect())
}
