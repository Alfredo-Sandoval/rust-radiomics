//! ROI texture extraction, mirroring `mlxradiomics/features/texture.py`.
//! Status: ported. CPU orchestration on a prepared gray-level grid; families run in parallel.
//! Native and explicit IBSI paths share their CPU matrix builders and statistics.

use crate::config::settings::{Family, Settings, features};
use crate::features::aggregation::aggregated_texture_statistics;
use crate::features::matrices::Domain;
use crate::features::selection::explicit_texture;
use crate::{Error, Features, Prepared, Result};
use rayon::prelude::*;

const FAMILIES: [Family; 6] = [
    Family::Glcm,
    Family::Glrlm,
    Family::Glszm,
    Family::Gldzm,
    Family::Gldm,
    Family::Ngtdm,
];

/// Evaluate the selected ROI texture features in input order. Distances and
/// directions are in voxels. Spacing follows the NumPy array axis order.
pub fn compute(
    prepared: &Prepared,
    spacing: &[f64],
    settings: &Settings,
    names: &[&str],
) -> Result<Features> {
    if names.is_empty() {
        return Ok(Vec::new());
    }
    let bins = prepared
        .bins
        .as_ref()
        .ok_or_else(|| Error::InvalidInput("texture features require discretized bins".into()))?;
    let ndim = bins.ndim();
    if !(2..=3).contains(&ndim) || bins.is_empty() {
        return Err(Error::InvalidInput(
            "texture bins must be a nonempty 2D or 3D array".into(),
        ));
    }
    if spacing.len() != ndim || spacing.iter().any(|&s| !s.is_finite() || s <= 0.) {
        return Err(Error::InvalidInput(
            "spacing must contain one finite positive value per array axis".into(),
        ));
    }
    if bins.shape() != prepared.input.sampling_mask.shape()
        || bins.shape() != prepared.input.shape_roi.shape()
    {
        return Err(Error::InvalidInput(
            "bins and masks must share a shape".into(),
        ));
    }
    let invalid = |&b: &i32, &masked: &bool| {
        if masked {
            b <= 0 || b as u32 > prepared.gray_levels
        } else {
            b != 0
        }
    };
    let bad = match (bins.as_slice(), prepared.input.sampling_mask.as_slice()) {
        (Some(b), Some(m)) => b
            .par_chunks(1 << 16)
            .zip(m.par_chunks(1 << 16))
            .any(|(b, m)| b.iter().zip(m).any(|(b, m)| invalid(b, m))),
        _ => bins
            .iter()
            .zip(&prepared.input.sampling_mask)
            .any(|(b, m)| invalid(b, m)),
    };
    if bad {
        return Err(Error::InvalidInput(
            "bins must be 1..Ng inside sampling_mask and 0 outside".into(),
        ));
    }
    if settings.force2d_axis.is_some_and(|i| i >= ndim) {
        return Err(Error::InvalidInput(
            "force2d_axis must be an integer indexing a NumPy image axis".into(),
        ));
    }
    if settings.slice_axis >= ndim {
        return Err(Error::InvalidInput(
            "slice_axis must be an integer indexing an image axis".into(),
        ));
    }
    if settings.neighborhood_radius == 0 {
        return Err(Error::InvalidInput(
            "neighborhood_radius must be a positive integer".into(),
        ));
    }
    if settings.scratch_mib == 0 {
        return Err(Error::InvalidInput(
            "scratch_mib must be a positive integer".into(),
        ));
    }
    if settings.distances.is_empty() || settings.distances.contains(&0) {
        return Err(Error::InvalidInput(
            "distances must contain positive integers".into(),
        ));
    }
    if settings
        .distances
        .iter()
        .enumerate()
        .any(|(i, v)| settings.distances[..i].contains(v))
    {
        return Err(Error::InvalidInput("distances must be unique".into()));
    }
    let mut selected = Vec::with_capacity(names.len());
    for &key in names {
        let spec = key
            .split_once('_')
            .and_then(|(prefix, name)| {
                features().iter().find(|s| {
                    s.family.as_str() == prefix && s.name == name && FAMILIES.contains(&s.family)
                })
            })
            .ok_or_else(|| Error::InvalidInput(format!("unsupported texture feature: {key}")))?;
        selected.push(spec);
    }
    let domain = Domain::new(bins.view(), prepared.input.shape_roi.view());
    let explicit = explicit_texture(settings, names);
    let mut result: Features = names
        .iter()
        .map(|&key| (key.to_owned(), f64::NAN))
        .collect();
    let mut groups = Vec::new();
    for family in FAMILIES {
        let mut indices = Vec::new();
        let mut statistics = Vec::new();
        for (i, spec) in selected
            .iter()
            .enumerate()
            .filter(|(_, s)| s.family == family)
        {
            indices.push(i);
            statistics.push(spec.name);
        }
        if !indices.is_empty() {
            groups.push((family, indices, statistics));
        }
    }
    // Families are independent and share the scratch budget. Results are written back in
    // family order.
    let share = ((settings.scratch_mib as usize) << 20) / groups.len().max(1);
    let outputs: Vec<Result<Vec<f64>>> = groups
        .par_iter()
        .map(|(family, _, statistics)| {
            aggregated_texture_statistics(
                domain,
                prepared.gray_levels,
                settings,
                *family,
                statistics,
                explicit,
                share,
            )
        })
        .collect();
    for ((_, indices, _), values) in groups.iter().zip(outputs) {
        for (&i, x) in indices.iter().zip(values?) {
            result[i].1 = x;
        }
    }
    Ok(result)
}
