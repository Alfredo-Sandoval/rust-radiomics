//! Radiomics features and voxel maps on the CPU, in float64.
//!
//! `radiomics` extracts IBSI-checked radiomics features from a 2-D or 3-D image and a
//! label mask. It returns one value per feature for the region of interest (ROI mode) or
//! one value per voxel neighbourhood (voxel mode). Voxel maps can be kept in memory,
//! streamed to a [`VoxelSink`], or written as NIfTI files with a JSON report. The crate is
//! the engine behind the `rustrad` Python package and mirrors the setting names and
//! semantics of mlx-radiomics. The user guides in the repository's `docs/` directory list
//! every feature, setting and validation check.
//!
//! # Quick start
//!
//! Arrays are [`ndarray::ArrayD`] values with 2 or 3 axes in NumPy order `(z, y, x)`.
//! Wrap them in a [`HostArray`] and call [`extract`]. Spacing is in millimetres in the
//! same axis order, and `None` means 1 mm on every axis.
//!
//! ```
//! use radiomics::ndarray::ArrayD;
//! use radiomics::{extract, FeatureValue, HostArray, Output, Settings, TransformSettings};
//!
//! # fn main() -> radiomics::Result<()> {
//! let image = ArrayD::from_shape_fn(vec![6, 8, 8], |ix| (ix[0] * 7 + ix[1] * 3 + ix[2]) as i16);
//! let mask = ArrayD::from_shape_fn(vec![6, 8, 8], |ix| {
//!     u8::from(ix[0] > 0 && ix[1] > 1 && ix[1] < 7 && ix[2] > 1 && ix[2] < 7)
//! });
//! let settings = Settings {
//!     features: vec!["firstorder_Mean".into(), "glcm_Contrast".into()],
//!     bin_width: 5.0,
//!     ..Settings::default()
//! };
//! let result = extract(
//!     &HostArray::from(image),
//!     &HostArray::from(mask),
//!     &settings,
//!     &TransformSettings::default(),
//!     Some(&[3.0, 1.0, 1.0]),
//!     Output::Roi,
//! )?;
//! let mean = result.features.get("original_firstorder_Mean");
//! assert!(matches!(mean, Some(FeatureValue::Scalar(v)) if v.is_finite()));
//! assert_eq!(result.metadata.roi_voxels, 5 * 5 * 5);
//! # Ok(())
//! # }
//! ```
//!
//! # Overview
//!
//! Everything below is reachable from the crate root or from one of four small modules.
//!
//! **Extraction**
//!
//! - [`extract`] computes features or voxel maps from aligned in-memory arrays.
//! - [`extract_files`] reads, aligns and optionally resamples a NIfTI or NRRD image and
//!   mask pair first.
//! - [`Output`] selects the delivery: [`Output::Roi`] scalars, [`Output::Maps`] retained
//!   maps, [`Output::Stream`] batches sent to one [`VoxelSink`], or [`Output::PerImage`]
//!   with one sink per derived image from a [`SinkFactory`].
//! - [`ExtractionResult`] carries the [`FeatureValue`] entries and the
//!   [`ExecutionMetadata`](crate::metadata::ExecutionMetadata) that records how they were made.
//!
//! **Export**
//!
//! - [`export`] and [`export_files`] write voxel maps as NIfTI files and a schema-2
//!   `features.json` report, configured with [`ExportOptions`] and [`ExportFilesOptions`].
//!   Both return a [`MapExport`].
//!
//! **Settings and conventions**
//!
//! - [`Settings`] holds feature selection, discretisation and the numerical conventions.
//!   [`Settings::pyradiomics_conventions`] selects the PyRadiomics convention set.
//! - [`ImageSettings`], [`IbsiInterpolation`] and [`Interpolator`] control file
//!   resampling.
//! - [`TransformSettings`] and [`ImageType`] select derived images such as wavelet or
//!   Laplacian-of-Gaussian responses.
//! - [`settings`] names the convention enums used by `Settings` and the per-filter
//!   settings structs used by `TransformSettings`.
//! - [`params`] reads PyRadiomics-style YAML parameter files.
//!
//! **Feature catalogue**
//!
//! - [`list_features`], [`features`] and [`default_features`] list the selectable
//!   features. [`Family`], [`FeatureSpec`] and [`Mode`] describe them.
//!
//! **Inputs, geometry and errors**
//!
//! - [`io`] has [`HostArray`] (an array of any supported pixel type),
//!   [`Volume`](crate::io::Volume) (an array with spacing, origin and direction) and the NIfTI and
//!   NRRD readers.
//! - [`metadata`] has the provenance records inside every result.
//! - [`Error`] and [`Result`] are the error type and alias of every fallible function.
//!
//! # Voxel maps and sinks
//!
//! A sink receives each batch of voxel centres and per-feature float32 values
//! synchronously. Any closure with the [`VoxelSink::consume`] signature is a sink.
//!
//! ```
//! use radiomics::indexmap::IndexMap;
//! use radiomics::ndarray::{ArrayD, ArrayView1, ArrayView2};
//! use radiomics::{extract, HostArray, MapDtype, Output, Settings, TransformSettings};
//!
//! # fn main() -> radiomics::Result<()> {
//! let image = HostArray::from(ArrayD::from_shape_fn(vec![5, 6, 6], |ix| (ix[1] + ix[2]) as i16));
//! let mask = HostArray::from(ArrayD::from_elem(vec![5, 6, 6], 1_u8));
//! let settings = Settings {
//!     features: vec!["firstorder_Mean".into()],
//!     kernel_radius: 1,
//!     ..Settings::default()
//! };
//! let mut voxels = 0usize;
//! let mut sink = |centers: ArrayView2<'_, i32>,
//!                 _features: &IndexMap<String, ArrayView1<'_, f32>>|
//!  -> radiomics::Result<()> {
//!     voxels += centers.nrows();
//!     Ok(())
//! };
//! extract(
//!     &image,
//!     &mask,
//!     &settings,
//!     &TransformSettings::default(),
//!     None,
//!     Output::Stream { dtype: MapDtype::Float32, sink: &mut sink },
//! )?;
//! assert_eq!(voxels, 5 * 6 * 6);
//! # Ok(())
//! # }
//! ```
//!
//! # Threading and precision
//!
//! - All numerical work runs on the CPU in IEEE binary64. Voxel maps are stored as
//!   float32 or float64 as requested, and streamed batches are float32.
//! - Computation uses Rayon's global thread pool. Set `RAYON_NUM_THREADS` to limit the
//!   worker count. A sink receives batches synchronously, one at a time, in order.
//! - [`Error`], [`Settings`], [`TransformSettings`] and the result types are `Send` and
//!   `Sync`, so independent extractions can run on separate threads.
//!
//! # Stability
//!
//! The items documented on this page, in [`io`], [`metadata`], [`params`] and
//! [`settings`] are the public API and follow semantic versioning. The rest of the crate,
//! which mirrors the file layout of mlx-radiomics, is private. A hidden module,
//! `__private`, supports the Python binding and may change in any release. Do not depend
//! on it.
//!
//! Enums that can gain variants are `#[non_exhaustive]`, so a `match` on them needs a
//! wildcard arm. Settings structs have public fields and implement `Default`; build them
//! with struct update syntax (`..Settings::default()`). Settings are validated when they
//! are used, not when they are built.

