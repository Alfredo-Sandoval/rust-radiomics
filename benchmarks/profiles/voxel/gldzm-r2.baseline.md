# Profile: voxel/gldzm-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 gldzm`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:56 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 13509 (idle worker-thread samples excluded: 15476)

## Command output

```text
{"median_ms":2798.8766250000003,"samples_ms":[2055.3172919999997,2798.8766250000003,4870.417583],"roi_voxels":54639,"maps":16,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 7370 | 54.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 662 | 4.9% | `_platform_memcmp` |
| 544 | 4.0% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 480 | 3.6% | `radiomics::features::texture::compute::_{{closure}}` |
| 336 | 2.5% | `_xzm_xzone_malloc_tiny` |
| 308 | 2.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 302 | 2.2% | `_xzm_free_main` |
| 248 | 1.8% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 213 | 1.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 207 | 1.5% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 202 | 1.5% | `_xzm_free` |
| 191 | 1.4% | `_<ndarray::indexes::IndicesIter<D> as core::iter::traits::iterator::Iterator>::fold` |
| 189 | 1.4% | `mach_absolute_time` |
| 166 | 1.2% | `radiomics::numerics::sparse_counts::marginals` |
| 149 | 1.1% | `radiomics::features::texture::compute` |
| 136 | 1.0% | `DYLD-STUB$$memcmp` |
| 135 | 1.0% | `_platform_memmove` |
| 134 | 1.0% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 117 | 0.9% | `<deduplicated_symbol>` |
| 96 | 0.7% | `log1p` |
| 91 | 0.7% | `xzm_malloc_zone_malloc_type_realloc` |
| 84 | 0.6% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 84 | 0.6% | `core::hash::BuildHasher::hash_one` |
| 67 | 0.5% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 65 | 0.5% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 63 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 56 | 0.4% | `__psynch_mutexwait` |
| 55 | 0.4% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 54 | 0.4% | `_malloc_zone_malloc` |
| 48 | 0.4% | `core::iter::traits::iterator::Iterator::try_fold` |
| 48 | 0.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 41 | 0.3% | `_xzm_malloc` |
| 41 | 0.3% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 33 | 0.2% | `xzm_malloc_zone_size` |
| 32 | 0.2% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 31 | 0.2% | `_free` |
| 30 | 0.2% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 25 | 0.2% | `DYLD-STUB$$memcpy` |
| 23 | 0.2% | `radiomics::features::distributions::size_statistics` |
| 22 | 0.2% | `_<alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::fold` |
| 21 | 0.2% | `DYLD-STUB$$free` |
| 21 | 0.2% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 20 | 0.1% | `__psynch_mutexdrop` |
| 20 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 19 | 0.1% | `radiomics::numerics::sparse_counts::variance` |
| 18 | 0.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 17 | 0.1% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 15 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 14 | 0.1% | `_malloc_zone_realloc` |
| 14 | 0.1% | `crossbeam_deque::deque::Worker<T>::pop` |
| 13 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 12 | 0.1% | `malloc` |
| 11 | 0.1% | `_realloc` |
| 11 | 0.1% | `core::str::_<impl str>::split_once` |
| 11 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 9 | 0.1% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 8 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 8 | 0.1% | `_RNvCsGIExRX8pES_7___rustc14___rust_dealloc` |
| 7 | 0.1% | `_<rayon::iter::map::MapFolder<C,F> as rayon::iter::plumbing::Folder<T>>::consume_iter` |
| 5 | 0.0% | `_<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 11450 | 84.8% | `radiomics::features::texture::compute` |
| 9232 | 68.3% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 2197 | 16.3% | `radiomics::workflows::extraction::extract` |
| 2197 | 16.3% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1966 | 14.6% | `radiomics::workflows::engine::run_prepared` |
| 1966 | 14.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1272 | 9.4% | `radiomics::features::texture::compute::_{{closure}}` |
| 1055 | 7.8% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 956 | 7.1% | `radiomics::features::distributions::size_statistics` |
| 357 | 2.6% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 276 | 2.0% | `radiomics::numerics::sparse_counts::marginals` |
| 231 | 1.7% | `radiomics::imaging::preprocessing::prepare` |
| 231 | 1.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 168 | 1.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 63 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 56 | 0.4% | `radiomics::numerics::sparse_counts::entropy` |
| 33 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 22 | 0.2% | `radiomics::numerics::sparse_counts::variance` |
| 9 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 9 | 0.1% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 5 | 0.0% | `radiomics::features::matrices::Domain::new` |
