//! Array and file extraction over original and filtered CPU images.
//!
//! The original masks are prepared once. Each derived image gets its own grey-level
//! lattice. Shape is measured on the original image. ROI extraction runs on the
//! calling thread and voxel extraction delivers ordered batches from the CPU engine.

use std::path::Path;

use crate::contracts::metadata::file_metadata;
use crate::contracts::results::ExtractionResult;
use crate::contracts::types::ImageGeometryPayload;
use crate::filters::lbp3d::SourceDtype;
use crate::imaging::io::HostArray;
use crate::imaging::preprocessing::{
    discretize_prepared, prepare_derived_input, prepare_sampled_input,
};
use crate::imaging::spatial::{ImagePair, load_image_pair};
use crate::imaging::transforms::{compute_feature_images, feature_image_names};
use crate::workflows::derived::{lbp_dtype, prepare_kurtosis, validate_filter_source};
use crate::workflows::engine::{self, FeatureValue, ImageContext, Output};
use crate::{Error, ImageSettings, ImageType, Result, Settings, TransformSettings};
use indexmap::IndexMap;

/// Compute the configured original and filtered images. A shared sink receives
/// fully prefixed feature keys. A per-image factory can independently open and
/// finish each image's resources. Retained maps share one total byte allowance.
///
/// `image` and `mask` are aligned arrays with 2 or 3 axes in NumPy order. `mask` holds
/// integer labels and `settings.label` selects one. `spacing` is in millimetres per
/// array axis, and `None` means 1 mm on every axis. Result keys are
/// `<image>_<family>_<name>`, such as `original_firstorder_Mean`.
///
/// # Errors
/// [`Error::InvalidInput`] for mismatched or unsupported arrays, invalid settings or an
/// empty ROI; [`Error::Memory`] if retained maps would exceed their byte limit;
/// [`Error::Execution`] for numerical refusals or a failing sink.
///
/// # Examples
///
/// Retain one float64 map per feature in voxel mode:
///
/// ```
/// use radiomics::ndarray::ArrayD;
/// use radiomics::{extract, FeatureValue, HostArray, MapDtype, Output, Settings, TransformSettings};
///
/// # fn main() -> radiomics::Result<()> {
/// let image = HostArray::from(ArrayD::from_shape_fn(vec![5, 6, 6], |ix| (ix[1] * 2 + ix[2]) as i16));
/// let mask = HostArray::from(ArrayD::from_elem(vec![5, 6, 6], 1_u8));
/// let settings = Settings {
///     features: vec!["firstorder_Mean".into()],
///     kernel_radius: 1,
///     ..Settings::default()
/// };
/// let result = extract(
///     &image,
///     &mask,
///     &settings,
///     &TransformSettings::default(),
///     None,
///     Output::Maps { dtype: MapDtype::Float64, max_bytes: None },
/// )?;
/// match result.features.get("original_firstorder_Mean") {
///     Some(FeatureValue::Map64(map)) => assert_eq!(map.shape(), &[5, 6, 6]),
///     other => panic!("unexpected {other:?}"),
/// }
/// # Ok(())
/// # }
/// ```
pub fn extract(
    image: &HostArray,
    mask: &HostArray,
    settings: &Settings,
    transforms: &TransformSettings,
    spacing: Option<&[f64]>,
    mut output: Output<'_>,
) -> Result<ExtractionResult<FeatureValue>> {
    transforms.validate()?;
    let only_original = transforms.image_types.as_slice() == [ImageType::Original];
    let voxel = output.map_dtype().is_some();
    let has_intensity = settings.features.iter().any(|k| !k.starts_with("shape"));
    if only_original {
        let mut result = engine::extract(image, mask, settings, spacing, output)?;
        result.metadata.gray_levels_by_image = Some(if has_intensity {
            IndexMap::from([("original".into(), result.metadata.gray_levels)])
        } else {
            IndexMap::new()
        });
        result.metadata.transforms = Some(transforms.clone());
        return Ok(result);
    }
    let units = [1.; 3];
    let spacing = spacing.unwrap_or(&units[..image.ndim().min(3)]);
    let voxels = engine::checked_source(image.shape(), spacing)?;
    let (original, samples) = prepare_sampled_input(image, mask, settings, voxel)?;
    let lbp3 = transforms.image_types.contains(&ImageType::Lbp3D);
    if lbp3 && voxel && !settings.masked_kernel {
        return Err(Error::InvalidInput(
            "pinned LBP3D leaves outside-ROI values undefined; unmasked voxels unsupported".into(),
        ));
    }
    let (shape, intensity): (Vec<_>, Vec<_>) = settings
        .features
        .iter()
        .cloned()
        .partition(|n| n.starts_with("shape"));
    if voxel && !shape.is_empty() {
        return Err(Error::InvalidInput(
            "shape features are unavailable in voxel mode".into(),
        ));
    }
    if voxel && intensity.iter().any(|k| k.starts_with("localintensity_")) {
        return Err(Error::InvalidInput(
            "physical local-intensity peak features are unavailable in voxel mode".into(),
        ));
    }
    if !intensity.is_empty() {
        let names = feature_image_names(image.shape(), spacing, transforms, settings.force2d_axis)?;
        let outputs = names
            .len()
            .checked_mul(intensity.len())
            .ok_or_else(|| Error::Execution("feature map count overflow".into()))?;
        output.admit(voxels, outputs)?;
    }
    let mut features = IndexMap::new();
    let mut validity = IndexMap::new();
    let mut levels = IndexMap::new();
    let mut metadata = None;
    let mut append = |mut r: ExtractionResult<FeatureValue>, name: Option<&str>| {
        if let Some(name) = name {
            levels.insert(name.to_string(), r.metadata.gray_levels);
        }
        features.extend(r.features);
        validity.append(&mut r.metadata.feature_validity);
        metadata = Some(r.metadata);
    };
    let context = ImageContext {
        shape: image.shape(),
        dtype: image.dtype_name(),
        name: "original",
        undefined: None,
    };
    if !shape.is_empty() {
        let selected = Settings {
            features: shape,
            ..settings.clone()
        };
        let prepared = discretize_prepared(&original, &samples, &selected)?;
        append(
            engine::run_prepared(&prepared, &selected, spacing, context, Output::Roi)?,
            None,
        );
    }
    if !intensity.is_empty() {
        validate_filter_source(&original.image, &transforms.image_types)?;
        let dtype = if lbp3 {
            lbp_dtype(image, settings.normalize)?
        } else {
            SourceDtype::Float
        };
        let selected = Settings {
            features: intensity,
            ..settings.clone()
        };
        let derived_settings = Settings {
            normalize: false,
            remove_outliers: None,
            resegment_range: None,
            resegment_outlier_sigma: None,
            ..selected.clone()
        };
        // Most filters see float32 source pixels. LBP comparisons also read the original
        // residual words, represented exactly here by the source binary64 values.
        let rounded = transforms
            .image_types
            .iter()
            .any(|kind| {
                !matches!(
                    kind,
                    ImageType::Original | ImageType::Lbp2D | ImageType::Lbp3D
                )
            })
            .then(|| original.image.mapv(|x| f64::from(x as f32)));
        for &kind in &transforms.image_types {
            if kind == ImageType::Original {
                let prepared = discretize_prepared(&original, &samples, &selected)?;
                append(
                    engine::run_prepared(
                        &prepared,
                        &selected,
                        spacing,
                        context,
                        output.reborrow(),
                    )?,
                    Some("original"),
                );
                continue;
            }
            let options = TransformSettings {
                image_types: vec![kind],
                ..transforms.clone()
            };
            let source = if matches!(kind, ImageType::Lbp2D | ImageType::Lbp3D) {
                &original.image
            } else {
                rounded.as_ref().expect("rounded filter source")
            };
            for (name, mut image) in compute_feature_images::<true>(
                source,
                Some(&original.roi),
                spacing,
                &options,
                settings.force2d_axis,
                dtype,
            )? {
                image.mapv_inplace(|x| f64::from(x as f32));
                let (undefined, empty) = if name == "lbp-3D-k" {
                    prepare_kurtosis(&mut image, &original.roi, &selected, voxel)
                } else {
                    (None, false)
                };
                let (frame, values) = prepare_derived_input(&HostArray::F64(image), &original)?;
                let prepared = discretize_prepared(&frame, &values, &derived_settings)?;
                let frame = ImageContext {
                    name: &name,
                    undefined: undefined.as_ref(),
                    ..context
                };
                let mut result = engine::run_prepared(
                    &prepared,
                    &derived_settings,
                    spacing,
                    frame,
                    output.reborrow(),
                )?;
                if empty {
                    result.metadata.gray_levels = 0;
                }
                append(result, Some(&name));
            }
        }
    }
    let mut metadata = metadata.ok_or_else(|| {
        Error::InvalidInput("no requested feature image can produce selected features".into())
    })?;
    metadata.settings = settings.clone();
    metadata.transforms = Some(transforms.clone());
    metadata.gray_levels_by_image = Some(levels);
    metadata.feature_validity = validity;
    Ok(ExtractionResult { features, metadata })
}

