# Profile: core/sparse-nifti-export.baseline

- Command: `target/bench-core/voxel-engine/before/benchmark_voxel_export`
- Measured library: `aa6355a`
- Checkout at sampling: `2c32523` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:49 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2693 (idle worker-thread samples excluded: 44541)

## Command output

```text
{"median_ms":53.561811999999996,"runs":118,"maps":3}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1127 | 41.8% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 248 | 9.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 217 | 8.1% | `radiomics::imaging::io::nifti_map_bytes` |
| 217 | 8.1% | `<deduplicated_symbol>` |
| 96 | 3.6% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 81 | 3.0% | `miniz_oxide::deflate::core::compress_inner` |
| 70 | 2.6% | `_platform_memmove` |
| 55 | 2.0% | `linkat` |
| 53 | 2.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 51 | 1.9% | `__psynch_mutexwait` |
| 49 | 1.8% | `core::iter::traits::iterator::Iterator::try_fold` |
| 36 | 1.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 34 | 1.3% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 33 | 1.2% | `crc32fast::specialized::aarch64::State::update` |
| 32 | 1.2% | `__open` |
| 32 | 1.2% | `__bzero` |
| 23 | 0.9% | `radiomics::features::first_order::RegionKernel::finish` |
| 22 | 0.8% | `miniz_oxide::deflate::core::flush_block` |
| 18 | 0.7% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 17 | 0.6% | `__unlinkat` |
| 16 | 0.6% | `__rename` |
| 15 | 0.6% | `radiomics::features::first_order::LevelCounts::add` |
| 12 | 0.4% | `close` |
| 11 | 0.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 10 | 0.4% | `write` |
| 8 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 8 | 0.3% | `crossbeam_epoch::default::with_handle` |
| 7 | 0.3% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 6 | 0.2% | `mkdir` |
| 6 | 0.2% | `__getattrlist` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2347 | 87.2% | `radiomics::workflows::exporting::export` |
| 2340 | 86.9% | `radiomics::workflows::reporting::publish_report` |
| 2212 | 82.1% | `radiomics::workflows::extraction::extract` |
| 2212 | 82.1% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1473 | 54.7% | `radiomics::workflows::engine::run_prepared` |
| 1293 | 48.0% | `radiomics::imaging::io::write_feature_map` |
| 739 | 27.4% | `radiomics::imaging::preprocessing::prepare` |
| 383 | 14.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 356 | 13.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 322 | 12.0% | `radiomics::imaging::preprocessing::gather` |
| 218 | 8.1% | `radiomics::imaging::io::nifti_map_bytes` |
| 151 | 5.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 109 | 4.0% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 90 | 3.3% | `radiomics::workflows::publication::Publication::publish` |
| 57 | 2.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 49 | 1.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 39 | 1.4% | `radiomics::features::first_order::RegionKernel::finish` |
| 20 | 0.7% | `radiomics::imaging::io::HostArray::eq_label` |
| 19 | 0.7% | `radiomics::workflows::publication::Publication::stage` |
| 16 | 0.6% | `radiomics::imaging::preprocessing::normalize_values` |
| 15 | 0.6% | `radiomics::features::first_order::LevelCounts::add` |
| 14 | 0.5% | `radiomics::voxel::storage::VoxelMapStore::new` |
| 11 | 0.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 10 | 0.4% | `radiomics::workflows::publication::Publication::create` |
| 6 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 6 | 0.2% | `radiomics::workflows::publication::resolve` |
| 6 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 5 | 0.2% | `radiomics::workflows::publication::reject_output_aliases` |
| 5 | 0.2% | `radiomics::contracts::json::json_safe` |
| 5 | 0.2% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
