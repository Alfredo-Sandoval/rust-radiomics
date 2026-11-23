# Profile: voxel/firstorder-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 firstorder`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:56 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2255 (idle worker-thread samples excluded: 45131)

## Command output

```text
{"median_ms":3545.510292,"samples_ms":[2449.60925,3545.510292,3646.7023750000003],"roi_voxels":54639,"maps":21,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 651 | 28.9% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 517 | 22.9% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 211 | 9.4% | `__bzero` |
| 113 | 5.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 104 | 4.6% | `core::slice::sort::unstable::quicksort::quicksort` |
| 87 | 3.9% | `<deduplicated_symbol>` |
| 80 | 3.5% | `_platform_memmove` |
| 54 | 2.4% | `radiomics::features::first_order::RegionKernel::finish` |
| 49 | 2.2% | `core::iter::traits::iterator::Iterator::try_fold` |
| 47 | 2.1% | `radiomics::features::first_order::first_order_maps` |
| 45 | 2.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 39 | 1.7% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 36 | 1.6% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 30 | 1.3% | `radiomics::features::first_order::LevelCounts::add` |
| 26 | 1.2% | `_xzm_xzone_malloc_tiny` |
| 24 | 1.1% | `_xzm_free_main` |
| 22 | 1.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 20 | 0.9% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 17 | 0.8% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next` |
| 13 | 0.6% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 8 | 0.4% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 8 | 0.4% | `radiomics::numerics::reduction::reported_mean` |
| 8 | 0.4% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 7 | 0.3% | `log2` |
| 5 | 0.2% | `core::slice::sort::shared::pivot::median3_rec` |
| 5 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 4 | 0.2% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 4 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 3 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 3 | 0.1% | `_malloc_zone_malloc` |
| 2 | 0.1% | `radiomics::numerics::count_statistics::bin_entropy` |
| 2 | 0.1% | `mach_absolute_time` |
| 1 | 0.0% | `__umodti3` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 1 | 0.0% | `_xzm_malloc_large_huge` |
| 1 | 0.0% | `madvise` |
| 1 | 0.0% | `DYLD-STUB$$malloc` |
| 1 | 0.0% | `DYLD-STUB$$free` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 1 | 0.0% | `__psynch_mutexwait` |
| 1 | 0.0% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 1 | 0.0% | `crossbeam_epoch::internal::Global::try_advance` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2481 | 110.0% | `radiomics::workflows::extraction::extract` |
| 2481 | 110.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1984 | 88.0% | `radiomics::workflows::engine::run_prepared` |
| 1983 | 87.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 562 | 24.9% | `radiomics::features::first_order::compute_maps` |
| 562 | 24.9% | `radiomics::features::first_order::first_order_maps` |
| 497 | 22.0% | `radiomics::imaging::preprocessing::prepare` |
| 276 | 12.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 227 | 10.1% | `radiomics::features::first_order::RegionKernel::finish` |
| 221 | 9.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 198 | 8.8% | `radiomics::imaging::preprocessing::gather` |
| 147 | 6.5% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 114 | 5.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 77 | 3.4% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 67 | 3.0% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 30 | 1.3% | `radiomics::features::first_order::LevelCounts::add` |
| 16 | 0.7% | `radiomics::features::first_order::LevelCounts::finish` |
| 8 | 0.4% | `radiomics::numerics::count_statistics::bin_entropy` |
| 8 | 0.4% | `radiomics::numerics::reduction::reported_mean` |
| 7 | 0.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 7 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 4 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 4 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.1% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
