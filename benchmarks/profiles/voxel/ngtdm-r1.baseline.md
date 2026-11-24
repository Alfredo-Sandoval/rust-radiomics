# Profile: voxel/ngtdm-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 ngtdm`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:44 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 4437 (idle worker-thread samples excluded: 52468)

## Command output

```text
{"median_ms":1065.541958,"samples_ms":[872.428667,922.462,1065.541958,1152.270375,1327.968791],"roi_voxels":54639,"maps":5,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1402 | 31.6% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 960 | 21.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 338 | 7.6% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 207 | 4.7% | `<deduplicated_symbol>` |
| 127 | 2.9% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 124 | 2.8% | `mach_absolute_time` |
| 124 | 2.8% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 118 | 2.7% | `_xzm_xzone_malloc_tiny` |
| 109 | 2.5% | `__psynch_mutexwait` |
| 105 | 2.4% | `_platform_memmove` |
| 94 | 2.1% | `_xzm_free_main` |
| 76 | 1.7% | `core::iter::traits::iterator::Iterator::try_fold` |
| 73 | 1.6% | `__bzero` |
| 65 | 1.5% | `__floattidf` |
| 48 | 1.1% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 41 | 0.9% | `radiomics::features::texture::compute::_{{closure}}` |
| 34 | 0.8% | `xzm_malloc_zone_malloc_type_realloc` |
| 29 | 0.7% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 29 | 0.7% | `_xzm_free` |
| 22 | 0.5% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 21 | 0.5% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 19 | 0.4% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 16 | 0.4% | `__psynch_mutexdrop` |
| 15 | 0.3% | `_platform_memcmp` |
| 15 | 0.3% | `_free` |
| 14 | 0.3% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 13 | 0.3% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 13 | 0.3% | `xzm_malloc_zone_size` |
| 13 | 0.3% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 11 | 0.2% | `_malloc_zone_malloc` |
| 10 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 10 | 0.2% | `radiomics::features::texture::compute` |
| 10 | 0.2% | `_pthread_cond_wait` |
| 9 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 9 | 0.2% | `_malloc_zone_realloc` |
| 9 | 0.2% | `DYLD-STUB$$free` |
| 9 | 0.2% | `_xzm_malloc` |
| 7 | 0.2% | `free` |
| 7 | 0.2% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 6 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 4 | 0.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 4 | 0.1% | `DYLD-STUB$$memcpy` |
| 4 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 4 | 0.1% | `DYLD-STUB$$memmove` |
| 4 | 0.1% | `_<rayon::iter::map::MapFolder<C,F> as rayon::iter::plumbing::Folder<T>>::consume_iter` |
| 3 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 3 | 0.1% | `DYLD-STUB$$mach_absolute_time` |
| 3 | 0.1% | `core::hash::BuildHasher::hash_one` |
| 3 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 3 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 3 | 0.1% | `ndarray::dimension::dimension_trait::_<impl core::ops::index::Index<usize> for ndarray::dimension::dim::Dim<ndarray::dimension::dynindeximpl::IxDynImpl>>::index` |
| 2 | 0.0% | `DYLD-STUB$$malloc` |
| 2 | 0.0% | `__psynch_cvsignal` |
| 2 | 0.0% | `core::ptr::drop_in_place<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<usize>,alloc::vec::into_iter::IntoIter<f64>>>` |
| 2 | 0.0% | `radiomics::features::distributions::ngtdm_statistics` |
| 2 | 0.0% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 2 | 0.0% | `core::ptr::drop_in_place<core::iter::adapters::zip::Zip<ndarray::iterators::Iter<f64,ndarray::dimension::dim::Dim<ndarray::dimension::dynindeximpl::IxDynImpl>>,` |
| 2 | 0.0% | `_RNvCsGIExRX8pES_7___rustc12___rust_alloc` |
| 1 | 0.0% | `_xzm_segment_group_span_mark_smaller` |
| 1 | 0.0% | `xzm_segment_group_alloc_chunk` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2931 | 66.1% | `radiomics::workflows::extraction::extract` |
| 2931 | 66.1% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2331 | 52.5% | `radiomics::workflows::engine::run_prepared` |
| 2328 | 52.5% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1198 | 27.0% | `radiomics::features::texture::compute` |
| 878 | 19.8% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 600 | 13.5% | `radiomics::imaging::preprocessing::prepare` |
| 308 | 6.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 292 | 6.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 255 | 5.7% | `radiomics::imaging::preprocessing::gather` |
| 198 | 4.5% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 152 | 3.4% | `radiomics::features::distributions::ngtdm_statistics` |
| 76 | 1.7% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 72 | 1.6% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 52 | 1.2% | `radiomics::features::texture::compute::_{{closure}}` |
| 14 | 0.3% | `radiomics::imaging::preprocessing::normalize_values` |
| 14 | 0.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 10 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 8 | 0.2% | `radiomics::features::matrices::Domain::new` |
| 1 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
