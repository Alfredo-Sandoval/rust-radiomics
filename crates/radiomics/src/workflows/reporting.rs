//! Shared extraction, map writing and atomic report publication.
//!
//! Mirrors `mlxradiomics/workflows/reporting.py` (265 lines at mlx-radiomics b58783c).
//! Status: ported for the public exporters (voxel reports) and the ROI report.
//!
//! CPU orchestration on the calling thread. It runs the CPU extraction of
//! [`crate::workflows::extraction::extract`] once, writes each derived image's maps as staged float32
//! NIfTI files, and publishes `features.json` (schema 2) last, through
//! [`crate::workflows::publication`]. The numerical work is the extraction's: float64 CPU arithmetic,
//! with first-order voxel maps computed in float32 by `first_order::compute_maps`. The
//! report records `"device": "cpu"` and `"precision": "float64"` through the execution
//! metadata.
//!
//! Differences from mlx:
//!
//! - `map_dtype` in the report is `float64`, mlx's default for a sink; the written maps are
//!   float32 either way.
//! - Export keeps compact batches of source offsets and float32 columns in memory and emits
//!   zero background directly to NIfTI. The per-image scratch admission limit charges what is
//!   held: 4 bytes per selected row for the shared offsets, 4 bytes per row and map for the
//!   float32 columns, and a fixed allowance for the map writer's buffers. The charge grows as
//!   batches arrive and is checked before each batch is retained, so an export that exceeds
//!   the limit stops early. mlx charges a dense float32 grid plus its spool; the two accounts
//!   differ, and an export mlx completes is not rejected here. An explicit scratch directory
//!   must exist.
//! - The output-filesystem free-space check is not ported: `std` has no portable query
//!   for it, so a full disk surfaces as an I/O error from the staged write and leaves no
//!   published output.
//! - `features.json` is serialized with sorted keys and two-space indentation, as in
//!   mlx. Python escapes non-ASCII characters in paths; this port writes UTF-8.
//! - The ROI branch (`features.csv` plus the report) is reachable in mlx only through the
//!   MedMLX runner, which is not ported. It is implemented here because the report layout
//!   is shared and tested against `validation/compare.py`.

use crate::contracts::json::json_safe;
use crate::contracts::metadata::{RunDetails, file_metadata, report_details};
use crate::contracts::types::{ExecutionMetadata, ImageGeometryPayload, MapDtype, VoxelSink};
use crate::imaging::io::{HostArray, Volume, write_sparse_nifti_map};
use crate::imaging::transforms::feature_image_names;
use crate::workflows::engine::{FeatureValue, Output, SinkFactory};
use crate::workflows::extraction::extract;
use crate::workflows::publication::{
    Publication, check_output_replacement, output_lock, prepare_output_targets,
    reclaim_stale_staging, reject_input_aliases, reject_output_aliases, resolve,
};
use crate::{Error, ImageSettings, Result, Settings, TransformSettings};
use indexmap::IndexMap;
use ndarray::{ArrayView1, ArrayView2};
use serde_json::{Map, Value};
use std::fs;
use std::path::{Path, PathBuf};

/// Source information supplied by a binding before the schema-2 report is staged.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExportProvenance<'a> {
    /// Original array dtype, including its byte order, before conversion to native storage.
    pub array_dtype: Option<&'a str>,
    /// Interpreter version for a Python caller.
    pub python_version: Option<&'a str>,
}

/// Published paths and the metadata used by the exporters (`PublishedReport`).
#[derive(Clone, Debug, PartialEq)]
pub struct PublishedReport {
    /// Path of `features.json`.
    pub report: PathBuf,
    /// Path of `features.csv`, ROI reports only.
    pub table: Option<PathBuf>,
    /// Map path by prefixed feature name, in output order.
    pub maps: IndexMap<String, PathBuf>,
    /// Provenance and validity counts of the extraction.
    pub metadata: ExecutionMetadata,
    /// The report's `details` object.
    pub details: RunDetails,
}

