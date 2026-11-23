# Profile: voxel/store-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 store`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:17 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 10218 (idle worker-thread samples excluded: 3994)

## Command output

```text
{"median_ms":612.998,"repeats":8,"min_ms":202.59712499999998,"max_ms":1056.922875,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 9986 | 97.7% | `__bzero` |
| 96 | 0.9% | `_kernelrpc_mach_vm_deallocate_trap` |
| 70 | 0.7% | `_<rayon::iter::for_each::ForEachConsumer<F> as rayon::iter::plumbing::Folder<T>>::consume_iter` |
| 47 | 0.5% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 8 | 0.1% | `__psynch_mutexwait` |
| 6 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 1 | 0.0% | `radiomics::voxel::storage::VoxelMapStore::append` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.0% | `_xzm_segment_group_init_segment` |
| 1 | 0.0% | `_pthread_cond_wait` |
| 1 | 0.0% | `_xzm_segment_table_allocated_at` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 627 | 6.1% | `radiomics::voxel::storage::VoxelMapStore::new` |
| 25 | 0.2% | `radiomics::voxel::storage::VoxelMapStore::append` |
