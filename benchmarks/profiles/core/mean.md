# Profile: core/mean

- Command: `target/bench-core/mask/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile firstorder_Mean`
- Measured library: `4f42922`
- Checkout at sampling: `4f42922` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:31 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 1211 (idle worker-thread samples excluded: 10550)

## Command output

```text
{"median_ms":4.214375,"runs":725,"feature_count":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 446 | 36.8% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 285 | 23.5% | `__psynch_mutexwait` |
| 153 | 12.6% | `<deduplicated_symbol>` |
| 46 | 3.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 46 | 3.8% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 36 | 3.0% | `__psynch_mutexdrop` |
| 20 | 1.7% | `radiomics::features::first_order::RegionKernel::finish` |
| 20 | 1.7% | `__psynch_cvsignal` |
| 18 | 1.5% | `_platform_memmove` |
| 17 | 1.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 13 | 1.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 12 | 1.0% | `crossbeam_epoch::default::with_handle` |
| 11 | 0.9% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 9 | 0.7% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 9 | 0.7% | `_pthread_cond_wait` |
| 9 | 0.7% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 8 | 0.7% | `crossbeam_epoch::internal::Global::try_advance` |
| 7 | 0.6% | `rayon_core::join::join_context::_{{closure}}` |
| 6 | 0.5% | `crossbeam_deque::deque::Worker<T>::pop` |
| 5 | 0.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 5 | 0.4% | `pthread_mutex_lock` |
| 4 | 0.3% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 3 | 0.2% | `rayon_core::sleep::Sleep::wake_specific_thread` |
| 3 | 0.2% | `_tlv_get_addr` |
| 3 | 0.2% | `_pthread_mutex_firstfit_lock_slow` |
| 2 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 2 | 0.2% | `rayon_core::registry::WorkerThread::find_work` |
| 2 | 0.2% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 1 | 0.1% | `__xzm_xzone_free_to_freelist_chunk` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 619 | 51.1% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 348 | 28.7% | `radiomics::workflows::engine::inputs::input_window` |
| 348 | 28.7% | `radiomics::workflows::roi::bounds::include` |
| 156 | 12.9% | `radiomics::imaging::preprocessing::prepare` |
| 141 | 11.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 118 | 9.7% | `radiomics::imaging::preprocessing::gather` |
| 101 | 8.3% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 76 | 6.3% | `radiomics::workflows::engine::run_prepared` |
| 71 | 5.9% | `radiomics::workflows::roi::compute_roi` |
| 69 | 5.7% | `radiomics::features::first_order::compute` |
| 69 | 5.7% | `radiomics::features::first_order::first_order_roi` |
| 68 | 5.6% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 46 | 3.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 38 | 3.1% | `radiomics::imaging::io::HostArray::crop` |
| 22 | 1.8% | `radiomics::features::first_order::RegionKernel::finish` |
| 15 | 1.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 12 | 1.0% | `radiomics::imaging::preprocessing::normalize_values` |
| 5 | 0.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 2 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 1 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.1% | `radiomics::workflows::roi_memory::check_roi_count_buffers` |
| 1 | 0.1% | `radiomics::features::morphology::require_pairwise_admission` |
| 1 | 0.1% | `radiomics::features::morphology::require_pairwise_admission_with` |
