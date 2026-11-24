# Profile: voxel/gather-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 gather`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:16 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 3336 (idle worker-thread samples excluded: 0)

## Command output

```text
{"median_ms":3.041333,"repeats":583,"min_ms":2.194125,"max_ms":178.258916,"roi_voxels":54639,"maps":0,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1573 | 47.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 1411 | 42.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 97 | 2.9% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 55 | 1.6% | `mach_absolute_time` |
| 52 | 1.6% | `madvise` |
| 43 | 1.3% | `mach_vm_reclaim_try_cancel` |
| 30 | 0.9% | `_xzm_reclaim_mark_used` |
| 22 | 0.7% | `xzm_malloc_zone_try_free_default_main` |
| 18 | 0.5% | `_platform_memset` |
| 11 | 0.3% | `_xzm_segment_group_span_mark_smaller` |
| 7 | 0.2% | `_xzm_chunk_list_pop` |
| 5 | 0.1% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 4 | 0.1% | `xzm_segment_group_alloc_chunk` |
| 3 | 0.1% | `<deduplicated_symbol>` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.0% | `xzm_reclaim_mark_free_locked` |
| 1 | 0.0% | `_xzm_segment_group_segment_span_mark_allocated` |
| 1 | 0.0% | `_xzm_free_outlined` |
| 1 | 0.0% | `_xzm_segment_group_segment_span_free_coalesce` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1521 | 45.6% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 987 | 29.6% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
