# Profile: core/voxel-admission-r1

- Command: `target/bench-core/voxel-admission/build/release/examples/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 8 1 defaults`
- Commit: `4bc9a28`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:00 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 8229 (idle worker-thread samples excluded: 10506)

## Command output

```text
{"median_ms":76.54425,"repeats":40,"min_ms":74.325166,"max_ms":226.042708,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3598 | 43.7% | `radiomics::voxel::extraction::extract_scheduled::_{{closure}}` |
| 1570 | 19.1% | `radiomics::voxel::spectrum::spectral_radius` |
| 707 | 8.6% | `__psynch_mutexwait` |
| 500 | 6.1% | `_platform_memset` |
| 369 | 4.5% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 187 | 2.3% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 136 | 1.7% | `madvise` |
| 97 | 1.2% | `__psynch_mutexdrop` |
| 78 | 0.9% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 62 | 0.8% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 60 | 0.7% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 59 | 0.7% | `crossbeam_epoch::default::with_handle` |
| 57 | 0.7% | `__psynch_cvsignal` |
| 55 | 0.7% | `core::slice::sort::unstable::quicksort::quicksort` |
| 54 | 0.7% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 42 | 0.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 33 | 0.4% | `_platform_memmove` |
| 31 | 0.4% | `expm1` |
| 29 | 0.4% | `radiomics::features::first_order_source::check_samples` |
| 26 | 0.3% | `_kernelrpc_mach_vm_deallocate_trap` |
| 24 | 0.3% | `radiomics::features::first_order::LevelCounts::add` |
| 21 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 21 | 0.3% | `acos` |
| 20 | 0.2% | `crossbeam_epoch::internal::Global::try_advance` |
| 20 | 0.2% | `_tlv_get_addr` |
| 19 | 0.2% | `__bzero` |
| 19 | 0.2% | `core::slice::sort::shared::smallsort::insertion_sort_shift_left` |
| 18 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 16 | 0.2% | `DYLD-STUB$$bzero` |
| 16 | 0.2% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 6287 | 76.4% | `radiomics::voxel::extraction::extract_scheduled::_{{closure}}` |
| 1756 | 21.3% | `radiomics::voxel::spectrum::spectral_radius` |
| 960 | 11.7% | `radiomics::workflows::extraction::extract` |
| 960 | 11.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 941 | 11.4% | `radiomics::workflows::engine::run_prepared_at` |
| 940 | 11.4% | `radiomics::voxel::extraction::extract_scheduled` |
| 369 | 4.5% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 359 | 4.4% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 303 | 3.7% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 243 | 3.0% | `radiomics::features::first_order::RegionKernel::finish` |
| 150 | 1.8% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 116 | 1.4% | `radiomics::numerics::runtime::zeroed_map` |
| 89 | 1.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 76 | 0.9% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 29 | 0.4% | `radiomics::features::first_order_source::check_samples` |
| 24 | 0.3% | `radiomics::features::first_order::LevelCounts::add` |
| 24 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 12 | 0.1% | `radiomics::imaging::preprocessing::prepare` |
| 11 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 10 | 0.1% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 10 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 7 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 5 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 5 | 0.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 5 | 0.1% | `radiomics::imaging::preprocessing::gather` |
| 5 | 0.1% | `radiomics::workflows::engine::inputs::input_window` |
| 5 | 0.1% | `radiomics::workflows::roi::bounds::include` |
| 4 | 0.0% | `radiomics::imaging::preprocessing::gather_at` |
| 3 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::true_positions` |
