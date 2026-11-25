//! Translate W8 workflow results and synchronous streaming callbacks for Python.
use crate::{arrays, config, sinks::with_output};
use numpy::{IntoPyArray, PyUntypedArray, PyUntypedArrayMethods};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pythonize::{PythonizeTypes, PythonizeUnnamedMappingAdapter};
use radiomics::__private::ExportProvenance;
use radiomics::ExtractionResult;
use radiomics::FeatureValue;
use radiomics::LesionOptions;
use std::path::PathBuf;

// Python geometry, spacing and settings sequences retain their tuple representation.
pub(crate) struct Payload;
impl PythonizeTypes for Payload {
    type Map = PyDict;
    type NamedMap = PythonizeUnnamedMappingAdapter<PyDict>;
    type List = PyTuple;
}

fn python_version() -> &'static str {
    Python::version_str()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

fn python_result(
    py: Python<'_>,
    result: ExtractionResult<FeatureValue>,
) -> PyResult<(Bound<'_, PyDict>, Bound<'_, PyAny>)> {
    let mut metadata = result.metadata;
    metadata.runtime.python = Some(python_version().to_owned());
    let features = PyDict::new(py);
    for (key, value) in result.features {
        match value {
            FeatureValue::Scalar(v) => features.set_item(key, v)?,
            FeatureValue::Map32(map) => features.set_item(key, map.into_pyarray(py))?,
            FeatureValue::Map64(map) => features.set_item(key, map.into_pyarray(py))?,
            other => {
                return Err(PyRuntimeError::new_err(format!(
                    "unsupported feature value for {key}: {other:?}"
                )));
            }
        }
    }
    Ok((
        features,
        pythonize::pythonize_custom::<Payload, _>(py, &metadata)?,
    ))
}

#[pyfunction]
#[pyo3(signature=(image, mask, settings, transforms, spacing, voxel, map_dtype, sink=None, sink_factory=None))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn extract<'py>(
    py: Python<'py>,
    image: &Bound<'py, PyUntypedArray>,
    mask: &Bound<'py, PyUntypedArray>,
    settings: &Bound<'_, PyAny>,
    transforms: &Bound<'_, PyAny>,
    spacing: Option<Vec<f64>>,
    voxel: bool,
    map_dtype: &str,
    sink: Option<Py<PyAny>>,
    sink_factory: Option<Py<PyAny>>,
) -> PyResult<(Bound<'py, PyDict>, Bound<'py, PyAny>)> {
    let source_dtype = image.dtype().str()?.to_str()?.to_owned();
    let image = arrays::host(image)?;
    let mask = arrays::host(mask)?;
    let settings = config::settings(settings)?;
    let transforms = config::transforms(transforms)?;
    let mut result = py.detach(|| {
        with_output(voxel, map_dtype, sink, sink_factory, |output| {
            radiomics::extract(
                &image,
                &mask,
                &settings,
                &transforms,
                spacing.as_deref().filter(|axes| !axes.is_empty()),
                output,
            )
        })
    })?;
    result.metadata.array_dtype = source_dtype;
    python_result(py, result)
}

#[pyfunction]
#[pyo3(signature=(image_path, mask_path, settings, image_settings, transforms, voxel, map_dtype, sink=None, sink_factory=None))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn extract_files<'py>(
    py: Python<'py>,
    image_path: PathBuf,
    mask_path: PathBuf,
    settings: &Bound<'_, PyAny>,
    image_settings: &Bound<'_, PyAny>,
    transforms: &Bound<'_, PyAny>,
    voxel: bool,
    map_dtype: &str,
    sink: Option<Py<PyAny>>,
    sink_factory: Option<Py<PyAny>>,
) -> PyResult<(Bound<'py, PyDict>, Bound<'py, PyAny>)> {
    let settings = config::settings(settings)?;
    let spatial = config::image_settings(image_settings)?;
    let transforms = config::transforms(transforms)?;
    let result = py.detach(|| {
        with_output(voxel, map_dtype, sink, sink_factory, |output| {
            radiomics::extract_files(
                &image_path,
                &mask_path,
                &settings,
                &spatial,
                &transforms,
                output,
            )
        })
    })?;
    python_result(py, result)
}

// index, voxels, lower, upper, features, metadata
type PyLesion<'py> = (
    usize,
    usize,
    Vec<usize>,
    Vec<usize>,
    Bound<'py, PyDict>,
    Bound<'py, PyAny>,
);

fn python_lesions<'py>(
    py: Python<'py>,
    lesions: Vec<radiomics::Lesion>,
    source_dtype: Option<&str>,
) -> PyResult<Vec<PyLesion<'py>>> {
    lesions
        .into_iter()
        .map(|mut lesion| {
            if let Some(dtype) = source_dtype {
                lesion.result.metadata.array_dtype = dtype.to_owned();
            }
            let (features, metadata) = python_result(py, lesion.result)?;
            Ok((
                lesion.index,
                lesion.voxels,
                lesion.lower,
                lesion.upper,
                features,
                metadata,
            ))
        })
        .collect()
}

