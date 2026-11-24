//! Extraction and map-export results, mirroring `mlxradiomics/contracts/results.py`.
//! Single-threaded CPU orchestration.
//! Status: ported.

use crate::contracts::json::serialize_safe;
use crate::contracts::types::ExecutionMetadata;
use indexmap::IndexMap;
use serde::Serialize;
use std::path::PathBuf;

/// Python `Result[V]`. Keys use `<image>_<family>_<name>` in extraction order.
/// Streaming extraction leaves the feature mapping empty.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExtractionResult<V> {
    /// ROI scalars or retained voxel maps by name.
    #[serde(serialize_with = "serialize_safe", bound(serialize = "V: Serialize"))]
    pub features: IndexMap<String, V>,
    /// Provenance and validity counts.
    pub metadata: ExecutionMetadata,
}

/// Absolute paths to the published report and voxel maps, plus the extraction provenance.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MapExport {
    /// Report path.
    pub report: PathBuf,
    /// Map paths by output feature name.
    pub maps: IndexMap<String, PathBuf>,
    /// Provenance and validity counts.
    pub metadata: ExecutionMetadata,
}
