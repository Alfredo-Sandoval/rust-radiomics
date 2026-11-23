# Profile: voxel/glcm-r1.compact

- Command: `target/bench-voxel/compact_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 glcm`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:00 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 8228 (idle worker-thread samples excluded: 35245)

## Command output

```text
{"median_ms":544.263334,"repeats":9,"min_ms":399.490167,"max_ms":1009.486041,"roi_voxels":54639,"maps":26,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3765 | 45.8% | `radiomics::voxel::spectrum::spectral_radius` |
| 1711 | 20.8% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 652 | 7.9% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 414 | 5.0% | `_platform_memset` |
| 362 | 4.4% | `__bzero` |
| 287 | 3.5% | `<deduplicated_symbol>` |
| 264 | 3.2% | `__psynch_mutexwait` |
| 105 | 1.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 104 | 1.3% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 98 | 1.2% | `_platform_memmove` |
| 92 | 1.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 75 | 0.9% | `core::iter::traits::iterator::Iterator::try_fold` |
| 61 | 0.7% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 37 | 0.4% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 24 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 18 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 16 | 0.2% | `expm1` |
| 14 | 0.2% | `DYLD-STUB$$bzero` |
| 13 | 0.2% | `_xzm_free_main` |
| 13 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 11 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 9 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 9 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 7 | 0.1% | `_pthread_cond_wait` |
| 7 | 0.1% | `__psynch_cvsignal` |
| 6 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 5 | 0.1% | `__psynch_mutexdrop` |
| 4 | 0.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 3 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 3 | 0.0% | `DYLD-STUB$$memcpy` |
| 3 | 0.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 3 | 0.0% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 3 | 0.0% | `DYLD-STUB$$expm1` |
| 2 | 0.0% | `rayon::iter::from_par_iter::collect_extended` |
| 2 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 2 | 0.0% | `rayon_core::sleep::Sleep::wake_any_threads` |
| 2 | 0.0% | `_RNvCsGIExRX8pES_7___rustc18___rdl_alloc_zeroed` |
| 2 | 0.0% | `rayon_core::join::join_context::_{{closure}}` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 1 | 0.0% | `_<num_bigint::biguint::BigUint as num_integer::Integer>::gcd` |
| 1 | 0.0% | `_xzm_reclaim_mark_used` |
| 1 | 0.0% | `_free` |
| 1 | 0.0% | `ndarray::dimension::do_slice` |
| 1 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 1 | 0.0% | `_xzm_xzone_free_freelist` |
| 1 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 1 | 0.0% | `rayon_core::sleep::Sleep::wake_specific_thread` |
| 1 | 0.0% | `_xzm_free` |
| 1 | 0.0% | `xzm_malloc_zone_size` |
| 1 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 1 | 0.0% | `mach_absolute_time` |
| 1 | 0.0% | `DYLD-STUB$$memset_pattern16` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.0% | `rayon_core::job::StackJob<L,F,R>::run_inline` |
| 1 | 0.0% | `DYLD-STUB$$_platform_memmove` |
| 1 | 0.0% | `DYLD-STUB$$memcmp` |
| 1 | 0.0% | `_platform_memcmp` |
| 1 | 0.0% | `_tlv_get_addr` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 5965 | 72.5% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 4069 | 49.5% | `radiomics::voxel::spectrum::spectral_radius` |
| 2257 | 27.4% | `radiomics::workflows::extraction::extract` |
| 2257 | 27.4% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1132 | 13.8% | `radiomics::imaging::preprocessing::prepare` |
| 1125 | 13.7% | `radiomics::workflows::engine::run_prepared` |
| 1124 | 13.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 588 | 7.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 544 | 6.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 478 | 5.8% | `radiomics::imaging::preprocessing::gather` |
| 378 | 4.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 31 | 0.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 30 | 0.4% | `radiomics::imaging::preprocessing::normalize_values` |
| 20 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::config::settings::list_features` |
