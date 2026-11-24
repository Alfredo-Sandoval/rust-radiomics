# Profile: voxel/intensityhistogram-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 intensityhistogram`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:45 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 4162 (idle worker-thread samples excluded: 51606)

## Command output

```text
{"median_ms":1066.822167,"samples_ms":[905.139,932.3953750000001,1066.822167,1237.793708,1386.8549159999998],"roi_voxels":54639,"maps":23,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1177 | 28.3% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 860 | 20.7% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 399 | 9.6% | `__bzero` |
| 210 | 5.0% | `<deduplicated_symbol>` |
| 149 | 3.6% | `_xzm_free_main` |
| 138 | 3.3% | `_xzm_xzone_malloc_tiny` |
| 132 | 3.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 109 | 2.6% | `mach_absolute_time` |
| 105 | 2.5% | `_platform_memmove` |
| 100 | 2.4% | `__psynch_mutexwait` |
| 74 | 1.8% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 69 | 1.7% | `_xzm_free` |
| 63 | 1.5% | `radiomics::features::intensity_histogram::compute` |
| 56 | 1.3% | `core::iter::traits::iterator::Iterator::try_fold` |
| 46 | 1.1% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 44 | 1.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 29 | 0.7% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 28 | 0.7% | `__floattidf` |
| 28 | 0.7% | `_malloc_zone_malloc` |
| 28 | 0.7% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 27 | 0.6% | `core::hash::BuildHasher::hash_one` |
| 26 | 0.6% | `xzm_malloc_zone_malloc_type_realloc` |
| 21 | 0.5% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 19 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 15 | 0.4% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 14 | 0.3% | `core::cell::lazy::LazyCell<T,F>::really_init` |
| 13 | 0.3% | `_free` |
| 13 | 0.3% | `_platform_memcmp` |
| 10 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 10 | 0.2% | `radiomics::features::intensity_histogram::linear_percentile` |
| 10 | 0.2% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 7 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 7 | 0.2% | `log2` |
| 7 | 0.2% | `_pthread_cond_wait` |
| 6 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 5 | 0.1% | `_malloc_zone_realloc` |
| 5 | 0.1% | `_xzm_malloc` |
| 5 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 5 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 5 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 5 | 0.1% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 5 | 0.1% | `xzm_malloc_zone_size` |
| 4 | 0.1% | `DYLD-STUB$$free` |
| 4 | 0.1% | `__floatuntidf` |
| 4 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 4 | 0.1% | `_realloc` |
| 4 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 3 | 0.1% | `DYLD-STUB$$memcpy` |
| 3 | 0.1% | `__psynch_cvsignal` |
| 3 | 0.1% | `__udivti3` |
| 3 | 0.1% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 3 | 0.1% | `ndarray::arraytraits::_<impl core::iter::traits::collect::IntoIterator for &ndarray::ArrayBase<S,D>>::into_iter` |
| 3 | 0.1% | `DYLD-STUB$$malloc` |
| 3 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 3 | 0.1% | `default_zone_realloc` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.0% | `rayon_core::join::join_context::_{{closure}}` |
| 2 | 0.0% | `ndarray::iterators::Iter<A,D>::new` |
| 2 | 0.0% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 2 | 0.0% | `_RNvCsGIExRX8pES_7___rustc12___rust_alloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2900 | 69.7% | `radiomics::workflows::extraction::extract` |
| 2900 | 69.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2303 | 55.3% | `radiomics::workflows::engine::run_prepared` |
| 2300 | 55.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 721 | 17.3% | `radiomics::features::intensity_histogram::compute` |
| 597 | 14.3% | `radiomics::imaging::preprocessing::prepare` |
| 416 | 10.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 318 | 7.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 279 | 6.7% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 248 | 6.0% | `radiomics::imaging::preprocessing::gather` |
| 237 | 5.7% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 29 | 0.7% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 28 | 0.7% | `radiomics::imaging::preprocessing::normalize_values` |
| 19 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 15 | 0.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 15 | 0.4% | `radiomics::features::intensity_histogram::linear_percentile` |
| 14 | 0.3% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
