# Profile: voxel/windows-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel_stages ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 windows`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:50 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 34225 (idle worker-thread samples excluded: 5656)

## Command output

```text
{"median_ms":3248.391958,"repeats":3,"min_ms":3002.226667,"max_ms":3873.079208,"roi_voxels":54639,"maps":96,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 7005 | 20.5% | `radiomics::numerics::eigen::jacobi` |
| 4581 | 13.4% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1935 | 5.7% | `radiomics::numerics::eigen::maximal_correlation` |
| 1453 | 4.2% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1267 | 3.7% | `_xzm_xzone_malloc_tiny` |
| 1196 | 3.5% | `__floattidf` |
| 1027 | 3.0% | `_platform_memmove` |
| 1007 | 2.9% | `_platform_memcmp` |
| 940 | 2.7% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 930 | 2.7% | `_xzm_free_main` |
| 815 | 2.4% | `radiomics::numerics::sparse_counts::marginals` |
| 774 | 2.3% | `mach_absolute_time` |
| 715 | 2.1% | `radiomics::features::texture::compute::_{{closure}}` |
| 673 | 2.0% | `radiomics::features::glcm::glcm_statistics` |
| 671 | 2.0% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 658 | 1.9% | `radiomics::features::glcm::mutual_information_bits` |
| 548 | 1.6% | `_xzm_free` |
| 536 | 1.6% | `log1p` |
| 512 | 1.5% | `<deduplicated_symbol>` |
| 484 | 1.4% | `hypot` |
| 426 | 1.2% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 393 | 1.1% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 338 | 1.0% | `radiomics::numerics::sparse_counts::variance` |
| 301 | 0.9% | `radiomics::numerics::sparse_counts::entropy` |
| 281 | 0.8% | `DYLD-STUB$$hypot` |
| 250 | 0.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 248 | 0.7% | `xzm_malloc_zone_size` |
| 229 | 0.7% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 227 | 0.7% | `xzm_malloc_zone_malloc_type_realloc` |
| 206 | 0.6% | `radiomics::features::texture::compute` |
| 206 | 0.6% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 200 | 0.6% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 178 | 0.5% | `_xzm_malloc` |
| 166 | 0.5% | `_realloc` |
| 160 | 0.5% | `DYLD-STUB$$log1p` |
| 152 | 0.4% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 144 | 0.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 135 | 0.4% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 117 | 0.3% | `_free` |
| 112 | 0.3% | `_malloc_zone_memalign` |
| 105 | 0.3% | `matrixmultiply::gemm::masked_kernel` |
| 101 | 0.3% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 97 | 0.3% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 91 | 0.3% | `core::hash::BuildHasher::hash_one` |
| 90 | 0.3% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 82 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 82 | 0.2% | `DYLD-STUB$$memcmp` |
| 81 | 0.2% | `__divti3` |
| 79 | 0.2% | `_malloc_zone_malloc` |
| 77 | 0.2% | `matrixmultiply::gemm::gemm_loop` |
| 77 | 0.2% | `DYLD-STUB$$memcpy` |
| 75 | 0.2% | `radiomics::features::glcm::cluster_shade` |
| 68 | 0.2% | `DYLD-STUB$$free` |
| 64 | 0.2% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 62 | 0.2% | `_malloc_zone_realloc` |
| 60 | 0.2% | `_RNvCsGIExRX8pES_7___rustc12___rust_alloc` |
| 45 | 0.1% | `DYLD-STUB$$realloc` |
| 38 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 37 | 0.1% | `free` |
| 36 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 32942 | 96.3% | `radiomics::features::texture::compute` |
| 29222 | 85.4% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 22221 | 64.9% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 18029 | 52.7% | `radiomics::features::glcm::glcm_statistics` |
| 11782 | 34.4% | `radiomics::numerics::eigen::maximal_correlation` |
| 8075 | 23.6% | `radiomics::numerics::eigen::jacobi` |
| 3166 | 9.3% | `radiomics::features::distributions::size_statistics` |
| 2099 | 6.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1779 | 5.2% | `radiomics::features::texture::compute::_{{closure}}` |
| 1292 | 3.8% | `radiomics::numerics::sparse_counts::marginals` |
| 947 | 2.8% | `radiomics::features::glcm::mutual_information_bits` |
| 770 | 2.2% | `radiomics::numerics::sparse_counts::entropy` |
| 766 | 2.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 609 | 1.8% | `radiomics::numerics::sparse_counts::variance` |
| 318 | 0.9% | `radiomics::features::glcm::cluster_shade` |
| 101 | 0.3% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 69 | 0.2% | `radiomics::features::distributions::ngtdm_statistics` |
| 59 | 0.2% | `radiomics::features::matrices::directions` |
| 20 | 0.1% | `radiomics::features::aggregation::resolved_aggregation` |
| 7 | 0.0% | `radiomics::features::matrices::Domain::new` |
| 6 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