// Indexed loops over axes and small matrices mirror the Python sources line for line;
// rewriting them as iterator chains would obscure the numerics.
#![allow(clippy::needless_range_loop)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_debug_implementations)]

mod config;
mod contracts;
mod error;
mod features;
mod filters;
mod imaging;
mod numerics;
mod voxel;
mod workflows;

mod native_first_order;

/// Re-export of the `indexmap` crate, whose `IndexMap` appears in [`ExtractionResult`],
/// [`MapExport`] and [`VoxelSink::consume`].
pub use indexmap;
/// Re-export of the `ndarray` crate, whose `ArrayD` and views appear throughout the API.
pub use ndarray;

pub use crate::config::settings::{
    Family, FeatureSpec, Mode, Settings, default_features, features, list_features,
};
pub use crate::config::spatial::{IbsiInterpolation, ImageSettings, Interpolator};
pub use crate::config::transforms::{ImageType, TransformSettings};
pub use crate::contracts::results::{ExtractionResult, MapExport};
pub use crate::contracts::types::{MapDtype, VoxelSink};
pub use crate::error::{Error, Result};
pub use crate::imaging::io::HostArray;
pub use crate::workflows::engine::{FeatureValue, Output, SinkFactory};
pub use crate::workflows::exporting::{ExportFilesOptions, ExportOptions, export, export_files};
pub use crate::workflows::extraction::{extract, extract_files};

