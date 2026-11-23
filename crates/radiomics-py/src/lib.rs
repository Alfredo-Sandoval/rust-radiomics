//! CPython interface to the CPU radiomics core. Owned inputs outlive the GIL.
use pyo3::exceptions::{PyMemoryError, PyNotImplementedError, PyOSError};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};
use radiomics::{Error, Family, Mode};
mod arrays;
mod config;
mod parameters;
mod sinks;
mod workflows;

pyo3::import_exception!(rustrad.errors, InvalidInputError);
pyo3::import_exception!(rustrad.errors, ModelExecutionError);
pyo3::import_exception!(rustrad.errors, MissingDependencyError);

fn invalid(message: impl Into<String>) -> PyErr {
    InvalidInputError::new_err(message.into())
}

fn map_error(error: Error) -> PyErr {
    match error {
        Error::InvalidInput(msg) => invalid(msg),
        Error::Execution(msg) => ModelExecutionError::new_err(msg),
        Error::Memory(msg) => PyMemoryError::new_err(msg),
        Error::MissingDependency(msg) => MissingDependencyError::new_err(msg),
        Error::Io(msg) => PyOSError::new_err(msg),
        Error::NotImplemented(stage) => PyNotImplementedError::new_err(stage),
        other => ModelExecutionError::new_err(other.to_string()),
    }
}

#[pyfunction]
fn default_features(py: Python<'_>) -> Vec<String> {
    py.detach(radiomics::default_features)
}

#[pyfunction]
#[pyo3(signature=(families, mode, ndim=None))]
fn list_features(
    py: Python<'_>,
    families: Vec<String>,
    mode: &str,
    ndim: Option<u8>,
) -> PyResult<Vec<String>> {
    let mode = match mode {
        "roi" => Mode::Roi,
        "voxel" => Mode::Voxel,
        _ => return Err(invalid("mode must be 'roi' or 'voxel'")),
    };
    let names: Vec<_> = families.iter().map(String::as_str).collect();
    match py.detach(|| radiomics::list_features(&names, mode, ndim)) {
        // Format the Python list and tuple with Python's own string quoting.
        Err(Error::InvalidInput(_)) if names.iter().any(|name| Family::parse(name).is_none()) => {
            let unknown: std::collections::BTreeSet<_> = names
                .into_iter()
                .filter(|name| Family::parse(name).is_none())
                .collect();
            let unknown = PyList::new(py, unknown)?.repr()?;
            let choices = PyTuple::new(py, Family::ALL.map(Family::as_str))?.repr()?;
            Err(invalid(format!(
                "unknown feature families: {unknown}; choose from {choices}"
            )))
        }
        result => result.map_err(map_error),
    }
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(list_features, m)?)?;
    m.add_function(wrap_pyfunction!(default_features, m)?)?;
    m.add_function(wrap_pyfunction!(crate::config::validate_settings, m)?)?;
    m.add_function(wrap_pyfunction!(crate::config::validate_block, m)?)?;
    m.add_function(wrap_pyfunction!(crate::parameters::load_params, m)?)?;
    m.add_function(wrap_pyfunction!(
        crate::parameters::settings_from_params,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(crate::parameters::parse_params, m)?)?;
    m.add_function(wrap_pyfunction!(crate::workflows::extract, m)?)?;
    m.add_function(wrap_pyfunction!(crate::workflows::extract_files, m)?)?;
    m.add_function(wrap_pyfunction!(crate::workflows::export, m)?)?;
    m.add_function(wrap_pyfunction!(crate::workflows::export_files, m)?)?;
    Ok(())
}
