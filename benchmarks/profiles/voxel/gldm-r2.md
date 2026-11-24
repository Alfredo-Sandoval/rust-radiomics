# Profile: voxel/gldm-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 gldm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:19 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 1623 (idle worker-thread samples excluded: 26041)

## Command output

```text
{"median_ms":732.9471669999999,"repeats":7,"min_ms":433.38824999999997,"max_ms":920.894333,"roi_voxels":54639,"maps":17,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 467 | 28.8% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 263 | 16.2% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 210 | 12.9% | `<deduplicated_symbol>` |
| 128 | 7.9% | `__bzero` |
| 100 | 6.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 89 | 5.5% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 61 | 3.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 55 | 3.4% | `core::iter::traits::iterator::Iterator::try_fold` |
| 43 | 2.6% | `__psynch_mutexwait` |
| 42 | 2.6% | `_platform_memmove` |
| 41 | 2.5% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 29 | 1.8% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 28 | 1.7% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 15 | 0.9% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 8 | 0.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 5 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 3 | 0.2% | `_platform_memset` |
| 3 | 0.2% | `_xzm_xzone_malloc_small_freelist` |
| 3 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 3 | 0.2% | `_xzm_free` |
| 2 | 0.1% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 2 | 0.1% | `mach_absolute_time` |
| 2 | 0.1% | `xzm_malloc_zone_malloc_type_realloc` |
| 2 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 2 | 0.1% | `_pthread_cond_wait` |
| 1 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 1 | 0.1% | `rayon::range::_<impl rayon::range::private::RangeInteger for usize>::opt_len` |
| 1 | 0.1% | `rayon::iter::extend::_<impl rayon::iter::ParallelExtend<T> for alloc::vec::Vec<T>>::par_extend` |
| 1 | 0.1% | `_xzm_reclaim_mark_used` |
| 1 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.1% | `core::ptr::drop_in_place<radiomics::contracts::types::PreparedInput>` |
| 1 | 0.1% | `__psynch_cvsignal` |
| 1 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 1 | 0.1% | `_RNvMNtNtNtCsaLOjE9VYtxK_3std4sync6poison7condvarNtB2_7Condvar10notify_one` |
| 1 | 0.1% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 1 | 0.1% | `_xzm_xzone_find_and_malloc_from_freelist_chunk` |
| 1 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.1% | `xzm_malloc_zone_malloc_type_calloc_main` |
| 1 | 0.1% | `_xzm_xzone_free_freelist` |
| 1 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1437 | 88.5% | `radiomics::workflows::extraction::extract` |
| 1437 | 88.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 835 | 51.4% | `radiomics::imaging::preprocessing::prepare` |
| 601 | 37.0% | `radiomics::workflows::engine::run_prepared` |
| 597 | 36.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 437 | 26.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 398 | 24.5% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 359 | 22.1% | `radiomics::imaging::preprocessing::gather` |
| 299 | 18.4% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 140 | 8.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 29 | 1.8% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 23 | 1.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 19 | 1.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 18 | 1.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.2% | `radiomics::imaging::preprocessing::resegment_samples` |
