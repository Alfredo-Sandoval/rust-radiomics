# Profile: voxel/glszm-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 glszm`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:19 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 1731 (idle worker-thread samples excluded: 31443)

## Command output

```text
{"median_ms":830.733958,"repeats":7,"min_ms":529.1596669999999,"max_ms":889.226833,"roi_voxels":54639,"maps":16,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 624 | 36.0% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 266 | 15.4% | `<deduplicated_symbol>` |
| 138 | 8.0% | `__bzero` |
| 132 | 7.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 126 | 7.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 68 | 3.9% | `core::iter::traits::iterator::Iterator::try_fold` |
| 68 | 3.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 59 | 3.4% | `_platform_memmove` |
| 56 | 3.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 51 | 2.9% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 34 | 2.0% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 28 | 1.6% | `__psynch_mutexwait` |
| 13 | 0.8% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 9 | 0.5% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 8 | 0.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 7 | 0.4% | `_platform_memset` |
| 5 | 0.3% | `_xzm_xzone_malloc_tiny` |
| 4 | 0.2% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 3 | 0.2% | `_xzm_free_main` |
| 3 | 0.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 3 | 0.2% | `crossbeam_epoch::internal::Global::try_advance` |
| 3 | 0.2% | `__psynch_mutexdrop` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 2 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 2 | 0.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 1 | 0.1% | `_<num_bigint::bigint::BigInt as num_integer::Integer>::div_mod_floor` |
| 1 | 0.1% | `mach_absolute_time` |
| 1 | 0.1% | `_xzm_reclaim_mark_used` |
| 1 | 0.1% | `hashbrown::raw::RawTable<T,A>::reserve_rehash` |
| 1 | 0.1% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 1 | 0.1% | `_pthread_cond_wait` |
| 1 | 0.1% | `_RNvCsGIExRX8pES_7___rustc13___rdl_dealloc` |
| 1 | 0.1% | `rayon_core::registry::WorkerThread::find_work` |
| 1 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 1 | 0.1% | `_xzm_xzone_free_freelist` |
| 1 | 0.1% | `xzm_malloc_zone_malloc_type_malloc_main` |
| 1 | 0.1% | `_malloc_zone_realloc` |
| 1 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 1 | 0.1% | `_xzm_free` |
| 1 | 0.1% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 1 | 0.1% | `_tlv_get_addr` |
| 1 | 0.1% | `DYLD-STUB$$memcpy` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1717 | 99.2% | `radiomics::workflows::extraction::extract` |
| 1717 | 99.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1097 | 63.4% | `radiomics::imaging::preprocessing::prepare` |
| 620 | 35.8% | `radiomics::workflows::engine::run_prepared` |
| 616 | 35.6% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 609 | 35.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 495 | 28.6% | `radiomics::imaging::preprocessing::gather` |
| 488 | 28.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 148 | 8.5% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 146 | 8.4% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 28 | 1.6% | `radiomics::imaging::preprocessing::normalize_values` |
| 28 | 1.6% | `radiomics::imaging::io::HostArray::eq_label` |
| 19 | 1.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 10 | 0.6% | `radiomics::imaging::preprocessing::resegment_samples` |
| 9 | 0.5% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.1% | `radiomics::imaging::preprocessing::floor_to_bigint` |
| 1 | 0.1% | `radiomics::config::settings::list_features` |
