# Profile: core/python-mean-bool.baseline

- Command: `../mlx-radiomics/.venv/bin/python benchmarks/python_roi.py ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd --seconds 6 --layout bool-mask --features firstorder_Mean`
- Measured library: `c23929c`
- Checkout at sampling: `2cfa716`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:46 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 2469 (idle worker-thread samples excluded: 15980)

## Command output

```text
{"layout": "bool-mask", "shape": [134, 512, 512], "image_dtype": "int32", "mask_dtype": "bool", "features": 1, "roi_voxels": 54639, "runs": 449, "median_ms": 9.258916998078348}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 969 | 39.2% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 529 | 21.4% | `__psynch_mutexwait` |
| 446 | 18.1% | `_platform_memmove` |
| 163 | 6.6% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 97 | 3.9% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 97 | 3.9% | `_aligned_contig_cast_bool_to_ubyte` |
| 21 | 0.9% | `<deduplicated_symbol>` |
| 16 | 0.6% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 14 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 8 | 0.3% | `__psynch_mutexdrop` |
| 8 | 0.3% | `__psynch_cvsignal` |
| 8 | 0.3% | `_pthread_cond_wait` |
| 7 | 0.3% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 7 | 0.3% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 5 | 0.2% | `crossbeam_epoch::default::with_handle` |
| 5 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 4 | 0.2% | `rayon_core::join::join_context::_{{closure}}` |
| 3 | 0.1% | `radiomics::features::first_order_source::check_samples` |
| 3 | 0.1% | `take_gil` |
| 3 | 0.1% | `_PyObject_Malloc` |
| 3 | 0.1% | `_tlv_get_addr` |
| 2 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::to_owned` |
| 2 | 0.1% | `_platform_memcmp` |
| 2 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.1% | `mach_absolute_time` |
| 2 | 0.1% | `_PyEval_EvalFrameDefault` |
| 2 | 0.1% | `rayon_core::sleep::Sleep::wake_specific_thread` |
| 2 | 0.1% | `rayon_core::registry::WorkerThread::find_work` |
| 2 | 0.1% | `crossbeam_epoch::internal::Global::try_advance` |
| 1 | 0.0% | `PyList_New` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 313 | 12.7% | `radiomics::workflows::extraction::extract` |
| 311 | 12.6% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 187 | 7.6% | `radiomics::workflows::engine::inputs::input_window` |
| 187 | 7.6% | `radiomics::workflows::roi::bounds::include` |
| 163 | 6.6% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 80 | 3.2% | `radiomics::imaging::preprocessing::prepare` |
| 75 | 3.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 62 | 2.5% | `radiomics::imaging::preprocessing::gather` |
| 44 | 1.8% | `radiomics::imaging::preprocessing::gather_at` |
| 22 | 0.9% | `radiomics::workflows::engine::run_prepared_at` |
| 21 | 0.9% | `radiomics::workflows::roi::compute_roi` |
| 21 | 0.9% | `radiomics::features::first_order::compute` |
| 21 | 0.9% | `radiomics::features::first_order::first_order_roi` |
| 21 | 0.9% | `radiomics::imaging::io::HostArray::crop` |
| 20 | 0.8% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 18 | 0.7% | `radiomics::imaging::preprocessing::true_positions` |
| 15 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 6 | 0.2% | `radiomics::contracts::types::_::_<impl serde_core::ser::Serialize for radiomics::contracts::types::ExecutionMetadata>::serialize` |
| 5 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 5 | 0.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 5 | 0.2% | `radiomics::features::first_order::RegionKernel::finish` |
| 4 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 3 | 0.1% | `radiomics::features::first_order_source::check_samples` |
| 3 | 0.1% | `radiomics::config::settings::_::_<impl serde_core::ser::Serialize for radiomics::config::settings::Settings>::serialize` |
| 3 | 0.1% | `radiomics::contracts::metadata::_<impl serde_core::ser::Serialize for radiomics::config::transforms::TransformSettings>::serialize` |
| 2 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::config::settings::Settings::validate` |
