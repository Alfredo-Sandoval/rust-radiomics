# Profile: voxel/glcm-r2.compact

- Command: `target/bench-voxel/compact_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 glcm`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:01 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 11931 (idle worker-thread samples excluded: 22763)

## Command output

```text
{"median_ms":1853.9440419999999,"repeats":3,"min_ms":1241.738292,"max_ms":2202.611166,"roi_voxels":54639,"maps":26,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 7873 | 66.0% | `radiomics::voxel::spectrum::spectral_radius` |
| 2384 | 20.0% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 277 | 2.3% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 271 | 2.3% | `_platform_memset` |
| 201 | 1.7% | `__psynch_mutexwait` |
| 185 | 1.6% | `__bzero` |
| 139 | 1.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 121 | 1.0% | `<deduplicated_symbol>` |
| 86 | 0.7% | `core::iter::traits::iterator::Iterator::try_fold` |
| 83 | 0.7% | `_platform_memmove` |
| 40 | 0.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 37 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 35 | 0.3% | `__psynch_mutexdrop` |
| 34 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 27 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 20 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 13 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 12 | 0.1% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 12 | 0.1% | `DYLD-STUB$$memcpy` |
| 11 | 0.1% | `xzm_malloc_zone_malloc_type_calloc_main` |
| 10 | 0.1% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 9 | 0.1% | `_pthread_cond_wait` |
| 8 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 8 | 0.1% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 5 | 0.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 4 | 0.0% | `DYLD-STUB$$bzero` |
| 3 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 3 | 0.0% | `_xzm_xzone_malloc_tiny` |
| 3 | 0.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 2 | 0.0% | `log1p` |
| 2 | 0.0% | `expm1` |
| 2 | 0.0% | `_tlv_get_addr` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 1 | 0.0% | `_free` |
| 1 | 0.0% | `_xzm_free_main` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_malloc_main` |
| 1 | 0.0% | `crossbeam_epoch::internal::Global::try_advance` |
| 1 | 0.0% | `_xzm_reclaim_mark_used` |
| 1 | 0.0% | `xzm_malloc_zone_size` |
| 1 | 0.0% | `_xzm_chunk_list_pop` |
| 1 | 0.0% | `mach_absolute_time` |
| 1 | 0.0% | `crossbeam_epoch::default::with_handle` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 10618 | 89.0% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 8148 | 68.3% | `radiomics::voxel::spectrum::spectral_radius` |
| 1798 | 15.1% | `radiomics::workflows::extraction::extract` |
| 1798 | 15.1% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1350 | 11.3% | `radiomics::workflows::engine::run_prepared` |
| 1348 | 11.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 448 | 3.8% | `radiomics::imaging::preprocessing::prepare` |
| 236 | 2.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 212 | 1.8% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 194 | 1.6% | `radiomics::imaging::preprocessing::gather` |
| 191 | 1.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 30 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 12 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 8 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 2 | 0.0% | `radiomics::voxel::window_glcm::WindowGlcm::new` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
