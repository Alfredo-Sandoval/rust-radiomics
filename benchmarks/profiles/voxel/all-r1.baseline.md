# Profile: voxel/all-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 all`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:49 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 19450 (idle worker-thread samples excluded: 2575)

## Command output

```text
{"median_ms":8488.074958000001,"samples_ms":[6357.34625,8488.074958000001,18602.40025],"roi_voxels":54639,"maps":147,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 4227 | 21.7% | `radiomics::numerics::eigen::jacobi` |
| 2140 | 11.0% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1020 | 5.2% | `radiomics::numerics::eigen::maximal_correlation` |
| 979 | 5.0% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 936 | 4.8% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 760 | 3.9% | `__floattidf` |
| 734 | 3.8% | `_xzm_xzone_malloc_tiny` |
| 581 | 3.0% | `_platform_memcmp` |
| 534 | 2.7% | `_platform_memmove` |
| 482 | 2.5% | `radiomics::numerics::sparse_counts::marginals` |
| 473 | 2.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 382 | 2.0% | `_xzm_free_main` |
| 377 | 1.9% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 352 | 1.8% | `mach_absolute_time` |
| 305 | 1.6% | `log1p` |
| 295 | 1.5% | `radiomics::features::glcm::mutual_information_bits` |
| 278 | 1.4% | `<deduplicated_symbol>` |
| 266 | 1.4% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 262 | 1.3% | `radiomics::features::texture::compute::_{{closure}}` |
| 243 | 1.2% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 234 | 1.2% | `radiomics::features::glcm::glcm_statistics` |
| 223 | 1.1% | `hypot` |
| 187 | 1.0% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 173 | 0.9% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 169 | 0.9% | `DYLD-STUB$$hypot` |
| 157 | 0.8% | `radiomics::numerics::sparse_counts::variance` |
| 152 | 0.8% | `radiomics::features::texture::compute` |
| 150 | 0.8% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 136 | 0.7% | `radiomics::numerics::sparse_counts::entropy` |
| 132 | 0.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 132 | 0.7% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 128 | 0.7% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 122 | 0.6% | `_xzm_free` |
| 109 | 0.6% | `xzm_malloc_zone_malloc_type_realloc` |
| 103 | 0.5% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 92 | 0.5% | `matrixmultiply::gemm::gemm_loop` |
| 77 | 0.4% | `_realloc` |
| 72 | 0.4% | `_malloc_zone_realloc` |
| 62 | 0.3% | `DYLD-STUB$$memcmp` |
| 58 | 0.3% | `radiomics::features::intensity_histogram::compute` |
| 57 | 0.3% | `_free` |
| 55 | 0.3% | `_<ndarray::indexes::IndicesIter<D> as core::iter::traits::iterator::Iterator>::fold` |
| 53 | 0.3% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 51 | 0.3% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 49 | 0.3% | `_malloc_zone_malloc` |
| 48 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 44 | 0.2% | `DYLD-STUB$$log1p` |
| 43 | 0.2% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 38 | 0.2% | `_platform_memset_pattern16` |
| 37 | 0.2% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 34 | 0.2% | `DYLD-STUB$$malloc` |
| 30 | 0.2% | `_xzm_malloc` |
| 28 | 0.1% | `radiomics::features::glcm::cluster_shade` |
| 27 | 0.1% | `core::iter::traits::iterator::Iterator::fold` |
| 26 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 26 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 24 | 0.1% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 23 | 0.1% | `default_zone_realloc` |
| 23 | 0.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 22 | 0.1% | `DYLD-STUB$$memmove` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 16657 | 85.6% | `radiomics::features::texture::compute` |
| 14830 | 76.2% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 11408 | 58.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 9405 | 48.4% | `radiomics::features::glcm::glcm_statistics` |
| 6585 | 33.9% | `radiomics::numerics::eigen::maximal_correlation` |
| 4729 | 24.3% | `radiomics::numerics::eigen::jacobi` |
| 2573 | 13.2% | `radiomics::workflows::extraction::extract` |
| 2573 | 13.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2573 | 13.2% | `radiomics::workflows::engine::run_prepared` |
| 2573 | 13.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1398 | 7.2% | `radiomics::features::distributions::size_statistics` |
| 869 | 4.5% | `radiomics::features::texture::compute::_{{closure}}` |
| 773 | 4.0% | `radiomics::numerics::sparse_counts::marginals` |
| 434 | 2.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 433 | 2.2% | `radiomics::features::glcm::mutual_information_bits` |
| 365 | 1.9% | `radiomics::numerics::sparse_counts::entropy` |
| 299 | 1.5% | `radiomics::features::intensity_histogram::compute` |
| 283 | 1.5% | `radiomics::numerics::sparse_counts::variance` |
| 114 | 0.6% | `radiomics::features::distributions::ngtdm_statistics` |
| 113 | 0.6% | `radiomics::features::ivh::ivh_statistics` |
| 43 | 0.2% | `radiomics::features::glcm::cluster_shade` |
| 32 | 0.2% | `radiomics::features::matrices::directions` |
| 27 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 21 | 0.1% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 14 | 0.1% | `radiomics::features::intensity_histogram::linear_percentile` |
| 10 | 0.1% | `radiomics::features::matrices::Domain::new` |
| 9 | 0.0% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 6 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
| 5 | 0.0% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 2 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 2 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
