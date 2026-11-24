# Profile: voxel/glszm-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 glszm`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:45 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 5549 (idle worker-thread samples excluded: 46113)

## Command output

```text
{"median_ms":1075.95975,"samples_ms":[882.527041,888.9522079999999,1075.95975,1187.5534169999999,1395.9243749999998],"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1140 | 20.5% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 896 | 16.1% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 863 | 15.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 257 | 4.6% | `__bzero` |
| 199 | 3.6% | `_xzm_xzone_malloc_tiny` |
| 179 | 3.2% | `_xzm_free_main` |
| 178 | 3.2% | `_platform_memcmp` |
| 162 | 2.9% | `radiomics::features::texture::compute::_{{closure}}` |
| 160 | 2.9% | `<deduplicated_symbol>` |
| 154 | 2.8% | `__psynch_mutexwait` |
| 137 | 2.5% | `mach_absolute_time` |
| 126 | 2.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 101 | 1.8% | `_platform_memmove` |
| 82 | 1.5% | `_xzm_free` |
| 81 | 1.5% | `radiomics::features::texture::compute` |
| 74 | 1.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 56 | 1.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 55 | 1.0% | `core::iter::traits::iterator::Iterator::try_fold` |
| 40 | 0.7% | `radiomics::numerics::sparse_counts::marginals` |
| 39 | 0.7% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 32 | 0.6% | `_malloc_zone_malloc` |
| 31 | 0.6% | `xzm_malloc_zone_size` |
| 25 | 0.5% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 23 | 0.4% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 22 | 0.4% | `core::hash::BuildHasher::hash_one` |
| 22 | 0.4% | `_realloc` |
| 20 | 0.4% | `_xzm_malloc` |
| 20 | 0.4% | `DYLD-STUB$$memcmp` |
| 17 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 16 | 0.3% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 14 | 0.3% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 14 | 0.3% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 13 | 0.2% | `_free` |
| 13 | 0.2% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 12 | 0.2% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 12 | 0.2% | `log1p` |
| 12 | 0.2% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 11 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 11 | 0.2% | `radiomics::features::aggregation::resolved_aggregation` |
| 11 | 0.2% | `radiomics::numerics::sparse_counts::variance` |
| 10 | 0.2% | `__psynch_mutexdrop` |
| 10 | 0.2% | `_<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop` |
| 10 | 0.2% | `pthread_mutex_unlock` |
| 9 | 0.2% | `_malloc_zone_realloc` |
| 9 | 0.2% | `xzm_malloc_zone_realloc` |
| 9 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 8 | 0.1% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 8 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 8 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 7 | 0.1% | `__floattidf` |
| 6 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 6 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 6 | 0.1% | `_<alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::fold` |
| 6 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 6 | 0.1% | `default_zone_realloc` |
| 6 | 0.1% | `DYLD-STUB$$mach_absolute_time` |
| 6 | 0.1% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 5 | 0.1% | `DYLD-STUB$$realloc` |
| 5 | 0.1% | `radiomics::numerics::sparse_counts::entropy` |
| 4 | 0.1% | `__psynch_cvsignal` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2767 | 49.9% | `radiomics::workflows::extraction::extract` |
| 2767 | 49.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2287 | 41.2% | `radiomics::workflows::engine::run_prepared` |
| 2284 | 41.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 2249 | 40.5% | `radiomics::features::texture::compute` |
| 1493 | 26.9% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 480 | 8.7% | `radiomics::imaging::preprocessing::prepare` |
| 357 | 6.4% | `radiomics::features::texture::compute::_{{closure}}` |
| 338 | 6.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 295 | 5.3% | `radiomics::features::distributions::size_statistics` |
| 269 | 4.8% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 249 | 4.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 231 | 4.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 192 | 3.5% | `radiomics::imaging::preprocessing::gather` |
| 152 | 2.7% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 71 | 1.3% | `radiomics::numerics::sparse_counts::marginals` |
| 24 | 0.4% | `radiomics::imaging::preprocessing::normalize_values` |
| 18 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 18 | 0.3% | `radiomics::numerics::sparse_counts::variance` |
| 15 | 0.3% | `radiomics::numerics::sparse_counts::entropy` |
| 12 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 11 | 0.2% | `radiomics::features::aggregation::resolved_aggregation` |
| 2 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::features::matrices::Domain::new` |
