# Profile: voxel/ivh-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 ivh`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:18 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2239 (idle worker-thread samples excluded: 31296)

## Command output

```text
{"median_ms":514.0656045000001,"repeats":10,"min_ms":435.01462499999997,"max_ms":630.147708,"roi_voxels":54639,"maps":7,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 444 | 19.8% | `<deduplicated_symbol>` |
| 348 | 15.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 234 | 10.5% | `radiomics::features::ivh::prepare_ivh` |
| 114 | 5.1% | `__psynch_mutexwait` |
| 113 | 5.0% | `core::iter::traits::iterator::Iterator::try_fold` |
| 110 | 4.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 101 | 4.5% | `__bzero` |
| 94 | 4.2% | `_platform_memmove` |
| 88 | 3.9% | `core::slice::sort::select::partition_at_index` |
| 78 | 3.5% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 68 | 3.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 60 | 2.7% | `mach_absolute_time` |
| 59 | 2.6% | `_xzm_xzone_malloc_tiny` |
| 54 | 2.4% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 45 | 2.0% | `radiomics::features::ivh::ivh_statistics` |
| 34 | 1.5% | `_xzm_free_main` |
| 26 | 1.2% | `_xzm_free` |
| 16 | 0.7% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 15 | 0.7% | `_xzm_malloc` |
| 15 | 0.7% | `xzm_malloc_zone_size` |
| 14 | 0.6% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 13 | 0.6% | `xzm_malloc_zone_malloc_type_realloc` |
| 11 | 0.5% | `_free` |
| 8 | 0.4% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 7 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 6 | 0.3% | `_malloc_zone_malloc` |
| 6 | 0.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 5 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 5 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 4 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 4 | 0.2% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 4 | 0.2% | `_realloc` |
| 4 | 0.2% | `_malloc_zone_realloc` |
| 4 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 3 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 3 | 0.1% | `__floatuntidf` |
| 2 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 2 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 2 | 0.1% | `_platform_memset` |
| 2 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 2 | 0.1% | `__ulock_wake` |
| 2 | 0.1% | `DYLD-STUB$$free` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 1 | 0.0% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 1 | 0.0% | `core::iter::adapters::try_process` |
| 1 | 0.0% | `xzm_chunk_mark_used` |
| 1 | 0.0% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 1 | 0.0% | `_platform_memcmp` |
| 1 | 0.0% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 1 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.0% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1765 | 78.8% | `radiomics::workflows::extraction::extract` |
| 1765 | 78.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 926 | 41.4% | `radiomics::imaging::preprocessing::prepare` |
| 863 | 38.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 839 | 37.5% | `radiomics::workflows::engine::run_prepared` |
| 719 | 32.1% | `radiomics::imaging::preprocessing::gather` |
| 585 | 26.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 383 | 17.1% | `radiomics::features::ivh::ivh_statistics` |
| 247 | 11.0% | `radiomics::features::ivh::prepare_ivh` |
| 94 | 4.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 63 | 2.8% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 43 | 1.9% | `radiomics::imaging::io::HostArray::eq_label` |
| 41 | 1.8% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 35 | 1.6% | `radiomics::imaging::preprocessing::normalize_values` |
| 19 | 0.8% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 4 | 0.2% | `radiomics::imaging::preprocessing::resegment_samples` |
