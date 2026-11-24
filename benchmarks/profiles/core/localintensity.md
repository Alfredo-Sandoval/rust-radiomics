# Profile: core/localintensity

- Command: `target/bench-core/mask/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile localintensity_LocalIntensityPeak localintensity_GlobalIntensityPeak`
- Measured library: `4f42922`
- Checkout at sampling: `a324330` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:32 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 1257 (idle worker-thread samples excluded: 25913)

## Command output

```text
{"median_ms":7.654854,"runs":374,"feature_count":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 397 | 31.6% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 359 | 28.6% | `radiomics::features::local_intensity::local_intensity_statistics::_{{closure}}` |
| 143 | 11.4% | `<deduplicated_symbol>` |
| 131 | 10.4% | `radiomics::features::local_intensity::local_intensity_statistics` |
| 82 | 6.5% | `__psynch_mutexwait` |
| 35 | 2.8% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 11 | 0.9% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 11 | 0.9% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 9 | 0.7% | `_platform_memmove` |
| 9 | 0.7% | `__psynch_cvsignal` |
| 9 | 0.7% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 8 | 0.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 7 | 0.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 7 | 0.6% | `_pthread_cond_wait` |
| 6 | 0.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 4 | 0.3% | `crossbeam_epoch::default::with_handle` |
| 3 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 3 | 0.2% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 3 | 0.2% | `__psynch_cvbroad` |
| 3 | 0.2% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 2 | 0.2% | `_malloc_zone_malloc` |
| 2 | 0.2% | `mach_vm_reclaim_try_cancel` |
| 1 | 0.1% | `_<radiomics::config::settings::Settings as core::clone::Clone>::clone` |
| 1 | 0.1% | `_RNvNvNtCs8dnTdrJsiec_5alloc3fmt6format12format_inner` |
| 1 | 0.1% | `indexmap::inner::Core<K,V>::get_index_of` |
| 1 | 0.1% | `_xzm_chunk_list_pop` |
| 1 | 0.1% | `_xzm_malloc_tc` |
| 1 | 0.1% | `DYLD-STUB$$pthread_mutex_lock` |
| 1 | 0.1% | `DYLD-STUB$$sched_yield` |
| 1 | 0.1% | `crossbeam_deque::deque::Worker<T>::pop` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1430 | 113.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 710 | 56.5% | `radiomics::workflows::engine::run_prepared` |
| 706 | 56.2% | `radiomics::workflows::roi::compute_roi` |
| 705 | 56.1% | `radiomics::features::local_intensity::compute` |
| 705 | 56.1% | `radiomics::features::local_intensity::local_intensity_statistics` |
| 545 | 43.4% | `radiomics::workflows::engine::inputs::input_window` |
| 545 | 43.4% | `radiomics::workflows::roi::bounds::include` |
| 364 | 29.0% | `radiomics::features::local_intensity::local_intensity_statistics::_{{closure}}` |
| 127 | 10.1% | `radiomics::imaging::preprocessing::prepare` |
| 119 | 9.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 103 | 8.2% | `radiomics::imaging::preprocessing::gather` |
| 76 | 6.0% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 48 | 3.8% | `radiomics::imaging::io::HostArray::crop` |
| 8 | 0.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 6 | 0.5% | `radiomics::imaging::io::HostArray::eq_label` |
| 2 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 1 | 0.1% | `radiomics::features::local_intensity::Sphere::new` |
| 1 | 0.1% | `radiomics::config::settings::Settings::validate` |
