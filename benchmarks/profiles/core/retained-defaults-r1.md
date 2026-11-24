# Profile: core/retained-defaults-r1

- Command: `/usr/bin/env target/bench-core/voxel-engine/after/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 defaults`
- Measured library: `df67aec`
- Checkout at sampling: `df67aec` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:50 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 10682 (idle worker-thread samples excluded: 13923)

## Command output

```text
{"median_ms":180.76404150000002,"repeats":30,"min_ms":71.979625,"max_ms":530.161792,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3123 | 29.2% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2190 | 20.5% | `radiomics::voxel::spectrum::spectral_radius` |
| 1837 | 17.2% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 725 | 6.8% | `_platform_memset` |
| 495 | 4.6% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 359 | 3.4% | `__psynch_mutexwait` |
| 275 | 2.6% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 237 | 2.2% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 169 | 1.6% | `madvise` |
| 115 | 1.1% | `radiomics::features::first_order::RegionKernel::finish` |
| 99 | 0.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 65 | 0.6% | `_platform_memmove` |
| 58 | 0.5% | `core::slice::sort::unstable::quicksort::quicksort` |
| 52 | 0.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 43 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 40 | 0.4% | `__psynch_mutexdrop` |
| 39 | 0.4% | `<deduplicated_symbol>` |
| 37 | 0.3% | `_kernelrpc_mach_vm_deallocate_trap` |
| 36 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 35 | 0.3% | `radiomics::features::first_order_source::check_samples` |
| 34 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 33 | 0.3% | `acos` |
| 32 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 29 | 0.3% | `expm1` |
| 27 | 0.3% | `_xzm_malloc` |
| 26 | 0.2% | `_xzm_free_tc` |
| 24 | 0.2% | `__xzm_xzone_free_to_freelist_chunk` |
| 24 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 23 | 0.2% | `__bzero` |
| 19 | 0.2% | `crossbeam_deque::deque::Stealer<T>::steal` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 6171 | 57.8% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2507 | 23.5% | `radiomics::voxel::spectrum::spectral_radius` |
| 2350 | 22.0% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1257 | 11.8% | `radiomics::workflows::extraction::extract` |
| 1257 | 11.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1225 | 11.5% | `radiomics::workflows::engine::run_prepared_at` |
| 1225 | 11.5% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 569 | 5.3% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 495 | 4.6% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 421 | 3.9% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 401 | 3.8% | `radiomics::features::first_order::RegionKernel::finish` |
| 200 | 1.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 170 | 1.6% | `radiomics::numerics::runtime::zeroed_map` |
| 120 | 1.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 78 | 0.7% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 62 | 0.6% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 43 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 35 | 0.3% | `radiomics::features::first_order_source::check_samples` |
| 34 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 20 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 17 | 0.2% | `radiomics::imaging::preprocessing::prepare` |
| 17 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 15 | 0.1% | `radiomics::features::first_order::RegionKernel::new` |
| 15 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 13 | 0.1% | `radiomics::workflows::engine::inputs::input_window` |
| 13 | 0.1% | `radiomics::workflows::roi::bounds::include` |
| 12 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 7 | 0.1% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 7 | 0.1% | `radiomics::numerics::count_statistics::bin_entropy` |
| 5 | 0.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
