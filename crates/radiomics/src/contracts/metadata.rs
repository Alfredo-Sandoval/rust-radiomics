//! Schema-2 serialization adapters, mirroring `mlxradiomics/contracts/metadata.py`.
//! Single-threaded CPU orchestration.
//! Status: ported for the native CPU runtime.

use crate::config::filtering::{
    FilterPadding, FourierGrid, GaborResponse, LbpMethod, LogKernel, OrientationPooling, RieszBase,
};
use crate::config::settings::{
    BinOrigin, ConstantRoiBin, CovarianceDivisor, DirectionalAggregation, EllipsoidArea, Kurtosis,
    MaskExtrapolation, MatrixAggregation, NeighborhoodSupport, NgtdmSingularCases, ResegmentMode,
    SplineBoundary, ZeroVarianceMoments,
};
use crate::config::spatial::Interpolator;
use crate::config::transforms::ImageType;
use crate::contracts::json::json_safe;
use crate::contracts::types::{ExecutionMetadata, ImageGeometryPayload};
use crate::{ImageSettings, Result, Settings, TransformSettings};
use serde::{Serialize, Serializer, ser::SerializeStruct};
use serde_json::Value;

// Serialize the public spellings defined by the configuration enums.
macro_rules! serialize_choice {
    ($($ty:ty),+ $(,)?) => {
        $(impl Serialize for $ty {
            fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok,S::Error> {
                serializer.serialize_str(self.as_str())
            }
        })+
    }
}

serialize_choice!(
    BinOrigin,
    ConstantRoiBin,
    CovarianceDivisor,
    DirectionalAggregation,
    EllipsoidArea,
    Kurtosis,
    MaskExtrapolation,
    MatrixAggregation,
    NeighborhoodSupport,
    NgtdmSingularCases,
    ResegmentMode,
    SplineBoundary,
    ZeroVarianceMoments,
    Interpolator,
    ImageType,
    FilterPadding,
    FourierGrid,
    GaborResponse,
    LbpMethod,
    LogKernel,
    OrientationPooling,
    RieszBase,
);

impl Serialize for TransformSettings {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut payload = serializer.serialize_struct("TransformPayload", 57)?;
        payload.serialize_field("scratch_mib", &self.scratch_mib)?;
        payload.serialize_field("image_types", &self.image_types)?;
        payload.serialize_field("wavelet", &self.wavelet.name)?;
        payload.serialize_field("wavelet_level", &self.wavelet.level)?;
        payload.serialize_field("wavelet_start_level", &self.wavelet.start_level)?;
        payload.serialize_field("sigma", &self.log.sigma)?;
        payload.serialize_field("log_kernel", &self.log.kernel)?;
        payload.serialize_field("log_cutoff", &self.log.cutoff)?;
        payload.serialize_field("log_padding", &self.log.padding)?;
        payload.serialize_field("log_constant", &self.log.constant)?;
        payload.serialize_field("log_planar", &self.log.planar)?;
        payload.serialize_field("lbp2d_radius", &self.lbp2d.radius)?;
        payload.serialize_field("lbp2d_samples", &self.lbp2d.samples)?;
        payload.serialize_field("lbp2d_method", &self.lbp2d.method)?;
        payload.serialize_field("lbp3d_levels", &self.lbp3d.levels)?;
        payload.serialize_field("lbp3d_radius", &self.lbp3d.radius)?;
        payload.serialize_field("lbp3d_subdivision", &self.lbp3d.subdivision)?;
        payload.serialize_field("gradient_use_spacing", &self.gradient.use_spacing)?;
        payload.serialize_field("mean_support", &self.mean.support)?;
        payload.serialize_field("mean_padding", &self.mean.padding)?;
        payload.serialize_field("mean_constant", &self.mean.constant)?;
        payload.serialize_field("mean_planar", &self.mean.planar)?;
        payload.serialize_field("laws_kernel", &self.laws.kernel)?;
        payload.serialize_field("laws_padding", &self.laws.padding)?;
        payload.serialize_field("laws_constant", &self.laws.constant)?;
        payload.serialize_field("laws_pooling", &self.laws.pooling)?;
        payload.serialize_field("laws_energy_distance", &self.laws.energy_distance)?;
        payload.serialize_field("gabor_sigma", &self.gabor.sigma)?;
        payload.serialize_field("gabor_wavelength", &self.gabor.wavelength)?;
        payload.serialize_field("gabor_gamma", &self.gabor.gamma)?;
        payload.serialize_field("gabor_theta", &self.gabor.theta)?;
        payload.serialize_field("gabor_response", &self.gabor.response)?;
        payload.serialize_field("gabor_pooling", &self.gabor.pooling)?;
        payload.serialize_field("gabor_slice_axis", &self.gabor.slice_axis)?;
        payload.serialize_field("gabor_padding", &self.gabor.padding)?;
        payload.serialize_field("gabor_constant", &self.gabor.constant)?;
        payload.serialize_field("simoncelli_levels", &self.simoncelli.levels)?;
        payload.serialize_field("simoncelli_padding", &self.simoncelli.padding)?;
        payload.serialize_field("simoncelli_grid", &self.simoncelli.grid)?;
        payload.serialize_field("simoncelli_planar", &self.simoncelli.planar)?;
        payload.serialize_field("riesz_base", &self.riesz.base)?;
        payload.serialize_field("riesz_orders", &self.riesz.orders)?;
        payload.serialize_field("riesz_log_sigma", &self.riesz.log_sigma)?;
        payload.serialize_field("riesz_log_cutoff", &self.riesz.log_cutoff)?;
        payload.serialize_field("riesz_simoncelli_levels", &self.riesz.simoncelli_levels)?;
        payload.serialize_field("riesz_padding", &self.riesz.padding)?;
        payload.serialize_field("riesz_constant", &self.riesz.constant)?;
        payload.serialize_field("riesz_grid", &self.riesz.grid)?;
        payload.serialize_field("riesz_steering", &self.riesz.steering)?;
        payload.serialize_field("riesz_tensor_sigma", &self.riesz.tensor_sigma)?;
        payload.serialize_field("separable_wavelet", &self.separable_wavelet.name)?;
        payload.serialize_field("separable_wavelet_bands", &self.separable_wavelet.bands)?;
        payload.serialize_field("separable_wavelet_level", &self.separable_wavelet.level)?;
        payload.serialize_field("separable_wavelet_padding", &self.separable_wavelet.padding)?;
        payload.serialize_field(
            "separable_wavelet_constant",
            &self.separable_wavelet.constant,
        )?;
        payload.serialize_field("separable_wavelet_pooling", &self.separable_wavelet.pooling)?;
        payload.serialize_field("separable_wavelet_planar", &self.separable_wavelet.planar)?;
        payload.end()
    }
}

