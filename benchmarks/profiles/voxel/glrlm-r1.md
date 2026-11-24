# Profile: voxel/glrlm-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 glrlm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:16 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2866 (idle worker-thread samples excluded: 43551)

## Command output

```text
{"median_ms":1565.4651875,"repeats":4,"min_ms":1098.053875,"max_ms":2405.040917,"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 647 | 22.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 594 | 20.7% | `__bzero` |
| 543 | 18.9% | `<deduplicated_symbol>` |
| 338 | 11.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 127 | 4.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 124 | 4.3% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 115 | 4.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 88 | 3.1% | `_platform_memmove` |
| 75 | 2.6% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 45 | 1.6% | `core::iter::traits::iterator::Iterator::try_fold` |
| 45 | 1.6% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 41 | 1.4% | `__psynch_mutexwait` |
| 27 | 0.9% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 10 | 0.3% | `_platform_memset` |
| 5 | 0.2% | `xzm_malloc_zone_size` |
| 4 | 0.1% | `__psynch_cvsignal` |
| 4 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 4 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 4 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 4 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 3 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 3 | 0.1% | `_xzm_free_main` |
| 3 | 0.1% | `xzm_malloc_zone_malloc_type_realloc` |
| 3 | 0.1% | `_malloc_zone_realloc` |
| 2 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 2 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.0% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 1 | 0.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.0% | `_tlv_get_addr` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2392 | 83.5% | `radiomics::workflows::extraction::extract` |
| 2392 | 83.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1335 | 46.6% | `radiomics::imaging::preprocessing::prepare` |
| 1057 | 36.9% | `radiomics::workflows::engine::run_prepared` |
| 1024 | 35.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 707 | 24.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 648 | 22.6% | `radiomics::imaging::preprocessing::gather` |
| 628 | 21.9% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 598 | 20.9% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 470 | 16.4% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 124 | 4.3% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 17 | 0.6% | `radiomics::imaging::io::HostArray::eq_label` |
| 10 | 0.3% | `radiomics::imaging::preprocessing::normalize_values` |
| 6 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
