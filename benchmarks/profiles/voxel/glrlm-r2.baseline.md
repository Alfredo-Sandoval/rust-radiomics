# Profile: voxel/glrlm-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 glrlm`
- Commit: `38ac856` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:52 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 24787 (idle worker-thread samples excluded: 7334)

## Command output

```text
{"median_ms":2487.0017080000002,"samples_ms":[2074.193666,2487.0017080000002,2646.016708],"roi_voxels":54639,"maps":16,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 11878 | 47.9% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1856 | 7.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1429 | 5.8% | `radiomics::numerics::sparse_counts::marginals` |
| 1388 | 5.6% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 956 | 3.9% | `_xzm_xzone_malloc_tiny` |
| 800 | 3.2% | `_platform_memmove` |
| 637 | 2.6% | `_xzm_free_main` |
| 516 | 2.1% | `mach_absolute_time` |
| 429 | 1.7% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 393 | 1.6% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 357 | 1.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 355 | 1.4% | `log1p` |
| 278 | 1.1% | `__floattidf` |
| 255 | 1.0% | `radiomics::numerics::sparse_counts::variance` |
| 242 | 1.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 234 | 0.9% | `<deduplicated_symbol>` |
| 229 | 0.9% | `_xzm_free` |
| 221 | 0.9% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 195 | 0.8% | `radiomics::features::texture::compute::_{{closure}}` |
| 191 | 0.8% | `radiomics::numerics::sparse_counts::entropy` |
| 149 | 0.6% | `radiomics::features::distributions::size_statistics` |
| 149 | 0.6% | `xzm_malloc_zone_malloc_type_realloc` |
| 122 | 0.5% | `default_zone_realloc` |
| 115 | 0.5% | `_malloc_zone_malloc` |
| 108 | 0.4% | `radiomics::features::texture::compute` |
| 107 | 0.4% | `_platform_memcmp` |
| 83 | 0.3% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 82 | 0.3% | `_free` |
| 74 | 0.3% | `DYLD-STUB$$memmove` |
| 55 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 51 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 51 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 49 | 0.2% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 49 | 0.2% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 48 | 0.2% | `core::hash::BuildHasher::hash_one` |
| 46 | 0.2% | `_xzm_chunk_list_slot_push` |
| 45 | 0.2% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 42 | 0.2% | `_realloc` |
| 34 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 30 | 0.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 28 | 0.1% | `xzm_malloc_zone_size` |
| 24 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 23 | 0.1% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 22 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 20 | 0.1% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 19 | 0.1% | `DYLD-STUB$$memcpy` |
| 19 | 0.1% | `DYLD-STUB$$malloc` |
| 18 | 0.1% | `DYLD-STUB$$free` |
| 18 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 18 | 0.1% | `_malloc_zone_realloc` |
| 17 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 15 | 0.1% | `DYLD-STUB$$memcmp` |
| 15 | 0.1% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 14 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 14 | 0.1% | `realloc` |
| 14 | 0.1% | `_xzm_malloc` |
| 13 | 0.1% | `free` |
| 13 | 0.1% | `core::ptr::drop_in_place<radiomics::contracts::types::PreparedInput>` |
| 11 | 0.0% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 11 | 0.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 23045 | 93.0% | `radiomics::features::texture::compute` |
| 21615 | 87.2% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 7861 | 31.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 6741 | 27.2% | `radiomics::features::distributions::size_statistics` |
| 2412 | 9.7% | `radiomics::workflows::extraction::extract` |
| 2412 | 9.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2412 | 9.7% | `radiomics::workflows::engine::run_prepared` |
| 2411 | 9.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 2057 | 8.3% | `radiomics::numerics::sparse_counts::marginals` |
| 533 | 2.2% | `radiomics::numerics::sparse_counts::variance` |
| 470 | 1.9% | `radiomics::numerics::sparse_counts::entropy` |
| 361 | 1.5% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 304 | 1.2% | `radiomics::features::texture::compute::_{{closure}}` |
| 51 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 9 | 0.0% | `radiomics::features::matrices::Domain::new` |
| 7 | 0.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 5 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 1 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
