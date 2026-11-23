# Profile: voxel/ivh-r2

- Command: `target/bench-voxel/final/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 2 ivh`
- Commit: `3982280` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:20 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3282 (idle worker-thread samples excluded: 27616)

## Command output

```text
{"median_ms":805.4739159999999,"repeats":5,"min_ms":568.000084,"max_ms":2145.9674999999997,"roi_voxels":54639,"maps":7,"radius":2}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 629 | 19.2% | `<deduplicated_symbol>` |
| 467 | 14.2% | `radiomics::features::ivh::prepare_ivh` |
| 426 | 13.0% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 229 | 7.0% | `core::slice::sort::select::partition_at_index` |
| 226 | 6.9% | `__bzero` |
| 198 | 6.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 195 | 5.9% | `_platform_memmove` |
| 137 | 4.2% | `core::iter::traits::iterator::Iterator::try_fold` |
| 130 | 4.0% | `radiomics::features::ivh::ivh_statistics` |
| 90 | 2.7% | `_<core::iter::adapters::take::Take<I> as core::iter::adapters::take::SpecTake>::spec_fold` |
| 80 | 2.4% | `_xzm_xzone_malloc_tiny` |
| 74 | 2.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 59 | 1.8% | `_xzm_free_main` |
| 58 | 1.8% | `mach_absolute_time` |
| 57 | 1.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 36 | 1.1% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 33 | 1.0% | `_xzm_free` |
| 24 | 0.7% | `_malloc_zone_realloc` |
| 15 | 0.5% | `xzm_malloc_zone_malloc_type_realloc` |
| 12 | 0.4% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 9 | 0.3% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 8 | 0.2% | `free` |
| 7 | 0.2% | `_free` |
| 6 | 0.2% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 6 | 0.2% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 6 | 0.2% | `_platform_memset` |
| 6 | 0.2% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 6 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 5 | 0.2% | `xzm_malloc_zone_size` |
| 5 | 0.2% | `_RNvNtNtCsc8ysn0H88rX_17compiler_builtins3int19specialized_div_rem12u128_div_rem` |
| 5 | 0.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 4 | 0.1% | `DYLD-STUB$$memcpy` |
| 4 | 0.1% | `_xzm_malloc` |
| 3 | 0.1% | `_realloc` |
| 3 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 3 | 0.1% | `_platform_memcmp` |
| 3 | 0.1% | `default_zone_realloc` |
| 3 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 3 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 2 | 0.1% | `_RNvXs4_NtCs8dnTdrJsiec_5alloc6stringNtB5_6StringNtNtCs6sq8b9ugfBC_4core5clone5Clone5clone` |
| 2 | 0.1% | `DYLD-STUB$$malloc` |
| 2 | 0.1% | `_malloc_ulock_self_owner_value` |
| 2 | 0.1% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 1 | 0.0% | `ndarray::impl_constructors::_<impl ndarray::ArrayBase<S,ndarray::dimension::dim::Dim<$u5b$usize$u3b$ 1$u5d$>>>::from_iter` |
| 1 | 0.0% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 1 | 0.0% | `core::slice::sort::shared::pivot::median3_rec` |
| 1 | 0.0% | `_malloc_zone_malloc` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2602 | 79.3% | `radiomics::workflows::extraction::extract` |
| 2602 | 79.3% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1304 | 39.7% | `radiomics::imaging::preprocessing::prepare` |
| 1298 | 39.5% | `radiomics::workflows::engine::run_prepared` |
| 1120 | 34.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 1035 | 31.5% | `radiomics::imaging::preprocessing::gather` |
| 735 | 22.4% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 650 | 19.8% | `radiomics::features::ivh::ivh_statistics` |
| 561 | 17.1% | `radiomics::features::ivh::prepare_ivh` |
| 184 | 5.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 147 | 4.5% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 64 | 2.0% | `radiomics::imaging::preprocessing::normalize_values` |
| 51 | 1.6% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 44 | 1.3% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 10 | 0.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
