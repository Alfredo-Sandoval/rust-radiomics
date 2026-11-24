# Profile: voxel/ivh-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 ivh`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:46 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3769 (idle worker-thread samples excluded: 11641)

## Command output

```text
{"median_ms":3110.164417,"samples_ms":[1867.603917,3110.164417,3814.66775],"roi_voxels":54639,"maps":7,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1137 | 30.2% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 1038 | 27.5% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 344 | 9.1% | `_platform_memmove` |
| 169 | 4.5% | `_xzm_xzone_malloc_tiny` |
| 145 | 3.8% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 114 | 3.0% | `_xzm_free` |
| 101 | 2.7% | `<deduplicated_symbol>` |
| 97 | 2.6% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 87 | 2.3% | `core::slice::sort::select::partition_at_index` |
| 67 | 1.8% | `_xzm_free_main` |
| 56 | 1.5% | `radiomics::features::ivh::ivh_statistics` |
| 55 | 1.5% | `__bzero` |
| 52 | 1.4% | `core::iter::traits::iterator::Iterator::try_fold` |
| 50 | 1.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 28 | 0.7% | `radiomics::features::ivh::prepare_ivh` |
| 28 | 0.7% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 19 | 0.5% | `mach_absolute_time` |
| 16 | 0.4% | `_malloc_zone_malloc` |
| 15 | 0.4% | `xzm_malloc_zone_size` |
| 15 | 0.4% | `core::hash::BuildHasher::hash_one` |
| 13 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 12 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 12 | 0.3% | `xzm_malloc_zone_malloc_type_realloc` |
| 11 | 0.3% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 9 | 0.2% | `_malloc_zone_realloc` |
| 8 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 7 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 7 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 6 | 0.2% | `_xzm_malloc` |
| 5 | 0.1% | `_<rayon::iter::map::MapFolder<C,F> as rayon::iter::plumbing::Folder<T>>::consume_iter` |
| 5 | 0.1% | `__psynch_mutexwait` |
| 4 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 4 | 0.1% | `ndarray::arraytraits::_<impl core::iter::traits::collect::IntoIterator for &ndarray::ArrayBase<S,D>>::into_iter` |
| 4 | 0.1% | `xzm_malloc_zone_realloc` |
| 3 | 0.1% | `_platform_memset_pattern16` |
| 3 | 0.1% | `__floatuntidf` |
| 3 | 0.1% | `_platform_memcmp` |
| 3 | 0.1% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 3 | 0.1% | `_xzm_segment_group_span_mark_smaller` |
| 2 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 2 | 0.1% | `ndarray::iterators::Iter<A,D>::new` |
| 2 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.0% | `madvise` |
| 1 | 0.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::to_owned` |
| 1 | 0.0% | `_realloc` |
| 1 | 0.0% | `_free` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 1 | 0.0% | `default_zone_realloc` |
| 1 | 0.0% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 1 | 0.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3013 | 79.9% | `radiomics::workflows::extraction::extract` |
| 3013 | 79.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2504 | 66.4% | `radiomics::workflows::engine::run_prepared` |
| 1589 | 42.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 594 | 15.8% | `radiomics::features::ivh::prepare_ivh` |
| 566 | 15.0% | `radiomics::features::ivh::ivh_statistics` |
| 509 | 13.5% | `radiomics::imaging::preprocessing::prepare` |
| 312 | 8.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 197 | 5.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 113 | 3.0% | `radiomics::imaging::preprocessing::gather` |
| 108 | 2.9% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 24 | 0.6% | `radiomics::imaging::io::HostArray::eq_label` |
| 22 | 0.6% | `radiomics::imaging::preprocessing::normalize_values` |
| 13 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 12 | 0.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 6 | 0.2% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
