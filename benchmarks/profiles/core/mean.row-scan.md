# Profile: core/mean.row-scan

- Command: `target/bench-core/final/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile firstorder_Mean`
- Measured library: `3ab52d3`
- Checkout at sampling: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:27 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 308 (idle worker-thread samples excluded: 15956)

## Command output

```text
{"median_ms":11.905000000000001,"runs":369,"feature_count":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 101 | 32.8% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 70 | 22.7% | `<deduplicated_symbol>` |
| 61 | 19.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 32 | 10.4% | `__psynch_mutexwait` |
| 8 | 2.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 4 | 1.3% | `radiomics::features::first_order::RegionKernel::finish` |
| 4 | 1.3% | `__psynch_cvsignal` |
| 4 | 1.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 4 | 1.3% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 3 | 1.0% | `radiomics::numerics::reduction::reported_mean` |
| 3 | 1.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 3 | 1.0% | `_platform_memmove` |
| 3 | 1.0% | `__psynch_mutexdrop` |
| 2 | 0.6% | `_tlv_get_addr` |
| 2 | 0.6% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.3% | `core::ops::function::FnOnce::call_once` |
| 1 | 0.3% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 1 | 0.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.3% | `pthread_mutex_unlock` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 856 | 277.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 728 | 236.4% | `radiomics::workflows::engine::inputs::input_window` |
| 728 | 236.4% | `radiomics::workflows::roi::bounds::include` |
| 70 | 22.7% | `radiomics::workflows::engine::run_prepared` |
| 69 | 22.4% | `radiomics::workflows::roi::compute_roi` |
| 68 | 22.1% | `radiomics::features::first_order::compute` |
| 68 | 22.1% | `radiomics::features::first_order::first_order_roi` |
| 68 | 22.1% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 61 | 19.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 50 | 16.2% | `radiomics::imaging::preprocessing::prepare` |
| 48 | 15.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 35 | 11.4% | `radiomics::imaging::preprocessing::gather` |
| 8 | 2.6% | `radiomics::imaging::io::HostArray::crop` |
| 7 | 2.3% | `radiomics::features::first_order::RegionKernel::finish` |
| 6 | 1.9% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 1.0% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.6% | `radiomics::imaging::io::HostArray::eq_label` |
| 2 | 0.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
