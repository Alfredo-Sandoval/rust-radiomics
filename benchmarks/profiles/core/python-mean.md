# Profile: core/python-mean

- Command: `../mlx-radiomics/.venv/bin/python benchmarks/python_roi.py ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd --seconds 6 --layout native --features firstorder_Mean`
- Measured library: `2cfa716`
- Checkout at sampling: `2cfa716`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:46 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 3940 (idle worker-thread samples excluded: 8581)

## Command output

```text
{"layout": "native", "shape": [134, 512, 512], "image_dtype": "int32", "mask_dtype": "uint8", "features": 1, "roi_voxels": 54639, "runs": 934, "median_ms": 3.9950829996087123}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2039 | 51.8% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 995 | 25.3% | `__psynch_mutexwait` |
| 375 | 9.5% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 62 | 1.6% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 52 | 1.3% | `__psynch_mutexdrop` |
| 50 | 1.3% | `__psynch_cvsignal` |
| 37 | 0.9% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 35 | 0.9% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 32 | 0.8% | `<deduplicated_symbol>` |
| 28 | 0.7% | `crossbeam_epoch::default::with_handle` |
| 25 | 0.6% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 15 | 0.4% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 14 | 0.4% | `rayon_core::join::join_context::_{{closure}}` |
| 12 | 0.3% | `_platform_memmove` |
| 12 | 0.3% | `_tlv_get_addr` |
| 12 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 10 | 0.3% | `radiomics::features::first_order_source::check_samples` |
| 10 | 0.3% | `crossbeam_epoch::internal::Global::try_advance` |
| 9 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 8 | 0.2% | `rayon_core::sleep::Sleep::wake_specific_thread` |
| 8 | 0.2% | `crossbeam_deque::deque::Worker<T>::pop` |
| 7 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 7 | 0.2% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 5 | 0.1% | `pthread_mutex_lock` |
| 5 | 0.1% | `rayon_core::sleep::Sleep::sleep` |
| 4 | 0.1% | `_PyEval_EvalFrameDefault` |
| 4 | 0.1% | `pthread_mutex_unlock` |
| 4 | 0.1% | `crossbeam_epoch::internal::Global::collect` |
| 3 | 0.1% | `_PyDict_GetItemRef_KnownHash` |
| 3 | 0.1% | `_platform_memcmp` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 300 | 7.6% | `radiomics::workflows::extraction::extract` |
| 299 | 7.6% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 105 | 2.7% | `radiomics::workflows::engine::inputs::input_window` |
| 104 | 2.6% | `radiomics::workflows::roi::bounds::include` |
| 103 | 2.6% | `radiomics::imaging::preprocessing::prepare` |
| 97 | 2.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 85 | 2.2% | `radiomics::imaging::preprocessing::gather` |
| 85 | 2.2% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 69 | 1.8% | `radiomics::imaging::preprocessing::gather_at` |
| 60 | 1.5% | `radiomics::workflows::engine::run_prepared_at` |
| 57 | 1.4% | `radiomics::workflows::roi::compute_roi` |
| 56 | 1.4% | `radiomics::features::first_order::compute` |
| 56 | 1.4% | `radiomics::features::first_order::first_order_roi` |
| 55 | 1.4% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 35 | 0.9% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 30 | 0.8% | `radiomics::imaging::io::HostArray::crop` |
| 20 | 0.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 16 | 0.4% | `radiomics::imaging::preprocessing::true_positions` |
| 10 | 0.3% | `radiomics::features::first_order_source::check_samples` |
| 9 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 8 | 0.2% | `radiomics::contracts::types::_::_<impl serde_core::ser::Serialize for radiomics::contracts::types::ExecutionMetadata>::serialize` |
| 6 | 0.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 6 | 0.2% | `radiomics::contracts::json::serialize_safe` |
| 4 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 2 | 0.1% | `radiomics::config::settings::Settings::validate` |
| 2 | 0.1% | `radiomics::config::settings::_::_<impl serde_core::ser::Serialize for radiomics::config::settings::Settings>::serialize` |
| 1 | 0.0% | `radiomics::features::first_order::RegionKernel::new` |
| 1 | 0.0% | `radiomics::features::first_order::SampleStatistics::new` |
| 1 | 0.0% | `radiomics::native_first_order::statistics::Calculation::new` |
