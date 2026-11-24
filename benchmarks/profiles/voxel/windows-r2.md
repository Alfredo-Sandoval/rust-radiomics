# Profile: voxel/windows-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 windows`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:17 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 20951 (idle worker-thread samples excluded: 14845)

## Command output

```text
{"median_ms":1309.524021,"repeats":4,"min_ms":881.903167,"max_ms":2042.666916,"roi_voxels":54639,"maps":96,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 15826 | 75.5% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2287 | 10.9% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 862 | 4.1% | `_platform_memset` |
| 475 | 2.3% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 358 | 1.7% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 220 | 1.1% | `__psynch_mutexwait` |
| 188 | 0.9% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 183 | 0.9% | `_platform_memmove` |
| 154 | 0.7% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 46 | 0.2% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 45 | 0.2% | `DYLD-STUB$$bzero` |
| 42 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 41 | 0.2% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 29 | 0.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 26 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 16 | 0.1% | `__ulock_wake` |
| 16 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 15 | 0.1% | `<deduplicated_symbol>` |
| 14 | 0.1% | `DYLD-STUB$$memcpy` |
| 13 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 11 | 0.1% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 10 | 0.0% | `__bzero` |
| 9 | 0.0% | `__psynch_mutexdrop` |
| 8 | 0.0% | `mach_absolute_time` |
| 7 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 7 | 0.0% | `xzm_malloc_zone_try_free_default_main` |
| 4 | 0.0% | `_xzm_xzone_malloc_from_freelist_chunk` |
| 3 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 3 | 0.0% | `malloc` |
| 3 | 0.0% | `expm1` |
| 3 | 0.0% | `acos` |
| 3 | 0.0% | `__psynch_cvsignal` |
| 2 | 0.0% | `_free` |
| 2 | 0.0% | `_xzm_free_main` |
| 2 | 0.0% | `_xzm_chunk_list_push` |
| 1 | 0.0% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 1 | 0.0% | `_xzm_segment_group_span_mark_smaller` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 1 | 0.0% | `mach_vm_reclaim_try_enter` |
| 1 | 0.0% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.0% | `_xzm_xzone_free_freelist` |
| 1 | 0.0% | `_tlv_get_addr` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 1 | 0.0% | `mach_vm_reclaim_try_cancel` |
| 1 | 0.0% | `crossbeam_deque::deque::Worker<T>::pop` |
| 1 | 0.0% | `_malloc_zone_malloc` |
| 1 | 0.0% | `crossbeam_epoch::internal::Global::try_advance` |
| 1 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 1 | 0.0% | `_platform_memset_pattern16` |
| 1 | 0.0% | `pthread_mutex_unlock` |
| 1 | 0.0% | `rayon::iter::plumbing::Folder::consume_iter` |
| 1 | 0.0% | `rayon_core::join::join_context::_{{closure}}` |
| 1 | 0.0% | `rayon_core::registry::WorkerThread::find_work` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 16867 | 80.5% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2650 | 12.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1884 | 9.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 358 | 1.7% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 200 | 1.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `radiomics::config::settings::list_features` |
| 1 | 0.0% | `radiomics::voxel::window_texture::WindowTexture::new` |
