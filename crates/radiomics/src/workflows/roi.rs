//! CPU ROI dispatch for prepared 2D and 3D images.
//!
//! Mirrors `mlxradiomics/workflows/roi.py`. Spacing follows the image axes in mm.
//! Texture families share one padded bounding box. Shape, moment invariants and local
//! intensity retain the full image so their masks and physical neighborhoods keep their original support.

pub(super) mod bounds;

use std::collections::HashMap;
use std::ops::Range;

use crate::features::{
    first_order, intensity_histogram, ivh, local_intensity, moment_invariants, morphology, shape,
    texture,
};
use crate::workflows::roi::bounds::include;
use crate::{Error, Family, Features, Prepared, PreparedInput, Result, Settings};
use ndarray::{ArrayD, Slice};

/// Index ranges containing the union of the sampling and morphology masks, plus
/// the requested symmetric halo. The ranges use original voxel coordinates.
fn occupied_window(data: &Prepared, reach: usize) -> Result<Vec<Range<usize>>> {
    let sampling = &data.input.sampling_mask;
    let mut lower = sampling.shape().to_vec();
    let mut upper = vec![0; lower.len()];
    include(sampling, true, &mut lower, &mut upper);
    include(&data.input.shape_roi, true, &mut lower, &mut upper);
    if upper.contains(&0) {
        return Err(Error::InvalidInput(
            "empty texture sampling and shape masks".into(),
        ));
    }
    Ok(lower
        .into_iter()
        .zip(upper)
        .zip(sampling.shape())
        .map(|((lo, hi), &extent)| lo.saturating_sub(reach)..hi.saturating_add(reach).min(extent))
        .collect())
}

fn cropped<T: Clone>(data: &ArrayD<T>, window: &[Range<usize>]) -> ArrayD<T> {
    data.slice_each_axis(|a| Slice::from(window[a.axis.index()].clone()))
        .to_owned()
}

fn texture_region(data: &Prepared, window: &[Range<usize>]) -> Option<Prepared> {
    if window
        .iter()
        .zip(data.input.image.shape())
        .all(|(r, &n)| r.start == 0 && r.end == n)
    {
        return None;
    }
    Some(Prepared {
        input: PreparedInput {
            image: cropped(&data.input.image, window),
            roi: cropped(&data.input.roi, window),
            sampling_mask: cropped(&data.input.sampling_mask, window),
            shape_roi: cropped(&data.input.shape_roi, window),
        },
        bins: data.bins.as_ref().map(|b| cropped(b, window)),
        gray_levels: data.gray_levels,
    })
}

/// Compute all selected feature families for one prepared image. The results keep
/// the order in `settings.features` and use full `<family>_<feature>` keys. An optional
/// precomputed IVH lattice is used only when IVH is requested.
pub fn compute_roi(
    data: &Prepared,
    settings: &Settings,
    spacing: &[f64],
    lattice: Option<&ivh::IvhLattice>,
) -> Result<Features> {
    let input = &data.input;
    let grid = input.image.shape();
    if !(2..=3).contains(&grid.len())
        || grid.contains(&0)
        || input.roi.shape() != grid
        || input.sampling_mask.shape() != grid
        || input.shape_roi.shape() != grid
        || data.bins.as_ref().is_some_and(|b| b.shape() != grid)
        || spacing.len() != grid.len()
        || spacing.iter().any(|v| !v.is_finite() || *v <= 0.)
    {
        return Err(Error::InvalidInput("ROI extraction requires aligned, nonempty 2D or 3D arrays and one positive finite spacing per axis".into()));
    }
    settings.validate(grid.len())?;
    if !input.roi.iter().any(|&inside| inside) {
        return Err(Error::InvalidInput(
            "the ROI contains no selected voxels".into(),
        ));
    }
    let mut first = Vec::new();
    let mut histogram = Vec::new();
    let mut volume_histogram = Vec::new();
    let mut local = Vec::new();
    let mut shapes = Vec::new();
    let mut intensity_morphology = Vec::new();
    let mut invariants = Vec::new();
    let mut textures = Vec::new();
    for key in &settings.features {
        let (prefix, name) = key.split_once('_').expect("validated feature");
        let group = match Family::parse(prefix).expect("validated family") {
            Family::FirstOrder => &mut first,
            Family::IntensityHistogram => &mut histogram,
            Family::Ivh => &mut volume_histogram,
            Family::LocalIntensity => &mut local,
            Family::Shape if morphology::INTENSITY_MORPHOLOGY.contains(&name) => {
                &mut intensity_morphology
            }
            Family::Shape | Family::Shape2D => &mut shapes,
            Family::MomentInvariant => &mut invariants,
            Family::Glcm
            | Family::Glrlm
            | Family::Glszm
            | Family::Gldzm
            | Family::Gldm
            | Family::Ngtdm => &mut textures,
        };
        group.push(key.as_str());
    }
    let window = if textures.is_empty() {
        None
    } else {
        let reach = settings
            .distances
            .iter()
            .copied()
            .max()
            .unwrap_or(0)
            .max(settings.neighborhood_radius);
        Some(occupied_window(data, reach as usize)?)
    };
    let domain: Vec<_> = window
        .as_ref()
        .map_or_else(|| grid.to_vec(), |w| w.iter().map(Range::len).collect());
    crate::workflows::roi_memory::check_roi_count_buffers(&domain, data.gray_levels, settings)?;
    type Compute = fn(&Prepared, &[f64], &Settings, &[&str]) -> Result<Features>;
    let families: [(Compute, &[&str]); 6] = [
        (local_intensity::compute, &local),
        (first_order::compute, &first),
        (intensity_histogram::compute, &histogram),
        (shape::compute, &shapes),
        (morphology::compute, &intensity_morphology),
        (moment_invariants::compute, &invariants),
    ];
    let mut results = HashMap::with_capacity(settings.features.len());
    for (compute, names) in families {
        if !names.is_empty() {
            results.extend(compute(data, spacing, settings, names)?);
        }
    }
    if !volume_histogram.is_empty() {
        let owned;
        let lattice = match lattice {
            Some(existing) => existing,
            None => {
                owned = ivh::prepare_ivh(&input.image, &input.roi, settings)?;
                &owned
            }
        };
        results.extend(ivh::ivh_statistics(lattice, &input.roi, &volume_histogram)?);
    }
    if !textures.is_empty() {
        let crop = texture_region(data, window.as_deref().expect("texture domain"));
        results.extend(texture::compute(
            crop.as_ref().unwrap_or(data),
            spacing,
            settings,
            &textures,
        )?);
    }
    settings
        .features
        .iter()
        .map(|key| {
            results.remove_entry(key).ok_or_else(|| {
                Error::Execution(format!("ROI extraction omitted requested feature {key}"))
            })
        })
        .collect()
}
