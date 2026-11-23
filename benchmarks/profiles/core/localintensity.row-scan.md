# Profile: core/localintensity.row-scan

- Command: `target/bench-core/final/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile localintensity_LocalIntensityPeak localintensity_GlobalIntensityPeak`
- Measured library: `3ab52d3`
- Checkout at sampling: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:25 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 375 (idle worker-thread samples excluded: 13419)

## Command output

```text
{"median_ms":8.260666500000001,"runs":284,"feature_count":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 111 | 29.6% | `radiomics::features::local_intensity::local_intensity_statistics::_{{closure}}` |
| 86 | 22.9% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 73 | 19.5% | `<deduplicated_symbol>` |
| 47 | 12.5% | `radiomics::features::local_intensity::local_intensity_statistics` |
| 10 | 2.7% | `__psynch_mutexwait` |
| 9 | 2.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 9 | 2.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 5 | 1.3% | `_platform_memmove` |
| 4 | 1.1% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 4 | 1.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 2 | 0.5% | `crossbeam_epoch::default::with_handle` |
| 2 | 0.5% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 2 | 0.5% | `crossbeam_epoch::internal::Global::try_advance` |
| 2 | 0.5% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 1 | 0.3% | `_xzm_malloc` |
| 1 | 0.3% | `_<$u5b$ndarray::slice::SliceInfoElem$u5d$ as ndarray::slice::SliceArg<ndarray::dimension::dim::Dim<ndarray::dimension::dynindeximpl::IxDynImpl>>>::in_ndim` |
| 1 | 0.3% | `mach_absolute_time` |
| 1 | 0.3% | `radiomics::imaging::preprocessing::validate_exact_float64` |
| 1 | 0.3% | `__bzero` |
| 1 | 0.3% | `__xzm_xzone_free_to_freelist_chunk` |
| 1 | 0.3% | `rayon_core::sleep::Sleep::wake_any_threads` |
| 1 | 0.3% | `pthread_mutex_lock` |
| 1 | 0.3% | `rayon_core::join::join_context::_{{closure}}` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 726 | 193.6% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 347 | 92.5% | `radiomics::workflows::engine::run_prepared` |
| 346 | 92.3% | `radiomics::workflows::roi::compute_roi` |
| 345 | 92.0% | `radiomics::features::local_intensity::compute` |
| 345 | 92.0% | `radiomics::features::local_intensity::local_intensity_statistics` |
| 299 | 79.7% | `radiomics::workflows::engine::inputs::input_window` |
| 299 | 79.7% | `radiomics::workflows::roi::bounds::include` |
| 113 | 30.1% | `radiomics::features::local_intensity::local_intensity_statistics::_{{closure}}` |
| 63 | 16.8% | `radiomics::imaging::preprocessing::prepare` |
| 59 | 15.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 47 | 12.5% | `radiomics::imaging::preprocessing::gather` |
| 15 | 4.0% | `radiomics::imaging::io::HostArray::crop` |
| 5 | 1.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 4 | 1.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 4 | 1.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 1 | 0.3% | `radiomics::imaging::preprocessing::validate_exact_float64` |
