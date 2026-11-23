//! CPU allocation checks corresponding to `mlxradiomics/numerics/runtime.py`.
//! The caller supplies its retained-map budget in bytes. Native temporary arrays
//! release their storage when they drop.

use crate::{Error, Result};

/// Full-sized, writable arrays for retained voxel maps. A large reused malloc
/// block on macOS can otherwise fault in every page just to clear it.
pub(crate) fn zeroed_map<T: num_traits::Zero + Copy + 'static>(
    shape: &[usize],
) -> ndarray::ArrayD<T> {
    #[cfg(target_os = "macos")]
    {
        use std::any::TypeId;
        let count = shape.iter().product::<usize>();
        // Only IEEE f32 and f64 have been admitted to bytewise initialization here.
        if (TypeId::of::<T>() == TypeId::of::<f32>() || TypeId::of::<T>() == TypeId::of::<f64>())
            && count >= (1 << 20) / size_of::<T>()
        {
            let mut values = Vec::<T>::with_capacity(count);
            let bytes = count * size_of::<T>();
            // SAFETY: the allocation owns at least `bytes` writable bytes; all-zero
            // bits are +0 for these two concrete types. Publish the length only
            // after initializing every byte, including both partial boundary pages.
            #[allow(unsafe_code)]
            unsafe {
                clear_map_bytes(values.as_mut_ptr().cast(), bytes);
                values.set_len(count);
            }
            return ndarray::ArrayD::from_shape_vec(ndarray::IxDyn(shape), values)
                .expect("validated retained shape");
        }
    }
    ndarray::ArrayD::zeros(ndarray::IxDyn(shape))
}

/// Initialize a writable allocation without touching absent interior pages.
///
/// # Safety
/// `ptr` must exclusively own at least `len` writable bytes.
#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
unsafe fn clear_map_bytes(ptr: *mut u8, len: usize) {
    static PAGE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    let page = *PAGE.get_or_init(|| {
        // SAFETY: sysconf takes no pointers.
        usize::try_from(unsafe { libc::sysconf(libc::_SC_PAGESIZE) }).unwrap_or(0)
    });
    if page.is_power_of_two() {
        let head = ptr.align_offset(page).min(len);
        let middle = (len - head) / page * page;
        // SAFETY: only complete pages strictly owned by this buffer are passed to
        // Darwin. MADV_ZERO initializes their contents, including previously dirty pages.
        if middle != 0
            && unsafe { libc::madvise(ptr.add(head).cast(), middle, libc::MADV_ZERO) } == 0
        {
            unsafe {
                ptr.write_bytes(0, head);
                ptr.add(head + middle).write_bytes(0, len - head - middle);
            }
            return;
        }
    }
    // Older kernels or unsupported mappings can reject the advice.
    unsafe { ptr.write_bytes(0, len) };
}

/// Required bytes for `maps` dense grids with `voxels` elements of type `T`.
pub fn retained_map_bytes<T>(voxels: usize, maps: usize) -> Result<usize> {
    voxels
        .checked_mul(maps)
        .and_then(|n| n.checked_mul(size_of::<T>()))
        .ok_or_else(|| {
            Error::Memory("retained voxel map dimensions overflow addressable memory".into())
        })
}

/// Check a caller's retained-output budget before allocating any output maps.
pub fn require_retained_capacity<T>(voxels: usize, maps: usize, budget: usize) -> Result<()> {
    let bytes = retained_map_bytes::<T>(voxels, maps)?;
    if bytes > budget {
        return Err(Error::Memory(format!(
            "retaining {maps} maps requires {bytes} bytes, exceeding the {budget}-byte retained-map budget"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::numerics::runtime::*;
    #[cfg(target_os = "macos")]
    #[test]
    fn page_zeroing_clears_dirty_interior_and_preserves_adjacent_bytes() {
        let mut bytes = vec![0xa5u8; (2 << 20) + 97];
        let last = bytes.len() - 19;
        let range = &mut bytes[17..last];
        // SAFETY: the entire range is writable and exclusively borrowed.
        #[allow(unsafe_code)]
        unsafe {
            clear_map_bytes(range.as_mut_ptr(), range.len())
        };
        assert!(bytes[..17].iter().all(|&b| b == 0xa5));
        assert!(bytes[17..last].iter().all(|&b| b == 0));
        assert!(bytes[last..].iter().all(|&b| b == 0xa5));
    }

    #[test]
    fn map_type_count_and_budget_are_checked_before_allocation() {
        assert!(require_retained_capacity::<f32>(3, 2, 24).is_ok());
        assert!(require_retained_capacity::<f64>(3, 2, 24).is_err());
        assert!(retained_map_bytes::<f64>(usize::MAX, 2).is_err());
        assert_eq!(retained_map_bytes::<f64>(usize::MAX, 0).unwrap(), 0);
    }
}
