# Profile: voxel/synthetic-10m-defaults-r1

- Command: `target/bench-voxel/final/benchmark_voxel synthetic - 1 1 defaults`
- Commit: `b144fec` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:10 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 12101 (idle worker-thread samples excluded: 14100)

## Command output

```text
{"median_ms":24984.626375,"repeats":1,"min_ms":24984.626375,"max_ms":24984.626375,"roi_voxels":10100000,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3290 | 27.2% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2855 | 23.6% | `radiomics::voxel::spectrum::spectral_radius` |
| 2190 | 18.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 894 | 7.4% | `_platform_memset` |
| 522 | 4.3% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 458 | 3.8% | `__psynch_mutexwait` |
| 242 | 2.0% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 242 | 2.0% | `radiomics::features::first_order::RegionKernel::finish` |
| 231 | 1.9% | `core::slice::sort::shared::smallsort::small_sort_network` |
| 133 | 1.1% | `radiomics::features::first_order::LevelCounts::add` |
| 104 | 0.9% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 87 | 0.7% | `expm1` |
| 80 | 0.7% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 69 | 0.6% | `__psynch_mutexdrop` |
| 68 | 0.6% | `_platform_memmove` |
| 51 | 0.4% | `radiomics::numerics::reduction::reported_mean` |
| 50 | 0.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 44 | 0.4% | `_xzm_xzone_malloc_tiny` |
| 42 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 39 | 0.3% | `_xzm_free_main` |
| 27 | 0.2% | `_platform_memset_pattern16` |
| 26 | 0.2% | `<deduplicated_symbol>` |
| 24 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 22 | 0.2% | `xzm_malloc_zone_malloc_type_realloc` |
| 18 | 0.1% | `radiomics::features::first_order::RegionKernel::new` |
| 17 | 0.1% | `__bzero` |
| 17 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 16 | 0.1% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 15 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 14 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 12 | 0.1% | `_pthread_cond_wait` |
| 12 | 0.1% | `mach_absolute_time` |
| 12 | 0.1% | `core::slice::sort::unstable::ipnsort` |
| 12 | 0.1% | `__ulock_wake` |
| 11 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 10 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 9 | 0.1% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 9 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 9 | 0.1% | `rayon_core::sleep::Sleep::wake_any_threads` |
| 8 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 8 | 0.1% | `DYLD-STUB$$bzero` |
| 8 | 0.1% | `_realloc` |
| 7 | 0.1% | `log1p` |
| 5 | 0.0% | `_free` |
| 5 | 0.0% | `_malloc_zone_malloc` |
| 5 | 0.0% | `__psynch_cvsignal` |
| 5 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 4 | 0.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 4 | 0.0% | `_platform_memcmp` |
| 4 | 0.0% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 4 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 4 | 0.0% | `_xzm_malloc` |
| 3 | 0.0% | `core::slice::sort::shared::smallsort::insertion_sort_shift_left` |
| 3 | 0.0% | `pthread_mutex_lock` |
| 3 | 0.0% | `DYLD-STUB$$expm1` |
| 2 | 0.0% | `_RNvCsGIExRX8pES_7___rustc14___rust_dealloc` |
| 2 | 0.0% | `DYLD-STUB$$free` |
| 2 | 0.0% | `DYLD-STUB$$memcpy` |
| 2 | 0.0% | `xzm_malloc_zone_size` |
| 2 | 0.0% | `radiomics::native_first_order::statistics::Calculation::new` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 7137 | 59.0% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 3505 | 29.0% | `radiomics::voxel::spectrum::spectral_radius` |
| 1379 | 11.4% | `radiomics::workflows::extraction::extract` |
| 1379 | 11.4% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1379 | 11.4% | `radiomics::workflows::engine::run_prepared` |
| 1377 | 11.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1121 | 9.3% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 813 | 6.7% | `radiomics::features::first_order::RegionKernel::finish` |
| 522 | 4.3% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 508 | 4.2% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 246 | 2.0% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 221 | 1.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 144 | 1.2% | `radiomics::features::first_order::LevelCounts::add` |
| 118 | 1.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 83 | 0.7% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 51 | 0.4% | `radiomics::numerics::reduction::reported_mean` |
| 48 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 27 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 24 | 0.2% | `radiomics::features::first_order::RegionKernel::new` |
| 13 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 5 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 2 | 0.0% | `radiomics::features::first_order::SampleStatistics::new` |
| 2 | 0.0% | `radiomics::native_first_order::statistics::Calculation::new` |