/// Load, align and optionally resample a NIfTI or NRRD image/mask pair before
/// extraction. The result records both the source and processed physical grids.
///
/// `image_settings` controls resampling and mask correction; the default leaves the
/// source grid unchanged apart from cropping to the ROI with a pad of 5 voxels.
///
/// # Errors
/// As [`extract`], plus [`Error::InvalidInput`] if a file is missing, has an unsupported
/// suffix, or the image and mask grids disagree and `correct_mask` is off.
///
/// # Examples
///
/// ```
/// use radiomics::{extract_files, FeatureValue, ImageSettings, Output, Settings, TransformSettings};
///
/// # fn nrrd(kind: &str, values: &[i32]) -> String {
/// #     let body: Vec<String> = values.iter().map(|v| v.to_string()).collect();
/// #     format!(
/// #         "NRRD0004\ntype: {kind}\ndimension: 3\nsizes: 8 8 5\n\
/// #          space directions: (1,0,0) (0,1,0) (0,0,3)\nspace: left-posterior-superior\n\
/// #          encoding: ascii\n\n{}\n",
/// #         body.join(" ")
/// #     )
/// # }
/// # fn main() -> radiomics::Result<()> {
/// let dir = std::env::temp_dir().join(format!("radiomics-doc-files-{}", std::process::id()));
/// std::fs::create_dir_all(&dir)?;
/// let image_values: Vec<i32> = (0..320).map(|i| (i * 7 % 50) as i32).collect();
/// let mask_values: Vec<i32> = (0..320).map(|i| i32::from(i % 8 > 1 && i % 8 < 7 && (i / 8) % 8 > 1 && (i / 8) % 8 < 7)).collect();
/// std::fs::write(dir.join("image.nrrd"), nrrd("short", &image_values))?;
/// std::fs::write(dir.join("mask.nrrd"), nrrd("uchar", &mask_values))?;
///
/// let settings = Settings {
///     features: vec!["firstorder_Mean".into()],
///     bin_width: 5.0,
///     ..Settings::default()
/// };
/// let result = extract_files(
///     &dir.join("image.nrrd"),
///     &dir.join("mask.nrrd"),
///     &settings,
///     &ImageSettings::default(),
///     &TransformSettings::default(),
///     Output::Roi,
/// )?;
/// assert!(matches!(
///     result.features.get("original_firstorder_Mean"),
///     Some(FeatureValue::Scalar(_))
/// ));
/// # std::fs::remove_dir_all(&dir)?;
/// # Ok(())
/// # }
/// ```
pub fn extract_files(
    image_path: &Path,
    mask_path: &Path,
    settings: &Settings,
    image_settings: &ImageSettings,
    transforms: &TransformSettings,
    output: Output<'_>,
) -> Result<ExtractionResult<FeatureValue>> {
    let pair = load_image_pair(image_path, mask_path, Some(settings), Some(image_settings))?;
    extract_pair(&pair, settings, image_settings, transforms, output)
}

/// Extract an already spatially prepared file pair. Normalization has been applied
/// by the file loader before resampling, so it is skipped at this stage.
pub fn extract_pair(
    pair: &ImagePair,
    settings: &Settings,
    image_settings: &ImageSettings,
    transforms: &TransformSettings,
    output: Output<'_>,
) -> Result<ExtractionResult<FeatureValue>> {
    let effective = Settings {
        normalize: false,
        remove_outliers: None,
        ..settings.clone()
    };
    let mut result = extract(
        &pair.image.array,
        &pair.mask.array,
        &effective,
        transforms,
        Some(&pair.image.spacing),
        output,
    )?;
    let geometry = ImageGeometryPayload {
        shape: pair.image.array.shape().to_vec(),
        spacing: pair.image.spacing.clone(),
        origin: pair.image.origin.clone(),
        direction: pair.image.direction.clone(),
    };
    let source = pair.source_geometry.as_ref().map(|g| ImageGeometryPayload {
        shape: g.shape.clone(),
        spacing: g.spacing.clone(),
        origin: g.origin.clone(),
        direction: g.direction.clone(),
    });
    result.metadata = file_metadata(
        &result.metadata,
        geometry,
        Some(settings),
        source,
        Some(image_settings),
    );
    Ok(result)
}
