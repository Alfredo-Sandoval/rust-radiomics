# Profile: voxel/gldzm-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 gldzm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:17 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 1870 (idle worker-thread samples excluded: 32399)

## Command output

```text
{"median_ms":552.8305829999999,"repeats":7,"min_ms":458.866583,"max_ms":1388.305125,"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 569 | 30.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 405 | 21.7% | `__bzero` |
| 226 | 12.1% | `<deduplicated_symbol>` |
| 131 | 7.0% | `_platform_memmove` |
| 112 | 6.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 70 | 3.7% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 63 | 3.4% | `core::iter::traits::iterator::Iterator::try_fold` |
| 62 | 3.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 53 | 2.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 40 | 2.1% | `__psynch_mutexwait` |
| 31 | 1.7% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 30 | 1.6% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 21 | 1.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 6 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 6 | 0.3% | `_platform_memset` |
| 5 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 4 | 0.2% | `_kernelrpc_mach_vm_map_trap` |
| 4 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 4 | 0.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 3 | 0.2% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.1% | `_malloc_zone_realloc` |
| 2 | 0.1% | `_xzm_xzone_malloc_small_freelist` |
| 2 | 0.1% | `__psynch_cvsignal` |
| 2 | 0.1% | `__psynch_mutexdrop` |
| 2 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 1 | 0.1% | `_xzm_segment_group_span_mark_smaller` |
| 1 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.1% | `rayon_core::registry::Registry::inject` |
| 1 | 0.1% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 1 | 0.1% | `_xzm_free_main` |
| 1 | 0.1% | `DYLD-STUB$$bzero` |
| 1 | 0.1% | `mach_absolute_time` |
| 1 | 0.1% | `pthread_cond_broadcast` |
| 1 | 0.1% | `rayon_core::registry::WorkerThread::find_work` |
| 1 | 0.1% | `_platform_memcmp` |
| 1 | 0.1% | `_realloc` |
| 1 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 1 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.1% | `xzm_malloc_zone_size` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1910 | 102.1% | `radiomics::workflows::extraction::extract` |
| 1910 | 102.1% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1078 | 57.6% | `radiomics::imaging::preprocessing::prepare` |
| 832 | 44.5% | `radiomics::workflows::engine::run_prepared` |
| 828 | 44.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 542 | 29.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 536 | 28.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 433 | 23.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 403 | 21.6% | `radiomics::imaging::preprocessing::gather` |
| 64 | 3.4% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 42 | 2.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 28 | 1.5% | `radiomics::imaging::io::HostArray::eq_label` |
| 7 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 4 | 0.2% | `radiomics::imaging::preprocessing::resegment_samples` |
| 4 | 0.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 3 | 0.2% | `radiomics::imaging::preprocessing::lattice_bins` |
