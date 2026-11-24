# Profile: voxel/intensityhistogram-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 intensityhistogram`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:17 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2537 (idle worker-thread samples excluded: 37534)

## Command output

```text
{"median_ms":1222.7098959999998,"repeats":4,"min_ms":815.505625,"max_ms":2107.5588340000004,"roi_voxels":54639,"maps":23,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 847 | 33.4% | `__bzero` |
| 421 | 16.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 202 | 8.0% | `_platform_memmove` |
| 198 | 7.8% | `<deduplicated_symbol>` |
| 140 | 5.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 95 | 3.7% | `__psynch_mutexwait` |
| 73 | 2.9% | `radiomics::features::intensity_histogram::compute` |
| 60 | 2.4% | `core::iter::traits::iterator::Iterator::try_fold` |
| 56 | 2.2% | `_xzm_xzone_malloc_tiny` |
| 37 | 1.5% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 35 | 1.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 35 | 1.4% | `_xzm_free_main` |
| 33 | 1.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 31 | 1.2% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 29 | 1.1% | `mach_absolute_time` |
| 23 | 0.9% | `_xzm_free` |
| 22 | 0.9% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 21 | 0.8% | `__floattidf` |
| 20 | 0.8% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 15 | 0.6% | `xzm_malloc_zone_malloc_type_realloc` |
| 14 | 0.6% | `_malloc_zone_malloc` |
| 14 | 0.6% | `radiomics::features::intensity_histogram::linear_percentile` |
| 12 | 0.5% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 11 | 0.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 7 | 0.3% | `xzm_malloc_zone_size` |
| 7 | 0.3% | `_free` |
| 7 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 6 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 6 | 0.2% | `_pthread_cond_wait` |
| 5 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 5 | 0.2% | `_xzm_malloc` |
| 4 | 0.2% | `log2` |
| 4 | 0.2% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 4 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 3 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 3 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 3 | 0.1% | `malloc` |
| 3 | 0.1% | `pthread_mutex_unlock` |
| 2 | 0.1% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 2 | 0.1% | `DYLD-STUB$$free` |
| 2 | 0.1% | `core::cell::lazy::LazyCell<T,F>::really_init` |
| 2 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 1 | 0.0% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 1 | 0.0% | `num_bigint::bigint::BigInt::from_biguint` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 1 | 0.0% | `DYLD-STUB$$mach_absolute_time` |
| 1 | 0.0% | `free` |
| 1 | 0.0% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 1 | 0.0% | `_platform_memset` |
| 1 | 0.0% | `_xzm_xzone_malloc_from_freelist_chunk` |
| 1 | 0.0% | `_platform_memset_pattern16` |
| 1 | 0.0% | `_malloc_zone_realloc` |
| 1 | 0.0% | `__udivti3` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 1 | 0.0% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.0% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 1 | 0.0% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 1 | 0.0% | `__floatuntidf` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2086 | 82.2% | `radiomics::workflows::extraction::extract` |
| 2086 | 82.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1174 | 46.3% | `radiomics::workflows::engine::run_prepared` |
| 1172 | 46.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 912 | 35.9% | `radiomics::imaging::preprocessing::prepare` |
| 853 | 33.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 610 | 24.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 411 | 16.2% | `radiomics::features::intensity_histogram::compute` |
| 302 | 11.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 260 | 10.2% | `radiomics::imaging::preprocessing::gather` |
| 43 | 1.7% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 32 | 1.3% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 16 | 0.6% | `radiomics::features::intensity_histogram::linear_percentile` |
| 14 | 0.6% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 12 | 0.5% | `radiomics::imaging::io::HostArray::eq_label` |
| 10 | 0.4% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::config::settings::list_features` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
