# Profile: voxel/defaults-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 defaults`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:20 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 10148 (idle worker-thread samples excluded: 27491)

## Command output

```text
{"median_ms":2289.7306249999997,"repeats":3,"min_ms":2261.97425,"max_ms":4131.0271250000005,"roi_voxels":54639,"maps":93,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 6258 | 61.7% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 797 | 7.9% | `__bzero` |
| 790 | 7.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 313 | 3.1% | `_platform_memset` |
| 267 | 2.6% | `core::slice::sort::unstable::quicksort::quicksort` |
| 250 | 2.5% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 189 | 1.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 163 | 1.6% | `radiomics::features::first_order::LevelCounts::add` |
| 124 | 1.2% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 109 | 1.1% | `_platform_memmove` |
| 107 | 1.1% | `radiomics::features::first_order::RegionKernel::finish` |
| 91 | 0.9% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 81 | 0.8% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 55 | 0.5% | `_kernelrpc_mach_vm_deallocate_trap` |
| 52 | 0.5% | `<deduplicated_symbol>` |
| 52 | 0.5% | `radiomics::numerics::reduction::reported_mean` |
| 42 | 0.4% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 35 | 0.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 34 | 0.3% | `log2` |
| 32 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 30 | 0.3% | `xzm_malloc_zone_malloc_type_realloc` |
| 28 | 0.3% | `__ulock_wake` |
| 25 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 25 | 0.2% | `__psynch_mutexwait` |
| 21 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 16 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 15 | 0.1% | `expm1` |
| 14 | 0.1% | `_xzm_xzone_malloc_tiny` |
| 13 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 13 | 0.1% | `DYLD-STUB$$memcpy` |
| 11 | 0.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 9 | 0.1% | `DYLD-STUB$$bzero` |
| 9 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 8 | 0.1% | `_platform_memset_pattern16` |
| 8 | 0.1% | `radiomics::numerics::count_statistics::bin_entropy` |
| 6 | 0.1% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 5 | 0.0% | `_xzm_free_main` |
| 4 | 0.0% | `core::slice::sort::shared::pivot::median3_rec` |
| 4 | 0.0% | `radiomics::features::first_order::RegionKernel::new` |
| 4 | 0.0% | `DYLD-STUB$$log2` |
| 4 | 0.0% | `_tlv_get_addr` |
| 3 | 0.0% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 3 | 0.0% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 3 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 3 | 0.0% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 3 | 0.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 2 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 2 | 0.0% | `radiomics::native_first_order::statistics::FirstOrderFeature::from_short_name` |
| 2 | 0.0% | `_pthread_cond_wait` |
| 1 | 0.0% | `xzm_segment_group_alloc_chunk` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc14___rust_dealloc` |
| 1 | 0.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &mut F>::call_mut` |
| 1 | 0.0% | `_realloc` |
| 1 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 1 | 0.0% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 1 | 0.0% | `_xzm_free_outlined` |
| 1 | 0.0% | `mach_vm_reclaim_try_cancel` |
| 1 | 0.0% | `mach_absolute_time` |
| 1 | 0.0% | `_malloc_zone_malloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 6659 | 65.6% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 1925 | 19.0% | `radiomics::workflows::extraction::extract` |
| 1925 | 19.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1620 | 16.0% | `radiomics::workflows::engine::run_prepared` |
| 1619 | 16.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1023 | 10.1% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1008 | 9.9% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 808 | 8.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 686 | 6.8% | `radiomics::features::first_order::RegionKernel::finish` |
| 460 | 4.5% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 309 | 3.0% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 305 | 3.0% | `radiomics::imaging::preprocessing::prepare` |
| 258 | 2.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 250 | 2.5% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 166 | 1.6% | `radiomics::features::first_order::LevelCounts::add` |
| 162 | 1.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 140 | 1.4% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 93 | 0.9% | `radiomics::imaging::preprocessing::gather` |
| 67 | 0.7% | `radiomics::features::first_order::LevelCounts::finish` |
| 52 | 0.5% | `radiomics::numerics::reduction::reported_mean` |
| 45 | 0.4% | `radiomics::numerics::count_statistics::bin_entropy` |
| 26 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 22 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 12 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 12 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 12 | 0.1% | `radiomics::features::first_order::RegionKernel::new` |
| 10 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 3 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.0% | `radiomics::native_first_order::statistics::FirstOrderFeature::from_short_name` |
| 1 | 0.0% | `radiomics::voxel::window_glcm::WindowGlcm::new` |
| 1 | 0.0% | `radiomics::voxel::window::directional_pairs` |
