# Profile: core/firstorder.baseline

- Command: `target/bench-core/baseline/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile firstorder_Mean firstorder_Minimum firstorder_Maximum firstorder_Range firstorder_Variance firstorder_Skewness firstorder_Kurtosis firstorder_Energy firstorder_TotalEnergy firstorder_RootMeanSquared firstorder_Entropy firstorder_Uniformity firstorder_10Percentile firstorder_90Percentile firstorder_Median firstorder_InterquartileRange firstorder_MeanAbsoluteDeviation firstorder_RobustMeanAbsoluteDeviation firstorder_MedianAbsoluteDeviation firstorder_CoefficientOfVariation firstorder_QuartileCoefficientOfDispersion`
- Measured library: `17d817c`
- Checkout at sampling: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:04 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3267 (idle worker-thread samples excluded: 0)

## Command output

```text
{"median_ms":20.994916500000002,"runs":282,"feature_count":21}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2664 | 81.5% | `radiomics::workflows::roi::bounds::include` |
| 138 | 4.2% | `<deduplicated_symbol>` |
| 85 | 2.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 63 | 1.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 62 | 1.9% | `core::slice::sort::unstable::quicksort::quicksort` |
| 43 | 1.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 41 | 1.3% | `radiomics::imaging::preprocessing::lattice_bins` |
| 34 | 1.0% | `radiomics::features::first_order::RegionKernel::finish` |
| 33 | 1.0% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 21 | 0.6% | `radiomics::features::first_order::LevelCounts::add` |
| 15 | 0.5% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 13 | 0.4% | `_platform_memmove` |
| 10 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 8 | 0.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 7 | 0.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 5 | 0.2% | `num_bigint::biguint::BigUint::trailing_zeros` |
| 4 | 0.1% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 3 | 0.1% | `_malloc_zone_realloc` |
| 2 | 0.1% | `core::slice::sort::shared::pivot::median3_rec` |
| 2 | 0.1% | `__xzm_xzone_free_to_freelist_chunk` |
| 1 | 0.0% | `num_bigint::biguint::shift::biguint_shr2` |
| 1 | 0.0% | `num_bigint::biguint::shift::_<impl core::ops::bit::ShrAssign<u64> for num_bigint::biguint::BigUint>::shr_assign` |
| 1 | 0.0% | `num_bigint::bigint::division::_<impl core::ops::arith::Div for num_bigint::bigint::BigInt>::div` |
| 1 | 0.0% | `_<num_bigint::bigint::BigInt as num_integer::Integer>::div_rem` |
| 1 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 1 | 0.0% | `_platform_memset` |
| 1 | 0.0% | `DYLD-STUB$$memcmp` |
| 1 | 0.0% | `_RNvXsZ_NtCs8dnTdrJsiec_5alloc6stringNtB5_6StringNtNtCs6sq8b9ugfBC_4core3fmt5Write9write_str` |
| 1 | 0.0% | `mach_absolute_time` |
| 1 | 0.0% | `xzm_malloc_zone_size` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3257 | 99.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2664 | 81.5% | `radiomics::workflows::engine::inputs::input_window` |
| 2664 | 81.5% | `radiomics::workflows::roi::bounds::include` |
| 324 | 9.9% | `radiomics::imaging::preprocessing::prepare` |
| 238 | 7.3% | `radiomics::workflows::engine::run_prepared` |
| 229 | 7.0% | `radiomics::workflows::roi::compute_roi` |
| 228 | 7.0% | `radiomics::features::first_order::compute` |
| 228 | 7.0% | `radiomics::features::first_order::first_order_roi` |
| 225 | 6.9% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 180 | 5.5% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 171 | 5.2% | `radiomics::features::first_order::RegionKernel::finish` |
| 144 | 4.4% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 127 | 3.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 118 | 3.6% | `radiomics::imaging::preprocessing::gather` |
| 50 | 1.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 48 | 1.5% | `radiomics::imaging::preprocessing::lattice_bins` |
| 29 | 0.9% | `radiomics::imaging::io::HostArray::crop` |
| 21 | 0.6% | `radiomics::features::first_order::LevelCounts::add` |
| 12 | 0.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 10 | 0.3% | `radiomics::numerics::reduction::reported_mean` |
| 6 | 0.2% | `radiomics::config::settings::Settings::validate` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.1% | `radiomics::config::validation::unique` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 1 | 0.0% | `radiomics::workflows::engine::checked_source` |
