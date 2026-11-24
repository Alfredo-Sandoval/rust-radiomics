# Profile: voxel/glrlm-r1.paired-final

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 glrlm`
- Commit: `eb3c8ce` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:21 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 2505 (idle worker-thread samples excluded: 36103)

## Command output

```text
{"median_ms":875.693083,"repeats":5,"min_ms":683.543834,"max_ms":2132.3650829999997,"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 727 | 29.0% | `__bzero` |
| 540 | 21.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 344 | 13.7% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 242 | 9.7% | `<deduplicated_symbol>` |
| 146 | 5.8% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 110 | 4.4% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 82 | 3.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 53 | 2.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 51 | 2.0% | `__psynch_mutexwait` |
| 41 | 1.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 32 | 1.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 32 | 1.3% | `_platform_memmove` |
| 25 | 1.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 15 | 0.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 9 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 7 | 0.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 5 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 5 | 0.2% | `rayon::iter::plumbing::Folder::consume_iter` |
| 5 | 0.2% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 4 | 0.2% | `_platform_memset` |
| 3 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 3 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 2 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 2 | 0.1% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 2 | 0.1% | `_platform_memcmp` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.0% | `mfm_alloc` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 1 | 0.0% | `crossbeam_epoch::internal::Global::try_advance` |
| 1 | 0.0% | `__psynch_mutexdrop` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc12___rust_alloc` |
| 1 | 0.0% | `DYLD-STUB$$memset_pattern16` |
| 1 | 0.0% | `_tlv_get_addr` |
| 1 | 0.0% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 1 | 0.0% | `_xzm_free_main` |
| 1 | 0.0% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.0% | `_pthread_cond_wait` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_malloc_main` |
| 1 | 0.0% | `_malloc_zone_malloc` |
| 1 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.0% | `pthread_mutex_unlock` |
| 1 | 0.0% | `rayon_core::sleep::Sleep::wake_specific_thread` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1974 | 78.8% | `radiomics::workflows::extraction::extract` |
| 1974 | 78.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1109 | 44.3% | `radiomics::workflows::engine::run_prepared` |
| 1107 | 44.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 865 | 34.5% | `radiomics::imaging::preprocessing::prepare` |
| 736 | 29.4% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 492 | 19.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 438 | 17.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 427 | 17.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 374 | 14.9% | `radiomics::imaging::preprocessing::gather` |
| 146 | 5.8% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 18 | 0.7% | `radiomics::imaging::io::HostArray::eq_label` |
| 14 | 0.6% | `radiomics::imaging::preprocessing::normalize_values` |
| 9 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
