# Profile: core/firstorder.row-scan

- Command: `target/bench-core/final/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile firstorder_Mean firstorder_Minimum firstorder_Maximum firstorder_Range firstorder_Variance firstorder_Skewness firstorder_Kurtosis firstorder_Energy firstorder_TotalEnergy firstorder_RootMeanSquared firstorder_Entropy firstorder_Uniformity firstorder_10Percentile firstorder_90Percentile firstorder_Median firstorder_InterquartileRange firstorder_MeanAbsoluteDeviation firstorder_RobustMeanAbsoluteDeviation firstorder_MedianAbsoluteDeviation firstorder_CoefficientOfVariation firstorder_QuartileCoefficientOfDispersion`
- Measured library: `3ab52d3`
- Checkout at sampling: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:24 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 615 (idle worker-thread samples excluded: 13502)

## Command output

```text
{"median_ms":9.708,"runs":321,"feature_count":21}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 183 | 29.8% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 125 | 20.3% | `__psynch_mutexwait` |
| 96 | 15.6% | `<deduplicated_symbol>` |
| 33 | 5.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 24 | 3.9% | `radiomics::imaging::preprocessing::lattice_bins` |
| 24 | 3.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 22 | 3.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 16 | 2.6% | `radiomics::features::first_order::RegionKernel::finish` |
| 12 | 2.0% | `radiomics::features::first_order::LevelCounts::add` |
| 8 | 1.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 7 | 1.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 7 | 1.1% | `_platform_memmove` |
| 4 | 0.7% | `__psynch_cvsignal` |
| 4 | 0.7% | `num_bigint::biguint::shift::_<impl core::ops::bit::ShrAssign<u64> for num_bigint::biguint::BigUint>::shr_assign` |
| 4 | 0.7% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 4 | 0.7% | `__psynch_mutexdrop` |
| 3 | 0.5% | `crossbeam_epoch::default::with_handle` |
| 3 | 0.5% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 3 | 0.5% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 2 | 0.3% | `_<num_bigint::biguint::BigUint as num_integer::Integer>::gcd` |
| 2 | 0.3% | `num_bigint::biguint::shift::biguint_shr2` |
| 2 | 0.3% | `num_bigint::bigint::multiplication::_<impl core::ops::arith::Mul for num_bigint::bigint::BigInt>::mul` |
| 2 | 0.3% | `num_bigint::biguint::division::div_rem_cow` |
| 2 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 2 | 0.3% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 2 | 0.3% | `_pthread_cond_wait` |
| 1 | 0.2% | `num_bigint::biguint::BigUint::trailing_zeros` |
| 1 | 0.2% | `num_bigint::biguint::subtraction::_<impl core::ops::arith::SubAssign<&num_bigint::biguint::BigUint> for num_bigint::biguint::BigUint>::sub_assign` |
| 1 | 0.2% | `num_bigint::bigint::division::_<impl core::ops::arith::Div for num_bigint::bigint::BigInt>::div` |
| 1 | 0.2% | `num_bigint::big_digit::BigDigits::normalize` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 740 | 120.3% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 500 | 81.3% | `radiomics::workflows::engine::inputs::input_window` |
| 500 | 81.3% | `radiomics::workflows::roi::bounds::include` |
| 139 | 22.6% | `radiomics::imaging::preprocessing::prepare` |
| 90 | 14.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 85 | 13.8% | `radiomics::workflows::engine::run_prepared` |
| 81 | 13.2% | `radiomics::workflows::roi::compute_roi` |
| 78 | 12.7% | `radiomics::features::first_order::compute` |
| 78 | 12.7% | `radiomics::features::first_order::first_order_roi` |
| 76 | 12.4% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 49 | 8.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 44 | 7.2% | `radiomics::imaging::preprocessing::gather` |
| 41 | 6.7% | `radiomics::imaging::preprocessing::lattice_bins` |
| 41 | 6.7% | `radiomics::features::first_order::RegionKernel::finish` |
| 33 | 5.4% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 24 | 3.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 16 | 2.6% | `radiomics::imaging::io::HostArray::crop` |
| 12 | 2.0% | `radiomics::features::first_order::LevelCounts::add` |
| 2 | 0.3% | `radiomics::features::first_order::RegionKernel::new` |
| 1 | 0.2% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 1 | 0.2% | `radiomics::imaging::preprocessing::check_gray_levels` |
| 1 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 1 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 1 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 1 | 0.2% | `radiomics::features::first_order::SampleStatistics::new` |
| 1 | 0.2% | `radiomics::native_first_order::statistics::Calculation::new` |
| 1 | 0.2% | `radiomics::workflows::roi_memory::check_roi_count_buffers` |
| 1 | 0.2% | `radiomics::config::settings::Settings::validate` |
| 1 | 0.2% | `radiomics::config::validation::unique` |
