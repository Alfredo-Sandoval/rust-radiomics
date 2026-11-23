# Profile: voxel/windows-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel_stages ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 windows`
- Commit: `38ac856` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:54 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 41044 (idle worker-thread samples excluded: 3188)

## Command output

```text
{"median_ms":15485.750790999999,"repeats":3,"min_ms":10861.514833,"max_ms":17330.059333,"roi_voxels":54639,"maps":96,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 22756 | 55.4% | `radiomics::numerics::eigen::jacobi` |
| 4036 | 9.8% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 2351 | 5.7% | `radiomics::numerics::eigen::maximal_correlation` |
| 1321 | 3.2% | `__floattidf` |
| 817 | 2.0% | `hypot` |
| 788 | 1.9% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 746 | 1.8% | `radiomics::features::glcm::glcm_statistics` |
| 736 | 1.8% | `radiomics::numerics::sparse_counts::marginals` |
| 635 | 1.5% | `radiomics::features::glcm::mutual_information_bits` |
| 625 | 1.5% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 605 | 1.5% | `_platform_memmove` |
| 528 | 1.3% | `DYLD-STUB$$hypot` |
| 455 | 1.1% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 338 | 0.8% | `log1p` |
| 338 | 0.8% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 337 | 0.8% | `_xzm_xzone_malloc_tiny` |
| 287 | 0.7% | `_xzm_free_main` |
| 272 | 0.7% | `_platform_memcmp` |
| 267 | 0.7% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 211 | 0.5% | `radiomics::numerics::sparse_counts::variance` |
| 166 | 0.4% | `mach_absolute_time` |
| 162 | 0.4% | `radiomics::numerics::sparse_counts::entropy` |
| 119 | 0.3% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 115 | 0.3% | `<deduplicated_symbol>` |
| 115 | 0.3% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 113 | 0.3% | `matrixmultiply::gemm::gemm_loop` |
| 109 | 0.3% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 103 | 0.3% | `radiomics::features::texture::compute` |
| 99 | 0.2% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 95 | 0.2% | `_xzm_free` |
| 87 | 0.2% | `radiomics::features::texture::compute::_{{closure}}` |
| 83 | 0.2% | `core::slice::sort::stable::quicksort::quicksort` |
| 75 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 65 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 61 | 0.1% | `_malloc_zone_memalign` |
| 60 | 0.1% | `xzm_malloc_zone_malloc_type_realloc` |
| 58 | 0.1% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 57 | 0.1% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 57 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 54 | 0.1% | `__divti3` |
| 50 | 0.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 50 | 0.1% | `_free` |
| 47 | 0.1% | `_xzm_malloc` |
| 47 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 37 | 0.1% | `_malloc_zone_realloc` |
| 37 | 0.1% | `xzm_malloc_zone_size` |
| 36 | 0.1% | `_platform_memset` |
| 32 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 29 | 0.1% | `_posix_memalign` |
| 27 | 0.1% | `core::hash::BuildHasher::hash_one` |
| 24 | 0.1% | `radiomics::features::distributions::size_statistics` |
| 24 | 0.1% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 24 | 0.1% | `DYLD-STUB$$memcmp` |
| 23 | 0.1% | `DYLD-STUB$$log1p` |
| 22 | 0.1% | `DYLD-STUB$$memmove` |
| 18 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 16 | 0.0% | `core::str::_<impl str>::split_once` |
| 14 | 0.0% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 13 | 0.0% | `_malloc_zone_malloc` |
| 12 | 0.0% | `matrixmultiply::gemm::masked_kernel` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 40558 | 98.8% | `radiomics::features::texture::compute` |
| 39659 | 96.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 34379 | 83.8% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 32813 | 79.9% | `radiomics::features::glcm::glcm_statistics` |
| 28203 | 68.7% | `radiomics::numerics::eigen::maximal_correlation` |
| 24267 | 59.1% | `radiomics::numerics::eigen::jacobi` |
| 2328 | 5.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1142 | 2.8% | `radiomics::features::distributions::size_statistics` |
| 1051 | 2.6% | `radiomics::numerics::sparse_counts::marginals` |
| 928 | 2.3% | `radiomics::features::glcm::mutual_information_bits` |
| 378 | 0.9% | `radiomics::features::texture::compute::_{{closure}}` |
| 377 | 0.9% | `radiomics::numerics::sparse_counts::variance` |
| 368 | 0.9% | `radiomics::numerics::sparse_counts::entropy` |
| 311 | 0.8% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 173 | 0.4% | `radiomics::features::glcm::cluster_shade` |
| 115 | 0.3% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 63 | 0.2% | `radiomics::features::distributions::ngtdm_statistics` |
| 2 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 2 | 0.0% | `radiomics::features::matrices::directions` |
| 1 | 0.0% | `radiomics::features::selection::explicit_texture` |
| 1 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
