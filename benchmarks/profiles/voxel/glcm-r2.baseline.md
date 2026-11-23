# Profile: voxel/glcm-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 glcm`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:51 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 40930 (idle worker-thread samples excluded: 4347)

## Command output

```text
{"median_ms":14886.404584,"samples_ms":[12160.565583,14886.404584,17298.246416],"roi_voxels":54639,"maps":26,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 26353 | 64.4% | `radiomics::numerics::eigen::jacobi` |
| 2997 | 7.3% | `radiomics::numerics::eigen::maximal_correlation` |
| 2021 | 4.9% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1311 | 3.2% | `__floattidf` |
| 937 | 2.3% | `hypot` |
| 830 | 2.0% | `radiomics::features::glcm::mutual_information_bits` |
| 792 | 1.9% | `radiomics::numerics::sparse_counts::marginals` |
| 730 | 1.8% | `radiomics::features::glcm::glcm_statistics` |
| 592 | 1.4% | `_platform_memmove` |
| 452 | 1.1% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 370 | 0.9% | `DYLD-STUB$$hypot` |
| 353 | 0.9% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 321 | 0.8% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 294 | 0.7% | `log1p` |
| 283 | 0.7% | `_xzm_xzone_malloc_tiny` |
| 241 | 0.6% | `radiomics::numerics::sparse_counts::variance` |
| 181 | 0.4% | `<deduplicated_symbol>` |
| 174 | 0.4% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 108 | 0.3% | `_xzm_free_main` |
| 93 | 0.2% | `radiomics::numerics::sparse_counts::entropy` |
| 88 | 0.2% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 83 | 0.2% | `matrixmultiply::gemm::gemm_loop` |
| 73 | 0.2% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 69 | 0.2% | `mach_absolute_time` |
| 64 | 0.2% | `_platform_memset` |
| 64 | 0.2% | `_xzm_free` |
| 62 | 0.2% | `_platform_memcmp` |
| 61 | 0.1% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 58 | 0.1% | `_free` |
| 50 | 0.1% | `core::hash::BuildHasher::hash_one` |
| 49 | 0.1% | `_malloc_zone_malloc` |
| 46 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 45 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 41 | 0.1% | `xzm_malloc_zone_size` |
| 39 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 35 | 0.1% | `ndarray::iterators::to_vec_mapped` |
| 31 | 0.1% | `_malloc_zone_realloc` |
| 31 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 29 | 0.1% | `_xzm_xzone_malloc_from_freelist_chunk` |
| 27 | 0.1% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 27 | 0.1% | `radiomics::features::glcm::cluster_shade` |
| 24 | 0.1% | `DYLD-STUB$$memcmp` |
| 23 | 0.1% | `radiomics::features::texture::compute::_{{closure}}` |
| 22 | 0.1% | `DYLD-STUB$$log1p` |
| 22 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 22 | 0.1% | `_xzm_xzone_free_freelist` |
| 20 | 0.0% | `core::slice::sort::stable::quicksort::quicksort` |
| 20 | 0.0% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 20 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 19 | 0.0% | `matrixmultiply::gemm::masked_kernel` |
| 18 | 0.0% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 16 | 0.0% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 14 | 0.0% | `core::iter::traits::iterator::Iterator::try_fold` |
| 14 | 0.0% | `_malloc_zone_calloc` |
| 12 | 0.0% | `_posix_memalign` |
| 10 | 0.0% | `_xzm_free_outlined` |
| 10 | 0.0% | `_xzm_xzone_malloc_freelist_outlined` |
| 10 | 0.0% | `DYLD-STUB$$free` |
| 9 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 9 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 40626 | 99.3% | `radiomics::features::texture::compute` |
| 40342 | 98.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 37543 | 91.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 37311 | 91.2% | `radiomics::features::glcm::glcm_statistics` |
| 32397 | 79.2% | `radiomics::numerics::eigen::maximal_correlation` |
| 27874 | 68.1% | `radiomics::numerics::eigen::jacobi` |
| 2383 | 5.8% | `radiomics::workflows::extraction::extract` |
| 2383 | 5.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2383 | 5.8% | `radiomics::workflows::engine::run_prepared` |
| 2383 | 5.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1228 | 3.0% | `radiomics::features::glcm::mutual_information_bits` |
| 1014 | 2.5% | `radiomics::numerics::sparse_counts::marginals` |
| 361 | 0.9% | `radiomics::numerics::sparse_counts::variance` |
| 229 | 0.6% | `radiomics::numerics::sparse_counts::entropy` |
| 130 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 102 | 0.2% | `radiomics::features::texture::compute::_{{closure}}` |
| 58 | 0.1% | `radiomics::features::matrices::directions` |
| 53 | 0.1% | `radiomics::features::glcm::cluster_shade` |
| 27 | 0.1% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 6 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 2 | 0.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
