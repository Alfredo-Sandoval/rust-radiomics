# Profile: core/defaults93

- Command: `target/bench-core/mask/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile`
- Measured library: `4f42922`
- Checkout at sampling: `6f79fbd` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:33 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 5116 (idle worker-thread samples excluded: 16962)

## Command output

```text
{"median_ms":6.278604,"runs":538,"feature_count":93}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 623 | 12.2% | `radiomics::features::matrices::matrices` |
| 617 | 12.1% | `radiomics::features::matrices::run_matrix` |
| 488 | 9.5% | `radiomics::numerics::eigen::maximal_correlation` |
| 476 | 9.3% | `__psynch_mutexwait` |
| 426 | 8.3% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 387 | 7.6% | `radiomics::features::cooccurrence::glcm_matrix` |
| 385 | 7.5% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 214 | 4.2% | `radiomics::features::aggregation::evaluate` |
| 129 | 2.5% | `radiomics::numerics::sparse_counts::marginals` |
| 112 | 2.2% | `__floattidf` |
| 92 | 1.8% | `<deduplicated_symbol>` |
| 92 | 1.8% | `radiomics::features::glcm::mutual_information_bits` |
| 62 | 1.2% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 61 | 1.2% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 45 | 0.9% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 43 | 0.8% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 41 | 0.8% | `__psynch_mutexdrop` |
| 40 | 0.8% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 37 | 0.7% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 34 | 0.7% | `core::iter::traits::iterator::Iterator::try_fold` |
| 34 | 0.7% | `_platform_memmove` |
| 33 | 0.6% | `crossbeam_epoch::default::with_handle` |
| 33 | 0.6% | `DYLD-STUB$$hypot` |
| 32 | 0.6% | `__psynch_cvsignal` |
| 31 | 0.6% | `radiomics::imaging::preprocessing::lattice_bins` |
| 30 | 0.6% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 28 | 0.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 28 | 0.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 28 | 0.5% | `hypot` |
| 24 | 0.5% | `madvise` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1588 | 31.0% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1586 | 31.0% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 1339 | 26.2% | `radiomics::features::matrices::matrices` |
| 1269 | 24.8% | `radiomics::features::aggregation::evaluate` |
| 1159 | 22.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 834 | 16.3% | `radiomics::workflows::engine::run_prepared` |
| 818 | 16.0% | `radiomics::workflows::roi::compute_roi` |
| 689 | 13.5% | `radiomics::features::texture::compute` |
| 662 | 12.9% | `radiomics::numerics::eigen::maximal_correlation` |
| 638 | 12.5% | `radiomics::features::matrices::run_matrix` |
| 441 | 8.6% | `radiomics::features::cooccurrence::glcm_matrix` |
| 210 | 4.1% | `radiomics::imaging::preprocessing::prepare` |
| 140 | 2.7% | `radiomics::numerics::sparse_counts::marginals` |
| 130 | 2.5% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 127 | 2.5% | `radiomics::features::glcm::mutual_information_bits` |
| 110 | 2.2% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 104 | 2.0% | `radiomics::workflows::roi::bounds::include` |
| 95 | 1.9% | `radiomics::features::first_order::compute` |
| 95 | 1.9% | `radiomics::features::first_order::first_order_roi` |
| 94 | 1.8% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 80 | 1.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 76 | 1.5% | `radiomics::workflows::engine::inputs::input_window` |
| 69 | 1.3% | `radiomics::features::matrices::DenseCounts::into_matrix` |
| 66 | 1.3% | `radiomics::imaging::preprocessing::gather` |
| 60 | 1.2% | `radiomics::features::first_order::RegionKernel::finish` |
| 47 | 0.9% | `radiomics::features::distributions::size_statistics` |
| 40 | 0.8% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 38 | 0.7% | `radiomics::imaging::preprocessing::lattice_bins` |
| 34 | 0.7% | `radiomics::imaging::io::HostArray::crop` |
| 33 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
