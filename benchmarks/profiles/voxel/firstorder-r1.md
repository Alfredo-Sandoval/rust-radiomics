# Profile: voxel/firstorder-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 firstorder`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:18 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2025 (idle worker-thread samples excluded: 34854)

## Command output

```text
{"median_ms":862.8418335,"repeats":6,"min_ms":672.415667,"max_ms":1280.451584,"roi_voxels":54639,"maps":21,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 644 | 31.8% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 483 | 23.9% | `__bzero` |
| 216 | 10.7% | `<deduplicated_symbol>` |
| 161 | 8.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 79 | 3.9% | `_platform_memmove` |
| 77 | 3.8% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 69 | 3.4% | `core::iter::traits::iterator::Iterator::try_fold` |
| 50 | 2.5% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 42 | 2.1% | `__psynch_mutexwait` |
| 30 | 1.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 26 | 1.3% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 24 | 1.2% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 21 | 1.0% | `core::slice::sort::unstable::quicksort::quicksort` |
| 17 | 0.8% | `radiomics::features::first_order::LevelCounts::add` |
| 15 | 0.7% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 12 | 0.6% | `radiomics::features::first_order::LevelCounts::finish` |
| 6 | 0.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 6 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 5 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 4 | 0.2% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 4 | 0.2% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 4 | 0.2% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 4 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 4 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 3 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 3 | 0.1% | `crossbeam_epoch::internal::Global::try_advance` |
| 1 | 0.0% | `_xzm_reclaim_mark_used` |
| 1 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 1 | 0.0% | `_<&$u5b$usize$u5d$ as ndarray::dimension::ndindex::NdIndex<ndarray::dimension::dim::Dim<ndarray::dimension::dynindeximpl::IxDynImpl>>>::index_checked` |
| 1 | 0.0% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 1 | 0.0% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 1 | 0.0% | `core::slice::sort::unstable::ipnsort` |
| 1 | 0.0% | `radiomics::native_first_order::statistics::Calculation::new` |
| 1 | 0.0% | `mach_absolute_time` |
| 1 | 0.0% | `__psynch_cvbroad` |
| 1 | 0.0% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 1 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 1 | 0.0% | `cthread_yield` |
| 1 | 0.0% | `_<core::iter::adapters::GenericShunt<I,R> as core::iter::traits::iterator::Iterator>::next` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 1 | 0.0% | `_xzm_free_main` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1892 | 93.4% | `radiomics::workflows::extraction::extract` |
| 1892 | 93.4% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1117 | 55.2% | `radiomics::imaging::preprocessing::prepare` |
| 775 | 38.3% | `radiomics::workflows::engine::run_prepared` |
| 771 | 38.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 629 | 31.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 492 | 24.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 488 | 24.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 373 | 18.4% | `radiomics::imaging::preprocessing::gather` |
| 155 | 7.7% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 107 | 5.3% | `radiomics::features::first_order::RegionKernel::finish` |
| 54 | 2.7% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 43 | 2.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 41 | 2.0% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 35 | 1.7% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 34 | 1.7% | `radiomics::imaging::io::HostArray::eq_label` |
| 18 | 0.9% | `radiomics::features::first_order::LevelCounts::add` |
| 14 | 0.7% | `radiomics::features::first_order::LevelCounts::finish` |
| 6 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 4 | 0.2% | `radiomics::imaging::preprocessing::resegment_samples` |
| 4 | 0.2% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 4 | 0.2% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 3 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 2 | 0.1% | `radiomics::features::first_order::RegionKernel::new` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 1 | 0.0% | `radiomics::features::first_order::SampleStatistics::new` |
| 1 | 0.0% | `radiomics::native_first_order::statistics::Calculation::new` |
