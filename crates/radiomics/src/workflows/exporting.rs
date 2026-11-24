//! Export of voxel feature maps for original and derived images.
//!
//! Mirrors `mlxradiomics/workflows/exporting.py` (167 lines at mlx-radiomics b58783c).
//! Status: ported (`export`, `export_files`).
//!
//! CPU orchestration on the calling thread around the CPU extraction of
//! [`crate::workflows::extraction::extract`] (float64 arithmetic; first-order voxel maps are
//! evaluated in float32 by `first_order::compute_maps`; texture batches use Rayon). Each
//! selected image type is extracted in turn, one float32 map at a time is written under
//! `<output_dir>/maps` through private staging, and `features.json` (schema 2) is
//! published atomically by [`crate::workflows::reporting::publish_report`]. The execution metadata in
//! the report records `"device": "cpu"` and `"precision": "float64"`.
//!
//! Differences from mlx, besides those of [`crate::workflows::reporting`] and
//! [`crate::workflows::publication`]:
//!
//! - `warnings.warn` has no counterpart in this crate, so the UserWarning that array
//!   [`export`] emits when `allow_synthetic_geometry` is false is not emitted. The flag is
//!   still part of [`ExportOptions`]; the maps always carry a zero origin and identity
//!   direction.
//! - Type errors that Python raises at run time (`overwrite` not a bool, a `mode` that is
//!   neither `'roi'` nor `'voxel'`) cannot be expressed through these typed arguments.

use crate::contracts::results::MapExport;
use crate::contracts::types::ImageGeometryPayload;
use crate::imaging::io::{HostArray, Volume};
use crate::imaging::spatial::load_image_pair;
use crate::workflows::reporting::{
    ExportProvenance, ReportRequest, publish_report_with_provenance,
};
use crate::{Error, ImageSettings, Result, Settings, TransformSettings};
use std::path::{Path, PathBuf};

/// Keyword arguments of array [`export`].
#[derive(Clone, Debug, Default)]
pub struct ExportOptions {
    /// Feature selection and preprocessing; `Settings::default()` if absent.
    pub settings: Option<Settings>,
    /// Derived images to export; the original image only if absent.
    pub transforms: Option<TransformSettings>,
    /// Positive millimetres per array axis; unit spacing if absent or empty.
    pub spacing: Option<Vec<f64>>,
    /// Acknowledge zero origin and identity direction in the saved maps.
    pub allow_synthetic_geometry: bool,
    /// Replace existing outputs instead of refusing them.
    pub overwrite: bool,
    /// Existing directory accepted for voxel scratch. It must exist; no scratch file is written there.
    pub scratch_dir: Option<PathBuf>,
    /// Positive cap in MiB on the rows and writer buffers one derived image retains.
    pub max_scratch_mib: Option<u64>,
}

/// Keyword arguments of [`export_files`].
#[derive(Clone, Debug, Default)]
pub struct ExportFilesOptions {
    /// Feature selection and preprocessing; `Settings::default()` if absent.
    pub settings: Option<Settings>,
    /// Resampling, IBSI interpolation and mask-correction options.
    pub image_settings: Option<ImageSettings>,
    /// Derived images to export; the original image only if absent.
    pub transforms: Option<TransformSettings>,
    /// Replace existing outputs instead of refusing them.
    pub overwrite: bool,
    /// Existing directory accepted for voxel scratch. It must exist; no scratch file is written there.
    pub scratch_dir: Option<PathBuf>,
    /// Positive cap in MiB on the rows and writer buffers one derived image retains.
    pub max_scratch_mib: Option<u64>,
}

