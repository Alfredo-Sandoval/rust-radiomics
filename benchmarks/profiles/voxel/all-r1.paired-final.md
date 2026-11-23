# Profile: voxel/all-r1.paired-final

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 all`
- Commit: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:29 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 4290 (idle worker-thread samples excluded: 40398)

## Command output

```text
{"median_ms":1291.848375,"repeats":5,"min_ms":1102.960458,"max_ms":1800.841375,"roi_voxels":54639,"maps":147,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1514 | 35.3% | `__bzero` |
| 708 | 16.5% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 282 | 6.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 156 | 3.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 135 | 3.1% | `_kernelrpc_mach_vm_deallocate_trap` |
| 119 | 2.8% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 108 | 2.5% | `_platform_memmove` |
| 105 | 2.4% | `<deduplicated_symbol>` |
| 98 | 2.3% | `_platform_memset` |
| 78 | 1.8% | `_xzm_xzone_malloc_tiny` |
| 57 | 1.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 53 | 1.2% | `radiomics::features::ivh::prepare_ivh` |
| 51 | 1.2% | `mach_absolute_time` |
| 50 | 1.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 48 | 1.1% | `_xzm_free_main` |
| 48 | 1.1% | `radiomics::features::first_order::RegionKernel::finish` |
| 46 | 1.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 43 | 1.0% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 37 | 0.9% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 37 | 0.9% | `_xzm_free` |
| 35 | 0.8% | `__psynch_mutexwait` |
| 29 | 0.7% | `core::slice::sort::unstable::quicksort::quicksort` |
| 27 | 0.6% | `core::iter::traits::iterator::Iterator::try_fold` |
| 25 | 0.6% | `radiomics::features::ivh::ivh_statistics` |
| 23 | 0.5% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 23 | 0.5% | `radiomics::features::first_order::LevelCounts::add` |
| 23 | 0.5% | `radiomics::features::intensity_histogram::compute` |
| 21 | 0.5% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 21 | 0.5% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 19 | 0.4% | `xzm_malloc_zone_malloc_type_realloc` |
| 19 | 0.4% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 16 | 0.4% | `radiomics::features::first_order::LevelCounts::finish` |
| 16 | 0.4% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 15 | 0.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 15 | 0.3% | `__floattidf` |
| 15 | 0.3% | `DYLD-STUB$$memcpy` |
| 14 | 0.3% | `radiomics::numerics::count_statistics::bin_entropy` |
| 12 | 0.3% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 12 | 0.3% | `core::slice::sort::select::partition_at_index` |
| 10 | 0.2% | `_free` |
| 8 | 0.2% | `xzm_malloc_zone_size` |
| 7 | 0.2% | `_malloc_zone_realloc` |
| 7 | 0.2% | `_malloc_zone_malloc` |
| 6 | 0.1% | `log2` |
| 6 | 0.1% | `_xzm_malloc` |
| 5 | 0.1% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 5 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 4 | 0.1% | `DYLD-STUB$$free` |
| 4 | 0.1% | `acos` |
| 4 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 4 | 0.1% | `radiomics::features::intensity_histogram::linear_percentile` |
| 3 | 0.1% | `cos` |
| 3 | 0.1% | `__psynch_mutexdrop` |
| 3 | 0.1% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 2 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 2 | 0.0% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 2 | 0.0% | `???` |
| 2 | 0.0% | `__psynch_cvsignal` |
| 2 | 0.0% | `__floatuntidf` |
| 2 | 0.0% | `radiomics::features::first_order::FirstOrderMapKernel::at` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2209 | 51.5% | `radiomics::workflows::extraction::extract` |
| 2209 | 51.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1842 | 42.9% | `radiomics::workflows::engine::run_prepared` |
| 1786 | 41.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1572 | 36.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 823 | 19.2% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 398 | 9.3% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 367 | 8.6% | `radiomics::imaging::preprocessing::prepare` |
| 288 | 6.7% | `radiomics::features::intensity_histogram::compute` |
| 228 | 5.3% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 184 | 4.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 183 | 4.3% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 159 | 3.7% | `radiomics::features::first_order::RegionKernel::finish` |
| 146 | 3.4% | `radiomics::features::ivh::ivh_statistics` |
| 119 | 2.8% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 115 | 2.7% | `radiomics::imaging::preprocessing::gather` |
| 74 | 1.7% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 53 | 1.2% | `radiomics::features::ivh::prepare_ivh` |
| 46 | 1.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 45 | 1.0% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 44 | 1.0% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 34 | 0.8% | `radiomics::imaging::preprocessing::normalize_values` |
| 24 | 0.6% | `radiomics::features::first_order::LevelCounts::add` |
| 18 | 0.4% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 16 | 0.4% | `radiomics::features::first_order::LevelCounts::finish` |
| 14 | 0.3% | `radiomics::numerics::count_statistics::bin_entropy` |
| 12 | 0.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 7 | 0.2% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 5 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 5 | 0.1% | `radiomics::features::intensity_histogram::linear_percentile` |
| 4 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 1 | 0.0% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 1 | 0.0% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 1 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
