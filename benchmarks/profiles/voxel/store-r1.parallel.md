# Profile: voxel/store-r1.parallel

- Command: `target/bench-voxel/draft/benchmark_store ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 store`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:06 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 9296 (idle worker-thread samples excluded: 5144)

## Command output

```text
{"median_ms":429.546208,"repeats":7,"min_ms":203.440958,"max_ms":1035.160542,"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 8961 | 96.4% | `__bzero` |
| 116 | 1.2% | `_kernelrpc_mach_vm_deallocate_trap` |
| 82 | 0.9% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 57 | 0.6% | `_<rayon::iter::for_each::ForEachConsumer<F> as rayon::iter::plumbing::Folder<T>>::consume_iter` |
| 39 | 0.4% | `__psynch_mutexwait` |
| 8 | 0.1% | `mach_vm_reclaim_query_state` |
| 7 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 5 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 5 | 0.1% | `crossbeam_deque::deque::Worker<T>::pop` |
| 4 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 4 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 3 | 0.0% | `_pthread_cond_wait` |
| 2 | 0.0% | `xzm_segment_group_alloc_chunk` |
| 1 | 0.0% | `benchmark_store::storage::VoxelMapStore::append` |
| 1 | 0.0% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc18___rdl_alloc_zeroed` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
