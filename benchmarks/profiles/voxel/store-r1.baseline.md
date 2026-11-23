# Profile: voxel/store-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel_stages ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 store`
- Commit: `d80f742` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:51 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 2914 (idle worker-thread samples excluded: 0)

## Command output

```text
{"median_ms":2689.979875,"repeats":3,"min_ms":1049.396667,"max_ms":5655.693791,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2914 | 100.0% | `__bzero` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2914 | 100.0% | `radiomics::voxel::storage::VoxelMapStore::new` |
