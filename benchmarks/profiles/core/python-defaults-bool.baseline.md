# Profile: core/python-defaults-bool.baseline

- Command: `../mlx-radiomics/.venv/bin/python benchmarks/python_roi.py ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd --seconds 6 --layout bool-mask`
- Measured library: `c23929c`
- Checkout at sampling: `2cfa716`
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 05:46 UTC
- Sampling: `sample`, 3 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 1743 (idle worker-thread samples excluded: 13400)

## Command output

```text
{"layout": "bool-mask", "shape": [134, 512, 512], "image_dtype": "int32", "mask_dtype": "bool", "features": 93, "roi_voxels": 54639, "runs": 239, "median_ms": 16.29595900158165}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 477 | 27.4% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 248 | 14.2% | `__psynch_mutexwait` |
| 196 | 11.2% | `_platform_memmove` |
| 126 | 7.2% | `radiomics::features::matrices::run_matrix` |
| 120 | 6.9% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 112 | 6.4% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 80 | 4.6% | `radiomics::features::matrices::Runs::join_rows` |
| 50 | 2.9% | `radiomics::features::cooccurrence::glcm_matrix` |
| 44 | 2.5% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 35 | 2.0% | `_aligned_contig_cast_bool_to_ubyte` |
| 25 | 1.4% | `radiomics::numerics::eigen::maximal_correlation` |
| 12 | 0.7% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 12 | 0.7% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 11 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 11 | 0.6% | `__psynch_cvsignal` |
| 11 | 0.6% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 10 | 0.6% | `radiomics::features::matrices::Padded::new` |
| 10 | 0.6% | `radiomics::features::matrices::zone_matrix` |
| 10 | 0.6% | `log1p` |
| 7 | 0.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 7 | 0.4% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 6 | 0.3% | `radiomics::features::first_order::LevelCounts::add` |
| 6 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 5 | 0.3% | `radiomics::features::glcm::glcm_statistics` |
| 5 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 5 | 0.3% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 4 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 4 | 0.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 4 | 0.2% | `<deduplicated_symbol>` |
| 4 | 0.2% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 528 | 30.3% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 517 | 29.7% | `radiomics::workflows::extraction::extract` |
| 517 | 29.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 482 | 27.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 395 | 22.7% | `radiomics::features::matrices::matrices` |
| 232 | 13.3% | `radiomics::workflows::engine::run_prepared_at` |
| 228 | 13.1% | `radiomics::workflows::roi::compute_roi` |
| 184 | 10.6% | `radiomics::features::texture::compute` |
| 154 | 8.8% | `radiomics::workflows::roi::bounds::include` |
| 145 | 8.3% | `radiomics::workflows::engine::inputs::input_window` |
| 139 | 8.0% | `radiomics::features::matrices::zone_matrix` |
| 137 | 7.9% | `radiomics::imaging::preprocessing::prepare` |
| 134 | 7.7% | `radiomics::features::matrices::run_matrix` |
| 112 | 6.4% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 104 | 6.0% | `radiomics::features::matrices::neighborhood_matrix` |
| 80 | 4.6% | `radiomics::features::matrices::Runs::join_rows` |
| 77 | 4.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 67 | 3.8% | `radiomics::features::glcm::glcm_statistics` |
| 61 | 3.5% | `radiomics::features::cooccurrence::glcm_matrix` |
| 60 | 3.4% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 58 | 3.3% | `radiomics::imaging::preprocessing::lattice_bins` |
| 51 | 2.9% | `radiomics::imaging::preprocessing::gather` |
| 36 | 2.1% | `radiomics::numerics::eigen::maximal_correlation` |
| 34 | 2.0% | `radiomics::features::first_order::compute` |
| 34 | 2.0% | `radiomics::features::first_order::first_order_roi` |
| 34 | 2.0% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 34 | 2.0% | `radiomics::imaging::preprocessing::gather_at` |
| 30 | 1.7% | `radiomics::imaging::preprocessing::true_positions` |
| 17 | 1.0% | `radiomics::features::first_order::RegionKernel::finish` |
| 17 | 1.0% | `radiomics::features::matrices::DenseCounts::into_matrix` |
