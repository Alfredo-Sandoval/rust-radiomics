# Profile: voxel/intensityhistogram-r1.paired-final

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 intensityhistogram`
- Commit: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:22 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 2567 (idle worker-thread samples excluded: 42045)

## Command output

```text
{"median_ms":772.008,"repeats":5,"min_ms":493.853792,"max_ms":3756.304209,"roi_voxels":54639,"maps":23,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1402 | 54.6% | `__bzero` |
| 351 | 13.7% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 148 | 5.8% | `core::iter::traits::iterator::Iterator::try_fold` |
| 112 | 4.4% | `<deduplicated_symbol>` |
| 72 | 2.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 68 | 2.6% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 63 | 2.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 43 | 1.7% | `_platform_memmove` |
| 42 | 1.6% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 26 | 1.0% | `mach_absolute_time` |
| 25 | 1.0% | `_xzm_xzone_malloc_tiny` |
| 23 | 0.9% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 23 | 0.9% | `radiomics::features::intensity_histogram::compute` |
| 21 | 0.8% | `__psynch_mutexwait` |
| 20 | 0.8% | `_xzm_free_main` |
| 19 | 0.7% | `_xzm_free` |
| 18 | 0.7% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 15 | 0.6% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 11 | 0.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 11 | 0.4% | `xzm_malloc_zone_malloc_type_realloc` |
| 11 | 0.4% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 5 | 0.2% | `radiomics::features::intensity_histogram::linear_percentile` |
| 5 | 0.2% | `__floattidf` |
| 4 | 0.2% | `core::cell::lazy::LazyCell<T,F>::really_init` |
| 4 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 4 | 0.2% | `__floatuntidf` |
| 3 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 3 | 0.1% | `DYLD-STUB$$free` |
| 2 | 0.1% | `_malloc_zone_malloc` |
| 2 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 1 | 0.0% | `__psynch_cvsignal` |
| 1 | 0.0% | `log2` |
| 1 | 0.0% | `_free` |
| 1 | 0.0% | `DYLD-STUB$$_platform_memmove` |
| 1 | 0.0% | `_platform_memcmp` |
| 1 | 0.0% | `_realloc` |
| 1 | 0.0% | `_malloc_zone_realloc` |
| 1 | 0.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 1 | 0.0% | `_xzm_malloc` |
| 1 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_calloc_main` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2331 | 90.8% | `radiomics::workflows::extraction::extract` |
| 2331 | 90.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1809 | 70.5% | `radiomics::workflows::engine::run_prepared` |
| 1808 | 70.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1405 | 54.7% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 522 | 20.3% | `radiomics::imaging::preprocessing::prepare` |
| 363 | 14.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 230 | 9.0% | `radiomics::features::intensity_histogram::compute` |
| 159 | 6.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 139 | 5.4% | `radiomics::imaging::preprocessing::gather` |
| 39 | 1.5% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 18 | 0.7% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 11 | 0.4% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 6 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 5 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 5 | 0.2% | `radiomics::features::intensity_histogram::linear_percentile` |
| 1 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
