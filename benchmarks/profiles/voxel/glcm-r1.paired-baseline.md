# Profile: voxel/glcm-r1.paired-baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 glcm`
- Commit: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:24 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 12689 (idle worker-thread samples excluded: 30669)

## Command output

```text
{"median_ms":2594.1836249999997,"samples_ms":[2575.215125,2580.1045839999997,2594.1836249999997,2722.944541,4647.4115],"roi_voxels":54639,"maps":26,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3473 | 27.4% | `radiomics::numerics::eigen::jacobi` |
| 819 | 6.5% | `__floattidf` |
| 805 | 6.3% | `radiomics::numerics::eigen::maximal_correlation` |
| 673 | 5.3% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 515 | 4.1% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 440 | 3.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 419 | 3.3% | `hypot` |
| 377 | 3.0% | `__bzero` |
| 307 | 2.4% | `_platform_memmove` |
| 293 | 2.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 293 | 2.3% | `_xzm_xzone_malloc_tiny` |
| 289 | 2.3% | `radiomics::features::glcm::mutual_information_bits` |
| 278 | 2.2% | `<deduplicated_symbol>` |
| 276 | 2.2% | `radiomics::features::glcm::glcm_statistics` |
| 252 | 2.0% | `__psynch_mutexwait` |
| 231 | 1.8% | `log1p` |
| 227 | 1.8% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 211 | 1.7% | `_xzm_free_main` |
| 211 | 1.7% | `radiomics::numerics::sparse_counts::marginals` |
| 189 | 1.5% | `radiomics::numerics::sparse_counts::variance` |
| 170 | 1.3% | `DYLD-STUB$$hypot` |
| 163 | 1.3% | `mach_absolute_time` |
| 157 | 1.2% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 108 | 0.9% | `_xzm_free` |
| 105 | 0.8% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 80 | 0.6% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 68 | 0.5% | `radiomics::features::texture::compute` |
| 65 | 0.5% | `matrixmultiply::gemm::gemm_loop` |
| 65 | 0.5% | `_platform_memcmp` |
| 64 | 0.5% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 59 | 0.5% | `_realloc` |
| 55 | 0.4% | `radiomics::numerics::sparse_counts::entropy` |
| 53 | 0.4% | `xzm_malloc_zone_malloc_type_realloc` |
| 46 | 0.4% | `DYLD-STUB$$log1p` |
| 46 | 0.4% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 45 | 0.4% | `__psynch_mutexdrop` |
| 43 | 0.3% | `_malloc_zone_memalign` |
| 42 | 0.3% | `_malloc_zone_malloc` |
| 41 | 0.3% | `core::iter::traits::iterator::Iterator::fold` |
| 40 | 0.3% | `xzm_malloc_zone_size` |
| 39 | 0.3% | `_xzm_xzone_malloc_freelist_outlined` |
| 35 | 0.3% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 33 | 0.3% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 33 | 0.3% | `_xzm_malloc` |
| 30 | 0.2% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 27 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 26 | 0.2% | `radiomics::features::texture::compute::_{{closure}}` |
| 25 | 0.2% | `_malloc_zone_realloc` |
| 24 | 0.2% | `core::iter::traits::iterator::Iterator::try_fold` |
| 24 | 0.2% | `matrixmultiply::gemm::masked_kernel` |
| 23 | 0.2% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 22 | 0.2% | `_platform_memset` |
| 22 | 0.2% | `DYLD-STUB$$memcmp` |
| 20 | 0.2% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 20 | 0.2% | `_<alloc::collections::btree::map::Values<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 18 | 0.1% | `_free` |
| 18 | 0.1% | `pthread_mutex_unlock` |
| 14 | 0.1% | `radiomics::features::glcm::cluster_shade` |
| 10 | 0.1% | `DYLD-STUB$$memmove` |
| 10 | 0.1% | `default_zone_memalign` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 10472 | 82.5% | `radiomics::features::texture::compute` |
| 10033 | 79.1% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 9132 | 72.0% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 8907 | 70.2% | `radiomics::features::glcm::glcm_statistics` |
| 5845 | 46.1% | `radiomics::numerics::eigen::maximal_correlation` |
| 4273 | 33.7% | `radiomics::numerics::eigen::jacobi` |
| 2252 | 17.7% | `radiomics::workflows::extraction::extract` |
| 2252 | 17.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2041 | 16.1% | `radiomics::workflows::engine::run_prepared` |
| 2040 | 16.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 467 | 3.7% | `radiomics::features::glcm::mutual_information_bits` |
| 381 | 3.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 363 | 2.9% | `radiomics::numerics::sparse_counts::marginals` |
| 303 | 2.4% | `radiomics::numerics::sparse_counts::variance` |
| 250 | 2.0% | `radiomics::numerics::sparse_counts::entropy` |
| 211 | 1.7% | `radiomics::imaging::preprocessing::prepare` |
| 110 | 0.9% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 101 | 0.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 96 | 0.8% | `radiomics::imaging::preprocessing::gather` |
| 95 | 0.7% | `radiomics::features::matrices::directions` |
| 90 | 0.7% | `radiomics::features::texture::compute::_{{closure}}` |
| 85 | 0.7% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 54 | 0.4% | `radiomics::features::glcm::cluster_shade` |
| 35 | 0.3% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 32 | 0.3% | `radiomics::features::matrices::Domain::new` |
| 5 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
