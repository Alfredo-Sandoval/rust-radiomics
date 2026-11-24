//! Copy NumPy arrays into native-endian, owned storage before releasing the GIL.
use crate::invalid;
use numpy::{
    Element, PyArrayDescrMethods, PyArrayDyn, PyArrayMethods, PyUntypedArray,
    PyUntypedArrayMethods,
    ndarray::{ArrayD, ArrayViewD},
};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use radiomics::HostArray;
use rayon::prelude::*;

fn contiguous<'py, T: Element>(
    array: &Bound<'py, PyUntypedArray>,
) -> PyResult<Bound<'py, PyArrayDyn<T>>> {
    // NumPy handles negative strides, byte order and unaligned buffer offsets.
    // require() also strips subclasses before creating a Rust view.
    let py = array.py();
    let kwargs = PyDict::new(py);
    kwargs.set_item("dtype", numpy::dtype::<T>(py))?;
    kwargs.set_item("requirements", ["C", "A", "E"])?;
    Ok(py
        .import("numpy")?
        .call_method("require", (array,), Some(&kwargs))?
        .cast_into::<PyArrayDyn<T>>()?)
}

// Keep the GIL until the snapshot is complete. Workers only read primitive
// values and write disjoint ranges of the new allocation.
fn parallel_snapshot<T: Sync, U: Send>(
    view: &ArrayViewD<'_, T>,
    convert: impl Fn(&T) -> U + Send + Sync,
) -> Option<ArrayD<U>> {
    let values = view.as_slice()?;
    if std::mem::size_of_val(values) < 8 * 1024 * 1024 || rayon::current_num_threads() == 1 {
        return None;
    }
    let owned = values
        .par_iter()
        .with_min_len(256 * 1024 / size_of::<T>())
        .map(convert)
        .collect();
    Some(ArrayD::from_shape_vec(view.raw_dim(), owned).expect("one value per source element"))
}

fn copy_as<T: Element + Copy + Send + Sync>(
    array: &Bound<'_, PyUntypedArray>,
) -> PyResult<ArrayD<T>> {
    let array = contiguous::<T>(array)?;
    let input = array.try_readonly()?;
    let view = input.as_array();
    Ok(parallel_snapshot(&view, |&v| v).unwrap_or_else(|| view.to_owned()))
}

fn bool_mask(array: &Bound<'_, PyUntypedArray>) -> PyResult<ArrayD<bool>> {
    // NumPy bools can contain any nonzero byte. Read them as u8 and construct
    // valid Rust bools in the destination, including for readonly NumPy inputs.
    let bytes = contiguous::<bool>(array)?
        .call_method1("view", (numpy::dtype::<u8>(array.py()),))?
        .cast_into::<PyArrayDyn<u8>>()?;
    let input = bytes.try_readonly()?;
    let view = input.as_array();
    let is_true = |&byte: &u8| byte != 0;
    Ok(parallel_snapshot(&view, is_true).unwrap_or_else(|| view.map(is_true)))
}

/// Preserve integer source types through core label selection and precision checks.
pub(crate) fn host(array: &Bound<'_, PyUntypedArray>) -> PyResult<HostArray> {
    let dtype = array.dtype();
    Ok(match (dtype.kind(), dtype.itemsize()) {
        (b'b', 1) => HostArray::Bool(bool_mask(array)?),
        (b'i', 1) => HostArray::I8(copy_as(array)?),
        (b'u', 1) => HostArray::U8(copy_as(array)?),
        (b'i', 2) => HostArray::I16(copy_as(array)?),
        (b'u', 2) => HostArray::U16(copy_as(array)?),
        (b'i', 4) => HostArray::I32(copy_as(array)?),
        (b'u', 4) => HostArray::U32(copy_as(array)?),
        (b'i', 8) => HostArray::I64(copy_as(array)?),
        (b'u', 8) => HostArray::U64(copy_as(array)?),
        (b'f', 2 | 4) => HostArray::F32(copy_as(array)?),
        (b'f', 8) => HostArray::F64(copy_as(array)?),
        (b'f', _) => {
            return Err(invalid(
                "image floating dtype exceeds float64 precision; convert the image explicitly",
            ));
        }
        _ => {
            return Err(invalid(
                "image must be real numeric and mask must contain integer labels",
            ));
        }
    })
}
