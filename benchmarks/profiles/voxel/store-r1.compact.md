# Profile: voxel/store-r1.compact

- Command: `target/bench-voxel/compact_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 store`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:03 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 2523 (idle worker-thread samples excluded: 0)

## Command output

```text
{"median_ms":2377.070834,"repeats":3,"min_ms":1701.375125,"max_ms":2677.057958,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2337 | 92.6% | `__bzero` |
| 134 | 5.3% | `radiomics::voxel::storage::VoxelMapStore::append` |
| 50 | 2.0% | `_kernelrpc_mach_vm_deallocate_trap` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.0% | `_xzm_reclaim_mark_free` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2338 | 92.7% | `radiomics::voxel::storage::VoxelMapStore::new` |
| 134 | 5.3% | `radiomics::voxel::storage::VoxelMapStore::append` |
