# Profile: voxel/ngtdm-r1

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 ngtdm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:17 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2208 (idle worker-thread samples excluded: 38737)

## Command output

```text
{"median_ms":626.118667,"repeats":9,"min_ms":327.221583,"max_ms":912.6300000000001,"roi_voxels":54639,"maps":5,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 987 | 44.7% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 330 | 14.9% | `<deduplicated_symbol>` |
| 175 | 7.9% | `_platform_memmove` |
| 154 | 7.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 105 | 4.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 87 | 3.9% | `__bzero` |
| 83 | 3.8% | `core::iter::traits::iterator::Iterator::try_fold` |
| 75 | 3.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 57 | 2.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 39 | 1.8% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 34 | 1.5% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 31 | 1.4% | `__psynch_mutexwait` |
| 9 | 0.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 6 | 0.3% | `_platform_memset` |
| 5 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 4 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 3 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 3 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 3 | 0.1% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 2 | 0.1% | `__psynch_cvsignal` |
| 2 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 2 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.0% | `_<num_bigint::biguint::BigUint as num_integer::Integer>::gcd` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.0% | `_platform_memcmp` |
| 1 | 0.0% | `DYLD-STUB$$free` |
| 1 | 0.0% | `mach_absolute_time` |
| 1 | 0.0% | `_RNvCsGIExRX8pES_7___rustc35___rust_no_alloc_shim_is_unstable_v2` |
| 1 | 0.0% | `_platform_memset_pattern16` |
| 1 | 0.0% | `mach_vm_reclaim_try_cancel` |
| 1 | 0.0% | `DYLD-STUB$$memcpy` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2110 | 95.6% | `radiomics::workflows::extraction::extract` |
| 2110 | 95.6% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1655 | 75.0% | `radiomics::imaging::preprocessing::prepare` |
| 978 | 44.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 677 | 30.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 572 | 25.9% | `radiomics::imaging::preprocessing::gather` |
| 455 | 20.6% | `radiomics::workflows::engine::run_prepared` |
| 449 | 20.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 107 | 4.8% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 93 | 4.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 30 | 1.4% | `radiomics::imaging::io::HostArray::eq_label` |
| 27 | 1.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 12 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::voxel::window_texture::WindowTexture::new` |
