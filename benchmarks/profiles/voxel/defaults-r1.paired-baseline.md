# Profile: voxel/defaults-r1.paired-baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 defaults`
- Commit: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:25 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 23375 (idle worker-thread samples excluded: 5107)

## Command output

```text
{"median_ms":3939.661,"samples_ms":[3715.103,3735.449375,3939.661,4008.7557919999995,5201.361917],"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 4929 | 21.1% | `radiomics::numerics::eigen::jacobi` |
| 2421 | 10.4% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1475 | 6.3% | `__floattidf` |
| 1213 | 5.2% | `radiomics::numerics::eigen::maximal_correlation` |
| 1131 | 4.8% | `_xzm_xzone_malloc_tiny` |
| 937 | 4.0% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 845 | 3.6% | `radiomics::numerics::sparse_counts::marginals` |
| 716 | 3.1% | `_xzm_free_main` |
| 673 | 2.9% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 528 | 2.3% | `mach_absolute_time` |
| 483 | 2.1% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 475 | 2.0% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 473 | 2.0% | `_platform_memmove` |
| 450 | 1.9% | `_platform_memcmp` |
| 416 | 1.8% | `log1p` |
| 399 | 1.7% | `radiomics::features::glcm::mutual_information_bits` |
| 382 | 1.6% | `hypot` |
| 359 | 1.5% | `radiomics::features::glcm::glcm_statistics` |
| 354 | 1.5% | `radiomics::features::texture::compute` |
| 353 | 1.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 298 | 1.3% | `_xzm_free` |
| 252 | 1.1% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 226 | 1.0% | `<deduplicated_symbol>` |
| 224 | 1.0% | `radiomics::numerics::sparse_counts::variance` |
| 204 | 0.9% | `radiomics::features::texture::compute::_{{closure}}` |
| 203 | 0.9% | `DYLD-STUB$$hypot` |
| 183 | 0.8% | `xzm_malloc_zone_malloc_type_realloc` |
| 180 | 0.8% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 176 | 0.8% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 170 | 0.7% | `xzm_malloc_zone_size` |
| 162 | 0.7% | `radiomics::numerics::sparse_counts::entropy` |
| 142 | 0.6% | `_xzm_malloc` |
| 125 | 0.5% | `_free` |
| 123 | 0.5% | `radiomics::features::glcm::cluster_shade` |
| 94 | 0.4% | `DYLD-STUB$$log1p` |
| 90 | 0.4% | `DYLD-STUB$$memcmp` |
| 83 | 0.4% | `matrixmultiply::gemm::gemm_loop` |
| 81 | 0.3% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 77 | 0.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 72 | 0.3% | `core::iter::traits::iterator::Iterator::fold` |
| 71 | 0.3% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 67 | 0.3% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 63 | 0.3% | `_realloc` |
| 53 | 0.2% | `_malloc_zone_malloc` |
| 53 | 0.2% | `__divti3` |
| 53 | 0.2% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 53 | 0.2% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 48 | 0.2% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 43 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 37 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 37 | 0.2% | `_malloc_zone_realloc` |
| 33 | 0.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 32 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 32 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 30 | 0.1% | `core::hash::BuildHasher::hash_one` |
| 29 | 0.1% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 28 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 25 | 0.1% | `radiomics::features::first_order::first_order_maps` |
| 22 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 22 | 0.1% | `DYLD-STUB$$memmove` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 21474 | 91.9% | `radiomics::features::texture::compute` |
| 19326 | 82.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 15453 | 66.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 12914 | 55.2% | `radiomics::features::glcm::glcm_statistics` |
| 8565 | 36.6% | `radiomics::numerics::eigen::maximal_correlation` |
| 5772 | 24.7% | `radiomics::numerics::eigen::jacobi` |
| 2637 | 11.3% | `radiomics::workflows::extraction::extract` |
| 2637 | 11.3% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2515 | 10.8% | `radiomics::workflows::engine::run_prepared` |
| 2514 | 10.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 2045 | 8.7% | `radiomics::features::distributions::size_statistics` |
| 1202 | 5.1% | `radiomics::numerics::sparse_counts::marginals` |
| 731 | 3.1% | `radiomics::features::texture::compute::_{{closure}}` |
| 582 | 2.5% | `radiomics::numerics::sparse_counts::entropy` |
| 576 | 2.5% | `radiomics::features::glcm::mutual_information_bits` |
| 562 | 2.4% | `radiomics::numerics::sparse_counts::variance` |
| 283 | 1.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 206 | 0.9% | `radiomics::features::glcm::cluster_shade` |
| 122 | 0.5% | `radiomics::imaging::preprocessing::prepare` |
| 122 | 0.5% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 103 | 0.4% | `radiomics::features::distributions::ngtdm_statistics` |
| 73 | 0.3% | `radiomics::features::first_order::compute_maps` |
| 73 | 0.3% | `radiomics::features::first_order::first_order_maps` |
| 29 | 0.1% | `radiomics::features::first_order::RegionKernel::finish` |
| 29 | 0.1% | `radiomics::features::matrices::directions` |
| 26 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 18 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 17 | 0.1% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 15 | 0.1% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 13 | 0.1% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 10 | 0.0% | `radiomics::features::first_order::LevelCounts::add` |
| 9 | 0.0% | `radiomics::features::first_order::LevelCounts::finish` |
| 6 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 3 | 0.0% | `radiomics::features::matrices::Domain::new` |
| 3 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
| 2 | 0.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
