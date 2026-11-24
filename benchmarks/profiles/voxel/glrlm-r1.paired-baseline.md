# Profile: voxel/glrlm-r1.paired-baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 glrlm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:21 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 5815 (idle worker-thread samples excluded: 43908)

## Command output

```text
{"median_ms":2888.517958,"samples_ms":[2016.6212500000001,2357.593041,2888.517958,3013.21475,3264.297],"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1160 | 19.9% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 1009 | 17.4% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 664 | 11.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 267 | 4.6% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 229 | 3.9% | `__bzero` |
| 205 | 3.5% | `_xzm_xzone_malloc_tiny` |
| 195 | 3.4% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 187 | 3.2% | `mach_absolute_time` |
| 161 | 2.8% | `_xzm_free_main` |
| 144 | 2.5% | `_platform_memmove` |
| 142 | 2.4% | `<deduplicated_symbol>` |
| 97 | 1.7% | `xzm_malloc_zone_malloc_type_realloc` |
| 96 | 1.7% | `radiomics::features::texture::compute::_{{closure}}` |
| 94 | 1.6% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 93 | 1.6% | `_xzm_free` |
| 90 | 1.5% | `radiomics::numerics::sparse_counts::marginals` |
| 85 | 1.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 83 | 1.4% | `__psynch_mutexwait` |
| 78 | 1.3% | `core::iter::traits::iterator::Iterator::try_fold` |
| 59 | 1.0% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 57 | 1.0% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 55 | 0.9% | `__floattidf` |
| 53 | 0.9% | `radiomics::features::texture::compute` |
| 43 | 0.7% | `_platform_memcmp` |
| 39 | 0.7% | `radiomics::numerics::sparse_counts::variance` |
| 39 | 0.7% | `log1p` |
| 32 | 0.6% | `_xzm_malloc` |
| 32 | 0.6% | `DYLD-STUB$$memcpy` |
| 27 | 0.5% | `_realloc` |
| 25 | 0.4% | `DYLD-STUB$$memmove` |
| 21 | 0.4% | `xzm_malloc_zone_size` |
| 21 | 0.4% | `_malloc_zone_malloc` |
| 21 | 0.4% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 18 | 0.3% | `DYLD-STUB$$memcmp` |
| 17 | 0.3% | `radiomics::features::distributions::size_statistics` |
| 14 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 14 | 0.2% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 13 | 0.2% | `radiomics::numerics::sparse_counts::entropy` |
| 13 | 0.2% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 11 | 0.2% | `_malloc_zone_realloc` |
| 10 | 0.2% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 8 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 8 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 7 | 0.1% | `core::str::_<impl str>::split_once` |
| 7 | 0.1% | `_free` |
| 7 | 0.1% | `malloc` |
| 6 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 5 | 0.1% | `_RNvCsGIExRX8pES_7___rustc13___rdl_realloc` |
| 5 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 4 | 0.1% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 4 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 4 | 0.1% | `core::hash::BuildHasher::hash_one` |
| 4 | 0.1% | `DYLD-STUB$$free` |
| 4 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 3 | 0.1% | `_xzm_chunk_batch_list_push` |
| 2 | 0.0% | `ndarray::iterators::Iter<A,D>::new` |
| 2 | 0.0% | `DYLD-STUB$$log1p` |
| 2 | 0.0% | `DYLD-STUB$$malloc` |
| 2 | 0.0% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 2 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3092 | 53.2% | `radiomics::features::texture::compute` |
| 2618 | 45.0% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 2587 | 44.5% | `radiomics::workflows::extraction::extract` |
| 2587 | 44.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2208 | 38.0% | `radiomics::workflows::engine::run_prepared` |
| 2206 | 37.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1298 | 22.3% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 1079 | 18.6% | `radiomics::features::distributions::size_statistics` |
| 379 | 6.5% | `radiomics::imaging::preprocessing::prepare` |
| 254 | 4.4% | `radiomics::numerics::sparse_counts::marginals` |
| 230 | 4.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 223 | 3.8% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 156 | 2.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 156 | 2.7% | `radiomics::features::texture::compute::_{{closure}}` |
| 137 | 2.4% | `radiomics::imaging::preprocessing::gather` |
| 122 | 2.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 94 | 1.6% | `radiomics::numerics::sparse_counts::variance` |
| 42 | 0.7% | `radiomics::numerics::sparse_counts::entropy` |
| 8 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 5 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 5 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 2 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::features::matrices::Domain::new` |