/// Arguments of [`publish_report`] (the keyword arguments of `publish_report`).
#[derive(Clone, Debug)]
pub struct ReportRequest<'a> {
    /// Directory for `features.json` and `maps/`; created if missing.
    pub output_dir: &'a Path,
    /// Settings used for the extraction.
    pub settings: &'a Settings,
    /// Derived images to extract.
    pub transforms: &'a TransformSettings,
    /// Grid, spacing and orientation that the written maps carry.
    pub reference: &'a Volume,
    /// Input files the outputs must not alias.
    pub inputs: &'a [PathBuf],
    /// Source grid recorded in the report, if the inputs were files.
    pub source_geometry: Option<ImageGeometryPayload>,
    /// Spatial options recorded in the report, if the inputs were files.
    pub image_settings: Option<&'a ImageSettings>,
    /// Replace existing outputs instead of refusing them.
    pub overwrite: bool,
    /// Existing directory accepted for scratch; no scratch file is written there.
    pub temporary_dir: Option<&'a Path>,
    /// Positive cap in MiB on the rows and writer buffers one derived image retains.
    pub max_temporary_mib: Option<u64>,
    /// Settings recorded in the metadata instead of `settings`, if different.
    pub metadata_settings: Option<&'a Settings>,
    /// Voxel maps (`true`) or a scalar ROI report (`false`).
    pub voxel: bool,
    /// Input volumes checked for detached payloads before outputs are replaced.
    pub input_volumes: &'a [PathBuf],
}

/// Prefixed map names of each derived image, in output order (`_map_layout`).
///
/// Shape and local-intensity peak features are refused because they have no voxel maps;
/// names must be nonempty and unique.
pub fn map_layout(
    reference: &Volume,
    settings: &Settings,
    transforms: &TransformSettings,
) -> Result<IndexMap<String, Vec<String>>> {
    if settings.features.iter().any(|n| n.starts_with("shape")) {
        return Err(Error::InvalidInput(
            "shape features are unavailable in voxel mode".into(),
        ));
    }
    if settings
        .features
        .iter()
        .any(|n| n.starts_with("localintensity_"))
    {
        return Err(Error::InvalidInput(
            "physical local-intensity peak features are unavailable in voxel mode".into(),
        ));
    }
    let images = feature_image_names(
        reference.array.shape(),
        &reference.spacing,
        transforms,
        settings.force2d_axis,
    )?;
    let layout: IndexMap<String, Vec<String>> = images
        .into_iter()
        .map(|image| {
            let names = settings
                .features
                .iter()
                .map(|feature| format!("{image}_{feature}"))
                .collect();
            (image, names)
        })
        .collect();
    let mut seen = std::collections::HashSet::new();
    let unique = layout.values().flatten().all(|n| seen.insert(n));
    if seen.is_empty() || !unique {
        return Err(Error::InvalidInput(
            "derived image names must be nonempty and unique".into(),
        ));
    }
    Ok(layout)
}

/// Fixed bytes charged for writing one map: the 64 KiB staging buffer, the 352-byte header and
/// an allowance for the gzip encoder's deflate state.
const MAP_WRITER_BYTES: usize = 64 * 1024 + 352 + 384 * 1024;

/// Running charge for the batches one derived image retains, against an optional limit.
#[derive(Clone, Copy, Debug)]
struct ExportBudget {
    limit: Option<usize>,
    held: usize,
}

impl ExportBudget {
    /// Start with the writer allowance already charged.
    fn new(limit: Option<usize>) -> Result<Self> {
        let budget = Self {
            limit,
            held: MAP_WRITER_BYTES,
        };
        budget.check(0)?;
        Ok(budget)
    }

    /// Admit `additional` more retained bytes or fail with the temporary-budget error.
    fn charge(&mut self, additional: usize) -> Result<()> {
        self.check(additional)?;
        self.held += additional;
        Ok(())
    }

