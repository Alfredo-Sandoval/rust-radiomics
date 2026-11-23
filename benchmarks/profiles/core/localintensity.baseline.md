# Profile: core/localintensity.baseline

- Command: `target/bench-core/baseline/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile localintensity_LocalIntensityPeak localintensity_GlobalIntensityPeak`
- Measured library: `17d817c`
- Checkout at sampling: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:04 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3381 (idle worker-thread samples excluded: 17918)

## Command output

```text
{"median_ms":43.499207999999996,"runs":100,"feature_count":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1712 | 50.6% | `radiomics::features::local_intensity::local_intensity_statistics::_{{closure}}` |
| 585 | 17.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::slice` |
| 518 | 15.3% | `radiomics::workflows::roi::bounds::include` |
| 336 | 9.9% | `ndarray::dimension::do_slice` |
| 139 | 4.1% | `__psynch_mutexwait` |
| 31 | 0.9% | `<deduplicated_symbol>` |
| 24 | 0.7% | `radiomics::features::local_intensity::local_intensity_statistics` |
| 8 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 4 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 3 | 0.1% | `_platform_memmove` |
| 3 | 0.1% | `pthread_mutex_unlock` |
| 2 | 0.1% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 2 | 0.1% | `crossbeam_epoch::internal::Global::try_advance` |
| 2 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 2 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.0% | `core::str::_<impl str>::split_once` |
| 1 | 0.0% | `xzm_segment_group_alloc_chunk` |
| 1 | 0.0% | `radiomics::imaging::io::HostArray::crop` |
| 1 | 0.0% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 1 | 0.0% | `pthread_cond_signal` |
| 1 | 0.0% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 1 | 0.0% | `crossbeam_deque::deque::Worker<T>::pop` |
| 1 | 0.0% | `_pthread_cond_wait` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2589 | 76.6% | `radiomics::features::local_intensity::local_intensity_statistics::_{{closure}}` |
| 1121 | 33.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 558 | 16.5% | `radiomics::workflows::engine::run_prepared` |
| 557 | 16.5% | `radiomics::workflows::roi::compute_roi` |
| 557 | 16.5% | `radiomics::features::local_intensity::compute` |
| 557 | 16.5% | `radiomics::features::local_intensity::local_intensity_statistics` |
| 518 | 15.3% | `radiomics::workflows::engine::inputs::input_window` |
| 518 | 15.3% | `radiomics::workflows::roi::bounds::include` |
| 34 | 1.0% | `radiomics::imaging::preprocessing::prepare` |
| 31 | 0.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 24 | 0.7% | `radiomics::imaging::preprocessing::gather` |
| 10 | 0.3% | `radiomics::imaging::io::HostArray::crop` |
| 3 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::normalize_values` |
| 1 | 0.0% | `radiomics::config::settings::Settings::validate` |
| 1 | 0.0% | `radiomics::config::validation::unique` |
