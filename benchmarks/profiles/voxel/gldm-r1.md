# Profile: voxel/gldm-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 gldm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:17 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2110 (idle worker-thread samples excluded: 35947)

## Command output

```text
{"median_ms":724.5794589999999,"repeats":7,"min_ms":676.9015830000001,"max_ms":841.653375,"roi_voxels":54639,"maps":17,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 776 | 36.8% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 335 | 15.9% | `__bzero` |
| 300 | 14.2% | `<deduplicated_symbol>` |
| 112 | 5.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 94 | 4.5% | `_platform_memmove` |
| 85 | 4.0% | `core::iter::traits::iterator::Iterator::try_fold` |
| 80 | 3.8% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 69 | 3.3% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 57 | 2.7% | `__psynch_mutexwait` |
| 54 | 2.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 41 | 1.9% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 32 | 1.5% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 19 | 0.9% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 9 | 0.4% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 8 | 0.4% | `_platform_memset` |
| 8 | 0.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 4 | 0.2% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 3 | 0.1% | `_xzm_reclaim_mark_used` |
| 3 | 0.1% | `_xzm_free_main` |
| 3 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 2 | 0.1% | `_xzm_xzone_malloc_tiny` |
| 2 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 1 | 0.0% | `num_bigint::biguint::shift::_<impl core::ops::bit::ShrAssign<u64> for num_bigint::biguint::BigUint>::shr_assign` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::zeros` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.0% | `DYLD-STUB$$realloc` |
| 1 | 0.0% | `DYLD-STUB$$malloc` |
| 1 | 0.0% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 1 | 0.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 1 | 0.0% | `xzm_malloc_zone_size` |
| 1 | 0.0% | `_realloc` |
| 1 | 0.0% | `mach_absolute_time` |
| 1 | 0.0% | `DYLD-STUB$$memcmp` |
| 1 | 0.0% | `_tlv_get_addr` |
| 1 | 0.0% | `_platform_memcmp` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1966 | 93.2% | `radiomics::workflows::extraction::extract` |
| 1966 | 93.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1296 | 61.4% | `radiomics::imaging::preprocessing::prepare` |
| 699 | 33.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 670 | 31.8% | `radiomics::workflows::engine::run_prepared` |
| 665 | 31.5% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 597 | 28.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 594 | 28.2% | `radiomics::imaging::preprocessing::gather` |
| 349 | 16.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 97 | 4.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 29 | 1.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 25 | 1.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 19 | 0.9% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 6 | 0.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
