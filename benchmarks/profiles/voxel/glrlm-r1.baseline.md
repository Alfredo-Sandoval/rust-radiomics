# Profile: voxel/glrlm-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 glrlm`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:43 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 8090 (idle worker-thread samples excluded: 39989)

## Command output

```text
{"median_ms":961.443583,"samples_ms":[908.3033750000001,911.162167,961.443583,1381.902,1677.471875],"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1378 | 17.0% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 939 | 11.6% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 811 | 10.0% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 565 | 7.0% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 445 | 5.5% | `_xzm_xzone_malloc_tiny` |
| 352 | 4.4% | `mach_absolute_time` |
| 338 | 4.2% | `_xzm_free_main` |
| 274 | 3.4% | `__bzero` |
| 270 | 3.3% | `radiomics::numerics::sparse_counts::marginals` |
| 266 | 3.3% | `_platform_memmove` |
| 227 | 2.8% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 219 | 2.7% | `<deduplicated_symbol>` |
| 187 | 2.3% | `_xzm_free` |
| 134 | 1.7% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 133 | 1.6% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 94 | 1.2% | `radiomics::features::texture::compute::_{{closure}}` |
| 90 | 1.1% | `radiomics::numerics::sparse_counts::variance` |
| 89 | 1.1% | `__floattidf` |
| 87 | 1.1% | `radiomics::features::texture::compute` |
| 82 | 1.0% | `log1p` |
| 78 | 1.0% | `core::iter::traits::iterator::Iterator::try_fold` |
| 78 | 1.0% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 77 | 1.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 75 | 0.9% | `_free` |
| 68 | 0.8% | `_platform_memcmp` |
| 50 | 0.6% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 44 | 0.5% | `radiomics::numerics::sparse_counts::entropy` |
| 42 | 0.5% | `xzm_malloc_zone_size` |
| 38 | 0.5% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 38 | 0.5% | `__psynch_mutexwait` |
| 35 | 0.4% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 32 | 0.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 32 | 0.4% | `_malloc_zone_realloc` |
| 28 | 0.3% | `_realloc` |
| 28 | 0.3% | `core::hash::BuildHasher::hash_one` |
| 27 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 25 | 0.3% | `_xzm_malloc` |
| 24 | 0.3% | `radiomics::features::distributions::size_statistics` |
| 20 | 0.2% | `_malloc_zone_malloc` |
| 18 | 0.2% | `DYLD-STUB$$malloc` |
| 15 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 15 | 0.2% | `DYLD-STUB$$memmove` |
| 15 | 0.2% | `_pthread_cond_wait` |
| 14 | 0.2% | `DYLD-STUB$$memcmp` |
| 14 | 0.2% | `DYLD-STUB$$free` |
| 11 | 0.1% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 10 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 10 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 9 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 9 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 9 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 8 | 0.1% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 8 | 0.1% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 7 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 7 | 0.1% | `ndarray::arraytraits::_<impl core::iter::traits::collect::IntoIterator for &ndarray::ArrayBase<S,D>>::into_iter` |
| 6 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 5 | 0.1% | `malloc` |
| 5 | 0.1% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 5 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 5 | 0.1% | `_xzm_chunk_batch_list_push` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 5115 | 63.2% | `radiomics::features::texture::compute` |
| 4432 | 54.8% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 2664 | 32.9% | `radiomics::workflows::extraction::extract` |
| 2664 | 32.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2545 | 31.5% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 2172 | 26.8% | `radiomics::features::distributions::size_statistics` |
| 2152 | 26.6% | `radiomics::workflows::engine::run_prepared` |
| 2150 | 26.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 512 | 6.3% | `radiomics::imaging::preprocessing::prepare` |
| 466 | 5.8% | `radiomics::numerics::sparse_counts::marginals` |
| 285 | 3.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 257 | 3.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 255 | 3.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 201 | 2.5% | `radiomics::imaging::preprocessing::gather` |
| 179 | 2.2% | `radiomics::numerics::sparse_counts::variance` |
| 173 | 2.1% | `radiomics::features::texture::compute::_{{closure}}` |
| 163 | 2.0% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 124 | 1.5% | `radiomics::numerics::sparse_counts::entropy` |
| 22 | 0.3% | `radiomics::imaging::preprocessing::normalize_values` |
| 15 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 12 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 3 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 3 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
| 3 | 0.0% | `radiomics::features::matrices::Domain::new` |
