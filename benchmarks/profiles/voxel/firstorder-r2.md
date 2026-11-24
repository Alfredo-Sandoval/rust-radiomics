# Profile: voxel/firstorder-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 firstorder`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:20 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2711 (idle worker-thread samples excluded: 35376)

## Command output

```text
{"median_ms":2886.149875,"repeats":3,"min_ms":2367.8945000000003,"max_ms":3492.439666,"roi_voxels":54639,"maps":21,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 836 | 30.8% | `__bzero` |
| 761 | 28.1% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 255 | 9.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 244 | 9.0% | `core::iter::traits::iterator::Iterator::try_fold` |
| 147 | 5.4% | `_platform_memmove` |
| 95 | 3.5% | `<deduplicated_symbol>` |
| 81 | 3.0% | `core::slice::sort::unstable::quicksort::quicksort` |
| 58 | 2.1% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 41 | 1.5% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 32 | 1.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 32 | 1.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 30 | 1.1% | `radiomics::features::first_order::LevelCounts::add` |
| 28 | 1.0% | `radiomics::features::first_order::RegionKernel::finish` |
| 18 | 0.7% | `radiomics::features::first_order::LevelCounts::finish` |
| 13 | 0.5% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 7 | 0.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 7 | 0.3% | `_xzm_free_main` |
| 5 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 3 | 0.1% | `core::slice::sort::shared::pivot::median3_rec` |
| 2 | 0.1% | `log2` |
| 2 | 0.1% | `__udivti3` |
| 2 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 2 | 0.1% | `_xzm_free` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `_xzm_chunk_list_pop` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 1 | 0.0% | `__psynch_mutexwait` |
| 1 | 0.0% | `DYLD-STUB$$log2` |
| 1 | 0.0% | `radiomics::numerics::reduction::reported_mean` |
| 1 | 0.0% | `_pthread_cond_wait` |
| 1 | 0.0% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 1 | 0.0% | `_xzm_xzone_malloc_tiny` |
| 1 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2475 | 91.3% | `radiomics::workflows::extraction::extract` |
| 2475 | 91.3% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1286 | 47.4% | `radiomics::workflows::engine::run_prepared` |
| 1284 | 47.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1189 | 43.9% | `radiomics::imaging::preprocessing::prepare` |
| 992 | 36.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 844 | 31.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 275 | 10.1% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 207 | 7.6% | `radiomics::features::first_order::RegionKernel::finish` |
| 197 | 7.3% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 154 | 5.7% | `radiomics::imaging::preprocessing::gather` |
| 145 | 5.3% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 67 | 2.5% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 60 | 2.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 30 | 1.1% | `radiomics::features::first_order::LevelCounts::add` |
| 29 | 1.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 27 | 1.0% | `radiomics::imaging::preprocessing::normalize_values` |
| 5 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 4 | 0.1% | `radiomics::numerics::count_statistics::bin_entropy` |
| 2 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 2 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 2 | 0.1% | `radiomics::features::first_order::RegionKernel::new` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::numerics::precision::QuantileRank::reported_pair` |
| 1 | 0.0% | `radiomics::numerics::reduction::reported_mean` |
