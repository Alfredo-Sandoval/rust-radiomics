# Profile: voxel/defaults-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 defaults`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:58 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 23089 (idle worker-thread samples excluded: 2455)

## Command output

```text
{"median_ms":17203.382125,"samples_ms":[15333.860709,17203.382125,18636.796666],"roi_voxels":54639,"maps":93,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 12162 | 52.7% | `radiomics::numerics::eigen::jacobi` |
| 2053 | 8.9% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1260 | 5.5% | `radiomics::numerics::eigen::maximal_correlation` |
| 761 | 3.3% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 525 | 2.3% | `__floattidf` |
| 484 | 2.1% | `radiomics::numerics::sparse_counts::marginals` |
| 472 | 2.0% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 382 | 1.7% | `radiomics::features::glcm::mutual_information_bits` |
| 378 | 1.6% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 374 | 1.6% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 349 | 1.5% | `radiomics::features::glcm::glcm_statistics` |
| 333 | 1.4% | `hypot` |
| 330 | 1.4% | `log1p` |
| 324 | 1.4% | `_platform_memmove` |
| 303 | 1.3% | `_xzm_xzone_malloc_tiny` |
| 274 | 1.2% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 215 | 0.9% | `_xzm_free_main` |
| 212 | 0.9% | `DYLD-STUB$$hypot` |
| 174 | 0.8% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 138 | 0.6% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 125 | 0.5% | `_platform_memcmp` |
| 112 | 0.5% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 105 | 0.5% | `radiomics::numerics::sparse_counts::variance` |
| 99 | 0.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 64 | 0.3% | `mach_absolute_time` |
| 62 | 0.3% | `<deduplicated_symbol>` |
| 62 | 0.3% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 58 | 0.3% | `matrixmultiply::gemm::masked_kernel` |
| 55 | 0.2% | `radiomics::numerics::sparse_counts::entropy` |
| 53 | 0.2% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 46 | 0.2% | `xzm_malloc_zone_try_free_default_main` |
| 39 | 0.2% | `radiomics::features::texture::compute::_{{closure}}` |
| 38 | 0.2% | `DYLD-STUB$$log1p` |
| 37 | 0.2% | `_xzm_free` |
| 33 | 0.1% | `core::iter::traits::iterator::Iterator::fold` |
| 33 | 0.1% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 33 | 0.1% | `_realloc` |
| 31 | 0.1% | `_platform_memset` |
| 30 | 0.1% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 30 | 0.1% | `xzm_malloc_zone_malloc_type_realloc` |
| 30 | 0.1% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 28 | 0.1% | `_xzm_malloc` |
| 25 | 0.1% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 24 | 0.1% | `_malloc_zone_malloc` |
| 22 | 0.1% | `_xzm_xzone_malloc_small_freelist` |
| 22 | 0.1% | `DYLD-STUB$$malloc` |
| 21 | 0.1% | `matrixmultiply::gemm::gemm_loop` |
| 20 | 0.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 19 | 0.1% | `_free` |
| 16 | 0.1% | `DYLD-STUB$$realloc` |
| 16 | 0.1% | `DYLD-STUB$$free` |
| 16 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 14 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 12 | 0.1% | `_<ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 2$u5d$>> as ndarray::linalg::impl_linalg::Dot<ndarray::ArrayBase<S2,ndarray::dimension::dim:` |
| 10 | 0.0% | `radiomics::features::texture::compute` |
| 10 | 0.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 9 | 0.0% | `DYLD-STUB$$memmove` |
| 8 | 0.0% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next` |
| 8 | 0.0% | `DYLD-STUB$$memcpy` |
| 8 | 0.0% | `_xzm_chunk_batch_list_push` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 21602 | 93.6% | `radiomics::features::texture::compute` |
| 21204 | 91.8% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 18439 | 79.9% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 17405 | 75.4% | `radiomics::features::glcm::glcm_statistics` |
| 14740 | 63.8% | `radiomics::numerics::eigen::maximal_correlation` |
| 12824 | 55.5% | `radiomics::numerics::eigen::jacobi` |
| 2550 | 11.0% | `radiomics::workflows::extraction::extract` |
| 2550 | 11.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2550 | 11.0% | `radiomics::workflows::engine::run_prepared` |
| 2550 | 11.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 692 | 3.0% | `radiomics::numerics::sparse_counts::marginals` |
| 660 | 2.9% | `radiomics::features::distributions::size_statistics` |
| 560 | 2.4% | `radiomics::features::glcm::mutual_information_bits` |
| 302 | 1.3% | `radiomics::numerics::sparse_counts::entropy` |
| 166 | 0.7% | `radiomics::features::texture::compute::_{{closure}}` |
| 139 | 0.6% | `radiomics::numerics::sparse_counts::variance` |
| 69 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 34 | 0.1% | `radiomics::features::distributions::ngtdm_statistics` |
| 30 | 0.1% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 17 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 14 | 0.1% | `radiomics::features::glcm::cluster_shade` |
| 13 | 0.1% | `radiomics::features::matrices::directions` |
| 5 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 5 | 0.0% | `radiomics::features::selection::explicit_texture` |
