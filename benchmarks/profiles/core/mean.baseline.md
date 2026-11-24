# Profile: core/mean.baseline

- Command: `target/bench-core/baseline/benchmark_roi ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd profile firstorder_Mean`
- Measured library: `17d817c`
- Checkout at sampling: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:18 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3296 (idle worker-thread samples excluded: 0)

## Command output

```text
{"median_ms":20.2335835,"runs":244,"feature_count":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3002 | 91.1% | `radiomics::workflows::roi::bounds::include` |
| 143 | 4.3% | `<deduplicated_symbol>` |
| 38 | 1.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 26 | 0.8% | `radiomics::features::first_order::RegionKernel::finish` |
| 20 | 0.6% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 19 | 0.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 15 | 0.5% | `_platform_memmove` |
| 10 | 0.3% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 8 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 5 | 0.2% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 2 | 0.1% | `radiomics::workflows::engine::inputs::input_window` |
| 1 | 0.0% | `_malloc_zone_realloc` |
| 1 | 0.0% | `xzm_segment_group_free_chunk` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 1 | 0.0% | `xzm_malloc_zone_size` |
| 1 | 0.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 1 | 0.0% | `_xzm_free_tc` |
| 1 | 0.0% | `radiomics::workflows::engine::run_prepared` |
| 1 | 0.0% | `_xzm_segment_group_span_mark_smaller` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3296 | 100.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 3004 | 91.1% | `radiomics::workflows::engine::inputs::input_window` |
| 3002 | 91.1% | `radiomics::workflows::roi::bounds::include` |
| 154 | 4.7% | `radiomics::imaging::preprocessing::prepare` |
| 142 | 4.3% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 121 | 3.7% | `radiomics::imaging::preprocessing::gather` |
| 104 | 3.2% | `radiomics::workflows::engine::run_prepared` |
| 103 | 3.1% | `radiomics::workflows::roi::compute_roi` |
| 101 | 3.1% | `radiomics::features::first_order::compute` |
| 101 | 3.1% | `radiomics::features::first_order::first_order_roi` |
| 100 | 3.0% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 54 | 1.6% | `radiomics::features::first_order::RegionKernel::finish` |
| 46 | 1.4% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 34 | 1.0% | `radiomics::imaging::io::HostArray::crop` |
| 20 | 0.6% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 12 | 0.4% | `radiomics::imaging::preprocessing::normalize_values` |
| 12 | 0.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 8 | 0.2% | `radiomics::numerics::reduction::reported_mean` |
| 7 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::features::first_order::RegionKernel::new` |
