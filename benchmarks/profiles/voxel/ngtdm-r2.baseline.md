# Profile: voxel/ngtdm-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 ngtdm`
- Commit: `38ac856` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:54 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 7203 (idle worker-thread samples excluded: 35851)

## Command output

```text
{"median_ms":1826.766667,"samples_ms":[1711.345,1826.766667,3635.29225],"roi_voxels":54639,"maps":5,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2280 | 31.7% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 834 | 11.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 743 | 10.3% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 571 | 7.9% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 332 | 4.6% | `__floattidf` |
| 239 | 3.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 205 | 2.8% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 205 | 2.8% | `_xzm_xzone_malloc_tiny` |
| 199 | 2.8% | `_platform_memmove` |
| 143 | 2.0% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 132 | 1.8% | `<deduplicated_symbol>` |
| 132 | 1.8% | `_xzm_free_main` |
| 118 | 1.6% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 98 | 1.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 77 | 1.1% | `mach_absolute_time` |
| 70 | 1.0% | `_xzm_free` |
| 64 | 0.9% | `__bzero` |
| 58 | 0.8% | `core::iter::traits::iterator::Iterator::try_fold` |
| 56 | 0.8% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 42 | 0.6% | `radiomics::features::distributions::ngtdm_statistics` |
| 39 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 39 | 0.5% | `__psynch_mutexwait` |
| 39 | 0.5% | `_xzm_malloc` |
| 35 | 0.5% | `_free` |
| 33 | 0.5% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 32 | 0.4% | `_platform_memcmp` |
| 29 | 0.4% | `radiomics::features::texture::compute::_{{closure}}` |
| 28 | 0.4% | `radiomics::features::texture::compute` |
| 27 | 0.4% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 27 | 0.4% | `DYLD-STUB$$memmove` |
| 26 | 0.4% | `DYLD-STUB$$memcmp` |
| 25 | 0.3% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 20 | 0.3% | `xzm_malloc_zone_malloc_type_realloc` |
| 20 | 0.3% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 18 | 0.2% | `_malloc_zone_malloc` |
| 17 | 0.2% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 15 | 0.2% | `xzm_malloc_zone_size` |
| 14 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 14 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 8 | 0.1% | `_malloc_zone_realloc` |
| 7 | 0.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 7 | 0.1% | `core::hash::BuildHasher::hash_one` |
| 7 | 0.1% | `DYLD-STUB$$free` |
| 7 | 0.1% | `_<rayon::iter::map::MapFolder<C,F> as rayon::iter::plumbing::Folder<T>>::consume_iter` |
| 7 | 0.1% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 6 | 0.1% | `_realloc` |
| 6 | 0.1% | `ndarray::iterators::Iter<A,D>::new` |
| 6 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 5 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 5 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 5 | 0.1% | `core::str::_<impl str>::split_once` |
| 4 | 0.1% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 4 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 3 | 0.0% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 3 | 0.0% | `DYLD-STUB$$memcpy` |
| 2 | 0.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 2 | 0.0% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 2 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 1 | 0.0% | `madvise` |
| 1 | 0.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 4964 | 68.9% | `radiomics::features::texture::compute` |
| 4401 | 61.1% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 2232 | 31.0% | `radiomics::workflows::extraction::extract` |
| 2232 | 31.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1811 | 25.1% | `radiomics::workflows::engine::run_prepared` |
| 1810 | 25.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 777 | 10.8% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 602 | 8.4% | `radiomics::features::distributions::ngtdm_statistics` |
| 421 | 5.8% | `radiomics::imaging::preprocessing::prepare` |
| 253 | 3.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 168 | 2.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 155 | 2.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 146 | 2.0% | `radiomics::imaging::preprocessing::gather` |
| 78 | 1.1% | `radiomics::features::texture::compute::_{{closure}}` |
| 68 | 0.9% | `radiomics::imaging::preprocessing::normalize_values` |
| 66 | 0.9% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 40 | 0.6% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 23 | 0.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 5 | 0.1% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::features::matrices::Domain::new` |
| 1 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
