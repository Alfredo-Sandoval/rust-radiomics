# Profile: voxel/glcm-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 glcm`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:43 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 17523 (idle worker-thread samples excluded: 18066)

## Command output

```text
{"median_ms":1357.1154585,"samples_ms":[1268.110125,1279.4959999999999,1434.7349170000002,1882.9369169999998],"roi_voxels":54639,"maps":26,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 5308 | 30.3% | `radiomics::numerics::eigen::jacobi` |
| 1078 | 6.2% | `__floattidf` |
| 972 | 5.5% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 931 | 5.3% | `radiomics::numerics::eigen::maximal_correlation` |
| 861 | 4.9% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 585 | 3.3% | `_xzm_xzone_malloc_tiny` |
| 584 | 3.3% | `radiomics::features::glcm::glcm_statistics` |
| 538 | 3.1% | `_platform_memmove` |
| 510 | 2.9% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 453 | 2.6% | `radiomics::numerics::sparse_counts::marginals` |
| 434 | 2.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 430 | 2.5% | `mach_absolute_time` |
| 404 | 2.3% | `_xzm_free_main` |
| 378 | 2.2% | `radiomics::features::glcm::mutual_information_bits` |
| 331 | 1.9% | `hypot` |
| 315 | 1.8% | `<deduplicated_symbol>` |
| 266 | 1.5% | `log1p` |
| 257 | 1.5% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 206 | 1.2% | `radiomics::numerics::sparse_counts::entropy` |
| 200 | 1.1% | `DYLD-STUB$$hypot` |
| 191 | 1.1% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 189 | 1.1% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 139 | 0.8% | `_xzm_free` |
| 134 | 0.8% | `__psynch_mutexwait` |
| 121 | 0.7% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 114 | 0.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 105 | 0.6% | `radiomics::numerics::sparse_counts::variance` |
| 102 | 0.6% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 86 | 0.5% | `matrixmultiply::gemm::gemm_loop` |
| 86 | 0.5% | `_platform_memcmp` |
| 84 | 0.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 75 | 0.4% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 75 | 0.4% | `__psynch_mutexdrop` |
| 74 | 0.4% | `radiomics::features::glcm::cluster_shade` |
| 71 | 0.4% | `radiomics::features::texture::compute` |
| 59 | 0.3% | `xzm_malloc_zone_malloc_type_realloc` |
| 51 | 0.3% | `_free` |
| 51 | 0.3% | `xzm_malloc_zone_size` |
| 46 | 0.3% | `radiomics::features::texture::compute::_{{closure}}` |
| 40 | 0.2% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 32 | 0.2% | `_xzm_malloc` |
| 30 | 0.2% | `core::iter::traits::iterator::Iterator::try_fold` |
| 28 | 0.2% | `_malloc_zone_realloc` |
| 27 | 0.2% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 26 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 22 | 0.1% | `ndarray::iterators::to_vec_mapped` |
| 22 | 0.1% | `_malloc_zone_malloc` |
| 21 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 19 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 19 | 0.1% | `pthread_mutex_unlock` |
| 18 | 0.1% | `DYLD-STUB$$memmove` |
| 18 | 0.1% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 17 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 17 | 0.1% | `DYLD-STUB$$log1p` |
| 15 | 0.1% | `core::iter::traits::iterator::Iterator::fold` |
| 15 | 0.1% | `matrixmultiply::gemm::masked_kernel` |
| 15 | 0.1% | `_platform_memset` |
| 12 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 12 | 0.1% | `core::hash::BuildHasher::hash_one` |
| 12 | 0.1% | `_realloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 15412 | 88.0% | `radiomics::features::texture::compute` |
| 14829 | 84.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 13399 | 76.5% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 13011 | 74.3% | `radiomics::features::glcm::glcm_statistics` |
| 8471 | 48.3% | `radiomics::numerics::eigen::maximal_correlation` |
| 6190 | 35.3% | `radiomics::numerics::eigen::jacobi` |
| 2446 | 14.0% | `radiomics::workflows::extraction::extract` |
| 2446 | 14.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2247 | 12.8% | `radiomics::workflows::engine::run_prepared` |
| 2246 | 12.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 656 | 3.7% | `radiomics::numerics::sparse_counts::marginals` |
| 550 | 3.1% | `radiomics::features::glcm::mutual_information_bits` |
| 399 | 2.3% | `radiomics::numerics::sparse_counts::entropy` |
| 256 | 1.5% | `radiomics::numerics::sparse_counts::variance` |
| 199 | 1.1% | `radiomics::imaging::preprocessing::prepare` |
| 163 | 0.9% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 140 | 0.8% | `radiomics::features::glcm::cluster_shade` |
| 126 | 0.7% | `radiomics::features::texture::compute::_{{closure}}` |
| 101 | 0.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 98 | 0.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 68 | 0.4% | `radiomics::imaging::preprocessing::gather` |
| 62 | 0.4% | `radiomics::features::matrices::directions` |
| 40 | 0.2% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 21 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 13 | 0.1% | `radiomics::features::matrices::Domain::new` |
| 9 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 6 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 4 | 0.0% | `radiomics::imaging::io::HostArray::eq_label` |
| 1 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
