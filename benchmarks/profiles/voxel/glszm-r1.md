# Profile: voxel/glszm-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 glszm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:17 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2318 (idle worker-thread samples excluded: 43130)

## Command output

```text
{"median_ms":1490.503708,"repeats":3,"min_ms":1082.789125,"max_ms":2656.857583,"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 986 | 42.5% | `__bzero` |
| 464 | 20.0% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 337 | 14.5% | `<deduplicated_symbol>` |
| 115 | 5.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 109 | 4.7% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 98 | 4.2% | `_platform_memmove` |
| 38 | 1.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 31 | 1.3% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 29 | 1.3% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 26 | 1.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 14 | 0.6% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 12 | 0.5% | `__psynch_mutexwait` |
| 11 | 0.5% | `_free` |
| 7 | 0.3% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 6 | 0.3% | `_xzm_free_main` |
| 5 | 0.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 4 | 0.2% | `_platform_memset` |
| 3 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 3 | 0.1% | `num_bigint::biguint::shift::biguint_shr2` |
| 3 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 3 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 2 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 2 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 2 | 0.1% | `_xzm_free` |
| 2 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `_xzm_reclaim_mark_used` |
| 1 | 0.0% | `_xzm_xzone_malloc_tiny` |
| 1 | 0.0% | `_malloc_zone_realloc` |
| 1 | 0.0% | `mfm_alloc` |
| 1 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 1 | 0.0% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2373 | 102.4% | `radiomics::workflows::extraction::extract` |
| 2373 | 102.4% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1353 | 58.4% | `radiomics::workflows::engine::run_prepared` |
| 1351 | 58.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1020 | 44.0% | `radiomics::imaging::preprocessing::prepare` |
| 997 | 43.0% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 598 | 25.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 527 | 22.7% | `radiomics::imaging::preprocessing::gather` |
| 422 | 18.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 38 | 1.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 30 | 1.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 8 | 0.3% | `radiomics::imaging::preprocessing::normalize_values` |
| 5 | 0.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.0% | `radiomics::voxel::window_texture::WindowTexture::new` |