pub(crate) use crate::contracts::types::{Features, Prepared, PreparedInput};

/// Support for the rust-radiomics Python binding. Not part of the public API; may change in
/// any release.
#[doc(hidden)]
pub mod __private {
    pub use crate::workflows::exporting::{export_files_with_runtime, export_with_provenance};
    pub use crate::workflows::reporting::ExportProvenance;
}

/// Reading volumes and the host array type.
///
/// [`HostArray`] wraps an array of any supported pixel type and is what [`extract`] and
/// [`export`] take. [`Volume`](crate::io::Volume) adds the physical grid.
/// [`read_image`](crate::io::read_image) and [`read_mask`](crate::io::read_mask) load NIfTI (`.nii`,
/// `.nii.gz`) and NRRD (`.nrrd`, `.nhdr`) files.
///
/// ```
/// use radiomics::io::{HostArray, Volume, read_image};
/// use radiomics::ndarray::ArrayD;
///
/// # fn main() -> radiomics::Result<()> {
/// let array = HostArray::from(ArrayD::from_elem(vec![2, 3, 4], 7_i16));
/// assert_eq!(array.shape(), &[2, 3, 4]);
/// assert_eq!(array.dtype_name(), "int16");
///
/// // Spacing is per array axis (z, y, x); origin and direction use ITK order.
/// let volume = Volume::new(
///     array,
///     vec![3.0, 1.0, 1.0],
///     vec![0.0; 3],
///     vec![1., 0., 0., 0., 1., 0., 0., 0., 1.],
/// )?;
/// assert_eq!(volume.spacing, [3.0, 1.0, 1.0]);
/// assert!(read_image(std::path::Path::new("missing.nii.gz")).is_err());
/// # Ok(())
/// # }
/// ```
pub mod io {
    pub use crate::imaging::io::{
        ArrayKind, HostArray, VOLUME_SUFFIXES, Volume, read_image, read_mask,
    };
}

/// Provenance records attached to every result and report.
///
/// [`ExecutionMetadata`](crate::metadata::ExecutionMetadata) records the effective settings, geometry, gray-level counts and
/// per-feature validity of one extraction. It serialises to the schema-2 JSON of the
/// export report.
pub mod metadata {
    pub use crate::contracts::types::{
        Device, ExecutionMetadata, ImageGeometryPayload, Precision, RuntimeMetadata,
    };
    pub use crate::contracts::validity::FeatureValidity;
}

