# Profile: voxel/ngtdm-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 ngtdm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:19 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 1976 (idle worker-thread samples excluded: 30134)

## Command output

```text
{"median_ms":644.971229,"repeats":8,"min_ms":384.655791,"max_ms":1057.751708,"roi_voxels":54639,"maps":5,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 569 | 28.8% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 464 | 23.5% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 249 | 12.6% | `<deduplicated_symbol>` |
| 134 | 6.8% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 119 | 6.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 72 | 3.6% | `core::iter::traits::iterator::Iterator::try_fold` |
| 58 | 2.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 50 | 2.5% | `_platform_memmove` |
| 45 | 2.3% | `__bzero` |
| 44 | 2.2% | `__psynch_mutexwait` |
| 42 | 2.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 39 | 2.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 31 | 1.6% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 18 | 0.9% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 8 | 0.4% | `_platform_memset` |
| 8 | 0.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 3 | 0.2% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.1% | `rayon::iter::from_par_iter::collect_extended` |
| 2 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 2 | 0.1% | `_xzm_xzone_malloc_tiny` |
| 2 | 0.1% | `_xzm_free_main` |
| 2 | 0.1% | `__psynch_cvsignal` |
| 1 | 0.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 1 | 0.1% | `num_bigint::biguint::division::div_rem_cow` |
| 1 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.1% | `DYLD-STUB$$log1p` |
| 1 | 0.1% | `_platform_memcmp` |
| 1 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 1 | 0.1% | `rayon_core::join::join_context::_{{closure}}` |
| 1 | 0.1% | `_tlv_get_addr` |
| 1 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::is_contiguous` |
| 1 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.1% | `DYLD-STUB$$bzero` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1665 | 84.3% | `radiomics::workflows::extraction::extract` |
| 1665 | 84.3% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1016 | 51.4% | `radiomics::imaging::preprocessing::prepare` |
| 649 | 32.8% | `radiomics::workflows::engine::run_prepared` |
| 644 | 32.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 533 | 27.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 483 | 24.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 472 | 23.9% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 451 | 22.8% | `radiomics::imaging::preprocessing::gather` |
| 46 | 2.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 40 | 2.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 24 | 1.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 18 | 0.9% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.2% | `radiomics::imaging::preprocessing::resegment_samples` |
| 3 | 0.2% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.1% | `radiomics::voxel::window_texture::WindowTexture::new` |
