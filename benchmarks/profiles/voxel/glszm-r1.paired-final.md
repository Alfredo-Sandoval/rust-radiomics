# Profile: voxel/glszm-r1.paired-final

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 glszm`
- Commit: `eb3c8ce` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:21 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 1947 (idle worker-thread samples excluded: 35103)

## Command output

```text
{"median_ms":698.0634585,"repeats":8,"min_ms":441.053917,"max_ms":952.562334,"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 708 | 36.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 322 | 16.5% | `<deduplicated_symbol>` |
| 279 | 14.3% | `__bzero` |
| 127 | 6.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 96 | 4.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 83 | 4.3% | `core::iter::traits::iterator::Iterator::try_fold` |
| 65 | 3.3% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 56 | 2.9% | `_platform_memmove` |
| 55 | 2.8% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 51 | 2.6% | `__psynch_mutexwait` |
| 37 | 1.9% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 30 | 1.5% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 7 | 0.4% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 6 | 0.3% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 3 | 0.2% | `radiomics::imaging::preprocessing::lattice_bins` |
| 3 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 2 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 2 | 0.1% | `_xzm_free` |
| 2 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 2 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.1% | `num_bigint::biguint::shift::biguint_shl2` |
| 1 | 0.1% | `_xzm_reclaim_mark_used` |
| 1 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.1% | `_xzm_free_outlined` |
| 1 | 0.1% | `_free` |
| 1 | 0.1% | `_xzm_free_main` |
| 1 | 0.1% | `_malloc_zone_malloc` |
| 1 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 1 | 0.1% | `_platform_memset` |
| 1 | 0.1% | `_RNvCsGIExRX8pES_7___rustc13___rdl_realloc` |
| 1 | 0.1% | `_xzm_xzone_free_freelist` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1924 | 98.8% | `radiomics::workflows::extraction::extract` |
| 1924 | 98.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1236 | 63.5% | `radiomics::imaging::preprocessing::prepare` |
| 688 | 35.3% | `radiomics::workflows::engine::run_prepared` |
| 683 | 35.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 626 | 32.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 610 | 31.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 522 | 26.8% | `radiomics::imaging::preprocessing::gather` |
| 287 | 14.7% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 76 | 3.9% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 32 | 1.6% | `radiomics::imaging::io::HostArray::eq_label` |
| 23 | 1.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 7 | 0.4% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 4 | 0.2% | `radiomics::imaging::preprocessing::resegment_samples` |
| 3 | 0.2% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 1 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
