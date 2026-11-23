# Profile: core/defaults93.baseline

- Command: `target/bench-core/baseline/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile`
- Measured library: `17d817c`
- Checkout at sampling: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:05 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3252 (idle worker-thread samples excluded: 0)

## Command output

```text
{"median_ms":191.07135449999998,"runs":32,"feature_count":93}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1747 | 53.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 715 | 22.0% | `radiomics::numerics::eigen::jacobi` |
| 260 | 8.0% | `radiomics::workflows::roi::bounds::include` |
| 180 | 5.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 68 | 2.1% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 59 | 1.8% | `radiomics::numerics::eigen::maximal_correlation` |
| 26 | 0.8% | `radiomics::features::glcm::glcm_statistics` |
| 25 | 0.8% | `radiomics::numerics::sparse_counts::marginals` |
| 20 | 0.6% | `__floattidf` |
| 16 | 0.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 15 | 0.5% | `radiomics::features::glcm::mutual_information_bits` |
| 15 | 0.5% | `<deduplicated_symbol>` |
| 10 | 0.3% | `_platform_memmove` |
| 10 | 0.3% | `core::slice::sort::unstable::quicksort::quicksort` |
| 9 | 0.3% | `radiomics::imaging::preprocessing::lattice_bins` |
| 8 | 0.2% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 6 | 0.2% | `DYLD-STUB$$hypot` |
| 6 | 0.2% | `hypot` |
| 6 | 0.2% | `log1p` |
| 4 | 0.1% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 4 | 0.1% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 3 | 0.1% | `_xzm_xzone_malloc_tiny` |
| 3 | 0.1% | `_malloc_zone_realloc` |
| 3 | 0.1% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 3 | 0.1% | `radiomics::features::first_order::LevelCounts::add` |
| 3 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 2 | 0.1% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 2 | 0.1% | `madvise` |
| 2 | 0.1% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 2 | 0.1% | `_xzm_free_tc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3250 | 99.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2956 | 90.9% | `radiomics::workflows::engine::run_prepared` |
| 2955 | 90.9% | `radiomics::workflows::roi::compute_roi` |
| 2927 | 90.0% | `radiomics::features::texture::compute` |
| 2880 | 88.6% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 905 | 27.8% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 889 | 27.3% | `radiomics::features::glcm::glcm_statistics` |
| 802 | 24.7% | `radiomics::numerics::eigen::maximal_correlation` |
| 727 | 22.4% | `radiomics::numerics::eigen::jacobi` |
| 260 | 8.0% | `radiomics::workflows::engine::inputs::input_window` |
| 260 | 8.0% | `radiomics::workflows::roi::bounds::include` |
| 30 | 0.9% | `radiomics::numerics::sparse_counts::marginals` |
| 28 | 0.9% | `radiomics::imaging::preprocessing::prepare` |
| 23 | 0.7% | `radiomics::features::first_order::compute` |
| 23 | 0.7% | `radiomics::features::first_order::first_order_roi` |
| 23 | 0.7% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 20 | 0.6% | `radiomics::features::glcm::mutual_information_bits` |
| 16 | 0.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 16 | 0.5% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 14 | 0.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 12 | 0.4% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 10 | 0.3% | `radiomics::features::distributions::size_statistics` |
| 9 | 0.3% | `radiomics::imaging::preprocessing::lattice_bins` |
| 9 | 0.3% | `radiomics::imaging::preprocessing::gather` |
| 6 | 0.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 5 | 0.2% | `radiomics::numerics::sparse_counts::entropy` |
| 5 | 0.2% | `radiomics::imaging::io::HostArray::crop` |
| 4 | 0.1% | `radiomics::workflows::roi::occupied_window` |
| 3 | 0.1% | `radiomics::features::first_order::LevelCounts::add` |
| 2 | 0.1% | `radiomics::features::texture::compute::_{{closure}}` |
