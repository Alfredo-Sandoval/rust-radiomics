# Profile: voxel/gldzm-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 gldzm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:19 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2153 (idle worker-thread samples excluded: 34403)

## Command output

```text
{"median_ms":726.448,"repeats":7,"min_ms":479.780958,"max_ms":1051.810375,"roi_voxels":54639,"maps":16,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 722 | 33.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 308 | 14.3% | `<deduplicated_symbol>` |
| 227 | 10.5% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 153 | 7.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 151 | 7.0% | `__bzero` |
| 115 | 5.3% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 87 | 4.0% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 72 | 3.3% | `core::iter::traits::iterator::Iterator::try_fold` |
| 71 | 3.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 62 | 2.9% | `_platform_memmove` |
| 52 | 2.4% | `__psynch_mutexwait` |
| 35 | 1.6% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 23 | 1.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 15 | 0.7% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 7 | 0.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 6 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 5 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 4 | 0.2% | `_platform_memset` |
| 4 | 0.2% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 3 | 0.1% | `_xzm_free_main` |
| 3 | 0.1% | `_xzm_xzone_malloc_tiny` |
| 3 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.1% | `_free` |
| 2 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 2 | 0.1% | `xzm_malloc_zone_size` |
| 2 | 0.1% | `_xzm_xzone_malloc_small_freelist` |
| 2 | 0.1% | `crossbeam_epoch::internal::Global::try_advance` |
| 1 | 0.0% | `xzm_segment_group_alloc_chunk` |
| 1 | 0.0% | `_<num_bigint::bigint::BigInt as num_integer::Integer>::div_mod_floor` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.0% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 1 | 0.0% | `crossbeam_deque::deque::Worker<T>::pop` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_malloc_main` |
| 1 | 0.0% | `_platform_memset_pattern16` |
| 1 | 0.0% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.0% | `rayon_core::registry::WorkerThread::find_work` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 1 | 0.0% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 1 | 0.0% | `_xzm_free_outlined` |
| 1 | 0.0% | `default_zone_realloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1899 | 88.2% | `radiomics::workflows::extraction::extract` |
| 1899 | 88.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1285 | 59.7% | `radiomics::imaging::preprocessing::prepare` |
| 668 | 31.0% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 617 | 28.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 614 | 28.5% | `radiomics::workflows::engine::run_prepared` |
| 609 | 28.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 497 | 23.1% | `radiomics::imaging::preprocessing::gather` |
| 249 | 11.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 161 | 7.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 42 | 2.0% | `radiomics::imaging::preprocessing::normalize_values` |
| 29 | 1.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 26 | 1.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 15 | 0.7% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 6 | 0.3% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::floor_to_bigint` |
