# Profile: voxel/glcm-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 glcm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:18 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 10610 (idle worker-thread samples excluded: 28606)

## Command output

```text
{"median_ms":1751.75775,"repeats":3,"min_ms":807.064,"max_ms":3927.727875,"roi_voxels":54639,"maps":26,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 8627 | 81.3% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 536 | 5.1% | `__bzero` |
| 414 | 3.9% | `_platform_memset` |
| 254 | 2.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 169 | 1.6% | `<deduplicated_symbol>` |
| 160 | 1.5% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 100 | 0.9% | `_platform_memmove` |
| 80 | 0.8% | `core::iter::traits::iterator::Iterator::try_fold` |
| 53 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 44 | 0.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 35 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 21 | 0.2% | `DYLD-STUB$$memcpy` |
| 18 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 18 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 14 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 11 | 0.1% | `DYLD-STUB$$bzero` |
| 8 | 0.1% | `expm1` |
| 7 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 7 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 6 | 0.1% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 5 | 0.0% | `_xzm_xzone_malloc_tiny` |
| 4 | 0.0% | `_platform_memcmp` |
| 2 | 0.0% | `_xzm_free_main` |
| 2 | 0.0% | `???` |
| 2 | 0.0% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.0% | `log1p` |
| 1 | 0.0% | `_xzm_xzone_free_freelist` |
| 1 | 0.0% | `DYLD-STUB$$expm1` |
| 1 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 1 | 0.0% | `acos` |
| 1 | 0.0% | `_xzm_free_outlined` |
| 1 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 1 | 0.0% | `_realloc` |
| 1 | 0.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 1 | 0.0% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 1 | 0.0% | `mach_absolute_time` |
| 1 | 0.0% | `rayon_core::join::join_context::_{{closure}}` |
| 1 | 0.0% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 9177 | 86.5% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2015 | 19.0% | `radiomics::workflows::extraction::extract` |
| 2015 | 19.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1536 | 14.5% | `radiomics::workflows::engine::run_prepared` |
| 1535 | 14.5% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 523 | 4.9% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 479 | 4.5% | `radiomics::imaging::preprocessing::prepare` |
| 271 | 2.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 221 | 2.1% | `radiomics::imaging::preprocessing::gather` |
| 208 | 2.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 55 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 25 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 10 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 1 | 0.0% | `radiomics::voxel::window_glcm::WindowGlcm::new` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
