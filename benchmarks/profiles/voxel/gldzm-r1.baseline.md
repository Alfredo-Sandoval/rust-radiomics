# Profile: voxel/gldzm-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 gldzm`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:45 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 5231 (idle worker-thread samples excluded: 48383)

## Command output

```text
{"median_ms":1121.456167,"samples_ms":[970.1555,993.2637090000001,1121.456167,1244.1991249999999,1372.573125],"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1176 | 22.5% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 1074 | 20.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 644 | 12.3% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 287 | 5.5% | `_platform_memcmp` |
| 205 | 3.9% | `radiomics::features::texture::compute::_{{closure}}` |
| 202 | 3.9% | `__bzero` |
| 168 | 3.2% | `<deduplicated_symbol>` |
| 124 | 2.4% | `_xzm_xzone_malloc_tiny` |
| 121 | 2.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 107 | 2.0% | `_xzm_free_main` |
| 107 | 2.0% | `mach_absolute_time` |
| 82 | 1.6% | `__psynch_mutexwait` |
| 73 | 1.4% | `_platform_memmove` |
| 68 | 1.3% | `_xzm_free` |
| 66 | 1.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 65 | 1.2% | `DYLD-STUB$$memcmp` |
| 57 | 1.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 48 | 0.9% | `radiomics::features::texture::compute` |
| 41 | 0.8% | `_<ndarray::indexes::IndicesIter<D> as core::iter::traits::iterator::Iterator>::fold` |
| 38 | 0.7% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 37 | 0.7% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 31 | 0.6% | `xzm_malloc_zone_malloc_type_realloc` |
| 29 | 0.6% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 23 | 0.4% | `radiomics::numerics::sparse_counts::marginals` |
| 22 | 0.4% | `core::hash::BuildHasher::hash_one` |
| 22 | 0.4% | `xzm_malloc_zone_size` |
| 19 | 0.4% | `_xzm_malloc` |
| 18 | 0.3% | `_realloc` |
| 16 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 16 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 15 | 0.3% | `_free` |
| 15 | 0.3% | `_malloc_zone_malloc` |
| 14 | 0.3% | `core::str::_<impl str>::split_once` |
| 11 | 0.2% | `_pthread_cond_wait` |
| 11 | 0.2% | `log1p` |
| 11 | 0.2% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 10 | 0.2% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 10 | 0.2% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 9 | 0.2% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 9 | 0.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 9 | 0.2% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 8 | 0.2% | `default_zone_realloc` |
| 7 | 0.1% | `_<alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::fold` |
| 6 | 0.1% | `DYLD-STUB$$free` |
| 6 | 0.1% | `_malloc_zone_realloc` |
| 6 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 5 | 0.1% | `radiomics::numerics::sparse_counts::variance` |
| 5 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 5 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 5 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 4 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 4 | 0.1% | `radiomics::features::aggregation::resolved_aggregation` |
| 4 | 0.1% | `_RNvCsGIExRX8pES_7___rustc12___rust_alloc` |
| 4 | 0.1% | `__floattidf` |
| 3 | 0.1% | `xzm_malloc_zone_realloc` |
| 3 | 0.1% | `ndarray::iterators::Iter<A,D>::new` |
| 3 | 0.1% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 3 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 3 | 0.1% | `DYLD-STUB$$memcpy` |
| 3 | 0.1% | `radiomics::features::distributions::size_statistics` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2882 | 55.1% | `radiomics::workflows::extraction::extract` |
| 2882 | 55.1% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2422 | 46.3% | `radiomics::workflows::engine::run_prepared` |
| 2417 | 46.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1995 | 38.1% | `radiomics::features::texture::compute` |
| 1156 | 22.1% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 542 | 10.4% | `radiomics::features::texture::compute::_{{closure}}` |
| 460 | 8.8% | `radiomics::imaging::preprocessing::prepare` |
| 287 | 5.5% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 262 | 5.0% | `radiomics::features::distributions::size_statistics` |
| 242 | 4.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 218 | 4.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 207 | 4.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 188 | 3.6% | `radiomics::imaging::preprocessing::gather` |
| 133 | 2.5% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 66 | 1.3% | `radiomics::numerics::sparse_counts::marginals` |
| 23 | 0.4% | `radiomics::imaging::preprocessing::normalize_values` |
| 17 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 11 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 9 | 0.2% | `radiomics::numerics::sparse_counts::variance` |
| 6 | 0.1% | `radiomics::numerics::sparse_counts::entropy` |
| 4 | 0.1% | `radiomics::features::aggregation::resolved_aggregation` |
| 3 | 0.1% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 3 | 0.1% | `radiomics::features::matrices::Domain::new` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