/// Write derived-image voxel maps of aligned arrays to NIfTI files.
///
/// The input arrays are aligned and use NumPy axis order. Every selected image type is
/// extracted in turn and written under `<output_dir>/maps`, each map carrying the
/// image-type prefix, for example `wavelet-LLH_glcm_Contrast`. `features.json` is a
/// strict schema-2 report with the processed geometry, effective settings and map paths.
/// Publication uses private staging and refuses existing outputs unless
/// `options.overwrite` is true. Shape features are unavailable in voxel mode and are
/// rejected before extraction.
///
/// # Errors
/// [`Error::InvalidInput`] for invalid inputs or settings, an unsafe or occupied output
/// target or an unusable scratch request; [`Error::Execution`] if extraction or map
/// writing fails; [`Error::Io`] for filesystem failures.
///
/// # Examples
///
/// ```
/// use radiomics::ndarray::ArrayD;
/// use radiomics::{export, ExportOptions, HostArray, Settings};
///
/// # fn main() -> radiomics::Result<()> {
/// let image = HostArray::from(ArrayD::from_shape_fn(vec![5, 6, 6], |ix| (ix[1] + ix[2]) as i16));
/// let mask = HostArray::from(ArrayD::from_elem(vec![5, 6, 6], 1_u8));
/// let dir = std::env::temp_dir().join(format!("radiomics-doc-export-{}", std::process::id()));
/// let options = ExportOptions {
///     settings: Some(Settings {
///         features: vec!["firstorder_Mean".into()],
///         kernel_radius: 1,
///         ..Settings::default()
///     }),
///     allow_synthetic_geometry: true,
///     overwrite: true,
///     ..ExportOptions::default()
/// };
/// let saved = export(&image, &mask, &dir, &options)?;
/// assert!(saved.report.is_file());
/// assert!(saved.maps.contains_key("original_firstorder_Mean"));
/// # std::fs::remove_dir_all(&dir)?;
/// # Ok(())
/// # }
/// ```
pub fn export(
    image: &HostArray,
    mask: &HostArray,
    output_dir: &Path,
    options: &ExportOptions,
) -> Result<MapExport> {
    export_with_provenance(
        image,
        mask,
        output_dir,
        options,
        ExportProvenance::default(),
    )
}

/// Export arrays with the binding's source dtype and runtime recorded in both report and result.
pub fn export_with_provenance(
    image: &HostArray,
    mask: &HostArray,
    output_dir: &Path,
    options: &ExportOptions,
    provenance: ExportProvenance<'_>,
) -> Result<MapExport> {
    let default_settings = Settings::default();
    let default_transforms = TransformSettings::default();
    let settings = options.settings.as_ref().unwrap_or(&default_settings);
    let transforms = options.transforms.as_ref().unwrap_or(&default_transforms);
    let ndim = image.ndim();
    let given = options.spacing.as_deref().unwrap_or(&[]);
    if !(given.is_empty() || given.len() == ndim) {
        return Err(Error::InvalidInput(
            "spacing must contain one value per image axis".into(),
        ));
    }
    let spacing = if given.is_empty() {
        vec![1.0; ndim]
    } else {
        given.to_vec()
    };
    let mut direction = vec![0.0; ndim * ndim];
    for i in 0..ndim {
        direction[i * ndim + i] = 1.0;
    }
    let reference = Volume::new(image.clone(), spacing, vec![0.0; ndim], direction)?;
    let saved = publish_report_with_provenance(
        image,
        mask,
        &ReportRequest {
            output_dir,
            settings,
            transforms,
            reference: &reference,
            inputs: &[],
            source_geometry: None,
            image_settings: None,
            overwrite: options.overwrite,
            temporary_dir: options.scratch_dir.as_deref(),
            max_temporary_mib: options.max_scratch_mib,
            metadata_settings: None,
            voxel: true,
            input_volumes: &[],
        },
        provenance,
    )?;
    Ok(MapExport {
        report: saved.report,
        maps: saved.maps,
        metadata: saved.metadata,
    })
}

