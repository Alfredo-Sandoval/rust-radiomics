# Profile: core/retained-glrlm-r1.baseline

- Command: `/usr/bin/env target/bench-core/voxel-engine/before/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 glrlm`
- Measured library: `aa6355a`
- Checkout at sampling: `aa6355a` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:47 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3571 (idle worker-thread samples excluded: 44442)

## Command output

```text
{"median_ms":319.397959,"repeats":15,"min_ms":244.35575,"max_ms":505.029916,"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 944 | 26.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 598 | 16.7% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 427 | 12.0% | `<deduplicated_symbol>` |
| 296 | 8.3% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 248 | 6.9% | `__bzero` |
| 225 | 6.3% | `__psynch_mutexwait` |
| 182 | 5.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 113 | 3.2% | `core::iter::traits::iterator::Iterator::try_fold` |
| 107 | 3.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 92 | 2.6% | `_platform_memmove` |
| 90 | 2.5% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 80 | 2.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 51 | 1.4% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 17 | 0.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 14 | 0.4% | `_platform_memset` |
| 11 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 7 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 7 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 6 | 0.2% | `__psynch_mutexdrop` |
| 6 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 4 | 0.1% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 3 | 0.1% | `_xzm_reclaim_mark_used` |
| 3 | 0.1% | `_xzm_free_tc` |
| 3 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 3 | 0.1% | `_malloc_zone_calloc` |
| 3 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.1% | `_malloc_zone_realloc` |
| 2 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 2 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2490 | 69.7% | `radiomics::workflows::extraction::extract` |
| 2490 | 69.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1689 | 47.3% | `radiomics::imaging::preprocessing::prepare` |
| 907 | 25.4% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 895 | 25.1% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 801 | 22.4% | `radiomics::workflows::engine::run_prepared` |
| 795 | 22.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 782 | 21.9% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 735 | 20.6% | `radiomics::imaging::preprocessing::gather` |
| 296 | 8.3% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 270 | 7.6% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 52 | 1.5% | `radiomics::imaging::io::HostArray::eq_label` |
| 38 | 1.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 18 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 5 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
