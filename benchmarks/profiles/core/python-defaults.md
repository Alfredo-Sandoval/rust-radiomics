# Profile: core/python-defaults

- Command: `../mlx-radiomics/.venv/bin/python benchmarks/python_roi.py ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd --seconds 6 --layout native`
- Measured library: `2cfa716`
- Checkout at sampling: `2cfa716`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:46 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 1408 (idle worker-thread samples excluded: 9802)

## Command output

```text
{"layout": "native", "shape": [134, 512, 512], "image_dtype": "int32", "mask_dtype": "uint8", "features": 93, "roi_voxels": 54639, "runs": 301, "median_ms": 10.833959000592586}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 539 | 38.3% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 117 | 8.3% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 100 | 7.1% | `radiomics::features::matrices::run_matrix` |
| 93 | 6.6% | `radiomics::features::matrices::Runs::join_rows` |
| 58 | 4.1% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 53 | 3.8% | `__psynch_mutexwait` |
| 52 | 3.7% | `radiomics::numerics::eigen::maximal_correlation` |
| 44 | 3.1% | `radiomics::features::cooccurrence::glcm_matrix` |
| 23 | 1.6% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 19 | 1.3% | `radiomics::features::matrices::Padded::new` |
| 16 | 1.1% | `_platform_memmove` |
| 16 | 1.1% | `__floattidf` |
| 15 | 1.1% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 14 | 1.0% | `log1p` |
| 12 | 0.9% | `<deduplicated_symbol>` |
| 12 | 0.9% | `radiomics::features::glcm::glcm_statistics` |
| 10 | 0.7% | `radiomics::features::glcm::mutual_information_bits` |
| 10 | 0.7% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 9 | 0.6% | `crossbeam_epoch::default::with_handle` |
| 8 | 0.6% | `radiomics::features::matrices::zone_matrix` |
| 8 | 0.6% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 7 | 0.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 7 | 0.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 7 | 0.5% | `radiomics::numerics::sparse_counts::marginals` |
| 6 | 0.4% | `radiomics::features::first_order::LevelCounts::add` |
| 6 | 0.4% | `_tlv_get_addr` |
| 6 | 0.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 6 | 0.4% | `crossbeam_deque::deque::Worker<T>::pop` |
| 5 | 0.4% | `rayon_core::join::join_context::_{{closure}}` |
| 5 | 0.4% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 576 | 40.9% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 480 | 34.1% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 399 | 28.3% | `radiomics::features::matrices::matrices` |
| 397 | 28.2% | `radiomics::workflows::extraction::extract` |
| 397 | 28.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 233 | 16.5% | `radiomics::workflows::engine::run_prepared_at` |
| 230 | 16.3% | `radiomics::workflows::roi::compute_roi` |
| 179 | 12.7% | `radiomics::features::texture::compute` |
| 141 | 10.0% | `radiomics::features::glcm::glcm_statistics` |
| 134 | 9.5% | `radiomics::features::matrices::zone_matrix` |
| 120 | 8.5% | `radiomics::features::matrices::neighborhood_matrix` |
| 108 | 7.7% | `radiomics::features::matrices::run_matrix` |
| 100 | 7.1% | `radiomics::imaging::preprocessing::prepare` |
| 93 | 6.6% | `radiomics::features::matrices::Runs::join_rows` |
| 66 | 4.7% | `radiomics::workflows::roi::bounds::include` |
| 64 | 4.5% | `radiomics::numerics::eigen::maximal_correlation` |
| 55 | 3.9% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 53 | 3.8% | `radiomics::features::cooccurrence::glcm_matrix` |
| 52 | 3.7% | `radiomics::workflows::engine::inputs::input_window` |
| 45 | 3.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 42 | 3.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 42 | 3.0% | `radiomics::imaging::preprocessing::gather` |
| 35 | 2.5% | `radiomics::imaging::preprocessing::gather_at` |
| 32 | 2.3% | `radiomics::features::first_order::compute` |
| 32 | 2.3% | `radiomics::features::first_order::first_order_roi` |
| 32 | 2.3% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 32 | 2.3% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 28 | 2.0% | `radiomics::features::matrices::Padded::new` |
| 19 | 1.3% | `radiomics::features::first_order::RegionKernel::finish` |
| 19 | 1.3% | `radiomics::features::glcm::mutual_information_bits` |
