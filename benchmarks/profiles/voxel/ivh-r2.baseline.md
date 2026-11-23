# Profile: voxel/ivh-r2.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 ivh`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:56 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3167 (idle worker-thread samples excluded: 15454)

## Command output

```text
{"median_ms":2665.4071249999997,"samples_ms":[1784.913792,2665.4071249999997,2864.547041],"roi_voxels":54639,"maps":7,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1209 | 38.2% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 852 | 26.9% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 129 | 4.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 117 | 3.7% | `core::slice::sort::select::partition_at_index` |
| 105 | 3.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 99 | 3.1% | `radiomics::features::ivh::ivh_statistics` |
| 97 | 3.1% | `<deduplicated_symbol>` |
| 96 | 3.0% | `_platform_memmove` |
| 69 | 2.2% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 57 | 1.8% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 52 | 1.6% | `_xzm_xzone_malloc_tiny` |
| 36 | 1.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 27 | 0.9% | `mach_absolute_time` |
| 24 | 0.8% | `_pthread_cond_wait` |
| 20 | 0.6% | `_xzm_free` |
| 16 | 0.5% | `_xzm_malloc` |
| 16 | 0.5% | `core::slice::sort::shared::pivot::median3_rec` |
| 15 | 0.5% | `_xzm_free_main` |
| 15 | 0.5% | `__psynch_mutexwait` |
| 13 | 0.4% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 11 | 0.3% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 10 | 0.3% | `radiomics::features::ivh::prepare_ivh` |
| 10 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 8 | 0.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 7 | 0.2% | `default_zone_realloc` |
| 7 | 0.2% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 6 | 0.2% | `ndarray::iterators::Iter<A,D>::new` |
| 6 | 0.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 5 | 0.2% | `__bzero` |
| 5 | 0.2% | `_free` |
| 4 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 3 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 3 | 0.1% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 2 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 2 | 0.1% | `_malloc_zone_malloc` |
| 2 | 0.1% | `xzm_malloc_zone_size` |
| 2 | 0.1% | `_RNvCsGIExRX8pES_7___rustc14___rust_dealloc` |
| 1 | 0.0% | `DYLD-STUB$$mach_absolute_time` |
| 1 | 0.0% | `_RNvMs_NtNtCsaLOjE9VYtxK_3std6thread6threadNtB4_6Thread3new` |
| 1 | 0.0% | `mach_vm_reclaim_query_state` |
| 1 | 0.0% | `DYLD-STUB$$free` |
| 1 | 0.0% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.0% | `core::hash::BuildHasher::hash_one` |
| 1 | 0.0% | `_realloc` |
| 1 | 0.0% | `crossbeam_deque::deque::Worker<T>::pop` |
| 1 | 0.0% | `core::ptr::drop_in_place<core::iter::adapters::zip::Zip<ndarray::iterators::Iter<f64,ndarray::dimension::dim::Dim<ndarray::dimension::dynindeximpl::IxDynImpl>>,` |
| 1 | 0.0% | `core::iter::adapters::try_process` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2670 | 84.3% | `radiomics::workflows::extraction::extract` |
| 2670 | 84.3% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2361 | 74.6% | `radiomics::workflows::engine::run_prepared` |
| 2088 | 65.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 472 | 14.9% | `radiomics::features::ivh::ivh_statistics` |
| 309 | 9.8% | `radiomics::imaging::preprocessing::prepare` |
| 246 | 7.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 142 | 4.5% | `radiomics::features::ivh::prepare_ivh` |
| 139 | 4.4% | `radiomics::imaging::preprocessing::gather` |
| 87 | 2.7% | `radiomics::imaging::preprocessing::normalize_values` |
| 63 | 2.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 57 | 1.8% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 48 | 1.5% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 10 | 0.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 9 | 0.3% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
| 6 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
