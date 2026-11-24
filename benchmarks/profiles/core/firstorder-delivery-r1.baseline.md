# Profile: core/firstorder-delivery-r1.baseline

- Command: `target/bench-core/retained-delivery/baseline ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 8 1 firstorder`
- Measured library: `bc7a192`
- Checkout at sampling: `2178bcb`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:22 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 4202 (idle worker-thread samples excluded: 12100)

## Command output

```text
{"median_ms":13.406521,"repeats":232,"min_ms":11.139125,"max_ms":61.288000000000004,"roi_voxels":54639,"maps":21,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 696 | 16.6% | `__psynch_mutexwait` |
| 380 | 9.0% | `core::slice::sort::unstable::quicksort::quicksort` |
| 355 | 8.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 302 | 7.2% | `radiomics::features::first_order::RegionKernel::finish` |
| 284 | 6.8% | `madvise` |
| 230 | 5.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 153 | 3.6% | `radiomics::features::first_order_source::check_samples` |
| 147 | 3.5% | `radiomics::features::first_order::LevelCounts::add` |
| 140 | 3.3% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 134 | 3.2% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 113 | 2.7% | `__psynch_mutexdrop` |
| 95 | 2.3% | `core::slice::sort::shared::smallsort::insertion_sort_shift_left` |
| 95 | 2.3% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 80 | 1.9% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 70 | 1.7% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 65 | 1.5% | `radiomics::numerics::reduction::reported_mean` |
| 64 | 1.5% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 63 | 1.5% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 58 | 1.4% | `__psynch_cvsignal` |
| 55 | 1.3% | `crossbeam_epoch::default::with_handle` |
| 50 | 1.2% | `_platform_memmove` |
| 42 | 1.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 32 | 0.8% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 31 | 0.7% | `rayon_core::join::join_context::_{{closure}}` |
| 28 | 0.7% | `_platform_memset` |
| 28 | 0.7% | `radiomics::numerics::count_statistics::bin_entropy` |
| 27 | 0.6% | `radiomics::features::first_order::LevelCounts::finish` |
| 24 | 0.6% | `core::slice::sort::unstable::ipnsort` |
| 19 | 0.5% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 16 | 0.4% | `crossbeam_epoch::internal::Global::try_advance` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2280 | 54.3% | `radiomics::voxel::extraction::extract_scheduled::_{{closure}}` |
| 2125 | 50.6% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 1574 | 37.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 894 | 21.3% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 857 | 20.4% | `radiomics::workflows::extraction::extract` |
| 857 | 20.4% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 743 | 17.7% | `radiomics::workflows::engine::run_prepared_at` |
| 741 | 17.6% | `radiomics::voxel::extraction::extract_scheduled` |
| 445 | 10.6% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 427 | 10.2% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 348 | 8.3% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 284 | 6.8% | `radiomics::numerics::runtime::zeroed_map` |
| 153 | 3.6% | `radiomics::features::first_order_source::check_samples` |
| 147 | 3.5% | `radiomics::features::first_order::LevelCounts::add` |
| 95 | 2.3% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 80 | 1.9% | `radiomics::imaging::preprocessing::prepare` |
| 65 | 1.5% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 65 | 1.5% | `radiomics::numerics::reduction::reported_mean` |
| 47 | 1.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 47 | 1.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 42 | 1.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 37 | 0.9% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 34 | 0.8% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 33 | 0.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 32 | 0.8% | `radiomics::imaging::preprocessing::lattice_bins` |
| 28 | 0.7% | `radiomics::numerics::count_statistics::bin_entropy` |
| 27 | 0.6% | `radiomics::imaging::preprocessing::gather` |
| 27 | 0.6% | `radiomics::features::first_order::RegionKernel::new` |
| 24 | 0.6% | `radiomics::workflows::engine::inputs::input_window` |
| 24 | 0.6% | `radiomics::workflows::roi::bounds::include` |
