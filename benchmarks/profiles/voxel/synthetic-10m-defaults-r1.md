# Profile: voxel/synthetic-10m-defaults-r1

- Command: `target/bench-voxel/final/benchmark_voxel synthetic - 3 1 defaults`
- Commit: `f6a4d68` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:15 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 7979 (idle worker-thread samples excluded: 15296)

## Command output

```text
{"median_ms":28050.628791000003,"repeats":3,"min_ms":24781.78475,"max_ms":33947.345624999994,"roi_voxels":10100000,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 3080 | 38.6% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 1708 | 21.4% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 445 | 5.6% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 432 | 5.4% | `_platform_memset` |
| 333 | 4.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 236 | 3.0% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 209 | 2.6% | `core::slice::sort::unstable::quicksort::quicksort` |
| 186 | 2.3% | `radiomics::features::first_order::RegionKernel::finish` |
| 138 | 1.7% | `__psynch_mutexwait` |
| 115 | 1.4% | `radiomics::features::first_order::LevelCounts::add` |
| 103 | 1.3% | `expm1` |
| 96 | 1.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 94 | 1.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 67 | 0.8% | `_xzm_xzone_malloc_tiny` |
| 64 | 0.8% | `acos` |
| 56 | 0.7% | `_xzm_free_main` |
| 52 | 0.7% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 44 | 0.6% | `__psynch_mutexdrop` |
| 42 | 0.5% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 37 | 0.5% | `<deduplicated_symbol>` |
| 36 | 0.5% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 29 | 0.4% | `_platform_memmove` |
| 23 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 23 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 22 | 0.3% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 18 | 0.2% | `???` |
| 18 | 0.2% | `__bzero` |
| 17 | 0.2% | `cos` |
| 16 | 0.2% | `mach_absolute_time` |
| 16 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 15 | 0.2% | `__ulock_wake` |
| 14 | 0.2% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 14 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 11 | 0.1% | `DYLD-STUB$$cos` |
| 10 | 0.1% | `_tlv_get_addr` |
| 9 | 0.1% | `DYLD-STUB$$bzero` |
| 8 | 0.1% | `xzm_malloc_zone_malloc_type_realloc` |
| 8 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 7 | 0.1% | `_<&$u5b$usize$u5d$ as ndarray::dimension::ndindex::NdIndex<ndarray::dimension::dim::Dim<ndarray::dimension::dynindeximpl::IxDynImpl>>>::index_checked` |
| 6 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 5 | 0.1% | `_free` |
| 5 | 0.1% | `_malloc_zone_malloc` |
| 5 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 5 | 0.1% | `DYLD-STUB$$expm1` |
| 5 | 0.1% | `_xzm_xzone_malloc_freelist_outlined` |
| 5 | 0.1% | `_pthread_cond_wait` |
| 5 | 0.1% | `_xzm_chunk_list_push` |
| 5 | 0.1% | `rayon::iter::plumbing::Folder::consume_iter` |
| 4 | 0.1% | `_platform_memcmp` |
| 4 | 0.1% | `_<alloc::vec::Vec<T,A1> as alloc::vec::spec_extend::SpecExtend<T,alloc::vec::into_iter::IntoIter<T,A2>>>::spec_extend` |
| 4 | 0.1% | `_xzm_free` |
| 4 | 0.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 4 | 0.1% | `_malloc_zone_calloc` |
| 4 | 0.1% | `_xzm_xzone_malloc_small_freelist` |
| 4 | 0.1% | `__psynch_cvsignal` |
| 4 | 0.1% | `_xzm_xzone_malloc_from_freelist_chunk` |
| 4 | 0.1% | `_platform_memset_pattern16` |
| 3 | 0.0% | `xzm_malloc_zone_size` |
| 3 | 0.0% | `core::slice::sort::unstable::ipnsort` |
| 3 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3702 | 46.4% | `radiomics::voxel::window_glcm::WindowGlcm::evaluate` |
| 2182 | 27.3% | `radiomics::voxel::window_texture::WindowTexture::evaluate` |
| 1225 | 15.4% | `radiomics::workflows::extraction::extract` |
| 1225 | 15.4% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1225 | 15.4% | `radiomics::workflows::engine::run_prepared` |
| 1224 | 15.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1022 | 12.8% | `radiomics::features::first_order::FirstOrderMapKernel::compute_batch` |
| 689 | 8.6% | `radiomics::features::first_order::RegionKernel::finish` |
| 475 | 6.0% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 445 | 5.6% | `radiomics::voxel::window_texture::SizeMatrix::statistics` |
| 258 | 3.2% | `radiomics::features::first_order::FirstOrderMapKernel::at` |
| 222 | 2.8% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 117 | 1.5% | `radiomics::features::first_order::LevelCounts::add` |
| 114 | 1.4% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 59 | 0.7% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 32 | 0.4% | `radiomics::features::first_order::FirstOrderMapKernel::new` |
| 19 | 0.2% | `radiomics::features::first_order::LevelCounts::finish` |
| 9 | 0.1% | `radiomics::features::first_order::FirstOrderMapKernel::from_names` |
| 8 | 0.1% | `radiomics::features::first_order::RegionKernel::new` |
| 8 | 0.1% | `radiomics::numerics::reduction::reported_mean` |
| 3 | 0.0% | `radiomics::numerics::count_statistics::bin_entropy` |
| 2 | 0.0% | `radiomics::numerics::precision::QuantileRank::interpolate` |
| 1 | 0.0% | `radiomics::features::first_order::SampleStatistics::new` |
| 1 | 0.0% | `radiomics::native_first_order::statistics::Calculation::new` |
