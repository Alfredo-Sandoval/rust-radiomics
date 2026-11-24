# Profile: core/sparse-nifti-export

- Command: `target/bench-core/voxel-engine/after/benchmark_voxel_export`
- Measured library: `df67aec`
- Checkout at sampling: `df67aec` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:50 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2559 (idle worker-thread samples excluded: 29399)

## Command output

```text
{"median_ms":11.069833,"runs":587,"maps":3}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 331 | 12.9% | `miniz_oxide::deflate::core::compress_inner` |
| 244 | 9.5% | `linkat` |
| 241 | 9.4% | `__psynch_mutexwait` |
| 205 | 8.0% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 113 | 4.4% | `_platform_memmove` |
| 113 | 4.4% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 106 | 4.1% | `miniz_oxide::deflate::core::flush_block` |
| 104 | 4.1% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 100 | 3.9% | `__open` |
| 96 | 3.8% | `radiomics::features::first_order::LevelCounts::add` |
| 94 | 3.7% | `crc32fast::specialized::aarch64::State::update` |
| 77 | 3.0% | `__unlinkat` |
| 62 | 2.4% | `__rename` |
| 54 | 2.1% | `radiomics::features::first_order_source::check_samples` |
| 52 | 2.0% | `write` |
| 44 | 1.7% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 34 | 1.3% | `radiomics::features::first_order::RegionKernel::finish` |
| 31 | 1.2% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 29 | 1.1% | `close` |
| 29 | 1.1% | `radiomics::numerics::reduction::reported_mean` |
| 27 | 1.1% | `__psynch_mutexdrop` |
| 25 | 1.0% | `_xzm_xzone_malloc_tiny` |
| 21 | 0.8% | `__bzero` |
| 18 | 0.7% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 17 | 0.7% | `_pthread_cond_wait` |
| 15 | 0.6% | `__psynch_cvsignal` |
| 14 | 0.5% | `_tlv_get_addr` |
| 13 | 0.5% | `__getattrlist` |
| 12 | 0.5% | `_xzm_free_tc` |
| 12 | 0.5% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1682 | 65.7% | `radiomics::workflows::exporting::export` |
| 1657 | 64.8% | `radiomics::workflows::reporting::publish_report` |
| 1148 | 44.9% | `radiomics::workflows::extraction::extract` |
| 1148 | 44.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 927 | 36.2% | `radiomics::workflows::engine::run_prepared_at` |
| 725 | 28.3% | `radiomics::imaging::io::write_sparse_nifti_map` |
| 540 | 21.1% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 391 | 15.3% | `radiomics::workflows::publication::Publication::publish` |
| 258 | 10.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 232 | 9.1% | `radiomics::features::first_order::RegionKernel::finish` |
| 205 | 8.0% | `radiomics::workflows::engine::inputs::input_window` |
| 205 | 8.0% | `radiomics::workflows::roi::bounds::include` |
| 205 | 8.0% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 191 | 7.5% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 149 | 5.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 104 | 4.1% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 96 | 3.8% | `radiomics::features::first_order::LevelCounts::add` |
| 69 | 2.7% | `radiomics::workflows::publication::Publication::stage` |
| 54 | 2.1% | `radiomics::features::first_order_source::check_samples` |
| 40 | 1.6% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 39 | 1.5% | `radiomics::workflows::publication::Publication::create` |
| 37 | 1.4% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 31 | 1.2% | `radiomics::features::first_order::RegionKernel::new` |
| 29 | 1.1% | `radiomics::numerics::reduction::reported_mean` |
| 15 | 0.6% | `radiomics::imaging::preprocessing::prepare` |
| 14 | 0.5% | `radiomics::workflows::publication::resolve` |
| 11 | 0.4% | `radiomics::features::first_order::LevelCounts::finish` |
| 10 | 0.4% | `radiomics::contracts::json::json_safe` |
| 10 | 0.4% | `radiomics::features::first_order::SampleStatistics::new` |
| 9 | 0.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
