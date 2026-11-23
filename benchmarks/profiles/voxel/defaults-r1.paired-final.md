# Profile: voxel/defaults-r1.paired-final

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 defaults`
- Commit: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:26 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 3806 (idle worker-thread samples excluded: 39685)

## Command output

```text
{"median_ms":1746.341875,"repeats":5,"min_ms":1253.820583,"max_ms":2109.006625,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1389 | 36.5% | `__bzero` |
| 878 | 23.1% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 233 | 6.1% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 232 | 6.1% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 157 | 4.1% | `_platform_memset` |
| 117 | 3.1% | `<deduplicated_symbol>` |
| 117 | 3.1% | `__psynch_mutexwait` |
| 94 | 2.5% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 80 | 2.1% | `_platform_memmove` |
| 77 | 2.0% | `_kernelrpc_mach_vm_deallocate_trap` |
| 63 | 1.7% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 58 | 1.5% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 34 | 0.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 28 | 0.7% | `core::iter::traits::iterator::Iterator::try_fold` |
| 24 | 0.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 24 | 0.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 18 | 0.5% | `radiomics::features::first_order::LevelCounts::add` |
| 14 | 0.4% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 13 | 0.3% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 12 | 0.3% | `radiomics::features::first_order::RegionKernel::finish` |
| 12 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 12 | 0.3% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 11 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 10 | 0.3% | `__ulock_wake` |
| 7 | 0.2% | `core::slice::sort::unstable::quicksort::quicksort` |
| 6 | 0.2% | `expm1` |
| 6 | 0.2% | `acos` |
| 6 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 6 | 0.2% | `mach_absolute_time` |
| 6 | 0.2% | `radiomics::features::first_order::RegionKernel::new::_{{closure}}` |
| 5 | 0.1% | `_xzm_free_main` |
| 5 | 0.1% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 3 | 0.1% | `_xzm_reclaim_mark_free` |
| 3 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 3 | 0.1% | `DYLD-STUB$$bzero` |
| 3 | 0.1% | `__psynch_mutexdrop` |
| 3 | 0.1% | `core::slice::sort::unstable::ipnsort` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 2 | 0.1% | `???` |
| 2 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 2 | 0.1% | `__udivti3` |
| 2 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 2 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::slice` |
| 2 | 0.1% | `_malloc_zone_malloc` |
| 2 | 0.1% | `crossbeam_epoch::internal::Global::try_advance` |
| 2 | 0.1% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 2 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 1 | 0.0% | `_xzm_xzone_madvise_batch` |
| 1 | 0.0% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 1 | 0.0% | `_<T as core::slice::cmp::SliceContains>::slice_contains` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 1 | 0.0% | `_malloc_zone_realloc` |
| 1 | 0.0% | `_xzm_chunk_batch_list_push` |
| 1 | 0.0% | `_xzm_reclaim_mark_used` |
| 1 | 0.0% | `_realloc` |
| 1 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 1 | 0.0% | `default_zone_realloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2205 | 57.9% | `radiomics::workflows::extraction::extract` |
| 2205 | 57.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1664 | 43.7% | `radiomics::workflows::engine::run_prepared` |
| 1664 | 43.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1401 | 36.8% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 1059 | 27.8% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 541 | 14.2% | `radiomics::imaging::preprocessing::prepare` |
| 325 | 8.5% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 271 | 7.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 266 | 7.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 182 | 4.8% | `radiomics::imaging::preprocessing::gather` |
| 126 | 3.3% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 94 | 2.5% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 84 | 2.2% | `radiomics::features::first_order::RegionKernel::finish` |
| 62 | 1.6% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 42 | 1.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 39 | 1.0% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 25 | 0.7% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 18 | 0.5% | `radiomics::features::first_order::LevelCounts::add` |
| 16 | 0.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 8 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 8 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 7 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 6 | 0.2% | `radiomics::imaging::preprocessing::resegment_samples` |
| 6 | 0.2% | `radiomics::features::first_order::RegionKernel::new` |
| 6 | 0.2% | `radiomics::features::first_order::RegionKernel::new::_{{closure}}` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 1 | 0.0% | `radiomics::numerics::reduction::reported_mean` |
