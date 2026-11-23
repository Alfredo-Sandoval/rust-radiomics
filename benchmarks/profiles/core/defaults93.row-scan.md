# Profile: core/defaults93.row-scan

- Command: `target/bench-core/final/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile`
- Measured library: `3ab52d3`
- Checkout at sampling: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:23 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 1961 (idle worker-thread samples excluded: 14550)

## Command output

```text
{"median_ms":25.71875,"runs":129,"feature_count":93}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 283 | 14.4% | `radiomics::features::matrices::run_matrix` |
| 280 | 14.3% | `radiomics::features::matrices::matrices` |
| 219 | 11.2% | `radiomics::features::cooccurrence::glcm_matrix` |
| 219 | 11.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 192 | 9.8% | `radiomics::numerics::eigen::maximal_correlation` |
| 84 | 4.3% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 83 | 4.2% | `__floattidf` |
| 83 | 4.2% | `radiomics::features::glcm::mutual_information_bits` |
| 70 | 3.6% | `radiomics::numerics::sparse_counts::marginals` |
| 66 | 3.4% | `radiomics::features::aggregation::evaluate` |
| 47 | 2.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 24 | 1.2% | `<deduplicated_symbol>` |
| 22 | 1.1% | `_platform_memmove` |
| 22 | 1.1% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 17 | 0.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 17 | 0.9% | `log1p` |
| 16 | 0.8% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 15 | 0.8% | `radiomics::imaging::preprocessing::lattice_bins` |
| 14 | 0.7% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 14 | 0.7% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 13 | 0.7% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 10 | 0.5% | `__psynch_mutexwait` |
| 10 | 0.5% | `madvise` |
| 8 | 0.4% | `_xzm_malloc_tc` |
| 8 | 0.4% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 7 | 0.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 7 | 0.4% | `hypot` |
| 6 | 0.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 6 | 0.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 6 | 0.3% | `DYLD-STUB$$hypot` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1247 | 63.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1247 | 63.6% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 978 | 49.9% | `radiomics::features::matrices::matrices` |
| 868 | 44.3% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 587 | 29.9% | `radiomics::workflows::engine::run_prepared` |
| 584 | 29.8% | `radiomics::workflows::roi::compute_roi` |
| 577 | 29.4% | `radiomics::features::aggregation::evaluate` |
| 531 | 27.1% | `radiomics::features::texture::compute` |
| 293 | 14.9% | `radiomics::features::matrices::run_matrix` |
| 285 | 14.5% | `radiomics::numerics::eigen::maximal_correlation` |
| 245 | 12.5% | `radiomics::features::cooccurrence::glcm_matrix` |
| 233 | 11.9% | `radiomics::workflows::roi::bounds::include` |
| 219 | 11.2% | `radiomics::workflows::engine::inputs::input_window` |
| 110 | 5.6% | `radiomics::features::glcm::mutual_information_bits` |
| 72 | 3.7% | `radiomics::numerics::sparse_counts::marginals` |
| 58 | 3.0% | `radiomics::imaging::preprocessing::prepare` |
| 41 | 2.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 38 | 1.9% | `radiomics::features::first_order::compute` |
| 38 | 1.9% | `radiomics::features::first_order::first_order_roi` |
| 37 | 1.9% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 28 | 1.4% | `radiomics::features::matrices::DenseCounts::into_matrix` |
| 24 | 1.2% | `radiomics::features::matrices::Padded::new` |
| 19 | 1.0% | `radiomics::features::first_order::RegionKernel::finish` |
| 18 | 0.9% | `radiomics::imaging::preprocessing::lattice_bins` |
| 17 | 0.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 17 | 0.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 15 | 0.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 14 | 0.7% | `radiomics::workflows::roi::occupied_window` |
| 14 | 0.7% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 14 | 0.7% | `radiomics::imaging::preprocessing::gather` |
