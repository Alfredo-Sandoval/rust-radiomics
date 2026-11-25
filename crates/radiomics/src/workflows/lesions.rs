//! ROI extraction for each connected lesion of one mask label.
//!
//! Mirrors `mlxradiomics/workflows/lesions.py`.

use std::path::Path;

use crate::contracts::results::ExtractionResult;
use crate::imaging::io::HostArray;
use crate::imaging::lesions::{LesionMap, LesionOptions, find_lesions};
use crate::imaging::preprocessing::validate_pair;
use crate::imaging::spatial::load_image_pair;
use crate::workflows::engine::{FeatureValue, Output};
use crate::workflows::extraction::{extract, extract_pair};
use crate::{ImageSettings, Result, Settings, TransformSettings};

/// One connected lesion of a mask label and its ROI features (Python `Lesion`).
#[derive(Clone, Debug, PartialEq)]
pub struct Lesion {
    /// 1 for the largest lesion of the label.
    pub index: usize,
    /// Number of mask voxels in the lesion.
    pub voxels: usize,
    /// First array index of the lesion per axis, on the grid the features were computed on.
    pub lower: Vec<usize>,
    /// One past the last array index of the lesion per axis.
    pub upper: Vec<usize>,
    /// ROI features and provenance of the lesion.
    pub result: ExtractionResult<FeatureValue>,
}

/// Run `extract_one` for every kept lesion, selecting it by its index in the lesion map.
/// The recorded settings keep the caller's mask label.
fn each_lesion(
    found: &LesionMap,
    settings: &Settings,
    mut extract_one: impl FnMut(&Settings) -> Result<ExtractionResult<FeatureValue>>,
) -> Result<Vec<Lesion>> {
    (1..=found.voxels.len())
        .map(|index| {
            let selected = Settings {
                label: index as i64,
                ..settings.clone()
            };
            let mut result = extract_one(&selected)?;
            result.metadata.settings.label = settings.label;
            Ok(Lesion {
                index,
                voxels: found.voxels[index - 1],
                lower: found.lower[index - 1].clone(),
                upper: found.upper[index - 1].clone(),
                result,
            })
        })
        .collect()
}

/// Extract ROI features for each connected lesion of `settings.label`.
///
/// The label is split as in [`label_lesions`](crate::label_lesions), and every kept
/// lesion is extracted as its own ROI exactly as [`extract`] would with a mask holding
/// only that lesion. ROI-relative preprocessing, such as sigma resegmentation and the
/// discretisation origin, therefore follows each lesion. Lesions are returned largest
/// first, and the vector is empty when no lesion reaches `options.min_voxels`. The arrays
/// and settings are validated before the label is split, so invalid inputs fail even when
/// no lesion is kept.
///
/// # Errors
/// [`Error::InvalidInput`](crate::Error::InvalidInput) if the label is absent or an option
/// is zero, and every error of [`extract`].
///
/// # Examples
///
/// ```
/// use radiomics::ndarray::ArrayD;
/// use radiomics::{extract_lesions, HostArray, LesionOptions, Settings, TransformSettings};
///
/// # fn main() -> radiomics::Result<()> {
/// let image = ArrayD::from_shape_fn(vec![4, 8, 8], |ix| (ix[0] + ix[1] * 3 + ix[2]) as i16);
/// // Two separate blocks of the same label.
/// let mask = ArrayD::from_shape_fn(vec![4, 8, 8], |ix| u8::from(ix[2] < 3 || ix[2] > 5));
/// let settings = Settings {
///     features: vec!["firstorder_Mean".into()],
///     ..Settings::default()
/// };
/// let lesions = extract_lesions(
///     &HostArray::from(image),
///     &HostArray::from(mask),
///     &settings,
///     &TransformSettings::default(),
///     Some(&[2.0, 1.0, 1.0]),
///     &LesionOptions::default(),
/// )?;
/// assert_eq!(lesions.len(), 2);
/// assert_eq!((lesions[0].index, lesions[0].voxels), (1, 4 * 8 * 3));
/// assert_eq!(lesions[1].lower, vec![0, 0, 6]);
/// # Ok(())
/// # }
/// ```
pub fn extract_lesions(
    image: &HostArray,
    mask: &HostArray,
    settings: &Settings,
    transforms: &TransformSettings,
    spacing: Option<&[f64]>,
    options: &LesionOptions,
) -> Result<Vec<Lesion>> {
    validate_pair(image, mask)?;
    settings.validate(image.ndim())?;
    let found = find_lesions(mask, settings.label, options)?;
    let labels = HostArray::from(found.labels.clone());
    each_lesion(&found, settings, |selected| {
        extract(image, &labels, selected, transforms, spacing, Output::Roi)
    })
}

/// Read an image and mask file and extract ROI features for each connected lesion.
///
/// The pair is loaded and spatially processed once, as in
/// [`extract_files`](crate::extract_files). The label is then split on the processed
/// grid, so the voxel count and index bounds of each lesion refer to the
/// `image_geometry` in its metadata.
///
/// # Errors
/// [`Error::InvalidInput`](crate::Error::InvalidInput) if an option is zero, and every
/// error of [`extract_files`](crate::extract_files).
pub fn extract_lesions_files(
    image_path: &Path,
    mask_path: &Path,
    settings: &Settings,
    image_settings: &ImageSettings,
    transforms: &TransformSettings,
    options: &LesionOptions,
) -> Result<Vec<Lesion>> {
    options.validate()?;
    let mut pair = load_image_pair(image_path, mask_path, Some(settings), Some(image_settings))?;
    let found = find_lesions(&pair.mask.array, settings.label, options)?;
    pair.mask.array = HostArray::from(found.labels.clone());
    each_lesion(&found, settings, |selected| {
        extract_pair(&pair, selected, image_settings, transforms, Output::Roi)
    })
}
