# Profile: voxel/glcm-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 glcm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:16 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 5454 (idle worker-thread samples excluded: 35795)

## Command output

```text
{"median_ms":1354.210479,"repeats":4,"min_ms":760.115334,"max_ms":2005.485625,"roi_voxels":54639,"maps":26,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2868 | 52.6% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 785 | 14.4% | `__bzero` |
| 473 | 8.7% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 381 | 7.0% | `_platform_memset` |
| 218 | 4.0% | `<deduplicated_symbol>` |
| 113 | 2.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 76 | 1.4% | `_platform_memmove` |
| 72 | 1.3% | `__psynch_mutexwait` |
| 61 | 1.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 58 | 1.1% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 54 | 1.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 35 | 0.6% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 30 | 0.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 29 | 0.5% | `expm1` |
| 27 | 0.5% | `core::iter::traits::iterator::Iterator::try_fold` |
| 23 | 0.4% | `acos` |
| 17 | 0.3% | `DYLD-STUB$$bzero` |
| 16 | 0.3% | `_xzm_xzone_malloc_small_freelist` |
| 13 | 0.2% | `crossbeam_deque::deque::Worker<T>::pop` |
| 10 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 8 | 0.1% | `_xzm_free_main` |
| 8 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 8 | 0.1% | `???` |
| 8 | 0.1% | `DYLD-STUB$$memcpy` |
| 7 | 0.1% | `rayon::iter::plumbing::Folder::consume_iter` |
| 6 | 0.1% | `_xzm_free` |
| 4 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 4 | 0.1% | `cos` |
| 4 | 0.1% | `DYLD-STUB$$free` |
| 3 | 0.1% | `__psynch_cvsignal` |
| 3 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 3 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 3 | 0.1% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 3 | 0.1% | `DYLD-STUB$$cos` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 2 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 2 | 0.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 2 | 0.0% | `_xzm_xzone_malloc_tiny` |
| 2 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 1 | 0.0% | `xzm_segment_group_alloc_chunk` |
| 1 | 0.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &mut F>::call_mut` |
| 1 | 0.0% | `_malloc_zone_malloc` |
| 1 | 0.0% | `__psynch_mutexdrop` |
| 1 | 0.0% | `_platform_memcmp` |
| 1 | 0.0% | `rayon_core::join::join_context::_{{closure}}` |
| 1 | 0.0% | `DYLD-STUB$$expm1` |
| 1 | 0.0% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 1 | 0.0% | `_xzm_xzone_free_freelist` |
| 1 | 0.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3331 | 61.1% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2152 | 39.5% | `radiomics::workflows::extraction::extract` |
| 2152 | 39.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1331 | 24.4% | `radiomics::workflows::engine::run_prepared` |
| 1328 | 24.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 821 | 15.1% | `radiomics::imaging::preprocessing::prepare` |
| 803 | 14.7% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 423 | 7.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 398 | 7.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 332 | 6.1% | `radiomics::imaging::preprocessing::gather` |
| 45 | 0.8% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 21 | 0.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 10 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::config::settings::Settings::validate` |
