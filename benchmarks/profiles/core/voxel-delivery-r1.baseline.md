# Profile: core/voxel-delivery-r1.baseline

- Command: `target/bench-core/retained-delivery/baseline ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 8 1 defaults`
- Measured library: `bc7a192`
- Checkout at sampling: `2178bcb`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:22 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 6920 (idle worker-thread samples excluded: 9991)

## Command output

```text
{"median_ms":95.00285400000001,"repeats":34,"min_ms":79.65404099999999,"max_ms":261.555208,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2913 | 42.1% | `radiomics::voxel::extraction::extract_scheduled::_{{closure}}` |
| 1304 | 18.8% | `radiomics::voxel::spectrum::spectral_radius` |
| 545 | 7.9% | `__psynch_mutexwait` |
| 434 | 6.3% | `_platform_memset` |
| 285 | 4.1% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 172 | 2.5% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 139 | 2.0% | `madvise` |
| 70 | 1.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 63 | 0.9% | `core::slice::sort::unstable::quicksort::quicksort` |
| 59 | 0.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 51 | 0.7% | `__psynch_mutexdrop` |
| 50 | 0.7% | `__psynch_cvsignal` |
| 50 | 0.7% | `radiomics::features::first_order::RegionKernel::finish` |
| 43 | 0.6% | `radiomics::features::first_order::LevelCounts::add` |
| 43 | 0.6% | `crossbeam_epoch::default::with_handle` |
| 40 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 38 | 0.5% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 36 | 0.5% | `_platform_memmove` |
| 36 | 0.5% | `acos` |
| 34 | 0.5% | `radiomics::features::first_order_source::check_samples` |
| 27 | 0.4% | `_kernelrpc_mach_vm_deallocate_trap` |
| 25 | 0.4% | `expm1` |
| 25 | 0.4% | `core::slice::sort::shared::smallsort::insertion_sort_shift_left` |
| 24 | 0.3% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 23 | 0.3% | `_tlv_get_addr` |
| 19 | 0.3% | `__bzero` |
| 17 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 17 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 16 | 0.2% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 16 | 0.2% | `crossbeam_epoch::internal::Global::try_advance` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 5163 | 74.6% | `radiomics::voxel::extraction::extract_scheduled::_{{closure}}` |
| 1468 | 21.2% | `radiomics::voxel::spectrum::spectral_radius` |
| 861 | 12.4% | `radiomics::workflows::extraction::extract` |
| 861 | 12.4% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 842 | 12.2% | `radiomics::workflows::engine::run_prepared_at` |
| 841 | 12.2% | `radiomics::voxel::extraction::extract_scheduled` |
| 405 | 5.9% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 300 | 4.3% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 285 | 4.1% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 266 | 3.8% | `radiomics::features::first_order::RegionKernel::finish` |
| 153 | 2.2% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 126 | 1.8% | `radiomics::numerics::runtime::zeroed_map` |
| 100 | 1.4% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 80 | 1.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 43 | 0.6% | `radiomics::features::first_order::LevelCounts::add` |
| 34 | 0.5% | `radiomics::features::first_order_source::check_samples` |
| 22 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 16 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 12 | 0.2% | `radiomics::imaging::preprocessing::prepare` |
| 11 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 9 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 8 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 6 | 0.1% | `radiomics::numerics::count_statistics::bin_entropy` |
| 5 | 0.1% | `radiomics::workflows::engine::inputs::input_window` |
| 5 | 0.1% | `radiomics::workflows::roi::bounds::include` |
| 3 | 0.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 3 | 0.0% | `radiomics::imaging::preprocessing::gather` |
| 3 | 0.0% | `radiomics::imaging::preprocessing::gather_at` |
| 2 | 0.0% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::fraction_of_f64` |