    fn check(&self, additional: usize) -> Result<()> {
        let needed = self.held.saturating_add(additional);
        match self.limit {
            Some(limit) if needed > limit => Err(Error::InvalidInput(format!(
                "voxel export temporary storage needs {needed} bytes, exceeding \
                 the temporary budget of {limit} bytes"
            ))),
            _ => Ok(()),
        }
    }
}

/// Source-grid offsets are shared by every float32 feature column in a batch.
struct ExportBatch {
    offsets: Vec<u32>,
    columns: Vec<Vec<f32>>,
}

struct StoreSink<'a> {
    shape: &'a [usize],
    names: &'a [String],
    batches: &'a mut Vec<ExportBatch>,
    budget: ExportBudget,
}

impl VoxelSink for StoreSink<'_> {
    fn consume(
        &mut self,
        centers: ArrayView2<'_, i32>,
        features: &IndexMap<String, ArrayView1<'_, f32>>,
    ) -> Result<()> {
        if centers.ncols() != self.shape.len() {
            return Err(Error::Execution(
                "voxel center dimension differs from export grid".into(),
            ));
        }
        let rows = centers.nrows();
        self.budget
            .charge(rows.saturating_mul(4 * (1 + self.names.len())))?;
        let mut offsets = Vec::with_capacity(rows);
        for row in centers.rows() {
            let mut offset = 0;
            for (&x, &extent) in row.iter().zip(self.shape) {
                if x < 0 || x as usize >= extent {
                    return Err(Error::Execution("voxel center outside export grid".into()));
                }
                offset = offset * extent + x as usize;
            }
            // The engine admits source grids below 2^30 voxels.
            offsets.push(offset as u32);
        }
        let columns = self
            .names
            .iter()
            .map(|name| {
                let col = features.get(name).ok_or_else(|| {
                    Error::Execution(format!("voxel batch is missing feature '{name}'"))
                })?;
                if col.len() != offsets.len() {
                    return Err(Error::Execution(
                        "voxel export column length mismatch".into(),
                    ));
                }
                Ok(col.to_vec())
            })
            .collect::<Result<_>>()?;
        self.batches.push(ExportBatch { offsets, columns });
        Ok(())
    }
}

struct MapWriter<'a> {
    reference: &'a Volume,
    layout: &'a IndexMap<String, Vec<String>>,
    paths: &'a IndexMap<String, PathBuf>,
    publication: &'a mut Publication,
    batches: Vec<ExportBatch>,
    limit: Option<usize>,
    written: IndexMap<String, String>,
}

impl SinkFactory for MapWriter<'_> {
    fn open(&mut self, image: &str) -> Result<Box<dyn VoxelSink + '_>> {
        let names = self.layout.get(image).ok_or_else(|| {
            Error::Execution(format!("unexpected derived image '{image}' in export"))
        })?;
        Ok(Box::new(StoreSink {
            shape: self.reference.array.shape(),
            names,
            batches: &mut self.batches,
            budget: ExportBudget::new(self.limit)?,
        }))
    }

    fn close(&mut self, image: &str, error: Option<&Error>) -> Result<()> {
        let batches = std::mem::take(&mut self.batches);
        if error.is_some() {
            return Ok(());
        }
        for (column, name) in self.layout[image].iter().enumerate() {
            let target = &self.paths[name];
            let staged = self.publication.stage(target)?;
            let samples = batches.iter().flat_map(|batch| {
                batch
                    .offsets
                    .iter()
                    .copied()
                    .zip(batch.columns[column].iter().copied())
            });
            write_sparse_nifti_map(&staged, self.reference, samples)?;
            self.written
                .insert(name.clone(), target.to_string_lossy().into_owned());
        }
        Ok(())
    }
}

/// Sort object keys recursively (`json.dumps(sort_keys=True)`).
fn sorted(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<_> = map.into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            Value::Object(entries.into_iter().map(|(k, v)| (k, sorted(v))).collect())
        }
        Value::Array(items) => Value::Array(items.into_iter().map(sorted).collect()),
        other => other,
    }
}

