# Profile: core/python-defaults-bool

- Command: `../mlx-radiomics/.venv/bin/python benchmarks/python_roi.py ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd --seconds 6 --layout bool-mask`
- Measured library: `2cfa716`
- Checkout at sampling: `2cfa716`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:47 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 1978 (idle worker-thread samples excluded: 10961)

## Command output

```text
{"layout": "bool-mask", "shape": [134, 512, 512], "image_dtype": "int32", "mask_dtype": "bool", "features": 93, "roi_voxels": 54639, "runs": 285, "median_ms": 12.77000000118278}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 979 | 49.5% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 128 | 6.5% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 114 | 5.8% | `__psynch_mutexwait` |
| 114 | 5.8% | `radiomics::features::matrices::run_matrix` |
| 92 | 4.7% | `radiomics::features::matrices::Runs::join_rows` |
| 82 | 4.1% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 60 | 3.0% | `radiomics::features::cooccurrence::glcm_matrix` |
| 59 | 3.0% | `radiomics::numerics::eigen::maximal_correlation` |
| 24 | 1.2% | `<deduplicated_symbol>` |
| 21 | 1.1% | `radiomics::features::matrices::Padded::new` |
| 17 | 0.9% | `_platform_memmove` |
| 17 | 0.9% | `log1p` |
| 16 | 0.8% | `radiomics::features::glcm::mutual_information_bits` |
| 16 | 0.8% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 15 | 0.8% | `radiomics::features::matrices::zone_matrix` |
| 14 | 0.7% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 13 | 0.7% | `__psynch_cvsignal` |
| 11 | 0.6% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 10 | 0.5% | `radiomics::features::first_order_source::check_samples` |
| 10 | 0.5% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 10 | 0.5% | `__floattidf` |
| 9 | 0.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 8 | 0.4% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 6 | 0.3% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 6 | 0.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 6 | 0.3% | `madvise` |
| 6 | 0.3% | `rayon_core::join::join_context::_{{closure}}` |
| 6 | 0.3% | `radiomics::features::glcm::glcm_statistics` |
| 5 | 0.3% | `radiomics::features::first_order::LevelCounts::add` |
| 5 | 0.3% | `core::slice::sort::unstable::quicksort::quicksort` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 601 | 30.4% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 508 | 25.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 502 | 25.4% | `radiomics::workflows::extraction::extract` |
| 502 | 25.4% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 404 | 20.4% | `radiomics::features::matrices::matrices` |
| 248 | 12.5% | `radiomics::workflows::engine::run_prepared_at` |
| 245 | 12.4% | `radiomics::workflows::roi::compute_roi` |
| 193 | 9.8% | `radiomics::features::texture::compute` |
| 152 | 7.7% | `radiomics::features::matrices::zone_matrix` |
| 132 | 6.7% | `radiomics::workflows::roi::bounds::include` |
| 128 | 6.5% | `radiomics::features::glcm::glcm_statistics` |
| 123 | 6.2% | `radiomics::workflows::engine::inputs::input_window` |
| 121 | 6.1% | `radiomics::features::matrices::run_matrix` |
| 115 | 5.8% | `radiomics::features::matrices::neighborhood_matrix` |
| 109 | 5.5% | `radiomics::imaging::preprocessing::prepare` |
| 92 | 4.7% | `radiomics::features::matrices::Runs::join_rows` |
| 82 | 4.1% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 78 | 3.9% | `radiomics::features::cooccurrence::glcm_matrix` |
| 71 | 3.6% | `radiomics::numerics::eigen::maximal_correlation` |
| 70 | 3.5% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 61 | 3.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 39 | 2.0% | `radiomics::features::first_order::compute` |
| 39 | 2.0% | `radiomics::features::first_order::first_order_roi` |
| 38 | 1.9% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 38 | 1.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 34 | 1.7% | `radiomics::imaging::preprocessing::gather` |
| 27 | 1.4% | `radiomics::features::matrices::Padded::new` |
| 27 | 1.4% | `radiomics::features::glcm::mutual_information_bits` |
| 25 | 1.3% | `radiomics::features::matrices::DenseCounts::into_matrix` |
| 24 | 1.2% | `radiomics::features::first_order::RegionKernel::finish` |
