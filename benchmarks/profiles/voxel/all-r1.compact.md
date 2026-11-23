# Profile: voxel/all-r1.compact

- Command: `target/bench-voxel/compact_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 all`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:01 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 3574 (idle worker-thread samples excluded: 38435)

## Command output

```text
{"median_ms":3072.224625,"repeats":3,"min_ms":2975.620833,"max_ms":3528.979041,"roi_voxels":54639,"maps":147,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1194 | 33.4% | `__bzero` |
| 451 | 12.6% | `radiomics::voxel::spectrum::spectral_radius` |
| 421 | 11.8% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 296 | 8.3% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 130 | 3.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 106 | 3.0% | `_kernelrpc_mach_vm_deallocate_trap` |
| 94 | 2.6% | `<deduplicated_symbol>` |
| 74 | 2.1% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 70 | 2.0% | `_platform_memmove` |
| 70 | 2.0% | `__psynch_mutexwait` |
| 61 | 1.7% | `_platform_memset` |
| 55 | 1.5% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 40 | 1.1% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 36 | 1.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 35 | 1.0% | `_xzm_free_main` |
| 34 | 1.0% | `_xzm_xzone_malloc_tiny` |
| 34 | 1.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 32 | 0.9% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 27 | 0.8% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 26 | 0.7% | `mach_absolute_time` |
| 24 | 0.7% | `radiomics::features::ivh::prepare_ivh` |
| 20 | 0.6% | `_xzm_free` |
| 16 | 0.4% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 16 | 0.4% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 14 | 0.4% | `radiomics::features::ivh::ivh_statistics` |
| 13 | 0.4% | `core::iter::traits::iterator::Iterator::try_fold` |
| 13 | 0.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 12 | 0.3% | `__floatuntidf` |
| 10 | 0.3% | `core::slice::sort::select::partition_at_index` |
| 10 | 0.3% | `radiomics::features::intensity_histogram::compute` |
| 10 | 0.3% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 9 | 0.3% | `_realloc` |
| 9 | 0.3% | `expm1` |
| 7 | 0.2% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 7 | 0.2% | `radiomics::features::first_order::RegionKernel::finish` |
| 6 | 0.2% | `__floattidf` |
| 6 | 0.2% | `xzm_malloc_zone_malloc_type_realloc` |
| 4 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 4 | 0.1% | `DYLD-STUB$$malloc` |
| 3 | 0.1% | `__psynch_cvsignal` |
| 3 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 3 | 0.1% | `radiomics::features::first_order::LevelCounts::add` |
| 3 | 0.1% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 3 | 0.1% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 3 | 0.1% | `_malloc_zone_malloc` |
| 2 | 0.1% | `log2` |
| 2 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 2 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 2 | 0.1% | `_RNvCsGIExRX8pES_7___rustc12___rust_alloc` |
| 2 | 0.1% | `xzm_malloc_zone_try_free_default_main` |
| 2 | 0.1% | `_platform_memset_pattern16` |
| 2 | 0.1% | `radiomics::features::intensity_histogram::linear_percentile` |
| 2 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 2 | 0.1% | `_free` |
| 2 | 0.1% | `_pthread_cond_wait` |
| 2 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 1 | 0.0% | `_xzm_segment_group_init_segment` |
| 1 | 0.0% | `calloc` |
| 1 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 1 | 0.0% | `hashbrown::raw::RawTable<T,A>::insert` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2097 | 58.7% | `radiomics::workflows::extraction::extract` |
| 2097 | 58.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1748 | 48.9% | `radiomics::workflows::engine::run_prepared` |
| 1400 | 39.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1238 | 34.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 813 | 22.7% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 498 | 13.9% | `radiomics::voxel::spectrum::spectral_radius` |
| 349 | 9.8% | `radiomics::imaging::preprocessing::prepare` |
| 265 | 7.4% | `radiomics::features::ivh::prepare_ivh` |
| 216 | 6.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 183 | 5.1% | `radiomics::features::intensity_histogram::compute` |
| 171 | 4.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 146 | 4.1% | `radiomics::imaging::preprocessing::gather` |
| 133 | 3.7% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 95 | 2.7% | `radiomics::features::ivh::ivh_statistics` |
| 48 | 1.3% | `radiomics::imaging::preprocessing::normalize_values` |
| 40 | 1.1% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 40 | 1.1% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 29 | 0.8% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 28 | 0.8% | `radiomics::features::first_order::RegionKernel::finish` |
| 17 | 0.5% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 15 | 0.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 8 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 8 | 0.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 7 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 5 | 0.1% | `radiomics::features::first_order::LevelCounts::add` |
| 4 | 0.1% | `radiomics::features::intensity_histogram::linear_percentile` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 3 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 3 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 2 | 0.1% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 2 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `radiomics::numerics::reduction::reported_mean` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 1 | 0.0% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 1 | 0.0% | `radiomics::features::first_order::RegionKernel::new` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
