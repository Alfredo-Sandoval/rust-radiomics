# Profile: voxel/windows-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 windows`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:16 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 15175 (idle worker-thread samples excluded: 14628)

## Command output

```text
{"median_ms":608.8314164999999,"repeats":8,"min_ms":242.380458,"max_ms":1200.564958,"roi_voxels":54639,"maps":96,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 8851 | 58.3% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2735 | 18.0% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1123 | 7.4% | `_platform_memset` |
| 741 | 4.9% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 566 | 3.7% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 118 | 0.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 98 | 0.6% | `_platform_memmove` |
| 97 | 0.6% | `core::iter::traits::iterator::Iterator::try_fold` |
| 86 | 0.6% | `expm1` |
| 83 | 0.5% | `_xzm_xzone_malloc_tiny` |
| 51 | 0.3% | `acos` |
| 49 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 41 | 0.3% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 41 | 0.3% | `_xzm_free_main` |
| 40 | 0.3% | `__bzero` |
| 36 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 34 | 0.2% | `__psynch_mutexdrop` |
| 25 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 24 | 0.2% | `DYLD-STUB$$bzero` |
| 23 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 22 | 0.1% | `__psynch_mutexwait` |
| 22 | 0.1% | `DYLD-STUB$$memcpy` |
| 22 | 0.1% | `__ulock_wake` |
| 21 | 0.1% | `crossbeam_epoch::internal::Global::try_advance` |
| 21 | 0.1% | `rayon_core::join::join_context::_{{closure}}` |
| 15 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 13 | 0.1% | `DYLD-STUB$$memcmp` |
| 13 | 0.1% | `DYLD-STUB$$expm1` |
| 12 | 0.1% | `_xzm_malloc` |
| 11 | 0.1% | `mach_absolute_time` |
| 10 | 0.1% | `???` |
| 10 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 9 | 0.1% | `_xzm_xzone_malloc_small_freelist` |
| 9 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 7 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 7 | 0.0% | `_platform_memset_pattern16` |
| 7 | 0.0% | `_xzm_segment_group_find_and_allocate_chunk` |
| 6 | 0.0% | `_xzm_xzone_free_freelist` |
| 5 | 0.0% | `_free` |
| 5 | 0.0% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 4 | 0.0% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 4 | 0.0% | `_xzm_chunk_list_slot_push` |
| 4 | 0.0% | `xzm_malloc_zone_size` |
| 4 | 0.0% | `cos` |
| 4 | 0.0% | `ndarray::dimension::do_slice` |
| 3 | 0.0% | `_xzm_xzone_madvise_batch` |
| 3 | 0.0% | `crossbeam_deque::deque::Worker<T>::pop` |
| 3 | 0.0% | `_platform_memcmp` |
| 3 | 0.0% | `rayon_core::job::StackJob<L,F,R>::run_inline` |
| 3 | 0.0% | `_malloc_ulock_self_owner_value` |
| 2 | 0.0% | `__psynch_cvsignal` |
| 2 | 0.0% | `_RNvCsGIExRX8pES_7___rustc14___rust_dealloc` |
| 2 | 0.0% | `rayon_core::registry::WorkerThread::find_work` |
| 2 | 0.0% | `_xzm_chunk_list_pop` |
| 2 | 0.0% | `_xzm_free_outlined` |
| 2 | 0.0% | `default_zone_realloc` |
| 2 | 0.0% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 2 | 0.0% | `_xzm_free` |
| 1 | 0.0% | `malloc` |
| 1 | 0.0% | `DYLD-STUB$$free` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 10278 | 67.7% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 3489 | 23.0% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1624 | 10.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 741 | 4.9% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 74 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 45 | 0.3% | `radiomics::imaging::preprocessing::prepare` |
| 45 | 0.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
