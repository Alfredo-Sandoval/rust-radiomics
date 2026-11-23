# Profile: core/retained-defaults-r1.baseline

- Command: `/usr/bin/env target/bench-core/voxel-engine/before/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 defaults`
- Measured library: `aa6355a`
- Checkout at sampling: `aa6355a` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:46 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 4388 (idle worker-thread samples excluded: 43701)

## Command output

```text
{"median_ms":920.053958,"repeats":5,"min_ms":746.800625,"max_ms":1981.537667,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1476 | 33.6% | `__bzero` |
| 773 | 17.6% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 398 | 9.1% | `radiomics::voxel::spectrum::spectral_radius` |
| 378 | 8.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 267 | 6.1% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 139 | 3.2% | `<deduplicated_symbol>` |
| 117 | 2.7% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 112 | 2.6% | `_platform_memset` |
| 97 | 2.2% | `_platform_memmove` |
| 87 | 2.0% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 72 | 1.6% | `_kernelrpc_mach_vm_deallocate_trap` |
| 58 | 1.3% | `__psynch_mutexwait` |
| 44 | 1.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 39 | 0.9% | `core::iter::traits::iterator::Iterator::try_fold` |
| 36 | 0.8% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 32 | 0.7% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 28 | 0.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 23 | 0.5% | `cos` |
| 20 | 0.5% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 18 | 0.4% | `core::slice::sort::unstable::quicksort::quicksort` |
| 14 | 0.3% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 12 | 0.3% | `radiomics::features::first_order::RegionKernel::finish` |
| 12 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 10 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 10 | 0.2% | `radiomics::features::first_order::LevelCounts::add` |
| 10 | 0.2% | `_malloc_zone_malloc` |
| 9 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 8 | 0.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 8 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 7 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2451 | 55.9% | `radiomics::workflows::extraction::extract` |
| 2451 | 55.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1828 | 41.7% | `radiomics::workflows::engine::run_prepared` |
| 1827 | 41.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1507 | 34.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 1326 | 30.2% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 623 | 14.2% | `radiomics::imaging::preprocessing::prepare` |
| 467 | 10.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 445 | 10.1% | `radiomics::voxel::spectrum::spectral_radius` |
| 359 | 8.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 258 | 5.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 192 | 4.4% | `radiomics::imaging::preprocessing::gather` |
| 106 | 2.4% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 87 | 2.0% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 65 | 1.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 44 | 1.0% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 32 | 0.7% | `radiomics::imaging::preprocessing::normalize_values` |
| 25 | 0.6% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 14 | 0.3% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 13 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 12 | 0.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 11 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 10 | 0.2% | `radiomics::features::first_order::LevelCounts::add` |
| 9 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 9 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 4 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.0% | `radiomics::features::first_order::RegionKernel::new` |
| 1 | 0.0% | `radiomics::voxel::window_glcm::WindowGlcm::new` |
| 1 | 0.0% | `radiomics::voxel::window::directional_pairs` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
