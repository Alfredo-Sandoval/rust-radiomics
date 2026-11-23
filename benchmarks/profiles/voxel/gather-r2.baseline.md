# Profile: voxel/gather-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel_stages ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 gather`
- Commit: `38ac856` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:52 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 2545 (idle worker-thread samples excluded: 0)

## Command output

```text
{"median_ms":47.9804585,"repeats":94,"min_ms":37.253625,"max_ms":113.322042,"roi_voxels":54639,"maps":0,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2098 | 82.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 393 | 15.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 32 | 1.3% | `madvise` |
| 11 | 0.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 5 | 0.2% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 3 | 0.1% | `_platform_memset` |
| 2 | 0.1% | `_xzm_segment_group_span_mark_smaller` |
| 1 | 0.0% | `xzm_segment_group_free_chunk` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2133 | 83.8% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 180 | 7.1% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
