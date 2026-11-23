# Profile: voxel/all-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 all`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:18 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 4251 (idle worker-thread samples excluded: 39848)

## Command output

```text
{"median_ms":4536.513416,"repeats":3,"min_ms":3495.6226250000004,"max_ms":5302.406333000001,"roi_voxels":54639,"maps":147,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2048 | 48.2% | `__bzero` |
| 824 | 19.4% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 286 | 6.7% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 110 | 2.6% | `_platform_memset` |
| 73 | 1.7% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 70 | 1.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 63 | 1.5% | `mach_absolute_time` |
| 63 | 1.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 50 | 1.2% | `_xzm_free_main` |
| 44 | 1.0% | `_platform_memmove` |
| 44 | 1.0% | `radiomics::features::first_order::RegionKernel::finish` |
| 40 | 0.9% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 40 | 0.9% | `__psynch_mutexdrop` |
| 38 | 0.9% | `core::slice::sort::select::partition_at_index` |
| 30 | 0.7% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 29 | 0.7% | `_xzm_xzone_malloc_tiny` |
| 27 | 0.6% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 25 | 0.6% | `radiomics::features::intensity_histogram::compute` |
| 25 | 0.6% | `_xzm_free` |
| 22 | 0.5% | `radiomics::features::intensity_histogram::linear_percentile` |
| 20 | 0.5% | `_malloc_zone_malloc` |
| 18 | 0.4% | `radiomics::features::ivh::ivh_statistics` |
| 18 | 0.4% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 17 | 0.4% | `<deduplicated_symbol>` |
| 16 | 0.4% | `radiomics::features::first_order::LevelCounts::finish` |
| 16 | 0.4% | `__psynch_mutexwait` |
| 15 | 0.4% | `xzm_malloc_zone_malloc_type_realloc` |
| 15 | 0.4% | `log2` |
| 15 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 11 | 0.3% | `__udivti3` |
| 10 | 0.2% | `_realloc` |
| 9 | 0.2% | `_<core::iter::adapters::GenericShunt<I,R> as core::iter::traits::iterator::Iterator>::next` |
| 8 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 7 | 0.2% | `_xzm_malloc` |
| 7 | 0.2% | `core::slice::sort::unstable::quicksort::quicksort` |
| 7 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 6 | 0.1% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 5 | 0.1% | `acos` |
| 5 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 4 | 0.1% | `DYLD-STUB$$free` |
| 4 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 4 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 4 | 0.1% | `expm1` |
| 4 | 0.1% | `DYLD-STUB$$_platform_memmove` |
| 4 | 0.1% | `_xzm_free_outlined` |
| 3 | 0.1% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 3 | 0.1% | `xzm_malloc_zone_size` |
| 3 | 0.1% | `_free` |
| 3 | 0.1% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 3 | 0.1% | `DYLD-STUB$$mach_absolute_time` |
| 3 | 0.1% | `DYLD-STUB$$malloc` |
| 2 | 0.0% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 2 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 2 | 0.0% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 2 | 0.0% | `__floattidf` |
| 2 | 0.0% | `_malloc_zone_realloc` |
| 2 | 0.0% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 2 | 0.0% | `DYLD-STUB$$bzero` |
| 2 | 0.0% | `rayon_core::registry::WorkerThread::find_work` |
| 1 | 0.0% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2321 | 54.6% | `radiomics::workflows::extraction::extract` |
| 2321 | 54.6% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2321 | 54.6% | `radiomics::workflows::engine::run_prepared` |
| 2321 | 54.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 2103 | 49.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 962 | 22.6% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 355 | 8.4% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 336 | 7.9% | `radiomics::features::intensity_histogram::compute` |
| 153 | 3.6% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 130 | 3.1% | `radiomics::features::ivh::ivh_statistics` |
| 110 | 2.6% | `radiomics::features::first_order::RegionKernel::finish` |
| 73 | 1.7% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 44 | 1.0% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 39 | 0.9% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 38 | 0.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 31 | 0.7% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 31 | 0.7% | `radiomics::features::intensity_histogram::linear_percentile` |
| 21 | 0.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 17 | 0.4% | `radiomics::features::first_order::LevelCounts::finish` |
| 15 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 10 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 8 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 4 | 0.1% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 1 | 0.0% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 1 | 0.0% | `radiomics::features::first_order::RegionKernel::new` |
| 1 | 0.0% | `radiomics::native_first_order::statistics::FirstOrderFeature::from_short_name` |
