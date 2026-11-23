# Profile: voxel/intensityhistogram-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 intensityhistogram`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:19 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2859 (idle worker-thread samples excluded: 32044)

## Command output

```text
{"median_ms":1270.8824585000002,"repeats":4,"min_ms":1121.564459,"max_ms":1616.696583,"roi_voxels":54639,"maps":23,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 476 | 16.6% | `__bzero` |
| 367 | 12.8% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 252 | 8.8% | `_platform_memmove` |
| 200 | 7.0% | `<deduplicated_symbol>` |
| 194 | 6.8% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 148 | 5.2% | `radiomics::features::intensity_histogram::compute` |
| 134 | 4.7% | `_xzm_xzone_malloc_tiny` |
| 121 | 4.2% | `mach_absolute_time` |
| 102 | 3.6% | `_xzm_free_main` |
| 99 | 3.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 94 | 3.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 85 | 3.0% | `_xzm_free` |
| 57 | 2.0% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 54 | 1.9% | `__floattidf` |
| 32 | 1.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 30 | 1.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 30 | 1.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 30 | 1.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 26 | 0.9% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 26 | 0.9% | `crossbeam_epoch::default::with_handle` |
| 25 | 0.9% | `core::iter::traits::iterator::Iterator::try_fold` |
| 23 | 0.8% | `_free` |
| 22 | 0.8% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 16 | 0.6% | `_malloc_zone_malloc` |
| 16 | 0.6% | `radiomics::features::intensity_histogram::linear_percentile` |
| 16 | 0.6% | `_realloc` |
| 14 | 0.5% | `log2` |
| 14 | 0.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 14 | 0.5% | `__psynch_mutexwait` |
| 13 | 0.5% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 13 | 0.5% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 11 | 0.4% | `_xzm_malloc` |
| 11 | 0.4% | `__udivti3` |
| 9 | 0.3% | `free` |
| 9 | 0.3% | `xzm_malloc_zone_size` |
| 7 | 0.2% | `_malloc_zone_realloc` |
| 7 | 0.2% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 6 | 0.2% | `crossbeam_deque::deque::Worker<T>::pop` |
| 6 | 0.2% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 6 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 4 | 0.1% | `DYLD-STUB$$log2` |
| 4 | 0.1% | `rayon_core::registry::WorkerThread::find_work` |
| 3 | 0.1% | `core::cell::lazy::LazyCell<T,F>::really_init` |
| 3 | 0.1% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 2 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 2 | 0.1% | `cthread_yield` |
| 2 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 2 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 2 | 0.1% | `_platform_memcmp` |
| 2 | 0.1% | `rayon_core::sleep::Sleep::wake_specific_thread` |
| 2 | 0.1% | `crossbeam_epoch::internal::Global::try_advance` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `malloc` |
| 1 | 0.0% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 1 | 0.0% | `__floatuntidf` |
| 1 | 0.0% | `_<rayon::range::IterProducer<usize> as rayon::iter::plumbing::Producer>::split_at` |
| 1 | 0.0% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 1 | 0.0% | `DYLD-STUB$$free` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc13___rdl_realloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1819 | 63.6% | `radiomics::workflows::extraction::extract` |
| 1819 | 63.6% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 964 | 33.7% | `radiomics::workflows::engine::run_prepared` |
| 962 | 33.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 938 | 32.8% | `radiomics::features::intensity_histogram::compute` |
| 855 | 29.9% | `radiomics::imaging::preprocessing::prepare` |
| 525 | 18.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 495 | 17.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 330 | 11.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 273 | 9.5% | `radiomics::imaging::preprocessing::gather` |
| 121 | 4.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 58 | 2.0% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 38 | 1.3% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 36 | 1.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 28 | 1.0% | `radiomics::features::intensity_histogram::linear_percentile` |
| 17 | 0.6% | `radiomics::imaging::io::HostArray::eq_label` |
| 13 | 0.5% | `radiomics::imaging::preprocessing::normalize_values` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
