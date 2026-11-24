# Profile: voxel/defaults-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 defaults`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:18 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2811 (idle worker-thread samples excluded: 44974)

## Command output

```text
{"median_ms":4188.438167,"repeats":3,"min_ms":1878.3677500000001,"max_ms":5135.451166,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2433 | 86.6% | `__bzero` |
| 176 | 6.3% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 78 | 2.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 37 | 1.3% | `_platform_memset` |
| 21 | 0.7% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 18 | 0.6% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 15 | 0.5% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 10 | 0.4% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 9 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 4 | 0.1% | `radiomics::features::first_order::LevelCounts::add` |
| 3 | 0.1% | `_RNvXs4_NtCs8dnTdrJsiec_5alloc6stringNtB5_6StringNtNtCs6sq8b9ugfBC_4core5clone5Clone5clone` |
| 2 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 1 | 0.0% | `_platform_memmove` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.0% | `_xzm_free` |
| 1 | 0.0% | `radiomics::features::first_order::LevelCounts::finish` |
| 1 | 0.0% | `acos` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2515 | 89.5% | `radiomics::workflows::extraction::extract` |
| 2515 | 89.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2515 | 89.5% | `radiomics::workflows::engine::run_prepared` |
| 2515 | 89.5% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 2459 | 87.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 214 | 7.6% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 96 | 3.4% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 32 | 1.1% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 25 | 0.9% | `radiomics::features::first_order::RegionKernel::finish` |
| 18 | 0.6% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 15 | 0.5% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 9 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 7 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 5 | 0.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 5 | 0.2% | `radiomics::features::first_order::LevelCounts::add` |
| 1 | 0.0% | `radiomics::features::first_order::LevelCounts::finish` |
