# Profile: core/python-mean-bool

- Command: `../mlx-radiomics/.venv/bin/python benchmarks/python_roi.py ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd --seconds 6 --layout bool-mask --features firstorder_Mean`
- Measured library: `2cfa716`
- Checkout at sampling: `2cfa716`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:46 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 2446 (idle worker-thread samples excluded: 10797)

## Command output

```text
{"layout": "bool-mask", "shape": [134, 512, 512], "image_dtype": "int32", "mask_dtype": "bool", "features": 1, "roi_voxels": 54639, "runs": 545, "median_ms": 6.371375002345303}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1840 | 75.2% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 228 | 9.3% | `__psynch_mutexwait` |
| 193 | 7.9% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 14 | 0.6% | `_platform_memmove` |
| 13 | 0.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 13 | 0.5% | `<deduplicated_symbol>` |
| 12 | 0.5% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 12 | 0.5% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 8 | 0.3% | `crossbeam_epoch::internal::Global::try_advance` |
| 8 | 0.3% | `crossbeam_epoch::default::with_handle` |
| 8 | 0.3% | `rayon_core::join::join_context::_{{closure}}` |
| 7 | 0.3% | `__psynch_mutexdrop` |
| 7 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 6 | 0.2% | `__psynch_cvsignal` |
| 5 | 0.2% | `crossbeam_deque::deque::Worker<T>::pop` |
| 4 | 0.2% | `radiomics::features::first_order_source::check_samples` |
| 4 | 0.2% | `PyDict_GetItemRef` |
| 4 | 0.2% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 3 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 3 | 0.1% | `_platform_memcmp` |
| 3 | 0.1% | `_tlv_get_addr` |
| 3 | 0.1% | `pthread_mutex_lock` |
| 3 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 2 | 0.1% | `hashbrown::raw::RawTable<T,A>::reserve_rehash` |
| 2 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.1% | `xzm_malloc_zone_try_free_default_tc` |
| 2 | 0.1% | `PyObject_GenericGetAttr` |
| 2 | 0.1% | `PyUnicode_DecodeUTF8Stateful` |
| 2 | 0.1% | `_xzm_xzone_malloc_tiny` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 373 | 15.2% | `radiomics::workflows::extraction::extract` |
| 372 | 15.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 250 | 10.2% | `radiomics::workflows::engine::inputs::input_window` |
| 250 | 10.2% | `radiomics::workflows::roi::bounds::include` |
| 193 | 7.9% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 89 | 3.6% | `radiomics::imaging::preprocessing::prepare` |
| 79 | 3.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 70 | 2.9% | `radiomics::imaging::preprocessing::gather` |
| 49 | 2.0% | `radiomics::imaging::preprocessing::gather_at` |
| 21 | 0.9% | `radiomics::imaging::preprocessing::true_positions` |
| 21 | 0.9% | `radiomics::workflows::engine::run_prepared_at` |
| 21 | 0.9% | `radiomics::workflows::roi::compute_roi` |
| 20 | 0.8% | `radiomics::features::first_order::compute` |
| 20 | 0.8% | `radiomics::features::first_order::first_order_roi` |
| 20 | 0.8% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 14 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 12 | 0.5% | `radiomics::imaging::io::HostArray::crop` |
| 10 | 0.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 6 | 0.2% | `radiomics::features::first_order::RegionKernel::finish` |
| 5 | 0.2% | `radiomics::contracts::types::_::_<impl serde_core::ser::Serialize for radiomics::contracts::types::ExecutionMetadata>::serialize` |
| 4 | 0.2% | `radiomics::features::first_order_source::check_samples` |
| 4 | 0.2% | `radiomics::contracts::json::serialize_safe` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.1% | `radiomics::config::settings::_::_<impl serde_core::ser::Serialize for radiomics::config::settings::Settings>::serialize` |
| 2 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 1 | 0.0% | `radiomics::config::settings::Settings::validate` |
| 1 | 0.0% | `radiomics::config::validation::unique` |
| 1 | 0.0% | `radiomics::imaging::io::HostArray::eq_label` |
| 1 | 0.0% | `radiomics::config::transforms::TransformSettings::validate` |
| 1 | 0.0% | `radiomics::contracts::metadata::_<impl serde_core::ser::Serialize for radiomics::config::transforms::TransformSettings>::serialize` |
