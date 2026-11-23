//! Texture execution choices, mirroring `mlxradiomics/features/selection.py`.
//! Status: ported. Single-threaded CPU orchestration on settings and selected keys.

use crate::config::settings::{
    DirectionalAggregation, MatrixAggregation, NeighborhoodSupport, NgtdmSingularCases, Settings,
};

pub(crate) fn explicit_texture(settings: &Settings, names: &[&str]) -> bool {
    settings.neighborhood_support == NeighborhoodSupport::ClosedRadius
        || settings.ngtdm_singular_cases == NgtdmSingularCases::IbsiCaps
        || [settings.glcm_aggregation, settings.glrlm_aggregation]
            .iter()
            .any(|&mode| mode != DirectionalAggregation::Native)
        || [
            settings.gldm_aggregation,
            settings.ngtdm_aggregation,
            settings.glszm_aggregation,
            settings.gldzm_aggregation,
        ]
        .iter()
        .any(|&mode| mode != MatrixAggregation::Native)
        || names.iter().any(|name| name.starts_with("gldzm_"))
}
