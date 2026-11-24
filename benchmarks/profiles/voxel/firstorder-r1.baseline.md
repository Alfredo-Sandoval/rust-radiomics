# Profile: voxel/firstorder-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 firstorder`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:46 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2965 (idle worker-thread samples excluded: 53009)

## Command output

```text
{"median_ms":1659.8469169999998,"samples_ms":[1129.226542,1659.8469169999998,4119.007457999999],"roi_voxels":54639,"maps":21,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1019 | 34.4% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 820 | 27.7% | `__bzero` |
| 534 | 18.0% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 87 | 2.9% | `_platform_memmove` |
| 84 | 2.8% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 80 | 2.7% | `<deduplicated_symbol>` |
| 60 | 2.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 52 | 1.8% | `radiomics::features::first_order::first_order_maps` |
| 48 | 1.6% | `core::iter::traits::iterator::Iterator::try_fold` |
| 29 | 1.0% | `radiomics::features::first_order::LevelCounts::add` |
| 27 | 0.9% | `radiomics::features::first_order::RegionKernel::finish` |
| 22 | 0.7% | `_xzm_free_main` |
| 19 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 17 | 0.6% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 13 | 0.4% | `core::slice::sort::unstable::quicksort::quicksort` |
| 9 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 8 | 0.3% | `mach_absolute_time` |
| 7 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 6 | 0.2% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next` |
| 5 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 2 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 2 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.1% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 2 | 0.1% | `__psynch_mutexwait` |
| 2 | 0.1% | `xzm_malloc_zone_try_free_default_main` |
| 1 | 0.0% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 1 | 0.0% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 1 | 0.0% | `xzm_segment_group_alloc_chunk` |
| 1 | 0.0% | `_free` |
| 1 | 0.0% | `indexmap::inner::entry::_<impl indexmap::map::entry::Entry<K,V>>::new` |
| 1 | 0.0% | `mfm_alloc` |
| 1 | 0.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1 | 0.0% | `ndarray::impl_clone::_<impl core::clone::Clone for ndarray::ArrayBase<S,D>>::clone` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2925 | 98.7% | `radiomics::workflows::extraction::extract` |
| 2925 | 98.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2469 | 83.3% | `radiomics::workflows::engine::run_prepared` |
| 2469 | 83.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 503 | 17.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 496 | 16.7% | `radiomics::features::first_order::compute_maps` |
| 496 | 16.7% | `radiomics::features::first_order::first_order_maps` |
| 456 | 15.4% | `radiomics::imaging::preprocessing::prepare` |
| 244 | 8.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 212 | 7.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 132 | 4.5% | `radiomics::imaging::preprocessing::gather` |
| 72 | 2.4% | `radiomics::features::first_order::RegionKernel::finish` |
| 52 | 1.8% | `radiomics::imaging::preprocessing::normalize_values` |
| 48 | 1.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 46 | 1.6% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 37 | 1.2% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 29 | 1.0% | `radiomics::features::first_order::LevelCounts::add` |
| 8 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 8 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 7 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 3 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 2 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.1% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
