# Profile: voxel/glrlm-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 glrlm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:19 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2531 (idle worker-thread samples excluded: 29777)

## Command output

```text
{"median_ms":908.021042,"repeats":5,"min_ms":602.4009580000001,"max_ms":2070.647875,"roi_voxels":54639,"maps":16,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 619 | 24.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 417 | 16.5% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 264 | 10.4% | `__bzero` |
| 260 | 10.3% | `_platform_memmove` |
| 257 | 10.2% | `core::iter::traits::iterator::Iterator::try_fold` |
| 213 | 8.4% | `<deduplicated_symbol>` |
| 132 | 5.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 109 | 4.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 69 | 2.7% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 60 | 2.4% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 42 | 1.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 25 | 1.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 20 | 0.8% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 9 | 0.4% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 7 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 5 | 0.2% | `__psynch_mutexwait` |
| 3 | 0.1% | `_platform_memset` |
| 2 | 0.1% | `__psynch_mutexdrop` |
| 2 | 0.1% | `_xzm_free_main` |
| 1 | 0.0% | `_<num_bigint::bigint::BigInt as num_integer::Integer>::div_rem` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `_xzm_segment_group_span_mark_smaller` |
| 1 | 0.0% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.0% | `mach_vm_reclaim_try_cancel` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 1 | 0.0% | `_xzm_xzone_malloc_freelist_outlined` |
| 1 | 0.0% | `rayon::iter::plumbing::Folder::consume_iter` |
| 1 | 0.0% | `_realloc` |
| 1 | 0.0% | `_xzm_xzone_malloc_tiny` |
| 1 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 1 | 0.0% | `_xzm_xzone_free_freelist` |
| 1 | 0.0% | `_xzm_free_outlined` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2092 | 82.7% | `radiomics::workflows::extraction::extract` |
| 2092 | 82.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1239 | 49.0% | `radiomics::imaging::preprocessing::prepare` |
| 853 | 33.7% | `radiomics::workflows::engine::run_prepared` |
| 851 | 33.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 813 | 32.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 552 | 21.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 426 | 16.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 334 | 13.2% | `radiomics::imaging::preprocessing::gather` |
| 273 | 10.8% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 132 | 5.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 51 | 2.0% | `radiomics::imaging::preprocessing::normalize_values` |
| 28 | 1.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 18 | 0.7% | `radiomics::imaging::io::HostArray::eq_label` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::config::settings::list_features` |
