# Profile: core/python-mean.baseline

- Command: `../mlx-radiomics/.venv/bin/python benchmarks/python_roi.py ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd --seconds 6 --layout native --features firstorder_Mean`
- Measured library: `c23929c`
- Checkout at sampling: `2cfa716`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:46 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 1457 (idle worker-thread samples excluded: 16498)

## Command output

```text
{"layout": "native", "shape": [134, 512, 512], "image_dtype": "int32", "mask_dtype": "uint8", "features": 1, "roi_voxels": 54639, "runs": 664, "median_ms": 6.114291500125546}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 614 | 42.1% | `_platform_memmove` |
| 243 | 16.7% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 219 | 15.0% | `__psynch_mutexwait` |
| 38 | 2.6% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 38 | 2.6% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 31 | 2.1% | `<deduplicated_symbol>` |
| 25 | 1.7% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 22 | 1.5% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 21 | 1.4% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 20 | 1.4% | `__psynch_cvsignal` |
| 19 | 1.3% | `rayon_core::join::join_context::_{{closure}}` |
| 16 | 1.1% | `__psynch_mutexdrop` |
| 14 | 1.0% | `_pthread_cond_wait` |
| 11 | 0.8% | `_tlv_get_addr` |
| 10 | 0.7% | `crossbeam_epoch::default::with_handle` |
| 9 | 0.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 9 | 0.6% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 8 | 0.5% | `radiomics::features::first_order_source::check_samples` |
| 7 | 0.5% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 7 | 0.5% | `crossbeam_epoch::internal::Global::try_advance` |
| 6 | 0.4% | `_PyEval_EvalFrameDefault` |
| 4 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 4 | 0.3% | `crossbeam_deque::deque::Worker<T>::pop` |
| 3 | 0.2% | `_xzm_free_tc` |
| 3 | 0.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 3 | 0.2% | `_PyDict_GetItemRef_KnownHash` |
| 3 | 0.2% | `rayon_core::job::StackJob<L,F,R>::run_inline` |
| 2 | 0.1% | `_PyObject_Free` |
| 2 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.1% | `core::ops::function::FnOnce::call_once` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 298 | 20.5% | `radiomics::workflows::extraction::extract` |
| 296 | 20.3% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 118 | 8.1% | `radiomics::imaging::preprocessing::prepare` |
| 112 | 7.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 110 | 7.5% | `radiomics::workflows::engine::inputs::input_window` |
| 110 | 7.5% | `radiomics::workflows::roi::bounds::include` |
| 96 | 6.6% | `radiomics::imaging::preprocessing::gather` |
| 79 | 5.4% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 74 | 5.1% | `radiomics::imaging::preprocessing::gather_at` |
| 41 | 2.8% | `radiomics::workflows::engine::run_prepared_at` |
| 39 | 2.7% | `radiomics::workflows::roi::compute_roi` |
| 36 | 2.5% | `radiomics::features::first_order::compute` |
| 36 | 2.5% | `radiomics::features::first_order::first_order_roi` |
| 36 | 2.5% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 26 | 1.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 26 | 1.8% | `radiomics::imaging::io::HostArray::crop` |
| 22 | 1.5% | `radiomics::imaging::preprocessing::true_positions` |
| 13 | 0.9% | `radiomics::contracts::types::_::_<impl serde_core::ser::Serialize for radiomics::contracts::types::ExecutionMetadata>::serialize` |
| 10 | 0.7% | `radiomics::features::first_order::RegionKernel::finish` |
| 8 | 0.5% | `radiomics::features::first_order_source::check_samples` |
| 7 | 0.5% | `radiomics::imaging::preprocessing::normalize_values` |
| 6 | 0.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 6 | 0.4% | `radiomics::config::settings::_::_<impl serde_core::ser::Serialize for radiomics::config::settings::Settings>::serialize` |
| 3 | 0.2% | `radiomics::config::settings::Settings::validate` |
| 2 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.1% | `radiomics::contracts::json::convert` |
| 1 | 0.1% | `radiomics::config::validation::unique` |
| 1 | 0.1% | `radiomics::contracts::metadata::_<impl serde_core::ser::Serialize for radiomics::config::transforms::TransformSettings>::serialize` |
