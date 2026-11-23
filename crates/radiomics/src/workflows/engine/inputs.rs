//! Array admission and ROI windowing for the native CPU engine.

use ndarray::{ArrayD, Slice};

use crate::contracts::results::ExtractionResult;
use crate::imaging::io::{ArrayKind, HostArray};
use crate::imaging::preprocessing::{prepare, validate_exact_float64};
use crate::{Error, Result, Settings};

use crate::workflows::engine::{FeatureValue, Output, checked_source, run_prepared_at};

/// Compute original-image features from aligned 2D or 3D host arrays. Spacing
/// defaults to one millimetre per axis. All work runs natively on the CPU.
pub fn extract(
    image: &HostArray,
    mask: &HostArray,
    settings: &Settings,
    spacing: Option<&[f64]>,
    output: Output<'_>,
) -> Result<ExtractionResult<FeatureValue>> {
    extract_with_shape_mask(image, mask, None, settings, spacing, output)
}

/// As [`extract`], with an independent optional morphology mask. The selected
/// intensity label still determines the presence of an ROI and its voxel count.
pub fn extract_with_shape_mask(
    image: &HostArray,
    mask: &HostArray,
    shape_mask: Option<&ArrayD<bool>>,
    settings: &Settings,
    spacing: Option<&[f64]>,
    output: Output<'_>,
) -> Result<ExtractionResult<FeatureValue>> {
    let shape = image.shape();
    let unit = [1.; 3];
    let spacing = spacing.unwrap_or(&unit[..shape.len().min(3)]);
    let voxels = checked_source(shape, spacing)?;
    if mask.shape() != shape {
        return Err(Error::InvalidInput(
            "image and mask must share a nonempty 2D or 3D array shape".into(),
        ));
    }
    if image.kind() == ArrayKind::Bool || mask.kind() == ArrayKind::Float {
        return Err(Error::InvalidInput(
            "image must be real numeric and mask must contain integer labels".into(),
        ));
    }
    if shape_mask.is_some_and(|m| m.shape() != shape) {
        return Err(Error::InvalidInput(
            "shape_mask must match the image shape".into(),
        ));
    }
    settings.validate(shape.len())?;
    let voxel = output.map_dtype().is_some();
    if voxel {
        if settings.features.iter().any(|k| k.starts_with("shape")) {
            return Err(Error::InvalidInput(
                "shape features are unavailable in voxel mode".into(),
            ));
        }
        if settings
            .features
            .iter()
            .any(|k| k.starts_with("localintensity_"))
        {
            return Err(Error::InvalidInput(
                "physical local-intensity peak features are unavailable in voxel mode".into(),
            ));
        }
    }
    output.admit(voxels, settings.features.len())?;
    let window = if settings.normalize || (voxel && !settings.masked_kernel) {
        None
    } else {
        input_window(mask, shape_mask, settings, spacing, voxel)
    };
    let mut prepared = if let Some((lo, hi)) = &window {
        validate_exact_float64(image)?;
        prepare(&image.crop(lo, hi), &mask.crop(lo, hi), settings, voxel)?
    } else {
        prepare(image, mask, settings, voxel)?
    };
    if let Some(m) = shape_mask {
        prepared.input.shape_roi = match &window {
            None => m.clone(),
            Some((lo, hi)) => m
                .slice_each_axis(|a| {
                    let i = a.axis.index();
                    Slice::from(lo[i]..hi[i])
                })
                .to_owned(),
        };
    }
    run_prepared_at(
        &prepared,
        settings,
        spacing,
        crate::workflows::engine::ImageContext::original(shape, image.dtype_name()),
        output,
        window
            .as_ref()
            .filter(|_| voxel)
            .map(|(lo, _)| lo.as_slice()),
    )
}

fn input_window(
    mask: &HostArray,
    shape_mask: Option<&ArrayD<bool>>,
    settings: &Settings,
    spacing: &[f64],
    voxel: bool,
) -> Option<(Vec<usize>, Vec<usize>)> {
    let shape = mask.shape();
    let mut lo = shape.to_vec();
    let mut hi = vec![0usize; shape.len()];
    use crate::workflows::roi::bounds::include;
    macro_rules! labels {
        ($array:expr) => {
            if let Ok(label) = settings.label.try_into() {
                include($array, label, &mut lo, &mut hi);
            }
        };
    }
    match mask {
        HostArray::Bool(a) => {
            if matches!(settings.label, 0 | 1) {
                include(a, settings.label == 1, &mut lo, &mut hi);
            }
        }
        HostArray::I8(a) => labels!(a),
        HostArray::U8(a) => labels!(a),
        HostArray::I16(a) => labels!(a),
        HostArray::U16(a) => labels!(a),
        HostArray::I32(a) => labels!(a),
        HostArray::U32(a) => labels!(a),
        HostArray::I64(a) => include(a, settings.label, &mut lo, &mut hi),
        HostArray::U64(a) => labels!(a),
        _ => unreachable!("validated integer mask"),
    }
    if !voxel && let Some(extra) = shape_mask {
        include(extra, true, &mut lo, &mut hi);
    }
    if hi.contains(&0) {
        return None;
    }
    let texture = settings
        .distances
        .iter()
        .copied()
        .max()
        .unwrap_or(0)
        .max(settings.neighborhood_radius) as usize;
    let sphere = settings
        .features
        .iter()
        .any(|k| k.starts_with("localintensity_"));
    let radius = (3000.0 / (4.0 * std::f64::consts::PI)).cbrt();
    for a in 0..shape.len() {
        let reach = if voxel {
            if settings.force2d_axis == Some(a) {
                0
            } else {
                settings.kernel_radius as usize
            }
        } else if sphere {
            texture.max((radius / spacing[a]).floor() as usize)
        } else {
            texture
        };
        lo[a] = lo[a].saturating_sub(reach);
        hi[a] = hi[a].saturating_add(reach).min(shape[a]);
    }
    (lo.iter().any(|&i| i != 0) || hi != shape).then_some((lo, hi))
}