/// Read one image and mask file pair and export derived-image voxel maps to disk.
///
/// Loads the pair as [`extract_files`](crate::extract_files) does, then exports as [`export`]
/// does. Normalization happens while loading,
/// before resampling and filtering. Map geometry follows the processed grid; the source
/// geometry and the effective spatial options are recorded in the report.
///
/// # Errors
/// As [`export`]; failures while loading (geometry, label, unreadable file) are
/// reported as [`Error::InvalidInput`], as mlx maps `ValueError`, `RuntimeError`, `OSError`
/// and `EOFError` from loading.
///
/// # Examples
///
/// The options mirror [`ExportOptions`] without `spacing`, because the grid comes from the
/// files. A call looks like this (see [`extract_files`](crate::extract_files) for writing
/// input files):
///
/// ```no_run
/// use radiomics::{export_files, ExportFilesOptions, ImageSettings, Settings};
/// use std::path::Path;
///
/// # fn main() -> radiomics::Result<()> {
/// let options = ExportFilesOptions {
///     settings: Some(Settings {
///         features: vec!["firstorder_Mean".into()],
///         ..Settings::default()
///     }),
///     image_settings: Some(ImageSettings::default()),
///     ..ExportFilesOptions::default()
/// };
/// let saved = export_files(
///     Path::new("image.nii.gz"),
///     Path::new("mask.nii.gz"),
///     Path::new("out"),
///     &options,
/// )?;
/// println!("{} maps, report {}", saved.maps.len(), saved.report.display());
/// # Ok(())
/// # }
/// ```
pub fn export_files(
    image_path: &Path,
    mask_path: &Path,
    output_dir: &Path,
    options: &ExportFilesOptions,
) -> Result<MapExport> {
    export_files_with_runtime(image_path, mask_path, output_dir, options, None)
}

/// Export files with the calling Python interpreter recorded in both report and result.
pub fn export_files_with_runtime(
    image_path: &Path,
    mask_path: &Path,
    output_dir: &Path,
    options: &ExportFilesOptions,
    python_version: Option<&str>,
) -> Result<MapExport> {
    let default_settings = Settings::default();
    let default_transforms = TransformSettings::default();
    let default_image_settings = ImageSettings::default();
    let settings = options.settings.as_ref().unwrap_or(&default_settings);
    let transforms = options.transforms.as_ref().unwrap_or(&default_transforms);
    let source_image = resolve_home(image_path);
    let source_mask = resolve_home(mask_path);
    let pair = load_image_pair(
        &source_image,
        &source_mask,
        Some(settings),
        options.image_settings.as_ref(),
    )
    .map_err(|error| match error {
        Error::InvalidInput(_) => error,
        other => Error::InvalidInput(other.to_string()),
    })?;
    let extraction = Settings {
        normalize: false,
        remove_outliers: None,
        ..settings.clone()
    };
    let source_geometry = pair.source_geometry.as_ref().map(|g| ImageGeometryPayload {
        shape: g.shape.clone(),
        spacing: g.spacing.clone(),
        origin: g.origin.clone(),
        direction: g.direction.clone(),
    });
    let inputs = [source_image, source_mask];
    let saved = publish_report_with_provenance(
        &pair.image.array,
        &pair.mask.array,
        &ReportRequest {
            output_dir,
            settings: &extraction,
            transforms,
            reference: &pair.image,
            inputs: &inputs,
            source_geometry,
            image_settings: Some(
                options
                    .image_settings
                    .as_ref()
                    .unwrap_or(&default_image_settings),
            ),
            overwrite: options.overwrite,
            temporary_dir: options.scratch_dir.as_deref(),
            max_temporary_mib: options.max_scratch_mib,
            metadata_settings: Some(settings),
            voxel: true,
            input_volumes: &inputs,
        },
        ExportProvenance {
            array_dtype: None,
            python_version,
        },
    )?;
    Ok(MapExport {
        report: saved.report,
        maps: saved.maps,
        metadata: saved.metadata,
    })
}

/// `Path.expanduser()`.
fn resolve_home(path: &Path) -> PathBuf {
    match path.to_str().and_then(|s| s.strip_prefix('~')) {
        Some(rest) if rest.is_empty() || rest.starts_with('/') => std::env::var_os("HOME")
            .map(|home| PathBuf::from(home).join(rest.trim_start_matches('/')))
            .unwrap_or_else(|| path.to_path_buf()),
        _ => path.to_path_buf(),
    }
}
