# Profile: voxel/intensityhistogram-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 intensityhistogram`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:56 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3753 (idle worker-thread samples excluded: 25142)

## Command output

```text
{"median_ms":2686.759167,"samples_ms":[1647.1675420000001,2686.759167,2991.7397499999997],"roi_voxels":54639,"maps":23,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1204 | 32.1% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 633 | 16.9% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 236 | 6.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 145 | 3.9% | `<deduplicated_symbol>` |
| 123 | 3.3% | `_platform_memmove` |
| 113 | 3.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 111 | 3.0% | `_xzm_xzone_malloc_tiny` |
| 101 | 2.7% | `mach_absolute_time` |
| 92 | 2.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 83 | 2.2% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 79 | 2.1% | `radiomics::features::intensity_histogram::compute` |
| 78 | 2.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 72 | 1.9% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 67 | 1.8% | `_xzm_free_main` |
| 49 | 1.3% | `_xzm_free` |
| 44 | 1.2% | `__floattidf` |
| 42 | 1.1% | `core::hash::BuildHasher::hash_one` |
| 40 | 1.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 36 | 1.0% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 33 | 0.9% | `__psynch_mutexwait` |
| 29 | 0.8% | `DYLD-STUB$$free` |
| 26 | 0.7% | `xzm_malloc_zone_malloc_type_realloc` |
| 21 | 0.6% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 21 | 0.6% | `crossbeam_deque::deque::Injector<T>::steal` |
| 19 | 0.5% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 19 | 0.5% | `_platform_memcmp` |
| 17 | 0.5% | `_malloc_zone_malloc` |
| 17 | 0.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 16 | 0.4% | `radiomics::features::intensity_histogram::linear_percentile` |
| 15 | 0.4% | `DYLD-STUB$$memcmp` |
| 12 | 0.3% | `log2` |
| 12 | 0.3% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 12 | 0.3% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 12 | 0.3% | `core::cell::lazy::LazyCell<T,F>::really_init` |
| 12 | 0.3% | `_xzm_malloc` |
| 11 | 0.3% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 10 | 0.3% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 9 | 0.2% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 8 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 7 | 0.2% | `_free` |
| 5 | 0.1% | `_malloc_zone_realloc` |
| 5 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 5 | 0.1% | `DYLD-STUB$$log2` |
| 5 | 0.1% | `__floatuntidf` |
| 4 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 4 | 0.1% | `_pthread_cond_wait` |
| 4 | 0.1% | `_<rayon::range::IterProducer<usize> as rayon::iter::plumbing::Producer>::split_at` |
| 3 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 3 | 0.1% | `_realloc` |
| 3 | 0.1% | `crossbeam_deque::deque::Worker<T>::pop` |
| 3 | 0.1% | `__ulock_wake` |
| 3 | 0.1% | `_tlv_get_addr` |
| 3 | 0.1% | `core::iter::adapters::try_process` |
| 2 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 2 | 0.1% | `__bzero` |
| 2 | 0.1% | `DYLD-STUB$$memcpy` |
| 2 | 0.1% | `DYLD-STUB$$malloc` |
| 2 | 0.1% | `_RNvCsGIExRX8pES_7___rustc12___rust_alloc` |
| 2 | 0.1% | `_xzm_xzone_find_and_malloc_from_freelist_chunk` |
| 1 | 0.0% | `DYLD-STUB$$calloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2631 | 70.1% | `radiomics::workflows::extraction::extract` |
| 2631 | 70.1% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2214 | 59.0% | `radiomics::workflows::engine::run_prepared` |
| 2213 | 59.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 802 | 21.4% | `radiomics::features::intensity_histogram::compute` |
| 417 | 11.1% | `radiomics::imaging::preprocessing::prepare` |
| 329 | 8.8% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 263 | 7.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 154 | 4.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 151 | 4.0% | `radiomics::imaging::preprocessing::gather` |
| 83 | 2.2% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 79 | 2.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 74 | 2.0% | `radiomics::imaging::preprocessing::normalize_values` |
| 26 | 0.7% | `radiomics::imaging::io::HostArray::eq_label` |
| 21 | 0.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 21 | 0.6% | `radiomics::features::intensity_histogram::linear_percentile` |
| 16 | 0.4% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 5 | 0.1% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