/// Attach processed/source geometry and effective settings to a copy of the provenance.
/// File I/O and spatial preprocessing are implemented by the radiomics crate itself.
pub fn file_metadata(
    metadata: &ExecutionMetadata,
    reference: ImageGeometryPayload,
    settings: Option<&Settings>,
    source_geometry: Option<ImageGeometryPayload>,
    image_settings: Option<&ImageSettings>,
) -> ExecutionMetadata {
    let mut result = metadata.clone();
    if let Some(settings) = settings {
        result.settings = settings.clone();
    }
    if let Some(source) = source_geometry {
        result.source_geometry = Some(source);
    }
    result.image_geometry = Some(reference);
    if let Some(settings) = image_settings {
        result.image_settings = Some(settings.clone());
        result.preprocessing =
            "native CPU file I/O, spatial preprocessing and float64 binning".into();
        result.runtime.versions.insert(
            env!("CARGO_PKG_NAME").into(),
            env!("CARGO_PKG_VERSION").into(),
        );
    }
    result
}

/// Processed geometry, feature count and strict JSON execution metadata in a report.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RunDetails {
    /// Extraction mode.
    pub voxel: bool,
    /// Selected mask label.
    pub label: i64,
    /// Processed array shape.
    pub shape: Vec<usize>,
    /// Processed spacing in NumPy order.
    pub spacing: Vec<f64>,
    /// Number of reported features.
    pub feature_count: usize,
    /// Strict JSON execution provenance.
    pub execution: Value,
}

/// Build report details, preferring the processed file grid when available.
pub fn report_details(metadata: &ExecutionMetadata, count: usize) -> Result<RunDetails> {
    let (shape, spacing) = metadata
        .image_geometry
        .as_ref()
        .map(|g| (&g.shape, &g.spacing))
        .unwrap_or((&metadata.array_shape, &metadata.spacing));
    Ok(RunDetails {
        voxel: metadata.voxel,
        label: metadata.settings.label,
        shape: shape.clone(),
        spacing: spacing.clone(),
        feature_count: count,
        execution: json_safe(metadata)?,
    })
}
