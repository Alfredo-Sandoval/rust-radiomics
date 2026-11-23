# Profile: voxel/gather-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 gather`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:17 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 3297 (idle worker-thread samples excluded: 0)

## Command output

```text
{"median_ms":16.5921875,"repeats":124,"min_ms":10.905541,"max_ms":256.07658399999997,"roi_voxels":54639,"maps":0,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1677 | 50.9% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 1395 | 42.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 135 | 4.1% | `madvise` |
| 45 | 1.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 10 | 0.3% | `_platform_memset` |
| 10 | 0.3% | `_xzm_free_main` |
| 8 | 0.2% | `_xzm_segment_group_span_mark_smaller` |
| 7 | 0.2% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 7 | 0.2% | `_xzm_reclaim_mark_used` |
| 1 | 0.0% | `xzm_malloc_zone_try_free_default_main` |
| 1 | 0.0% | `_malloc_zone_malloc` |
| 1 | 0.0% | `_xzm_segment_group_segment_span_free_coalesce` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1555 | 47.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 833 | 25.3% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
