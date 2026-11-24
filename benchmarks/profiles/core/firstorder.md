# Profile: core/firstorder

- Command: `target/bench-core/mask/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile firstorder_Mean firstorder_Minimum firstorder_Maximum firstorder_Range firstorder_Variance firstorder_Skewness firstorder_Kurtosis firstorder_Energy firstorder_TotalEnergy firstorder_RootMeanSquared firstorder_Entropy firstorder_Uniformity firstorder_10Percentile firstorder_90Percentile firstorder_Median firstorder_InterquartileRange firstorder_MeanAbsoluteDeviation firstorder_RobustMeanAbsoluteDeviation firstorder_MedianAbsoluteDeviation firstorder_CoefficientOfVariation firstorder_QuartileCoefficientOfDispersion`
- Measured library: `4f42922`
- Checkout at sampling: `9a7a2d5` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:32 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2283 (idle worker-thread samples excluded: 15824)

## Command output

```text
{"median_ms":2.3848545000000003,"runs":1294,"feature_count":21}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 698 | 30.6% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 477 | 20.9% | `__psynch_mutexwait` |
| 224 | 9.8% | `<deduplicated_symbol>` |
| 114 | 5.0% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 101 | 4.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 75 | 3.3% | `__psynch_mutexdrop` |
| 60 | 2.6% | `radiomics::imaging::preprocessing::lattice_bins` |
| 59 | 2.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 54 | 2.4% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 48 | 2.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 39 | 1.7% | `radiomics::features::first_order::RegionKernel::finish` |
| 37 | 1.6% | `__psynch_cvsignal` |
| 36 | 1.6% | `radiomics::features::first_order::LevelCounts::add` |
| 28 | 1.2% | `crossbeam_epoch::default::with_handle` |
| 24 | 1.1% | `_platform_memmove` |
| 22 | 1.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 19 | 0.8% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 16 | 0.7% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 15 | 0.7% | `_pthread_cond_wait` |
| 14 | 0.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 14 | 0.6% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 9 | 0.4% | `radiomics::numerics::reduction::reported_mean` |
| 9 | 0.4% | `crossbeam_epoch::internal::Global::try_advance` |
| 8 | 0.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 8 | 0.4% | `rayon_core::join::join_context::_{{closure}}` |
| 7 | 0.3% | `_<num_bigint::biguint::BigUint as num_integer::Integer>::gcd` |
| 7 | 0.3% | `_tlv_get_addr` |
| 3 | 0.1% | `num_bigint::biguint::shift::biguint_shr2` |
| 3 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 3 | 0.1% | `pthread_mutex_lock` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 940 | 41.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 449 | 19.7% | `radiomics::imaging::preprocessing::prepare` |
| 261 | 11.4% | `radiomics::workflows::engine::run_prepared` |
| 255 | 11.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 249 | 10.9% | `radiomics::workflows::roi::compute_roi` |
| 246 | 10.8% | `radiomics::features::first_order::compute` |
| 246 | 10.8% | `radiomics::features::first_order::first_order_roi` |
| 246 | 10.8% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 194 | 8.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 167 | 7.3% | `radiomics::imaging::preprocessing::gather` |
| 164 | 7.2% | `radiomics::workflows::engine::inputs::input_window` |
| 164 | 7.2% | `radiomics::workflows::roi::bounds::include` |
| 149 | 6.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 125 | 5.5% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 101 | 4.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 86 | 3.8% | `radiomics::imaging::preprocessing::lattice_bins` |
| 86 | 3.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 63 | 2.8% | `radiomics::imaging::io::HostArray::crop` |
| 36 | 1.6% | `radiomics::features::first_order::LevelCounts::add` |
| 10 | 0.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 9 | 0.4% | `radiomics::numerics::reduction::reported_mean` |
| 7 | 0.3% | `radiomics::config::settings::Settings::validate` |
| 6 | 0.3% | `radiomics::imaging::preprocessing::normalize_values` |
| 4 | 0.2% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 4 | 0.2% | `radiomics::config::validation::unique` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::workflows::roi_memory::check_roi_count_buffers` |
