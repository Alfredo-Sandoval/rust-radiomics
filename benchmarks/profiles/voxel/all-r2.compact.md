# Profile: voxel/all-r2.compact

- Command: `target/bench-voxel/compact_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 all`
- Commit: `2de4a73` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:01 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 7690 (idle worker-thread samples excluded: 32818)

## Command output

```text
{"median_ms":2435.235,"repeats":3,"min_ms":1944.829792,"max_ms":2480.092583,"roi_voxels":54639,"maps":147,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3249 | 42.2% | `radiomics::voxel::spectrum::spectral_radius` |
| 1172 | 15.2% | `__bzero` |
| 1004 | 13.1% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 414 | 5.4% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 264 | 3.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 120 | 1.6% | `_platform_memset` |
| 113 | 1.5% | `__psynch_mutexwait` |
| 93 | 1.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 88 | 1.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 84 | 1.1% | `_platform_memmove` |
| 83 | 1.1% | `core::slice::sort::unstable::quicksort::quicksort` |
| 75 | 1.0% | `mach_absolute_time` |
| 65 | 0.8% | `_kernelrpc_mach_vm_deallocate_trap` |
| 62 | 0.8% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 60 | 0.8% | `_xzm_xzone_malloc_tiny` |
| 44 | 0.6% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 43 | 0.6% | `<deduplicated_symbol>` |
| 43 | 0.6% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 43 | 0.6% | `radiomics::features::intensity_histogram::compute` |
| 41 | 0.5% | `radiomics::features::ivh::ivh_statistics` |
| 39 | 0.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 39 | 0.5% | `core::slice::sort::select::partition_at_index` |
| 35 | 0.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 30 | 0.4% | `_xzm_free_main` |
| 25 | 0.3% | `radiomics::features::first_order::LevelCounts::add` |
| 23 | 0.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 22 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 22 | 0.3% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 17 | 0.2% | `_xzm_free` |
| 16 | 0.2% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 15 | 0.2% | `core::slice::sort::shared::pivot::median3_rec` |
| 14 | 0.2% | `xzm_malloc_zone_malloc_type_realloc` |
| 13 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 13 | 0.2% | `_pthread_cond_wait` |
| 12 | 0.2% | `__floattidf` |
| 11 | 0.1% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 10 | 0.1% | `log2` |
| 10 | 0.1% | `_xzm_malloc` |
| 9 | 0.1% | `core::iter::traits::iterator::Iterator::try_fold` |
| 8 | 0.1% | `_malloc_zone_malloc` |
| 8 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 7 | 0.1% | `radiomics::features::ivh::prepare_ivh` |
| 7 | 0.1% | `_realloc` |
| 6 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 5 | 0.1% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 5 | 0.1% | `DYLD-STUB$$memcpy` |
| 4 | 0.1% | `radiomics::features::first_order::LevelCounts::finish` |
| 4 | 0.1% | `_malloc_zone_realloc` |
| 4 | 0.1% | `xzm_malloc_zone_size` |
| 4 | 0.1% | `expm1` |
| 4 | 0.1% | `radiomics::features::intensity_histogram::linear_percentile` |
| 4 | 0.1% | `DYLD-STUB$$bzero` |
| 4 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 4 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 4 | 0.1% | `_tlv_get_addr` |
| 3 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 3 | 0.0% | `core::cell::lazy::LazyCell<T,F>::really_init` |
| 3 | 0.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 3 | 0.0% | `__psynch_cvsignal` |
| 3 | 0.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 4407 | 57.3% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 3360 | 43.7% | `radiomics::voxel::spectrum::spectral_radius` |
| 2064 | 26.8% | `radiomics::workflows::extraction::extract` |
| 2064 | 26.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1886 | 24.5% | `radiomics::workflows::engine::run_prepared` |
| 1686 | 21.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1235 | 16.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 508 | 6.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 282 | 3.7% | `radiomics::features::intensity_histogram::compute` |
| 251 | 3.3% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 195 | 2.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 192 | 2.5% | `radiomics::features::ivh::ivh_statistics` |
| 178 | 2.3% | `radiomics::imaging::preprocessing::prepare` |
| 145 | 1.9% | `radiomics::features::ivh::prepare_ivh` |
| 143 | 1.9% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 93 | 1.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 93 | 1.2% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 85 | 1.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 60 | 0.8% | `radiomics::imaging::preprocessing::gather` |
| 55 | 0.7% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 44 | 0.6% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 43 | 0.6% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 25 | 0.3% | `radiomics::features::first_order::LevelCounts::add` |
| 17 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 17 | 0.2% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 13 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 11 | 0.1% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 8 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 5 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 5 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 5 | 0.1% | `radiomics::numerics::count_statistics::bin_entropy` |
| 4 | 0.1% | `radiomics::features::first_order::RegionKernel::new` |
| 4 | 0.1% | `radiomics::features::intensity_histogram::linear_percentile` |
| 4 | 0.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 4 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::native_first_order::statistics::FirstOrderFeature::from_short_name` |
