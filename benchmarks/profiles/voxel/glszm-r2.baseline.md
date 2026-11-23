# Profile: voxel/glszm-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 glszm`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:55 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 9859 (idle worker-thread samples excluded: 30156)

## Command output

```text
{"median_ms":5099.358541,"samples_ms":[3364.221875,5099.358541,5144.855958],"roi_voxels":54639,"maps":16,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 5478 | 55.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 561 | 5.7% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 376 | 3.8% | `_platform_memmove` |
| 278 | 2.8% | `_xzm_xzone_malloc_tiny` |
| 244 | 2.5% | `_xzm_free_main` |
| 204 | 2.1% | `_platform_memcmp` |
| 185 | 1.9% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 176 | 1.8% | `<deduplicated_symbol>` |
| 162 | 1.6% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 159 | 1.6% | `radiomics::numerics::sparse_counts::marginals` |
| 156 | 1.6% | `radiomics::features::texture::compute::_{{closure}}` |
| 153 | 1.6% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 131 | 1.3% | `core::iter::traits::iterator::Iterator::try_fold` |
| 119 | 1.2% | `mach_absolute_time` |
| 118 | 1.2% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 102 | 1.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 101 | 1.0% | `_xzm_free` |
| 75 | 0.8% | `xzm_malloc_zone_size` |
| 71 | 0.7% | `radiomics::features::texture::compute` |
| 68 | 0.7% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 67 | 0.7% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 65 | 0.7% | `xzm_malloc_zone_malloc_type_realloc` |
| 61 | 0.6% | `DYLD-STUB$$memcmp` |
| 59 | 0.6% | `log1p` |
| 55 | 0.6% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 53 | 0.5% | `_xzm_malloc` |
| 48 | 0.5% | `core::hash::BuildHasher::hash_one` |
| 40 | 0.4% | `radiomics::numerics::sparse_counts::variance` |
| 37 | 0.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 34 | 0.3% | `_free` |
| 32 | 0.3% | `__psynch_mutexwait` |
| 30 | 0.3% | `_malloc_zone_realloc` |
| 28 | 0.3% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 24 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 23 | 0.2% | `ndarray::iterators::Iter<A,D>::new` |
| 21 | 0.2% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 21 | 0.2% | `_malloc_zone_malloc` |
| 19 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 19 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 18 | 0.2% | `radiomics::numerics::sparse_counts::entropy` |
| 15 | 0.2% | `rayon_core::join::join_context::_{{closure}}` |
| 13 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 12 | 0.1% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 11 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 11 | 0.1% | `_<alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::fold` |
| 10 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 10 | 0.1% | `radiomics::features::aggregation::resolved_aggregation` |
| 9 | 0.1% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 8 | 0.1% | `__bzero` |
| 8 | 0.1% | `_RNvXsC_NtNtCs6sq8b9ugfBC_4core5slice3cmpmNtB5_13SliceContains14slice_contains` |
| 7 | 0.1% | `radiomics::features::matrices::Domain::new` |
| 7 | 0.1% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 7 | 0.1% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 5 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 5 | 0.1% | `__floattidf` |
| 5 | 0.1% | `DYLD-STUB$$memcpy` |
| 4 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 4 | 0.0% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 3 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 3 | 0.0% | `_<rayon::iter::map::MapFolder<C,F> as rayon::iter::plumbing::Folder<T>>::consume_iter` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 7835 | 79.5% | `radiomics::features::texture::compute` |
| 6723 | 68.2% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 2216 | 22.5% | `radiomics::workflows::extraction::extract` |
| 2216 | 22.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1541 | 15.6% | `radiomics::workflows::engine::run_prepared` |
| 1540 | 15.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 802 | 8.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 685 | 6.9% | `radiomics::features::distributions::size_statistics` |
| 675 | 6.8% | `radiomics::imaging::preprocessing::prepare` |
| 449 | 4.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 401 | 4.1% | `radiomics::features::texture::compute::_{{closure}}` |
| 295 | 3.0% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 266 | 2.7% | `radiomics::numerics::sparse_counts::marginals` |
| 226 | 2.3% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 186 | 1.9% | `radiomics::imaging::preprocessing::gather` |
| 102 | 1.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 69 | 0.7% | `radiomics::numerics::sparse_counts::entropy` |
| 45 | 0.5% | `radiomics::numerics::sparse_counts::variance` |
| 24 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 13 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 11 | 0.1% | `radiomics::features::matrices::Domain::new` |
| 10 | 0.1% | `radiomics::features::aggregation::resolved_aggregation` |
| 5 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 1 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
