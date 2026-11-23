//! Validate the selected ROI count buffers before allocation on the CPU.
//!
//! Mirrors `mlxradiomics/workflows/roi_memory.py`. The scratch budget applies to
//! each selected primary table. The input shape is the cropped ROI plus its halo.

use crate::config::settings::{DirectionalAggregation as Mode, NeighborhoodSupport};
use crate::features::{aggregation::resolved_aggregation, selection::explicit_texture};
use crate::{Error, Family, Result, Settings};

fn admit(
    family: &str,
    cells: u128,
    bytes_per_cell: u128,
    index_bits: u32,
    allowance: u128,
) -> Result<()> {
    if cells >= 1u128 << index_bits {
        return Err(Error::Memory(format!(
            "{family} ROI count buffer exceeds {index_bits}-bit indexing capacity"
        )));
    }
    let bytes = cells * bytes_per_cell;
    if bytes > allowance {
        return Err(Error::Memory(format!(
            "{family} ROI count buffer requires {bytes} bytes, above the {allowance}-byte scratch budget"
        )));
    }
    Ok(())
}

// Count all positive and negative offsets at once. Chebyshev shells are differences
// of nested axis-aligned boxes clipped to the image and any forced slice axis.
fn dependence_columns(shape: &[usize], force: Option<usize>, settings: &Settings) -> u128 {
    let box_size = |r: u32| {
        shape.iter().enumerate().fold(1_u128, |count, (a, &n)| {
            let d = if force == Some(a) {
                0
            } else {
                (n - 1).min(r as usize)
            };
            count.saturating_mul(2 * d as u128 + 1)
        })
    };
    match settings.neighborhood_support {
        NeighborhoodSupport::ClosedRadius => box_size(settings.neighborhood_radius),
        _ => {
            1 + settings
                .distances
                .iter()
                .map(|&r| box_size(r) - box_size(r - 1))
                .sum::<u128>()
        }
    }
}

/// Check index widths and configured scratch capacity for count tables needed by
/// a selection. Sparse gray indices keep their original values.
pub fn check_roi_count_buffers(
    shape: &[usize],
    gray_levels: u32,
    settings: &Settings,
) -> Result<()> {
    if !(2..=3).contains(&shape.len())
        || shape.contains(&0)
        || gray_levels == 0
        || settings.distances.contains(&0)
        || settings.slice_axis >= shape.len()
        || settings.force2d_axis.is_some_and(|a| a >= shape.len())
    {
        return Err(Error::InvalidInput(
            "invalid ROI count-buffer domain".into(),
        ));
    }
    let ng = u128::from(gray_levels);
    let budget = u128::from(settings.scratch_mib) << 20;
    let names: Vec<_> = settings.features.iter().map(String::as_str).collect();
    let asks = |keys: &[&str]| keys.iter().any(|k| names.contains(k));
    if asks(&["firstorder_Entropy", "firstorder_Uniformity"]) {
        admit("firstorder", ng, 4, 31, budget)?;
    }
    if names.iter().any(|n| n.starts_with("intensityhistogram_")) {
        admit("intensityhistogram", ng, 8, 31, budget)?;
    }
    if ng > 512 {
        if asks(&[
            "glcm_Imc1",
            "glcm_Imc2",
            "glcm_Correlation",
            "glcm_DifferenceEntropy",
        ]) {
            admit("glcm", ng, 4, 31, budget)?;
        }
        if asks(&["glcm_SumEntropy", "glcm_ClusterShade"]) {
            admit("glcm", 2 * ng - 1, 4, 31, budget)?;
        }
    }
    let explicit = explicit_texture(settings, &names);
    for family in [Family::Glrlm, Family::Gldm, Family::Ngtdm] {
        let name = family.as_str();
        if !names
            .iter()
            .any(|key| key.split_once('_').is_some_and(|(p, _)| p == name))
        {
            continue;
        }
        let slice = if explicit
            && shape.len() == 3
            && !matches!(
                resolved_aggregation(family, settings, 3)?,
                Mode::Average3D | Mode::Merge3D
            ) {
            Some(settings.force2d_axis.unwrap_or(settings.slice_axis))
        } else {
            None
        };
        let plane: Vec<_> = shape
            .iter()
            .enumerate()
            .filter_map(|(a, &n)| (Some(a) != slice).then_some(n))
            .collect();
        let force = if slice.is_some() {
            None
        } else {
            settings.force2d_axis
        };
        match family {
            Family::Glrlm => admit(
                name,
                ng * *plane.iter().max().unwrap() as u128,
                4,
                32,
                budget,
            )?,
            Family::Gldm => admit(
                name,
                ng.saturating_mul(dependence_columns(&plane, force, settings)),
                4,
                32,
                budget,
            )?,
            Family::Ngtdm => {
                let width = plane
                    .iter()
                    .fold(1_u128, |size, &n| size.saturating_mul(n as u128));
                if !explicit && width > 64 {
                    admit(name, ng, 4, 31, budget)?;
                } else {
                    let chunks = width.div_ceil(2048).min(width / ng).max(1);
                    admit(
                        name,
                        ng.saturating_mul(chunks),
                        4,
                        32,
                        budget.max(width.saturating_mul(4)),
                    )?;
                    admit(name, ng, 8, 31, budget)?;
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
