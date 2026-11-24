# Profile: core/firstorder-delivery-r1

- Command: `target/bench-core/retained-delivery/candidate ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 8 1 firstorder`
- Measured library: `2178bcb`
- Checkout at sampling: `2178bcb`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:22 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 4516 (idle worker-thread samples excluded: 11577)

## Command output

```text
{"median_ms":11.274916,"repeats":275,"min_ms":8.840959,"max_ms":41.85975,"roi_voxels":54639,"maps":21,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 825 | 18.3% | `__psynch_mutexwait` |
| 400 | 8.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 392 | 8.7% | `core::slice::sort::unstable::quicksort::quicksort` |
| 277 | 6.1% | `radiomics::features::first_order::RegionKernel::finish` |
| 270 | 6.0% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 268 | 5.9% | `madvise` |
| 177 | 3.9% | `radiomics::features::first_order::LevelCounts::add` |
| 175 | 3.9% | `radiomics::features::first_order_source::check_samples` |
| 139 | 3.1% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 132 | 2.9% | `__psynch_mutexdrop` |
| 97 | 2.1% | `core::slice::sort::shared::smallsort::insertion_sort_shift_left` |
| 91 | 2.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 81 | 1.8% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 79 | 1.7% | `radiomics::workflows::engine::delivery::store_column` |
| 79 | 1.7% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 76 | 1.7% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 74 | 1.6% | `_platform_memmove` |
| 63 | 1.4% | `__psynch_cvsignal` |
| 60 | 1.3% | `radiomics::numerics::reduction::reported_mean` |
| 59 | 1.3% | `crossbeam_epoch::default::with_handle` |
| 50 | 1.1% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 46 | 1.0% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 41 | 0.9% | `radiomics::features::first_order::LevelCounts::finish` |
| 39 | 0.9% | `radiomics::numerics::count_statistics::bin_entropy` |
| 32 | 0.7% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 28 | 0.6% | `core::slice::sort::unstable::ipnsort` |
| 26 | 0.6% | `_xzm_xzone_malloc_tiny` |
| 24 | 0.5% | `<deduplicated_symbol>` |
| 23 | 0.5% | `rayon_core::join::join_context::_{{closure}}` |
| 22 | 0.5% | `_platform_memset` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2485 | 55.0% | `radiomics::voxel::extraction::extract_scheduled::_{{closure}}` |
| 2320 | 51.4% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 1659 | 36.7% | `radiomics::features::first_order::RegionKernel::finish` |
| 967 | 21.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 845 | 18.7% | `radiomics::workflows::extraction::extract` |
| 845 | 18.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 711 | 15.7% | `radiomics::workflows::engine::run_prepared_at` |
| 708 | 15.7% | `radiomics::voxel::extraction::extract_scheduled` |
| 519 | 11.5% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 412 | 9.1% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 352 | 7.8% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 268 | 5.9% | `radiomics::numerics::runtime::zeroed_map` |
| 178 | 3.9% | `radiomics::features::first_order::LevelCounts::add` |
| 175 | 3.9% | `radiomics::features::first_order_source::check_samples` |
| 91 | 2.0% | `radiomics::imaging::preprocessing::prepare` |
| 91 | 2.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 79 | 1.7% | `radiomics::workflows::engine::delivery::store_column` |
| 79 | 1.7% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 66 | 1.5% | `radiomics::features::first_order::LevelCounts::finish` |
| 60 | 1.3% | `radiomics::numerics::reduction::reported_mean` |
| 53 | 1.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 50 | 1.1% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 42 | 0.9% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 39 | 0.9% | `radiomics::numerics::count_statistics::bin_entropy` |
| 38 | 0.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 36 | 0.8% | `radiomics::features::first_order::RegionKernel::new` |
| 34 | 0.8% | `radiomics::imaging::preprocessing::lattice_bins` |
| 34 | 0.8% | `radiomics::imaging::preprocessing::gather` |
| 28 | 0.6% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 25 | 0.6% | `radiomics::imaging::preprocessing::gather_at` |