/// PyRadiomics-style YAML parameter files.
///
/// A parameter file holds `setting`, `featureClass` and `imageType` sections.
/// [`ExtractionParameters::from_yaml`](crate::params::ExtractionParameters::from_yaml) and
/// [`from_file`](crate::params::ExtractionParameters::from_file) parse one into the three
/// settings objects that [`extract_files`] takes.
///
/// ```
/// use radiomics::params::ExtractionParameters;
///
/// # fn main() -> radiomics::Result<()> {
/// let yaml = "
/// setting:
///   binWidth: 10
/// featureClass:
///   firstorder: [Mean, Median]
/// imageType:
///   Original: {}
/// ";
/// let params = ExtractionParameters::from_yaml(yaml, 3)?;
/// assert_eq!(params.settings.bin_width, 10.0);
/// assert_eq!(params.settings.features, ["firstorder_Mean", "firstorder_Median"]);
/// assert!(ExtractionParameters::from_yaml("setting: {noSuchKey: 1}", 3).is_err());
/// # Ok(())
/// # }
/// ```
pub mod params {
    pub use crate::config::parameters::{
        ExtractionParameters, Value, load_params, parse_params, settings_from_params,
    };
}

/// Convention enums of [`Settings`] and per-filter settings of [`TransformSettings`].
///
/// Each enum round-trips the exact mlx-radiomics spelling through `Display`, `FromStr`,
/// `as_str` and `ALL`. Each filter settings struct implements `Default` with the
/// mlx-radiomics defaults, so set only the fields you need:
///
/// ```
/// use radiomics::settings::{FilterPadding, LogSettings};
/// use radiomics::{ImageType, TransformSettings};
///
/// let transforms = TransformSettings {
///     image_types: vec![ImageType::Original, ImageType::Log],
///     log: LogSettings { sigma: vec![1.0, 2.0], ..LogSettings::default() },
///     ..TransformSettings::default()
/// };
/// assert!(transforms.validate().is_ok());
/// assert_eq!("mirror".parse::<FilterPadding>().unwrap(), FilterPadding::Mirror);
/// ```
pub mod settings {
    pub use crate::config::filtering::{
        FilterPadding, FourierGrid, GaborResponse, LbpMethod, LogKernel, OrientationPooling,
        RieszBase,
    };
    pub use crate::config::settings::{
        BinOrigin, ConstantRoiBin, CovarianceDivisor, DirectionalAggregation, EllipsoidArea,
        Kurtosis, MaskExtrapolation, MatrixAggregation, NeighborhoodSupport, NgtdmSingularCases,
        ResegmentMode, SplineBoundary, ZeroVarianceMoments,
    };
    pub use crate::config::transforms::{
        GaborSettings, GradientSettings, LawsSettings, Lbp2DSettings, Lbp3DSettings, LogSettings,
        MeanSettings, RieszSettings, SeparableWaveletSettings, SimoncelliSettings, WaveletSettings,
    };
}

/// Seal check: every internal module path is private to an external crate.
#[cfg(doctest)]
mod sealed_internals {
    /// ```compile_fail,E0603
    /// use radiomics::config;
    /// ```
    pub struct Config;

    /// ```compile_fail,E0603
    /// use radiomics::contracts;
    /// ```
    pub struct Contracts;

    /// ```compile_fail,E0603
    /// use radiomics::error;
    /// ```
    pub struct Error;

    /// ```compile_fail,E0603
    /// use radiomics::features::texture;
    /// ```
    pub struct Features;

    /// ```compile_fail,E0603
    /// use radiomics::filters;
    /// ```
    pub struct Filters;

    /// ```compile_fail,E0603
    /// use radiomics::imaging;
    /// ```
    pub struct Imaging;

    /// ```compile_fail,E0603
    /// use radiomics::numerics;
    /// ```
    pub struct Numerics;

    /// ```compile_fail,E0603
    /// use radiomics::voxel;
    /// ```
    pub struct Voxel;

    /// ```compile_fail,E0603
    /// use radiomics::workflows;
    /// ```
    pub struct Workflows;

    /// The former hidden root re-exports are gone too.
    ///
    /// ```compile_fail,E0603
    /// use radiomics::Prepared;
    /// ```
    pub struct Prepared;
}
