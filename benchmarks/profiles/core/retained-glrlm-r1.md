# Profile: core/retained-glrlm-r1

- Command: `/usr/bin/env target/bench-core/voxel-engine/after/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 glrlm`
- Measured library: `df67aec`
- Checkout at sampling: `df67aec` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:50 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 5620 (idle worker-thread samples excluded: 15261)

## Command output

```text
{"median_ms":21.981625,"repeats":101,"min_ms":13.543542,"max_ms":322.041333,"roi_voxels":54639,"maps":16,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 2561 | 45.6% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1158 | 20.6% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 558 | 9.9% | `__psynch_mutexwait` |
| 360 | 6.4% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 118 | 2.1% | `madvise` |
| 94 | 1.7% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 60 | 1.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 60 | 1.1% | `__psynch_mutexdrop` |
| 59 | 1.0% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 55 | 1.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 52 | 0.9% | `_platform_memset` |
| 30 | 0.5% | `__psynch_cvsignal` |
| 29 | 0.5% | `_platform_memmove` |
| 26 | 0.5% | `<deduplicated_symbol>` |
| 26 | 0.5% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 22 | 0.4% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 21 | 0.4% | `_xzm_free_tc` |
| 21 | 0.4% | `_platform_memcmp` |
| 19 | 0.3% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 17 | 0.3% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 17 | 0.3% | `crossbeam_epoch::default::with_handle` |
| 15 | 0.3% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 14 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 14 | 0.2% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,D>>::from_elem` |
| 14 | 0.2% | `_xzm_malloc_tc` |
| 13 | 0.2% | `_pthread_cond_wait` |
| 11 | 0.2% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 10 | 0.2% | `__xzm_xzone_free_to_freelist_chunk` |
| 10 | 0.2% | `mach_absolute_time` |
| 10 | 0.2% | `_tlv_get_addr` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3751 | 66.7% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1158 | 20.6% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 1094 | 19.5% | `radiomics::workflows::extraction::extract` |
| 1094 | 19.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 979 | 17.4% | `radiomics::workflows::engine::run_prepared_at` |
| 979 | 17.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 185 | 3.3% | `radiomics::workflows::engine::run_prepared_at::_{{closure}}` |
| 118 | 2.1% | `radiomics::numerics::runtime::zeroed_map` |
| 80 | 1.4% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 61 | 1.1% | `radiomics::workflows::engine::inputs::input_window` |
| 61 | 1.1% | `radiomics::workflows::roi::bounds::include` |
| 44 | 0.8% | `radiomics::imaging::preprocessing::prepare` |
| 22 | 0.4% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 22 | 0.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 19 | 0.3% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 18 | 0.3% | `radiomics::imaging::preprocessing::gather` |
| 9 | 0.2% | `radiomics::imaging::io::HostArray::crop` |
| 4 | 0.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::config::settings::list_features` |
| 1 | 0.0% | `radiomics::imaging::io::HostArray::eq_label` |
| 1 | 0.0% | `radiomics::config::settings::Settings::validate` |
| 1 | 0.0% | `radiomics::config::validation::unique` |
