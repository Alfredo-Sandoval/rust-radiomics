# Profile: voxel/synthetic-10m-defaults-r1-confirm

- Command: `target/bench-voxel/final/benchmark_voxel synthetic - 5 1 defaults`
- Commit: `4f42922` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:31 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 11514 (idle worker-thread samples excluded: 14269)

## Command output

```text
{"median_ms":11022.408208,"repeats":5,"min_ms":7732.100334,"max_ms":18235.643875,"roi_voxels":10100000,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 4476 | 38.9% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2170 | 18.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1015 | 8.8% | `__psynch_mutexwait` |
| 599 | 5.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 587 | 5.1% | `_platform_memset` |
| 400 | 3.5% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 224 | 1.9% | `core::slice::sort::unstable::quicksort::quicksort` |
| 205 | 1.8% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 191 | 1.7% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 183 | 1.6% | `radiomics::features::first_order::RegionKernel::finish` |
| 143 | 1.2% | `acos` |
| 135 | 1.2% | `expm1` |
| 113 | 1.0% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 106 | 0.9% | `radiomics::features::first_order::LevelCounts::add` |
| 86 | 0.7% | `_xzm_free_main` |
| 63 | 0.5% | `_xzm_xzone_malloc_tiny` |
| 47 | 0.4% | `_platform_memmove` |
| 45 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 38 | 0.3% | `__psynch_mutexdrop` |
| 35 | 0.3% | `mach_absolute_time` |
| 33 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 32 | 0.3% | `???` |
| 30 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 25 | 0.2% | `__bzero` |
| 23 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 22 | 0.2% | `_pthread_cond_wait` |
| 21 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 20 | 0.2% | `cos` |
| 20 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 20 | 0.2% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 20 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 18 | 0.2% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 18 | 0.2% | `__psynch_cvsignal` |
| 17 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 17 | 0.1% | `_platform_memset_pattern16` |
| 16 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 16 | 0.1% | `xzm_malloc_zone_malloc_type_realloc` |
| 16 | 0.1% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 14 | 0.1% | `<deduplicated_symbol>` |
| 14 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 13 | 0.1% | `DYLD-STUB$$bzero` |
| 12 | 0.1% | `_RNvCsGIExRX8pES_7___rustc14___rust_dealloc` |
| 12 | 0.1% | `_malloc_zone_malloc` |
| 11 | 0.1% | `_xzm_xzone_malloc_from_freelist_chunk` |
| 9 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 8 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 8 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 8 | 0.1% | `_xzm_free` |
| 8 | 0.1% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 7 | 0.1% | `_free` |
| 7 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 7 | 0.1% | `_xzm_chunk_list_pop` |
| 7 | 0.1% | `DYLD-STUB$$expm1` |
| 6 | 0.1% | `crossbeam_epoch::internal::Global::try_advance` |
| 6 | 0.1% | `rayon_core::sleep::Sleep::wake_specific_thread` |
| 5 | 0.0% | `radiomics::features::first_order::RegionKernel::new` |
| 5 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 5 | 0.0% | `_platform_memcmp` |
| 5 | 0.0% | `xzm_malloc_zone_size` |
| 5 | 0.0% | `_tlv_get_addr` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 5387 | 46.8% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2778 | 24.1% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1357 | 11.8% | `radiomics::workflows::extraction::extract` |
| 1357 | 11.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1357 | 11.8% | `radiomics::workflows::engine::run_prepared` |
| 1354 | 11.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 964 | 8.4% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 676 | 5.9% | `radiomics::features::first_order::RegionKernel::finish` |
| 599 | 5.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 442 | 3.8% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 249 | 2.2% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 232 | 2.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 206 | 1.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 108 | 0.9% | `radiomics::features::first_order::LevelCounts::add` |
| 73 | 0.6% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 69 | 0.6% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 40 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 39 | 0.3% | `radiomics::features::first_order::RegionKernel::new` |
| 33 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 18 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 5 | 0.0% | `radiomics::features::first_order::RegionKernel::new::_{{closure}}` |
| 4 | 0.0% | `radiomics::native_first_order::statistics::FirstOrderFeature::from_short_name` |
| 2 | 0.0% | `radiomics::features::first_order::SampleStatistics::new` |
| 2 | 0.0% | `radiomics::native_first_order::statistics::Calculation::new` |
| 2 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 2 | 0.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