#[pyfunction]
pub(crate) fn label_lesions<'py>(
    py: Python<'py>,
    mask: &Bound<'py, PyUntypedArray>,
    label: i64,
    min_voxels: usize,
    max_lesions: Option<usize>,
) -> PyResult<Bound<'py, PyAny>> {
    let mask = arrays::host(mask)?;
    let options = LesionOptions {
        min_voxels,
        max_lesions,
    };
    let labels = py
        .detach(|| radiomics::label_lesions(&mask, label, &options))
        .map_err(crate::map_error)?;
    Ok(labels.into_pyarray(py).into_any())
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub(crate) fn extract_lesions<'py>(
    py: Python<'py>,
    image: &Bound<'py, PyUntypedArray>,
    mask: &Bound<'py, PyUntypedArray>,
    settings: &Bound<'_, PyAny>,
    transforms: &Bound<'_, PyAny>,
    spacing: Option<Vec<f64>>,
    min_voxels: usize,
    max_lesions: Option<usize>,
) -> PyResult<Vec<PyLesion<'py>>> {
    let source_dtype = image.dtype().str()?.to_str()?.to_owned();
    let image = arrays::host(image)?;
    let mask = arrays::host(mask)?;
    let settings = config::settings(settings)?;
    let transforms = config::transforms(transforms)?;
    let options = LesionOptions {
        min_voxels,
        max_lesions,
    };
    let lesions = py
        .detach(|| {
            radiomics::extract_lesions(
                &image,
                &mask,
                &settings,
                &transforms,
                spacing.as_deref().filter(|axes| !axes.is_empty()),
                &options,
            )
        })
        .map_err(crate::map_error)?;
    python_lesions(py, lesions, Some(&source_dtype))
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub(crate) fn extract_lesions_files<'py>(
    py: Python<'py>,
    image_path: PathBuf,
    mask_path: PathBuf,
    settings: &Bound<'_, PyAny>,
    image_settings: &Bound<'_, PyAny>,
    transforms: &Bound<'_, PyAny>,
    min_voxels: usize,
    max_lesions: Option<usize>,
) -> PyResult<Vec<PyLesion<'py>>> {
    let settings = config::settings(settings)?;
    let spatial = config::image_settings(image_settings)?;
    let transforms = config::transforms(transforms)?;
    let options = LesionOptions {
        min_voxels,
        max_lesions,
    };
    let lesions = py
        .detach(|| {
            radiomics::extract_lesions_files(
                &image_path,
                &mask_path,
                &settings,
                &spatial,
                &transforms,
                &options,
            )
        })
        .map_err(crate::map_error)?;
    python_lesions(py, lesions, None)
}

fn map_export(py: Python<'_>, saved: radiomics::MapExport) -> PyResult<Bound<'_, PyAny>> {
    Ok(pythonize::pythonize_custom::<Payload, _>(
        py,
        &(saved.report, saved.maps, saved.metadata),
    )?)
}

#[pyfunction]
#[pyo3(signature=(image, mask, output_dir, settings, transforms, spacing, allow_synthetic_geometry, overwrite, scratch_dir, max_scratch_mib))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn export<'py>(
    py: Python<'py>,
    image: &Bound<'py, PyUntypedArray>,
    mask: &Bound<'py, PyUntypedArray>,
    output_dir: PathBuf,
    settings: &Bound<'_, PyAny>,
    transforms: &Bound<'_, PyAny>,
    spacing: Option<Vec<f64>>,
    allow_synthetic_geometry: bool,
    overwrite: bool,
    scratch_dir: Option<PathBuf>,
    max_scratch_mib: Option<u64>,
) -> PyResult<Bound<'py, PyAny>> {
    use radiomics::__private;
    use radiomics::ExportOptions;
    let options = ExportOptions {
        settings: Some(config::settings(settings)?),
        transforms: Some(config::transforms(transforms)?),
        spacing,
        allow_synthetic_geometry,
        overwrite,
        scratch_dir,
        max_scratch_mib,
    };
    let dtype = image.dtype().str()?.to_str()?.to_owned();
    let provenance = ExportProvenance {
        array_dtype: Some(&dtype),
        python_version: Some(python_version()),
    };
    let image = arrays::host(image)?;
    let mask = arrays::host(mask)?;
    let saved = py
        .detach(|| {
            __private::export_with_provenance(&image, &mask, &output_dir, &options, provenance)
        })
        .map_err(crate::map_error)?;
    map_export(py, saved)
}

#[pyfunction]
#[pyo3(signature=(image_path, mask_path, output_dir, settings, image_settings, transforms, overwrite, scratch_dir, max_scratch_mib))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn export_files<'py>(
    py: Python<'py>,
    image_path: PathBuf,
    mask_path: PathBuf,
    output_dir: PathBuf,
    settings: &Bound<'_, PyAny>,
    image_settings: &Bound<'_, PyAny>,
    transforms: &Bound<'_, PyAny>,
    overwrite: bool,
    scratch_dir: Option<PathBuf>,
    max_scratch_mib: Option<u64>,
) -> PyResult<Bound<'py, PyAny>> {
    use radiomics::__private;
    use radiomics::ExportFilesOptions;
    let options = ExportFilesOptions {
        settings: Some(config::settings(settings)?),
        image_settings: Some(config::image_settings(image_settings)?),
        transforms: Some(config::transforms(transforms)?),
        overwrite,
        scratch_dir,
        max_scratch_mib,
    };
    let version = python_version();
    let saved = py
        .detach(|| {
            __private::export_files_with_runtime(
                &image_path,
                &mask_path,
                &output_dir,
                &options,
                Some(version),
            )
        })
        .map_err(crate::map_error)?;
    map_export(py, saved)
}
