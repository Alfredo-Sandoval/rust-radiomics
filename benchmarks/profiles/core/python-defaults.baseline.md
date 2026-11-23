# Profile: core/python-defaults.baseline

- Command: `../mlx-radiomics/.venv/bin/python benchmarks/python_roi.py ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd --seconds 6 --layout native`
- Measured library: `c23929c`
- Checkout at sampling: `2cfa716`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:46 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 1257 (idle worker-thread samples excluded: 12670)

## Command output

```text
{"layout": "native", "shape": [134, 512, 512], "image_dtype": "int32", "mask_dtype": "uint8", "features": 93, "roi_voxels": 54639, "runs": 300, "median_ms": 12.391416501486674}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 248 | 19.7% | `_platform_memmove` |
| 125 | 9.9% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 117 | 9.3% | `radiomics::features::matrices::run_matrix` |
| 102 | 8.1% | `__psynch_mutexwait` |
| 90 | 7.2% | `radiomics::features::matrices::Runs::join_rows` |
| 80 | 6.4% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 63 | 5.0% | `radiomics::numerics::eigen::maximal_correlation` |
| 46 | 3.7% | `radiomics::features::cooccurrence::glcm_matrix` |
| 24 | 1.9% | `log1p` |
| 20 | 1.6% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 20 | 1.6% | `radiomics::features::matrices::Padded::new` |
| 20 | 1.6% | `radiomics::features::glcm::mutual_information_bits` |
| 15 | 1.2% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 14 | 1.1% | `madvise` |
| 14 | 1.1% | `radiomics::features::matrices::zone_matrix` |
| 12 | 1.0% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 12 | 1.0% | `<deduplicated_symbol>` |
| 12 | 1.0% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 11 | 0.9% | `__psynch_cvsignal` |
| 10 | 0.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 10 | 0.8% | `__psynch_mutexdrop` |
| 9 | 0.7% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 9 | 0.7% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 8 | 0.6% | `__floattidf` |
| 7 | 0.6% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 6 | 0.5% | `radiomics::features::first_order_source::check_samples` |
| 6 | 0.5% | `_xzm_xzone_malloc_tiny` |
| 6 | 0.5% | `radiomics::features::glcm::glcm_statistics` |
| 6 | 0.5% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 5 | 0.4% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 678 | 53.9% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 575 | 45.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 484 | 38.5% | `radiomics::workflows::extraction::extract` |
| 484 | 38.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 472 | 37.5% | `radiomics::features::matrices::matrices` |
| 266 | 21.2% | `radiomics::workflows::engine::run_prepared_at` |
| 262 | 20.8% | `radiomics::workflows::roi::compute_roi` |
| 218 | 17.3% | `radiomics::features::texture::compute` |
| 170 | 13.5% | `radiomics::features::matrices::zone_matrix` |
| 153 | 12.2% | `radiomics::features::glcm::glcm_statistics` |
| 133 | 10.6% | `radiomics::imaging::preprocessing::prepare` |
| 126 | 10.0% | `radiomics::features::matrices::run_matrix` |
| 121 | 9.6% | `radiomics::features::matrices::neighborhood_matrix` |
| 90 | 7.2% | `radiomics::features::matrices::Runs::join_rows` |
| 87 | 6.9% | `radiomics::numerics::eigen::maximal_correlation` |
| 80 | 6.4% | `radiomics::workflows::roi::bounds::include` |
| 75 | 6.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 71 | 5.6% | `radiomics::workflows::engine::inputs::input_window` |
| 69 | 5.5% | `radiomics::features::cooccurrence::glcm_matrix` |
| 58 | 4.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 57 | 4.5% | `radiomics::imaging::preprocessing::lattice_bins` |
| 48 | 3.8% | `radiomics::imaging::preprocessing::gather` |
| 39 | 3.1% | `radiomics::features::glcm::mutual_information_bits` |
| 36 | 2.9% | `radiomics::imaging::preprocessing::gather_at` |
| 36 | 2.9% | `radiomics::features::matrices::Padded::new` |
| 33 | 2.6% | `radiomics::features::first_order::compute` |
| 33 | 2.6% | `radiomics::features::first_order::first_order_roi` |
| 33 | 2.6% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 32 | 2.5% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 29 | 2.3% | `radiomics::features::matrices::DenseCounts::into_matrix` |
