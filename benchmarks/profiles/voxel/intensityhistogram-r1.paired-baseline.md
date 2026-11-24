# Profile: voxel/intensityhistogram-r1.paired-baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 intensityhistogram`
- Commit: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:22 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 3987 (idle worker-thread samples excluded: 24138)

## Command output

```text
{"median_ms":2896.9267919999998,"samples_ms":[2560.9576660000002,2809.565333,2896.9267919999998,3272.6235410000004,5399.975625],"roi_voxels":54639,"maps":23,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1129 | 28.3% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 967 | 24.3% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 237 | 5.9% | `<deduplicated_symbol>` |
| 186 | 4.7% | `_platform_memmove` |
| 175 | 4.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 166 | 4.2% | `_xzm_xzone_malloc_tiny` |
| 163 | 4.1% | `_xzm_free_main` |
| 70 | 1.8% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 62 | 1.6% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 58 | 1.5% | `_malloc_zone_malloc` |
| 57 | 1.4% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 56 | 1.4% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 55 | 1.4% | `__floattidf` |
| 54 | 1.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 53 | 1.3% | `_xzm_free` |
| 50 | 1.3% | `mach_absolute_time` |
| 49 | 1.2% | `core::hash::BuildHasher::hash_one` |
| 41 | 1.0% | `radiomics::features::intensity_histogram::compute` |
| 26 | 0.7% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 25 | 0.6% | `radiomics::features::intensity_histogram::linear_percentile` |
| 24 | 0.6% | `core::iter::traits::iterator::Iterator::try_fold` |
| 22 | 0.6% | `DYLD-STUB$$malloc` |
| 21 | 0.5% | `_free` |
| 21 | 0.5% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 19 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 18 | 0.5% | `mfm_alloc` |
| 17 | 0.4% | `_platform_memcmp` |
| 15 | 0.4% | `xzm_malloc_zone_malloc_type_realloc` |
| 12 | 0.3% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 11 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 11 | 0.3% | `log2` |
| 10 | 0.3% | `_xzm_malloc` |
| 10 | 0.3% | `core::cell::lazy::LazyCell<T,F>::really_init` |
| 10 | 0.3% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 7 | 0.2% | `__bzero` |
| 7 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 7 | 0.2% | `xzm_malloc_zone_size` |
| 6 | 0.2% | `_malloc_zone_realloc` |
| 5 | 0.1% | `xzm_malloc_zone_try_free_default_main` |
| 5 | 0.1% | `__floatuntidf` |
| 5 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 4 | 0.1% | `DYLD-STUB$$mach_absolute_time` |
| 4 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 4 | 0.1% | `DYLD-STUB$$free` |
| 4 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 3 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 3 | 0.1% | `DYLD-STUB$$memcmp` |
| 3 | 0.1% | `DYLD-STUB$$memcpy` |
| 2 | 0.1% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 2 | 0.1% | `ndarray::iterators::Iter<A,D>::new` |
| 2 | 0.1% | `dyld::ThreadLocalVariables::addTermFunc(void (*)(void*), void*)` |
| 2 | 0.1% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 2 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 2 | 0.1% | `cthread_yield` |
| 1 | 0.0% | `_xzm_chunk_list_pop` |
| 1 | 0.0% | `_<T as core::slice::cmp::SliceContains>::slice_contains` |
| 1 | 0.0% | `_platform_memset_pattern16` |
| 1 | 0.0% | `ndarray::arraytraits::_<impl core::iter::traits::collect::IntoIterator for &ndarray::ArrayBase<S,D>>::into_iter` |
| 1 | 0.0% | `_malloc_zone_calloc` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2848 | 71.4% | `radiomics::workflows::extraction::extract` |
| 2848 | 71.4% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2153 | 54.0% | `radiomics::workflows::engine::run_prepared` |
| 2152 | 54.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 783 | 19.6% | `radiomics::features::intensity_histogram::compute` |
| 695 | 17.4% | `radiomics::imaging::preprocessing::prepare` |
| 416 | 10.4% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 313 | 7.9% | `radiomics::imaging::preprocessing::gather` |
| 279 | 7.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 270 | 6.8% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 99 | 2.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 57 | 1.4% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 55 | 1.4% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 40 | 1.0% | `radiomics::imaging::preprocessing::normalize_values` |
| 26 | 0.7% | `radiomics::features::intensity_histogram::linear_percentile` |
| 19 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 9 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 5 | 0.1% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
