# Profile: voxel/glcm-r1.paired-final

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 glcm`
- Commit: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:24 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 4485 (idle worker-thread samples excluded: 27777)

## Command output

```text
{"median_ms":899.324334,"repeats":5,"min_ms":823.1017499999999,"max_ms":1469.812875,"roi_voxels":54639,"maps":26,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2448 | 54.6% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 464 | 10.3% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 301 | 6.7% | `_platform_memset` |
| 260 | 5.8% | `__bzero` |
| 205 | 4.6% | `<deduplicated_symbol>` |
| 140 | 3.1% | `__psynch_mutexwait` |
| 103 | 2.3% | `core::iter::traits::iterator::Iterator::try_fold` |
| 90 | 2.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 71 | 1.6% | `_platform_memmove` |
| 68 | 1.5% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 63 | 1.4% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 58 | 1.3% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 42 | 0.9% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 33 | 0.7% | `acos` |
| 19 | 0.4% | `_xzm_xzone_malloc_tiny` |
| 16 | 0.4% | `DYLD-STUB$$bzero` |
| 15 | 0.3% | `expm1` |
| 13 | 0.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 11 | 0.2% | `__psynch_mutexdrop` |
| 9 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 6 | 0.1% | `_xzm_free_main` |
| 5 | 0.1% | `???` |
| 5 | 0.1% | `DYLD-STUB$$pthread_mutex_unlock` |
| 4 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 4 | 0.1% | `_malloc_zone_malloc` |
| 3 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 3 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 2 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 2 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 2 | 0.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 1 | 0.0% | `hashbrown::raw::RawTable<T,A>::insert` |
| 1 | 0.0% | `DYLD-STUB$$free` |
| 1 | 0.0% | `_xzm_reclaim_mark_used` |
| 1 | 0.0% | `num_bigint::biguint::BigUint::trailing_zeros` |
| 1 | 0.0% | `mach_absolute_time` |
| 1 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.0% | `_xzm_xzone_malloc_from_freelist_chunk` |
| 1 | 0.0% | `_pthread_cond_wait` |
| 1 | 0.0% | `DYLD-STUB$$memcpy` |
| 1 | 0.0% | `DYLD-STUB$$expm1` |
| 1 | 0.0% | `_xzm_free_outlined` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 1 | 0.0% | `DYLD-STUB$$acos` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 1 | 0.0% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 1 | 0.0% | `_platform_memcmp` |
| 1 | 0.0% | `_free` |
| 1 | 0.0% | `rayon_core::registry::WorkerThread::find_work` |
| 1 | 0.0% | `DYLD-STUB$$cos` |
| 1 | 0.0% | `_tlv_get_addr` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2829 | 63.1% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 1683 | 37.5% | `radiomics::workflows::extraction::extract` |
| 1683 | 37.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 849 | 18.9% | `radiomics::workflows::engine::run_prepared` |
| 847 | 18.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 834 | 18.6% | `radiomics::imaging::preprocessing::prepare` |
| 434 | 9.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 400 | 8.9% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 306 | 6.8% | `radiomics::imaging::preprocessing::gather` |
| 269 | 6.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 26 | 0.6% | `radiomics::imaging::io::HostArray::eq_label` |
| 21 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 16 | 0.4% | `radiomics::imaging::preprocessing::normalize_values` |
| 14 | 0.3% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::config::settings::list_features` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
