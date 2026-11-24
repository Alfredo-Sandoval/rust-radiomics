# Profile: voxel/gldm-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 gldm`
- Commit: `38ac856` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:53 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 8335 (idle worker-thread samples excluded: 5720)

## Command output

```text
{"median_ms":2540.2841249999997,"samples_ms":[2338.756167,2540.2841249999997,2708.1007910000003],"roi_voxels":54639,"maps":17,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1899 | 22.8% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 1634 | 19.6% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 688 | 8.3% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 631 | 7.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 334 | 4.0% | `_platform_memcmp` |
| 310 | 3.7% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 299 | 3.6% | `_xzm_xzone_malloc_tiny` |
| 247 | 3.0% | `_platform_memmove` |
| 204 | 2.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 189 | 2.3% | `radiomics::numerics::sparse_counts::marginals` |
| 182 | 2.2% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 179 | 2.1% | `mach_absolute_time` |
| 162 | 1.9% | `_xzm_free_main` |
| 123 | 1.5% | `radiomics::features::texture::compute::_{{closure}}` |
| 103 | 1.2% | `DYLD-STUB$$memcmp` |
| 67 | 0.8% | `xzm_malloc_zone_size` |
| 65 | 0.8% | `core::str::_<impl str>::split_once` |
| 63 | 0.8% | `radiomics::numerics::sparse_counts::variance` |
| 62 | 0.7% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 62 | 0.7% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 60 | 0.7% | `<deduplicated_symbol>` |
| 59 | 0.7% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 53 | 0.6% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 50 | 0.6% | `_xzm_free` |
| 47 | 0.6% | `_realloc` |
| 36 | 0.4% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 35 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 35 | 0.4% | `radiomics::features::texture::compute` |
| 35 | 0.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 33 | 0.4% | `log1p` |
| 27 | 0.3% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 25 | 0.3% | `_xzm_malloc` |
| 25 | 0.3% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 20 | 0.2% | `core::iter::traits::iterator::Iterator::try_fold` |
| 19 | 0.2% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 18 | 0.2% | `xzm_malloc_zone_malloc_type_realloc` |
| 18 | 0.2% | `DYLD-STUB$$mach_absolute_time` |
| 17 | 0.2% | `radiomics::numerics::sparse_counts::entropy` |
| 15 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 15 | 0.2% | `core::hash::BuildHasher::hash_one` |
| 13 | 0.2% | `__psynch_mutexwait` |
| 13 | 0.2% | `_malloc_zone_malloc` |
| 12 | 0.1% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 12 | 0.1% | `_<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop` |
| 11 | 0.1% | `DYLD-STUB$$memmove` |
| 10 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 10 | 0.1% | `_RNvCsGIExRX8pES_7___rustc14___rust_dealloc` |
| 9 | 0.1% | `DYLD-STUB$$malloc` |
| 8 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 8 | 0.1% | `_free` |
| 7 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 7 | 0.1% | `radiomics::features::distributions::size_statistics` |
| 7 | 0.1% | `DYLD-STUB$$free` |
| 7 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 6 | 0.1% | `radiomics::features::aggregation::resolved_aggregation` |
| 6 | 0.1% | `rayon_core::join::join_context::_{{closure}}` |
| 4 | 0.0% | `__floattidf` |
| 4 | 0.0% | `__psynch_cvsignal` |
| 4 | 0.0% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 4 | 0.0% | `radiomics::features::matrices::Domain::new` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 5321 | 63.8% | `radiomics::features::texture::compute` |
| 4010 | 48.1% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 2924 | 35.1% | `radiomics::workflows::extraction::extract` |
| 2924 | 35.1% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2924 | 35.1% | `radiomics::workflows::engine::run_prepared` |
| 2924 | 35.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1014 | 12.2% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 888 | 10.7% | `radiomics::features::distributions::size_statistics` |
| 547 | 6.6% | `radiomics::features::texture::compute::_{{closure}}` |
| 272 | 3.3% | `radiomics::numerics::sparse_counts::marginals` |
| 194 | 2.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 67 | 0.8% | `radiomics::numerics::sparse_counts::variance` |
| 49 | 0.6% | `radiomics::numerics::sparse_counts::entropy` |
| 35 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 8 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 6 | 0.1% | `radiomics::features::aggregation::resolved_aggregation` |
| 4 | 0.0% | `radiomics::features::matrices::Domain::new` |
| 3 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
