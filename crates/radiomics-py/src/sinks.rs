//! W8's synchronous stream/factory interface. Keep Python exceptions intact across Rust.
use crate::{invalid, map_error};
use indexmap::IndexMap;
use numpy::{
    IntoPyArray,
    ndarray::{ArrayView1, ArrayView2},
};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use radiomics::Error;
use radiomics::{MapDtype, VoxelSink};
use radiomics::{Output, SinkFactory};

fn callback_error(slot: &mut Option<PyErr>, error: PyErr) -> Error {
    *slot = Some(error);
    Error::Execution("Python voxel callback failed".into())
}

struct Sink<'a> {
    callback: Py<PyAny>,
    error: &'a mut Option<PyErr>,
}

impl VoxelSink for Sink<'_> {
    fn consume(
        &mut self,
        centers: ArrayView2<'_, i32>,
        columns: &IndexMap<String, ArrayView1<'_, f32>>,
    ) -> radiomics::Result<()> {
        Python::attach(|py| -> PyResult<()> {
            let batch = PyDict::new(py);
            for (key, column) in columns {
                batch.set_item(key, column.to_owned().into_pyarray(py))?;
            }
            self.callback
                .call1(py, (centers.to_owned().into_pyarray(py), batch))?;
            Ok(())
        })
        .map_err(|e| callback_error(self.error, e))
    }
}

struct Factory<'a> {
    factory: Py<PyAny>,
    manager: Option<Py<PyAny>>,
    error: &'a mut Option<PyErr>,
}

impl Factory<'_> {
    fn enter(&mut self, name: &str) -> radiomics::Result<(bool, Sink<'_>)> {
        let (manager, callback, stream) = Python::attach(|py| -> PyResult<_> {
            let manager = py
                .import("rustradiomics._workflows")?
                .getattr("_sink_context")?
                .call1((self.factory.bind(py), name))?;
            let callback = manager.call_method0("__enter__")?;
            let stream = !callback.is_none();
            Ok((manager.unbind(), callback.unbind(), stream))
        })
        .map_err(|e| callback_error(self.error, e))?;
        self.manager = Some(manager);
        Ok((
            stream,
            Sink {
                callback,
                error: self.error,
            },
        ))
    }
}

impl SinkFactory for Factory<'_> {
    fn open(&mut self, name: &str) -> radiomics::Result<Box<dyn VoxelSink + '_>> {
        let (_, sink) = self.enter(name)?;
        Ok(Box::new(sink))
    }

    fn open_optional(&mut self, name: &str) -> radiomics::Result<Option<Box<dyn VoxelSink + '_>>> {
        let (stream, sink) = self.enter(name)?;
        Ok(if stream { Some(Box::new(sink)) } else { None })
    }

    fn close(&mut self, _: &str, error: Option<&Error>) -> radiomics::Result<()> {
        let manager = self
            .manager
            .take()
            .ok_or_else(|| Error::Execution("sink factory closed outside its context".into()))?;
        Python::attach(|py| -> PyResult<()> {
            if let Some(error) = error {
                let err = self
                    .error
                    .as_ref()
                    .map(|e| e.clone_ref(py))
                    .unwrap_or_else(|| map_error(error.clone()));
                manager.call_method1(
                    py,
                    "__exit__",
                    (err.get_type(py), err.value(py), err.traceback(py)),
                )?;
            } else {
                manager.call_method1(py, "__exit__", (py.None(), py.None(), py.None()))?;
            }
            Ok(())
        })
        .map_err(|e| callback_error(self.error, e))
    }
}

pub(crate) fn with_output<T>(
    voxel: bool,
    map_dtype: &str,
    sink: Option<Py<PyAny>>,
    factory: Option<Py<PyAny>>,
    run: impl FnOnce(Output<'_>) -> radiomics::Result<T>,
) -> PyResult<T> {
    let dtype = match map_dtype {
        "float32" => MapDtype::Float32,
        "float64" => MapDtype::Float64,
        _ => return Err(invalid("map_dtype must be 'float64' or 'float32'")),
    };
    let mut error = None;
    let result = if !voxel {
        run(Output::Roi)
    } else if let Some(factory) = factory {
        run(Output::PerImage {
            dtype,
            factory: &mut Factory {
                factory,
                manager: None,
                error: &mut error,
            },
        })
    } else if let Some(callback) = sink {
        run(Output::Stream {
            dtype,
            sink: &mut Sink {
                callback,
                error: &mut error,
            },
        })
    } else {
        run(Output::Maps {
            dtype,
            max_bytes: None,
        })
    };
    match error {
        Some(e) => Err(e),
        None => result.map_err(map_error),
    }
}
