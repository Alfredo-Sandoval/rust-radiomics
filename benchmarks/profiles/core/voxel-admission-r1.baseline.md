# Profile: core/voxel-admission-r1.baseline

- Command: `target/bench-core/voxel-admission/baseline-build/release/examples/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 8 1 defaults`
- Commit: `faa6ef9`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:00 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 11326 (idle worker-thread samples excluded: 12576)

## Command output

```text
{"median_ms":71.157021,"repeats":50,"min_ms":61.705000000000005,"max_ms":152.24320799999998,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 5444 | 48.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}` |
| 2280 | 20.1% | `radiomics::voxel::spectrum::spectral_radius` |
| 685 | 6.0% | `_platform_memset` |
| 536 | 4.7% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 508 | 4.5% | `__psynch_mutexwait` |
| 265 | 2.3% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 175 | 1.5% | `madvise` |
| 115 | 1.0% | `radiomics::features::first_order::RegionKernel::finish` |
| 96 | 0.8% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 94 | 0.8% | `core::slice::sort::unstable::quicksort::quicksort` |
| 71 | 0.6% | `_platform_memmove` |
| 63 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 49 | 0.4% | `acos` |
| 46 | 0.4% | `expm1` |
| 45 | 0.4% | `_xzm_xzone_malloc_tiny` |
| 42 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 41 | 0.4% | `_kernelrpc_mach_vm_deallocate_trap` |
| 39 | 0.3% | `__psynch_mutexdrop` |
| 35 | 0.3% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 33 | 0.3% | `_xzm_free_tc` |
| 32 | 0.3% | `__bzero` |
| 32 | 0.3% | `__psynch_cvsignal` |
| 28 | 0.2% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 25 | 0.2% | `mach_absolute_time` |
| 21 | 0.2% | `DYLD-STUB$$bzero` |
| 20 | 0.2% | `core::slice::sort::shared::smallsort::insertion_sort_shift_left` |
| 20 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 20 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 20 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 19 | 0.2% | `radiomics::numerics::reduction::reported_mean` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 9979 | 88.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}` |
| 2562 | 22.6% | `radiomics::voxel::spectrum::spectral_radius` |
| 1215 | 10.7% | `radiomics::workflows::extraction::extract` |
| 1215 | 10.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1186 | 10.5% | `radiomics::workflows::engine::run_prepared_at` |
| 1183 | 10.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 571 | 5.0% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 536 | 4.7% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 453 | 4.0% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 381 | 3.4% | `radiomics::features::first_order::RegionKernel::finish` |
| 222 | 2.0% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 172 | 1.5% | `radiomics::numerics::runtime::zeroed_map` |
| 129 | 1.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 104 | 0.9% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 42 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 37 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 34 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 26 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 22 | 0.2% | `radiomics::features::first_order::RegionKernel::new` |
| 21 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 19 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 18 | 0.2% | `radiomics::imaging::preprocessing::prepare` |
| 11 | 0.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 10 | 0.1% | `radiomics::imaging::preprocessing::gather` |
| 10 | 0.1% | `radiomics::numerics::count_statistics::bin_entropy` |
| 9 | 0.1% | `radiomics::workflows::engine::inputs::input_window` |
| 9 | 0.1% | `radiomics::workflows::roi::bounds::include` |
| 8 | 0.1% | `radiomics::imaging::preprocessing::gather_at` |
| 7 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 4 | 0.0% | `radiomics::imaging::preprocessing::true_positions` |
