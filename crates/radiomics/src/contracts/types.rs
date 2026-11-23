//! Shared data contracts between preprocessing, feature families, voxel mode and workflows.
//!
//! Mirrors `mlxradiomics/contracts/types.py` (284 lines at mlx-radiomics b58783c) and the
//! `PreparedInput`/`Prepared` dataclasses of `mlxradiomics/imaging/preprocessing.py`.
//! Single-threaded CPU orchestration.
//! Status: input, result metadata and streaming contracts ported.
//!
//! Arrays are NumPy-shaped [`ArrayD`] values with 2 or 3 axes in (z, y, x) order, so a
//! family port reads like its Python source. Spacing is in mm per NumPy axis.

use ndarray::ArrayD;

/// Validated float64 image and the masks derived from the label (`PreparedInput`).
#[derive(Clone, Debug, PartialEq)]
pub struct PreparedInput {
    /// Float64 copy of the input, normalised if requested; same shape as the input.
    pub image: ArrayD<f64>,
    /// Voxels of the selected label that remain after every resegmentation filter.
    pub roi: ArrayD<bool>,
    /// Mask for shape and morphology: the original label mask, or `roi` when
    /// `resegment_shape` is set.
    pub shape_roi: ArrayD<bool>,
    /// Voxels used for discretisation and neighbourhood sampling: `roi`, or the whole
    /// image in voxel mode with `masked_kernel = false`.
    pub sampling_mask: ArrayD<bool>,
}

/// [`PreparedInput`] plus gray-level discretisation (`Prepared`).
#[derive(Clone, Debug, PartialEq)]
pub struct Prepared {
    /// Validated image and masks.
    pub input: PreparedInput,
    /// 1-based gray level inside `sampling_mask`, 0 elsewhere; `None` when no selected
    /// feature reads the lattice.
    pub bins: Option<ArrayD<i32>>,
    /// Highest bin index, empty levels included; 1 when the lattice is unused.
    pub gray_levels: u32,
}

/// Feature values in output order, keyed `<family>_<Name>` (no image prefix).
///
/// NaN means undefined, matching mlx-radiomics; infinities are kept as computed.
pub type Features = Vec<(String, f64)>;

use crate::config::{settings::Settings, spatial::ImageSettings, transforms::TransformSettings};
pub use crate::contracts::validity::FeatureValidity;
use indexmap::IndexMap;
use ndarray::{ArrayView1, ArrayView2};
use serde::Serialize;

/// Consume one borrowed batch of C NumPy-order centres (C × ndim) and C values per feature.
/// The wire format matches mlx's float32 sink. CPU feature calculations use float64.
pub trait VoxelSink {
    /// Consume or copy the batch before returning, propagating any write failure.
    fn consume(
        &mut self,
        centers: ArrayView2<'_, i32>,
        features: &IndexMap<String, ArrayView1<'_, f32>>,
    ) -> crate::Result<()>;
}

impl<F> VoxelSink for F
where
    F: FnMut(ArrayView2<'_, i32>, &IndexMap<String, ArrayView1<'_, f32>>) -> crate::Result<()>,
{
    fn consume(
        &mut self,
        centers: ArrayView2<'_, i32>,
        features: &IndexMap<String, ArrayView1<'_, f32>>,
    ) -> crate::Result<()> {
        self(centers, features)
    }
}

/// Physical grid. Shape and spacing use NumPy order, origin and direction use ITK order.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ImageGeometryPayload {
    /// Voxels along each array axis.
    pub shape: Vec<usize>,
    /// Millimetres along each array axis.
    pub spacing: Vec<f64>,
    /// Physical coordinate of the first voxel centre.
    pub origin: Vec<f64>,
    /// Flattened row-major ITK direction matrix.
    pub direction: Vec<f64>,
}

/// Non-secret runtime provenance using mlx's schema-2 keys.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RuntimeMetadata {
    /// Versions of packages actually used.
    pub versions: IndexMap<String, String>,
    /// Python version when called from Python, otherwise null.
    pub python: Option<String>,
    /// Operating system and architecture.
    pub platform: String,
    /// Execution device name.
    pub device_name: String,
    /// MLX precision flag, null on the native CPU route.
    pub mlx_enable_tf32: Option<String>,
}

impl Default for RuntimeMetadata {
    fn default() -> Self {
        Self {
            versions: [(
                env!("CARGO_PKG_NAME").into(),
                env!("CARGO_PKG_VERSION").into(),
            )]
            .into(),
            python: None,
            platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
            device_name: "cpu".into(),
            mlx_enable_tf32: None,
        }
    }
}

/// This port reports `"cpu"` for native execution. mlx reports `"gpu"` on Metal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub enum Device {
    /// CPU execution.
    #[default]
    #[serde(rename = "cpu")]
    Cpu,
}

/// This port computes in float64. mlx records float32 here.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub enum Precision {
    /// IEEE binary64 arithmetic.
    #[default]
    #[serde(rename = "float64")]
    Float64,
}

/// Requested retained voxel-map storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub enum MapDtype {
    /// Store retained maps as float32.
    #[serde(rename = "float32")]
    Float32,
    /// Store retained maps as float64.
    #[serde(rename = "float64")]
    Float64,
}

/// Schema-2 provenance for a completed extraction. Optional fields are omitted when absent.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExecutionMetadata {
    /// Effective feature settings in mlx's snake_case payload.
    #[serde(serialize_with = "crate::contracts::json::serialize_safe")]
    pub settings: Settings,
    /// Native execution device.
    pub device: Device,
    /// Native arithmetic precision.
    pub precision: Precision,
    /// Requested map storage in voxel mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_dtype: Option<MapDtype>,
    /// Whether voxel mode was requested.
    pub voxel: bool,
    /// Effective spacing in NumPy axis order.
    pub spacing: Vec<f64>,
    /// Histogram implementation used.
    pub histogram_method: String,
    /// Preprocessing applied.
    pub preprocessing: String,
    /// Highest gray-level index.
    pub gray_levels: u32,
    /// Runtime provenance.
    pub runtime: RuntimeMetadata,
    /// Processed array shape.
    pub array_shape: Vec<usize>,
    /// Source array dtype.
    pub array_dtype: String,
    /// Size of the intensity ROI.
    pub roi_voxels: usize,
    /// Volume outside the intensity ROI in voxels.
    pub excluded_voxels: usize,
    /// Counts per output feature.
    pub feature_validity: IndexMap<String, FeatureValidity>,
    /// Effective matrix aggregation by family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<IndexMap<String, String>>,
    /// Integer or floating-point IVH lattice parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ivh_lattice: Option<IndexMap<String, serde_json::Number>>,
    /// Highest gray level for each derived image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gray_levels_by_image: Option<IndexMap<String, u32>>,
    /// Voxel delivery batch size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<usize>,
    /// Estimated scratch bytes for voxel delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scratch_estimate_bytes: Option<usize>,
    /// First-order computation group size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_order_batch_size: Option<usize>,
    /// Estimated scratch bytes for first-order computation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_order_scratch_estimate_bytes: Option<usize>,
    /// Cached first-order results in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_order_cache_bytes: Option<usize>,
    /// Processed file grid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_geometry: Option<ImageGeometryPayload>,
    /// Original file grid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_geometry: Option<ImageGeometryPayload>,
    /// Effective file preprocessing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_settings: Option<ImageSettings>,
    /// Effective flat transform payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<TransformSettings>,
}
