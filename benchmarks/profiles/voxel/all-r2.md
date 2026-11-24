# Profile: voxel/all-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 all`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:20 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 7495 (idle worker-thread samples excluded: 32253)

## Command output

```text
{"median_ms":3352.398667,"repeats":3,"min_ms":2862.713416,"max_ms":5192.451042,"roi_voxels":54639,"maps":147,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3754 | 50.1% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 1534 | 20.5% | `__bzero` |
| 532 | 7.1% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 224 | 3.0% | `core::slice::sort::unstable::quicksort::quicksort` |
| 123 | 1.6% | `_platform_memset` |
| 110 | 1.5% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 97 | 1.3% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 95 | 1.3% | `_xzm_xzone_malloc_tiny` |
| 79 | 1.1% | `_xzm_free_main` |
| 78 | 1.0% | `core::slice::sort::select::partition_at_index` |
| 76 | 1.0% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 71 | 0.9% | `_platform_memmove` |
| 65 | 0.9% | `radiomics::features::first_order::RegionKernel::finish` |
| 61 | 0.8% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 57 | 0.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 47 | 0.6% | `_malloc_zone_realloc` |
| 38 | 0.5% | `radiomics::features::intensity_histogram::compute` |
| 36 | 0.5% | `radiomics::features::first_order::LevelCounts::add` |
| 31 | 0.4% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 27 | 0.4% | `radiomics::features::ivh::ivh_statistics` |
| 24 | 0.3% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 22 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 21 | 0.3% | `xzm_malloc_zone_size` |
| 21 | 0.3% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 18 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 18 | 0.2% | `mach_absolute_time` |
| 16 | 0.2% | `_xzm_free` |
| 16 | 0.2% | `core::slice::sort::unstable::ipnsort` |
| 14 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 11 | 0.1% | `__floattidf` |
| 9 | 0.1% | `<deduplicated_symbol>` |
| 9 | 0.1% | `core::cell::lazy::LazyCell<T,F>::really_init` |
| 8 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 8 | 0.1% | `log2` |
| 8 | 0.1% | `DYLD-STUB$$memcpy` |
| 7 | 0.1% | `_xzm_malloc` |
| 7 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 7 | 0.1% | `default_zone_realloc` |
| 6 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 6 | 0.1% | `xzm_malloc_zone_malloc_type_realloc` |
| 6 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 6 | 0.1% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 5 | 0.1% | `DYLD-STUB$$bzero` |
| 5 | 0.1% | `_realloc` |
| 5 | 0.1% | `radiomics::native_first_order::statistics::FirstOrderFeature::from_short_name` |
| 5 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 5 | 0.1% | `core::slice::sort::shared::pivot::median3_rec` |
| 4 | 0.1% | `__psynch_mutexwait` |
| 4 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 4 | 0.1% | `__udivti3` |
| 3 | 0.0% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 3 | 0.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 3 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 2 | 0.0% | `_xzm_reclaim_mark_used` |
| 2 | 0.0% | `radiomics::features::first_order::RegionKernel::new` |
| 2 | 0.0% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 2 | 0.0% | `_platform_memset_pattern16` |
| 2 | 0.0% | `_platform_memcmp` |
| 2 | 0.0% | `expm1` |
| 2 | 0.0% | `_malloc_zone_malloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3937 | 52.5% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2092 | 27.9% | `radiomics::workflows::extraction::extract` |
| 2092 | 27.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2092 | 27.9% | `radiomics::workflows::engine::run_prepared` |
| 2092 | 27.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1570 | 20.9% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 616 | 8.2% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 565 | 7.5% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 432 | 5.8% | `radiomics::features::first_order::RegionKernel::finish` |
| 354 | 4.7% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 331 | 4.4% | `radiomics::features::intensity_histogram::compute` |
| 195 | 2.6% | `radiomics::features::ivh::ivh_statistics` |
| 119 | 1.6% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 114 | 1.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 76 | 1.0% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 60 | 0.8% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 46 | 0.6% | `radiomics::features::first_order::LevelCounts::add` |
| 21 | 0.3% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 14 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 14 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 9 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 7 | 0.1% | `radiomics::features::first_order::RegionKernel::new` |
| 7 | 0.1% | `radiomics::features::intensity_histogram::linear_percentile` |
| 6 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 5 | 0.1% | `radiomics::native_first_order::statistics::FirstOrderFeature::from_short_name` |
| 2 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 2 | 0.0% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 1 | 0.0% | `radiomics::numerics::reduction::reported_mean` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 1 | 0.0% | `radiomics::numerics::pair_arithmetic::subtract_pairs` |
