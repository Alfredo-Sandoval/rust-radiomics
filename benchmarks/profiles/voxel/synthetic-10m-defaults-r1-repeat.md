# Profile: voxel/synthetic-10m-defaults-r1-repeat

- Command: `target/bench-voxel/final/benchmark_voxel synthetic - 3 1 defaults`
- Commit: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:29 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 11629 (idle worker-thread samples excluded: 16529)

## Command output

```text
{"median_ms":13782.660709,"repeats":3,"min_ms":10260.423959,"max_ms":14137.407334000001,"roi_voxels":10100000,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 4736 | 40.7% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2189 | 18.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 740 | 6.4% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 710 | 6.1% | `_platform_memset` |
| 482 | 4.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 317 | 2.7% | `core::slice::sort::unstable::quicksort::quicksort` |
| 243 | 2.1% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 225 | 1.9% | `radiomics::features::first_order::RegionKernel::finish` |
| 212 | 1.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 203 | 1.7% | `radiomics::features::first_order::LevelCounts::add` |
| 145 | 1.2% | `expm1` |
| 123 | 1.1% | `acos` |
| 115 | 1.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 99 | 0.9% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 89 | 0.8% | `_xzm_free_main` |
| 83 | 0.7% | `_xzm_xzone_malloc_tiny` |
| 71 | 0.6% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 61 | 0.5% | `__psynch_mutexwait` |
| 47 | 0.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 47 | 0.4% | `cos` |
| 40 | 0.3% | `_platform_memmove` |
| 38 | 0.3% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 35 | 0.3% | `radiomics::features::first_order::LevelCounts::finish` |
| 35 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 35 | 0.3% | `DYLD-STUB$$bzero` |
| 33 | 0.3% | `_platform_memset_pattern16` |
| 31 | 0.3% | `core::slice::sort::unstable::ipnsort` |
| 30 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 27 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 23 | 0.2% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 23 | 0.2% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 22 | 0.2% | `_xzm_malloc` |
| 18 | 0.2% | `radiomics::features::first_order::RegionKernel::new` |
| 18 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 17 | 0.1% | `radiomics::native_first_order::statistics::Calculation::new` |
| 15 | 0.1% | `_platform_memcmp` |
| 15 | 0.1% | `xzm_malloc_zone_size` |
| 15 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 14 | 0.1% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 13 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 12 | 0.1% | `_realloc` |
| 12 | 0.1% | `???` |
| 12 | 0.1% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 10 | 0.1% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 10 | 0.1% | `_pthread_cond_wait` |
| 9 | 0.1% | `__bzero` |
| 8 | 0.1% | `_malloc_zone_malloc` |
| 8 | 0.1% | `cthread_yield` |
| 7 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 7 | 0.1% | `__psynch_cvsignal` |
| 7 | 0.1% | `__psynch_mutexdrop` |
| 6 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 6 | 0.1% | `mach_absolute_time` |
| 6 | 0.1% | `rayon::iter::plumbing::Folder::consume_iter` |
| 5 | 0.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 5 | 0.0% | `_malloc_zone_realloc` |
| 5 | 0.0% | `_xzm_xzone_malloc_freelist_outlined` |
| 5 | 0.0% | `pthread_mutex_unlock` |
| 4 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 4 | 0.0% | `DYLD-STUB$$expm1` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 5736 | 49.3% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2970 | 25.5% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1547 | 13.3% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 1482 | 12.7% | `radiomics::workflows::extraction::extract` |
| 1482 | 12.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1482 | 12.7% | `radiomics::workflows::engine::run_prepared` |
| 1481 | 12.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 924 | 7.9% | `radiomics::features::first_order::RegionKernel::finish` |
| 740 | 6.4% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 625 | 5.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 475 | 4.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 408 | 3.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 204 | 1.8% | `radiomics::features::first_order::LevelCounts::add` |
| 184 | 1.6% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 124 | 1.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 104 | 0.9% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 104 | 0.9% | `radiomics::features::first_order::RegionKernel::new` |
| 45 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 39 | 0.3% | `radiomics::features::first_order::LevelCounts::finish` |
| 35 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 17 | 0.1% | `radiomics::features::first_order::SampleStatistics::new` |
| 17 | 0.1% | `radiomics::native_first_order::statistics::Calculation::new` |
| 14 | 0.1% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 4 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 1 | 0.0% | `radiomics::native_first_order::statistics::FirstOrderFeature::from_short_name` |