fn csv_field(text: &str) -> String {
    if text.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_string()
    }
}

/// `repr(float)` as `csv.writer` writes it.
fn csv_number(value: f64) -> String {
    if value.is_nan() {
        "nan".into()
    } else if value.is_infinite() {
        if value > 0.0 { "inf" } else { "-inf" }.into()
    } else {
        crate::imaging::transforms::python_float_repr(value)
    }
}

pub(super) fn publish_report_with_provenance(
    image: &HostArray,
    mask: &HostArray,
    request: &ReportRequest<'_>,
    provenance: ExportProvenance<'_>,
) -> Result<PublishedReport> {
    if request.max_temporary_mib == Some(0) {
        return Err(Error::InvalidInput(
            "max_scratch_mib must be a positive integer".into(),
        ));
    }
    request.settings.validate(image.ndim())?;
    let layout = if request.voxel {
        map_layout(request.reference, request.settings, request.transforms)?
    } else {
        IndexMap::new()
    };
    let directory = resolve(request.output_dir);
    if directory.exists() && !directory.is_dir() {
        return Err(Error::InvalidInput(format!(
            "output_dir is not a directory: {}",
            directory.display()
        )));
    }
    let report = directory.join("features.json");
    let table = (!request.voxel).then(|| directory.join("features.csv"));
    let paths: IndexMap<String, PathBuf> = layout
        .values()
        .flatten()
        .map(|name| {
            (
                name.clone(),
                directory.join("maps").join(format!("{name}.nii.gz")),
            )
        })
        .collect();
    let mut outputs = vec![report.clone()];
    outputs.extend(table.clone());
    outputs.extend(paths.values().cloned());
    let _lock = output_lock(&directory)?;
    reclaim_stale_staging(&directory);
    prepare_output_targets(&directory, &outputs)?;
    reject_input_aliases(&outputs, request.inputs)?;
    reject_output_aliases(&outputs)?;
    let existing: Vec<String> = outputs
        .iter()
        .filter(|p| p.exists())
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    check_output_replacement(&existing, request.overwrite, request.input_volumes)?;
    let mut publication = Publication::create(&directory).map_err(|_| {
        Error::InvalidInput(format!("cannot stage outputs in: {}", directory.display()))
    })?;
    admit_scratch(request)?;
    let (result, written) = if request.voxel {
        let mut writer = MapWriter {
            reference: request.reference,
            layout: &layout,
            paths: &paths,
            publication: &mut publication,
            batches: Vec::new(),
            limit: scratch_limit(request)?,
            written: IndexMap::new(),
        };
        let result = extract(
            image,
            mask,
            request.settings,
            request.transforms,
            Some(&request.reference.spacing),
            Output::PerImage {
                dtype: MapDtype::Float64,
                factory: &mut writer,
            },
        );
        (result, writer.written)
    } else {
        let result = extract(
            image,
            mask,
            request.settings,
            request.transforms,
            Some(&request.reference.spacing),
            Output::Roi,
        );
        (result, IndexMap::new())
    };
    let result = result.map_err(|error| match error {
        Error::Execution(message) | Error::MissingDependency(message) => {
            Error::Execution(format!("mlxradiomics extraction failed: {message}"))
        }
        other => other,
    })?;
    let reference = ImageGeometryPayload {
        shape: request.reference.array.shape().to_vec(),
        spacing: request.reference.spacing.clone(),
        origin: request.reference.origin.clone(),
        direction: request.reference.direction.clone(),
    };
    let mut metadata = file_metadata(
        &result.metadata,
        reference,
        request.metadata_settings,
        request.source_geometry.clone(),
        request.image_settings,
    );
    if request.image_settings.is_some() {
        // mlx names its host I/O stack here; this port reads and resamples natively.
        metadata.preprocessing =
            "native CPU file I/O, spatial preprocessing and float64 binning".into();
    }
    if let Some(dtype) = provenance.array_dtype {
        metadata.array_dtype = dtype.into();
    }
    if let Some(version) = provenance.python_version {
        metadata.runtime.python = Some(version.into());
    }
    let count = if request.voxel {
        paths.len()
    } else {
        result.features.len()
    };
    let details = report_details(&metadata, count)?;
    let mut payload = Map::new();
    payload.insert("schema_version".into(), Value::from(2));
    payload.insert(
        "mode".into(),
        Value::from(if request.voxel { "voxel" } else { "roi" }),
    );
    payload.insert("details".into(), json_safe(&details)?);
    if request.voxel {
        let complete =
            paths.keys().all(|name| written.contains_key(name)) && written.len() == paths.len();
        if !complete {
            return Err(Error::Execution(
                "voxel extraction did not produce every selected map".into(),
            ));
        }
        // Order by the planned layout, as mlx's `maps` dictionary is filled per image.
        let maps: IndexMap<&String, &String> = paths.keys().map(|k| (k, &written[k])).collect();
        payload.insert("maps".into(), json_safe(&maps)?);
    } else {
        let mut values: IndexMap<String, f64> = IndexMap::new();
        for (name, value) in &result.features {
            let FeatureValue::Scalar(x) = value else {
                return Err(Error::Execution(format!(
                    "ROI feature '{name}' did not return a scalar"
                )));
            };
            values.insert(name.clone(), *x);
        }
        let header: Vec<String> = values.keys().map(|k| csv_field(k)).collect();
        let row: Vec<String> = values.values().map(|&v| csv_number(v)).collect();
        let table = table.as_ref().expect("ROI report has a table");
        fs::write(
            publication.stage(table)?,
            format!("{}\r\n{}\r\n", header.join(","), row.join(",")),
        )?;
        payload.insert("features".into(), json_safe(&values)?);
    }
    let text = serde_json::to_string_pretty(&sorted(Value::Object(payload)))
        .map_err(|e| Error::Execution(e.to_string()))?;
    fs::write(publication.stage(&report)?, text)?;
    publication.publish(request.overwrite)?;
    Ok(PublishedReport {
        report,
        table,
        maps: paths,
        metadata,
        details,
    })
}

