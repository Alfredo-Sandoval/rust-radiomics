# Profile: voxel/glszm-r1.paired-baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 glszm`
- Commit: `eb3c8ce` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:21 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 4930 (idle worker-thread samples excluded: 37573)

## Command output

```text
{"median_ms":1634.4706250000002,"samples_ms":[1509.784542,1604.909625,1634.4706250000002,2095.021417,3165.103792],"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 946 | 19.2% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 648 | 13.1% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 638 | 12.9% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 237 | 4.8% | `__bzero` |
| 204 | 4.1% | `_platform_memcmp` |
| 201 | 4.1% | `radiomics::features::texture::compute::_{{closure}}` |
| 200 | 4.1% | `_xzm_xzone_malloc_tiny` |
| 162 | 3.3% | `_xzm_free_main` |
| 159 | 3.2% | `<deduplicated_symbol>` |
| 128 | 2.6% | `__psynch_mutexwait` |
| 111 | 2.3% | `mach_absolute_time` |
| 109 | 2.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 89 | 1.8% | `_platform_memmove` |
| 74 | 1.5% | `_xzm_free` |
| 69 | 1.4% | `radiomics::features::texture::compute` |
| 51 | 1.0% | `core::iter::traits::iterator::Iterator::try_fold` |
| 48 | 1.0% | `__floattidf` |
| 47 | 1.0% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 44 | 0.9% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 42 | 0.9% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 41 | 0.8% | `xzm_malloc_zone_size` |
| 41 | 0.8% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 39 | 0.8% | `_<alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::fold` |
| 38 | 0.8% | `DYLD-STUB$$memcmp` |
| 35 | 0.7% | `radiomics::numerics::sparse_counts::marginals` |
| 35 | 0.7% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 33 | 0.7% | `radiomics::numerics::sparse_counts::entropy` |
| 32 | 0.6% | `_realloc` |
| 27 | 0.5% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 25 | 0.5% | `_free` |
| 24 | 0.5% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 24 | 0.5% | `radiomics::numerics::sparse_counts::variance` |
| 23 | 0.5% | `DYLD-STUB$$memcpy` |
| 22 | 0.4% | `xzm_malloc_zone_malloc_type_realloc` |
| 20 | 0.4% | `DYLD-STUB$$malloc` |
| 18 | 0.4% | `ndarray::iterators::Iter<A,D>::new` |
| 17 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 17 | 0.3% | `DYLD-STUB$$free` |
| 14 | 0.3% | `core::hash::BuildHasher::hash_one` |
| 13 | 0.3% | `_malloc_zone_realloc` |
| 11 | 0.2% | `malloc` |
| 11 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 11 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 10 | 0.2% | `_xzm_malloc` |
| 8 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 8 | 0.2% | `default_zone_realloc` |
| 8 | 0.2% | `_<rayon::iter::map::MapFolder<C,F> as rayon::iter::plumbing::Folder<T>>::consume_iter` |
| 8 | 0.2% | `log1p` |
| 7 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 7 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 7 | 0.1% | `core::str::_<impl str>::split_once` |
| 7 | 0.1% | `_malloc_zone_malloc` |
| 6 | 0.1% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 6 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 5 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 5 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 4 | 0.1% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 4 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 4 | 0.1% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 4 | 0.1% | `rayon_core::join::join_context::_{{closure}}` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2535 | 51.4% | `radiomics::features::texture::compute` |
| 2167 | 44.0% | `radiomics::workflows::extraction::extract` |
| 2167 | 44.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1660 | 33.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1621 | 32.9% | `radiomics::workflows::engine::run_prepared` |
| 1620 | 32.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 546 | 11.1% | `radiomics::imaging::preprocessing::prepare` |
| 439 | 8.9% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 437 | 8.9% | `radiomics::features::texture::compute::_{{closure}}` |
| 347 | 7.0% | `radiomics::features::distributions::size_statistics` |
| 292 | 5.9% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 254 | 5.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 248 | 5.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 195 | 4.0% | `radiomics::imaging::preprocessing::gather` |
| 122 | 2.5% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 116 | 2.4% | `radiomics::numerics::sparse_counts::marginals` |
| 72 | 1.5% | `radiomics::numerics::sparse_counts::variance` |
| 39 | 0.8% | `radiomics::numerics::sparse_counts::entropy` |
| 28 | 0.6% | `radiomics::imaging::preprocessing::normalize_values` |
| 17 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 12 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 3 | 0.1% | `radiomics::features::matrices::Domain::new` |
| 1 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
