# Profile: core/voxel-delivery-r1

- Command: `target/bench-core/retained-delivery/candidate ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 8 1 defaults`
- Measured library: `2178bcb`
- Checkout at sampling: `2178bcb`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:22 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 7302 (idle worker-thread samples excluded: 9971)

## Command output

```text
{"median_ms":86.208271,"repeats":36,"min_ms":71.601958,"max_ms":244.743292,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3213 | 44.0% | `radiomics::voxel::extraction::extract_scheduled::_{{closure}}` |
| 1433 | 19.6% | `radiomics::voxel::spectrum::spectral_radius` |
| 511 | 7.0% | `__psynch_mutexwait` |
| 458 | 6.3% | `_platform_memset` |
| 321 | 4.4% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 150 | 2.1% | `radiomics::workflows::engine::delivery::store_column` |
| 143 | 2.0% | `madvise` |
| 70 | 1.0% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 67 | 0.9% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 65 | 0.9% | `core::slice::sort::unstable::quicksort::quicksort` |
| 60 | 0.8% | `radiomics::features::first_order::RegionKernel::finish` |
| 53 | 0.7% | `__psynch_cvsignal` |
| 47 | 0.6% | `crossbeam_epoch::default::with_handle` |
| 42 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 38 | 0.5% | `__psynch_mutexdrop` |
| 37 | 0.5% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 36 | 0.5% | `expm1` |
| 31 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 30 | 0.4% | `_platform_memmove` |
| 29 | 0.4% | `_kernelrpc_mach_vm_deallocate_trap` |
| 24 | 0.3% | `acos` |
| 23 | 0.3% | `radiomics::features::first_order_source::check_samples` |
| 22 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 21 | 0.3% | `crossbeam_epoch::internal::Global::try_advance` |
| 20 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 19 | 0.3% | `_xzm_free_tc` |
| 17 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 14 | 0.2% | `DYLD-STUB$$bzero` |
| 14 | 0.2% | `core::slice::sort::shared::smallsort::insertion_sort_shift_left` |
| 14 | 0.2% | `_tlv_get_addr` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 5645 | 77.3% | `radiomics::voxel::extraction::extract_scheduled::_{{closure}}` |
| 1615 | 22.1% | `radiomics::voxel::spectrum::spectral_radius` |
| 878 | 12.0% | `radiomics::workflows::extraction::extract` |
| 878 | 12.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 862 | 11.8% | `radiomics::workflows::engine::run_prepared_at` |
| 861 | 11.8% | `radiomics::voxel::extraction::extract_scheduled` |
| 400 | 5.5% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 322 | 4.4% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 283 | 3.9% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 265 | 3.6% | `radiomics::features::first_order::RegionKernel::finish` |
| 163 | 2.2% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 150 | 2.1% | `radiomics::workflows::engine::delivery::store_column` |
| 131 | 1.8% | `radiomics::numerics::runtime::zeroed_map` |
| 99 | 1.4% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 78 | 1.1% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 31 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 29 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 23 | 0.3% | `radiomics::features::first_order_source::check_samples` |
| 13 | 0.2% | `radiomics::imaging::preprocessing::prepare` |
| 10 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 9 | 0.1% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 8 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 7 | 0.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 6 | 0.1% | `radiomics::imaging::preprocessing::gather` |
| 6 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 5 | 0.1% | `radiomics::imaging::preprocessing::gather_at` |
| 5 | 0.1% | `radiomics::numerics::count_statistics::bin_entropy` |
| 4 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 3 | 0.0% | `radiomics::workflows::engine::inputs::input_window` |
| 3 | 0.0% | `radiomics::workflows::roi::bounds::include` |