/// Byte limit of `max_temporary_mib`, if given.
fn scratch_limit(request: &ReportRequest<'_>) -> Result<Option<usize>> {
    request
        .max_temporary_mib
        .map(|mib| {
            usize::try_from(mib)
                .ok()
                .and_then(|m| m.checked_mul(1024 * 1024))
                .ok_or_else(|| Error::InvalidInput("max_scratch_mib is too large".into()))
        })
        .transpose()
}

/// Admit the scratch directory and the fixed writer allowance before any extraction. The
/// retained batches are charged as they arrive.
fn admit_scratch(request: &ReportRequest<'_>) -> Result<()> {
    if !request.voxel {
        return Ok(());
    }
    if let Some(dir) = request.temporary_dir {
        let dir = resolve(dir);
        if !dir.is_dir() {
            return Err(Error::InvalidInput(format!(
                "No such file or directory: '{}'",
                dir.display()
            )));
        }
    }
    ExportBudget::new(scratch_limit(request)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::workflows::reporting::{ExportBudget, MAP_WRITER_BYTES};

    #[test]
    fn budget_admits_its_exact_requirement_and_refuses_one_byte_more() {
        let limit = MAP_WRITER_BYTES + 1000;
        let mut budget = ExportBudget::new(Some(limit)).unwrap();
        budget.charge(600).unwrap();
        budget.charge(400).unwrap();
        let error = budget.charge(1).unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "voxel export temporary storage needs {} bytes, exceeding \
                 the temporary budget of {limit} bytes",
                limit + 1
            )
        );
    }

    #[test]
    fn budget_below_the_writer_allowance_is_refused_up_front() {
        assert!(ExportBudget::new(Some(MAP_WRITER_BYTES - 1)).is_err());
        assert!(ExportBudget::new(None).is_ok());
    }
}
