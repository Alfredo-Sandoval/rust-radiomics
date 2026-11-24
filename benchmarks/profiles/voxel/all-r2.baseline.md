# Profile: voxel/all-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 all`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:59 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 10233 (idle worker-thread samples excluded: 985)

## Command output

```text
{"median_ms":22180.507667,"samples_ms":[12700.336625,22180.507667,22681.702],"roi_voxels":54639,"maps":147,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3385 | 33.1% | `radiomics::numerics::eigen::jacobi` |
| 1265 | 12.4% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 945 | 9.2% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 911 | 8.9% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 560 | 5.5% | `radiomics::numerics::eigen::maximal_correlation` |
| 264 | 2.6% | `__floattidf` |
| 210 | 2.1% | `_xzm_xzone_malloc_tiny` |
| 193 | 1.9% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 179 | 1.7% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 164 | 1.6% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 161 | 1.6% | `log1p` |
| 147 | 1.4% | `core::slice::sort::unstable::quicksort::quicksort` |
| 147 | 1.4% | `radiomics::numerics::sparse_counts::marginals` |
| 127 | 1.2% | `radiomics::features::glcm::mutual_information_bits` |
| 123 | 1.2% | `_platform_memmove` |
| 119 | 1.2% | `_xzm_free_main` |
| 83 | 0.8% | `core::slice::sort::shared::smallsort::sort8_stable` |
| 80 | 0.8% | `_platform_memcmp` |
| 75 | 0.7% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 74 | 0.7% | `radiomics::features::glcm::glcm_statistics` |
| 70 | 0.7% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 68 | 0.7% | `DYLD-STUB$$memcmp` |
| 65 | 0.6% | `hypot` |
| 63 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 59 | 0.6% | `DYLD-STUB$$hypot` |
| 59 | 0.6% | `xzm_malloc_zone_size` |
| 50 | 0.5% | `radiomics::features::first_order::first_order_maps` |
| 49 | 0.5% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 47 | 0.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 46 | 0.4% | `radiomics::numerics::sparse_counts::entropy` |
| 44 | 0.4% | `core::iter::traits::iterator::Iterator::try_fold` |
| 40 | 0.4% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 38 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 22 | 0.2% | `radiomics::numerics::sparse_counts::variance` |
| 22 | 0.2% | `radiomics::features::texture::compute` |
| 19 | 0.2% | `DYLD-STUB$$log1p` |
| 17 | 0.2% | `matrixmultiply::gemm::gemm_loop` |
| 15 | 0.1% | `DYLD-STUB$$memmove` |
| 13 | 0.1% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 13 | 0.1% | `_xzm_malloc` |
| 13 | 0.1% | `_realloc` |
| 12 | 0.1% | `xzm_malloc_zone_malloc_type_realloc` |
| 12 | 0.1% | `radiomics::features::texture::compute::_{{closure}}` |
| 12 | 0.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 10 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 9 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 9 | 0.1% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 9 | 0.1% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |
| 8 | 0.1% | `mach_absolute_time` |
| 8 | 0.1% | `<deduplicated_symbol>` |
| 8 | 0.1% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 7 | 0.1% | `_platform_memset` |
| 7 | 0.1% | `xzm_malloc_zone_try_free_default_main` |
| 6 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 6 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 6 | 0.1% | `core::hash::BuildHasher::hash_one` |
| 6 | 0.1% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 4 | 0.0% | `DYLD-STUB$$free` |
| 4 | 0.0% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 4 | 0.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 7308 | 71.4% | `radiomics::features::texture::compute` |
| 7009 | 68.5% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 5664 | 55.4% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 5274 | 51.5% | `radiomics::features::glcm::glcm_statistics` |
| 4328 | 42.3% | `radiomics::numerics::eigen::maximal_correlation` |
| 3599 | 35.2% | `radiomics::numerics::eigen::jacobi` |
| 3173 | 31.0% | `radiomics::workflows::extraction::extract` |
| 3173 | 31.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 3173 | 31.0% | `radiomics::workflows::engine::run_prepared` |
| 3173 | 31.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 429 | 4.2% | `radiomics::features::first_order::compute_maps` |
| 429 | 4.2% | `radiomics::features::first_order::first_order_maps` |
| 267 | 2.6% | `radiomics::features::first_order::RegionKernel::finish` |
| 262 | 2.6% | `radiomics::features::distributions::size_statistics` |
| 217 | 2.1% | `radiomics::numerics::sparse_counts::entropy` |
| 197 | 1.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 180 | 1.8% | `radiomics::numerics::sparse_counts::marginals` |
| 159 | 1.6% | `radiomics::features::texture::compute::_{{closure}}` |
| 156 | 1.5% | `radiomics::features::glcm::mutual_information_bits` |
| 114 | 1.1% | `radiomics::numerics::sparse_counts::variance` |
| 107 | 1.0% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 85 | 0.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 39 | 0.4% | `radiomics::features::intensity_histogram::compute` |
| 38 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 15 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 15 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 11 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 6 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 5 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 2 | 0.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 2 | 0.0% | `radiomics::features::ivh::ivh_statistics` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 1 | 0.0% | `radiomics::features::distributions::ngtdm_statistics` |
| 1 | 0.0% | `radiomics::features::glcm::cluster_shade` |
