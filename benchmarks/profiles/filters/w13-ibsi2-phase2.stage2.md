# Profile: filters/w13-ibsi2-phase2.stage2

- Command: `target/bench-filters/stage2/w13 --nocapture`
- Commit: `df67aec` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:50 UTC
- Sampling: `sample`, 5 s after 0.5 s warm-up, 1 ms interval, all threads
- Busy samples: 28871 (idle worker-thread samples excluded: 12309)

## Command output

```text
running 2 tests
test reference_table_parses_and_every_filter_id_has_a_configured_case ... ok
1.A  original            count 18 pass 18 worst 0.300 (stat_range)
2.A  mean                count 18 pass 18 worst 0.400 (stat_p90)
3.A  log                 count 18 pass 18 worst 0.774 (stat_median)
4.A  laws                count 18 pass 18 worst 0.480 (stat_min)
5.A  gabor               count 18 pass 18 worst 0.377 (stat_var)
6.A  separable wavelet   count 18 pass 18 worst 0.296 (stat_iqr)
7.A  separable wavelet   count 18 pass 18 worst 0.184 (stat_rmad)
8.A  simoncelli centred  count 18 pass 18 worst 0.501 (stat_qcod)
  FAIL 8.A [simoncelli dft] stat_qcod: value -3.147793e2 consensus -4.410000e2 tolerance 2.900e1 (|d|/tol 4.352)
8.A  simoncelli dft      count 18 pass 17 worst 4.352 (stat_qcod)
9.A  simoncelli centred  count 18 pass 18 worst 0.538 (stat_min)
9.A  simoncelli dft      count 18 pass 18 worst 0.561 (stat_range)
1.B  original            count 18 pass 18 worst 0.200 (stat_range)
2.B  mean                count 18 pass 18 worst 0.480 (stat_p90)
3.B  log                 count 18 pass 18 worst 0.326 (stat_median)
4.B  laws                count 18 pass 18 worst 0.373 (stat_min)
5.B  gabor               count 18 pass 18 worst 0.497 (stat_iqr)
6.B  separable wavelet   count 18 pass 18 worst 0.476 (stat_min)
7.B  separable wavelet   count 18 pass 18 worst 0.230 (stat_mad)
8.B  simoncelli centred  count 17 pass 17 worst 0.703 (stat_median)
8.B  simoncelli dft      count 17 pass 17 worst 0.397 (stat_range)
9.B  simoncelli centred  count 18 pass 18 worst 0.369 (stat_iqr)
  FAIL 9.B [simoncelli dft] stat_min: value -5.994526e2 consensus -6.050000e2 tolerance 2.000e0 (|d|/tol 2.774)
9.B  simoncelli dft      count 18 pass 17 worst 2.774 (stat_min)
total 392/394
test ibsi2_filtered_ct_statistics_match_consensus ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.26s
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 6171 | 21.4% | `rustfft::algorithm::mixed_radix::MixedRadixSmall<T>::perform_fft_inplace` |
| 3140 | 10.9% | `rustfft::algorithm::good_thomas_algorithm::GoodThomasAlgorithmSmall<T>::perform_fft_inplace` |
| 3055 | 10.6% | `rustfft::algorithm::good_thomas_algorithm::GoodThomasAlgorithmSmall<T>::perform_fft_out_of_place` |
| 2268 | 7.9% | `radiomics::filters::fft_conv::transpose` |
| 2059 | 7.1% | `semaphore_timedwait_trap` |
| 1247 | 4.3% | `_<rustfft::neon::neon_butterflies::NeonF64Butterfly5<T> as rustfft::Fft<T>>::process_outofplace_with_scratch` |
| 1241 | 4.3% | `core::ops::function::impls::_<impl core::ops::function::Fn<A> for &F>::call` |
| 1193 | 4.1% | `radiomics::numerics::spline::evaluate_flat` |
| 1030 | 3.6% | `_<rustfft::neon::neon_butterflies::NeonF64Butterfly4<T> as rustfft::Fft<T>>::process_with_scratch` |
| 999 | 3.5% | `_<rustfft::neon::neon_butterflies::NeonF64Butterfly5<T> as rustfft::Fft<T>>::process_with_scratch` |
| 918 | 3.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 753 | 2.6% | `_<rayon::iter::for_each::ForEachConsumer<F> as rayon::iter::plumbing::Folder<T>>::consume_iter` |
| 669 | 2.3% | `_platform_memmove` |
| 579 | 2.0% | `radiomics::imaging::spatial::IndexMap::map` |
| 547 | 1.9% | `<deduplicated_symbol>` |
| 440 | 1.5% | `__psynch_mutexwait` |
| 425 | 1.5% | `rustfft::array_utils::validate_and_iter` |
| 329 | 1.1% | `hypot` |
| 291 | 1.0% | `__bzero` |
| 171 | 0.6% | `DYLD-STUB$$hypot` |
| 163 | 0.6% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 115 | 0.4% | `rustfft::algorithm::raders_algorithm::RadersAlgorithm<T>::perform_fft_out_of_place` |
| 103 | 0.4% | `_platform_memset_pattern16` |
| 97 | 0.3% | `radiomics::numerics::spline::prefilter_line` |
| 77 | 0.3% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 68 | 0.2% | `_<rustfft::algorithm::good_thomas_algorithm::GoodThomasAlgorithmSmall<T> as rustfft::Fft<T>>::process_outofplace_with_scratch` |
| 60 | 0.2% | `_<rustfft::algorithm::good_thomas_algorithm::GoodThomasAlgorithmSmall<T> as rustfft::Fft<T>>::process_with_scratch` |
| 52 | 0.2% | `miniz_oxide::inflate::core::decompress_fast` |
| 50 | 0.2% | `_RNvMNtCs6sq8b9ugfBC_4core3f64d3max` |
| 43 | 0.1% | `miniz_oxide::inflate::core::transfer` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2268 | 7.9% | `radiomics::filters::fft_conv::transpose` |
| 1931 | 6.7% | `radiomics::workflows::extraction::extract_files` |
| 1617 | 5.6% | `radiomics::workflows::extraction::extract` |
| 1506 | 5.2% | `radiomics::imaging::transforms::compute_feature_images` |
| 1500 | 5.2% | `radiomics::workflows::extraction::extract_pair` |
| 1316 | 4.6% | `radiomics::filters::convolution::gabor_responses` |
| 1253 | 4.3% | `radiomics::filters::fft_conv::valid_complex_responses` |
| 1193 | 4.1% | `radiomics::numerics::spline::evaluate_flat` |
| 579 | 2.0% | `radiomics::imaging::spatial::IndexMap::map` |
| 431 | 1.5% | `radiomics::imaging::spatial::load_image_pair` |
| 321 | 1.1% | `radiomics::imaging::spatial::prepare_image_pair` |
| 291 | 1.0% | `radiomics::imaging::spatial::resample_ibsi` |
| 238 | 0.8% | `radiomics::imaging::spatial::resample_f64` |
| 167 | 0.6% | `radiomics::filters::fft_conv::Plan2d::workspace` |
| 110 | 0.4% | `radiomics::imaging::io::read` |
| 105 | 0.4% | `radiomics::imaging::io::read_stream` |
| 100 | 0.3% | `radiomics::numerics::spline::prefilter_line` |
| 85 | 0.3% | `radiomics::numerics::spline::prefilter_all` |
| 68 | 0.2% | `radiomics::filters::kernels::valid_conv_axis` |
| 50 | 0.2% | `radiomics::filters::convolution::laws_response` |
| 47 | 0.2% | `radiomics::workflows::engine::run_prepared_at` |
| 46 | 0.2% | `radiomics::workflows::roi::compute_roi` |
| 44 | 0.2% | `radiomics::features::first_order::compute` |
| 44 | 0.2% | `radiomics::features::first_order::first_order_roi` |
| 44 | 0.2% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 40 | 0.1% | `radiomics::filters::convolution::separable_wavelet_responses` |
| 38 | 0.1% | `radiomics::filters::kernels::pad_vol` |
| 34 | 0.1% | `radiomics::filters::spectral::simoncelli_response` |
| 32 | 0.1% | `radiomics::features::first_order::RegionKernel::finish` |
| 28 | 0.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
