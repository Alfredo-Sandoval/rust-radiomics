# Profile: voxel/gather-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel_stages ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 gather`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:48 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 3356 (idle worker-thread samples excluded: 0)

## Command output

```text
{"median_ms":22.306917,"repeats":195,"min_ms":7.489459,"max_ms":146.18675000000002,"roi_voxels":54639,"maps":0,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2894 | 86.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 369 | 11.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 41 | 1.2% | `madvise` |
| 27 | 0.8% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 13 | 0.4% | `_platform_memset` |
| 3 | 0.1% | `xzm_malloc_zone_malloc_type_malloc_main` |
| 3 | 0.1% | `mach_absolute_time` |
| 2 | 0.1% | `_xzm_segment_group_find_and_allocate_chunk` |
| 2 | 0.1% | `_xzm_segment_group_span_mark_smaller` |
| 1 | 0.0% | `_xzm_segment_group_segment_span_free_coalesce` |
| 1 | 0.0% | `benchmark_voxel::main::_{{closure}}` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2951 | 87.9% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 234 | 7.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
