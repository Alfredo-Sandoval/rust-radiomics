//! Bridge ordered parameter mappings to the Rust YAML loader and extraction parser.
use crate::{invalid, map_error};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use radiomics::params::{self as core_parameters, Value};
use std::path::PathBuf;

#[pyfunction]
pub(crate) fn load_params(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    let value = py
        .detach(|| core_parameters::load_params(&path))
        .map_err(map_error)?;
    yaml_object(py, &value).map_err(|err| {
        if err.is_instance_of::<PyTypeError>(py) {
            invalid(format!("invalid params YAML: {}: {err}", path.display()))
        } else {
            err
        }
    })
}

// A raw YAML document exposes mutable lists and ordinary Python scalar types.
// The parsed Settings payload instead uses tuples for its frozen fields.
fn yaml_object<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, PyAny>> {
    match value {
        Value::Mapping(mapping) => {
            let out = PyDict::new(py);
            for (key, item) in mapping {
                out.set_item(yaml_object(py, key)?, yaml_object(py, item)?)?;
            }
            Ok(out.into_any())
        }
        Value::Sequence(sequence) => {
            let out = PyList::empty(py);
            for item in sequence {
                out.append(yaml_object(py, item)?)?;
            }
            Ok(out.into_any())
        }
        Value::Tagged(tagged) => {
            let raw = tagged
                .value
                .as_str()
                .ok_or_else(|| invalid("expected a tagged scalar"))?;
            if tagged.tag == "timestamp" {
                py.import("rustrad.config.parameters")?
                    .getattr("_timestamp")?
                    .call1((raw,))
            } else if tagged.tag == "binary" {
                py.import("base64")?.getattr("b64decode")?.call1((raw,))
            } else {
                Err(invalid(format!("unsupported parameter tag {}", tagged.tag)))
            }
        }
        _ => Ok(pythonize::pythonize(py, value)?),
    }
}

#[pyfunction]
pub(crate) fn settings_from_params<'py>(
    py: Python<'py>,
    params: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let value = pythonize::depythonize(params).map_err(|e| invalid(e.to_string()))?;
    let settings = py
        .detach(|| core_parameters::settings_from_params(&value, None))
        .map_err(map_error)?;
    Ok(pythonize::pythonize_custom::<crate::workflows::Payload, _>(
        py, &settings,
    )?)
}

#[pyfunction]
pub(crate) fn parse_params<'py>(
    py: Python<'py>,
    params: &Bound<'py, PyAny>,
    ndim: usize,
) -> PyResult<Bound<'py, PyAny>> {
    let value = pythonize::depythonize(params).map_err(|e| invalid(e.to_string()))?;
    let params = py
        .detach(|| core_parameters::parse_params(&value, ndim))
        .map_err(map_error)?;
    Ok(pythonize::pythonize_custom::<crate::workflows::Payload, _>(
        py,
        &(params.settings, params.image_settings, params.transforms),
    )?)
}
