# Profile: filters/w13-ibsi2-phase2

- Command: `target/bench-filters/final/w13 --nocapture`
- Commit: `4b9649b` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 03:23 UTC
- Sampling: `sample`, 2.5 s after 0.3 s warm-up, 1 ms interval, all threads
- Busy samples: 8197 (idle worker-thread samples excluded: 6423)

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

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.99s
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1295 | 15.8% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 869 | 10.6% | `radiomics::numerics::spline::evaluate_flat` |
| 839 | 10.2% | `rustfft::algorithm::mixed_radix::MixedRadixSmall<T>::perform_fft_inplace` |
| 731 | 8.9% | `semaphore_timedwait_trap` |
| 584 | 7.1% | `rustfft::algorithm::good_thomas_algorithm::GoodThomasAlgorithmSmall<T>::perform_fft_inplace` |
| 537 | 6.6% | `rustfft::algorithm::good_thomas_algorithm::GoodThomasAlgorithmSmall<T>::perform_fft_out_of_place` |
| 487 | 5.9% | `radiomics::imaging::spatial::IndexMap::map` |
| 317 | 3.9% | `_platform_memmove` |
| 312 | 3.8% | `core::ops::function::impls::_<impl core::ops::function::Fn<A> for &F>::call` |
| 278 | 3.4% | `radiomics::filters::fft_conv::transpose` |
| 246 | 3.0% | `__psynch_mutexwait` |
| 193 | 2.4% | `_<rustfft::neon::neon_butterflies::NeonF64Butterfly5<T> as rustfft::Fft<T>>::process_with_scratch` |
| 188 | 2.3% | `_<rustfft::neon::neon_butterflies::NeonF64Butterfly4<T> as rustfft::Fft<T>>::process_with_scratch` |
| 183 | 2.2% | `_<rustfft::neon::neon_butterflies::NeonF64Butterfly5<T> as rustfft::Fft<T>>::process_outofplace_with_scratch` |
| 174 | 2.1% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 123 | 1.5% | `rustfft::algorithm::raders_algorithm::RadersAlgorithm<T>::perform_fft_out_of_place` |
| 113 | 1.4% | `__bzero` |
| 76 | 0.9% | `radiomics::numerics::spline::prefilter_line` |
| 57 | 0.7% | `_RNvMNtCs6sq8b9ugfBC_4core3f64d3max` |
| 42 | 0.5% | `<deduplicated_symbol>` |
| 42 | 0.5% | `miniz_oxide::inflate::core::decompress_fast` |
| 38 | 0.5% | `rayon_core::join::join_context::_{{closure}}` |
| 35 | 0.4% | `miniz_oxide::inflate::core::transfer` |
| 31 | 0.4% | `hypot` |
| 29 | 0.4% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 26 | 0.3% | `__psynch_mutexdrop` |
| 23 | 0.3% | `transpose::out_of_place::transpose` |
| 19 | 0.2% | `_<rustfft::algorithm::good_thomas_algorithm::GoodThomasAlgorithmSmall<T> as rustfft::Fft<T>>::process_outofplace_with_scratch` |
| 17 | 0.2% | `crossbeam_epoch::default::with_handle` |
| 16 | 0.2% | `core::slice::sort::unstable::quicksort::quicksort` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 869 | 10.6% | `radiomics::numerics::spline::evaluate_flat` |
| 663 | 8.1% | `radiomics::workflows::extraction::extract_files` |
| 487 | 5.9% | `radiomics::imaging::spatial::IndexMap::map` |
| 452 | 5.5% | `radiomics::workflows::extraction::extract` |
| 386 | 4.7% | `radiomics::workflows::extraction::extract_pair` |
| 364 | 4.4% | `radiomics::imaging::transforms::compute_feature_images` |
| 278 | 3.4% | `radiomics::filters::fft_conv::transpose` |
| 276 | 3.4% | `radiomics::imaging::spatial::load_image_pair` |
| 231 | 2.8% | `radiomics::filters::convolution::gabor_responses` |
| 205 | 2.5% | `radiomics::filters::fft_conv::valid_complex_responses` |
| 182 | 2.2% | `radiomics::imaging::spatial::prepare_image_pair` |
| 170 | 2.1% | `radiomics::imaging::spatial::resample_ibsi` |
| 127 | 1.5% | `radiomics::imaging::spatial::resample_f64` |
| 94 | 1.1% | `radiomics::imaging::io::read` |
| 88 | 1.1% | `radiomics::imaging::io::read_stream` |
| 78 | 1.0% | `radiomics::numerics::spline::prefilter_line` |
| 52 | 0.6% | `radiomics::filters::kernels::valid_conv_axis` |
| 44 | 0.5% | `radiomics::filters::fft_conv::Plan2d::workspace` |
| 42 | 0.5% | `radiomics::filters::convolution::laws_response` |
| 36 | 0.4% | `radiomics::workflows::engine::run_prepared_at` |
| 35 | 0.4% | `radiomics::workflows::roi::compute_roi` |
| 34 | 0.4% | `radiomics::features::first_order::compute` |
| 34 | 0.4% | `radiomics::features::first_order::first_order_roi` |
| 34 | 0.4% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 27 | 0.3% | `radiomics::filters::spectral::log_response` |
| 27 | 0.3% | `radiomics::filters::spectral::simoncelli_response` |
| 26 | 0.3% | `radiomics::filters::convolution::convolve_separable_native` |
| 23 | 0.3% | `radiomics::features::first_order::RegionKernel::finish` |
| 23 | 0.3% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 21 | 0.3% | `radiomics::numerics::spline::prefilter_all` |
