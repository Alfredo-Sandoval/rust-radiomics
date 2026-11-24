# Profile: voxel/gldm-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 gldm`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:44 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 4995 (idle worker-thread samples excluded: 48891)

## Command output

```text
{"median_ms":1041.65525,"samples_ms":[887.943541,930.297458,1041.65525,1353.2431250000002,1437.801417],"roi_voxels":54639,"maps":17,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1225 | 24.5% | `_<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next` |
| 878 | 17.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 314 | 6.3% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 296 | 5.9% | `__bzero` |
| 283 | 5.7% | `mach_absolute_time` |
| 202 | 4.0% | `_platform_memcmp` |
| 156 | 3.1% | `_xzm_xzone_malloc_tiny` |
| 153 | 3.1% | `<deduplicated_symbol>` |
| 136 | 2.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 122 | 2.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 114 | 2.3% | `_xzm_free_main` |
| 91 | 1.8% | `_platform_memmove` |
| 84 | 1.7% | `radiomics::features::texture::compute::_{{closure}}` |
| 83 | 1.7% | `__psynch_mutexwait` |
| 76 | 1.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 64 | 1.3% | `core::iter::traits::iterator::Iterator::try_fold` |
| 62 | 1.2% | `_xzm_free` |
| 54 | 1.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 44 | 0.9% | `radiomics::features::texture::compute` |
| 41 | 0.8% | `radiomics::numerics::sparse_counts::variance` |
| 40 | 0.8% | `radiomics::numerics::sparse_counts::marginals` |
| 37 | 0.7% | `xzm_malloc_zone_malloc_type_realloc` |
| 33 | 0.7% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 31 | 0.6% | `xzm_malloc_zone_size` |
| 24 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 24 | 0.5% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 23 | 0.5% | `DYLD-STUB$$memcmp` |
| 22 | 0.4% | `core::hash::BuildHasher::hash_one` |
| 22 | 0.4% | `_xzm_malloc` |
| 19 | 0.4% | `_malloc_zone_malloc` |
| 19 | 0.4% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 12 | 0.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 12 | 0.2% | `_free` |
| 12 | 0.2% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 12 | 0.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 8 | 0.2% | `_malloc_zone_realloc` |
| 8 | 0.2% | `DYLD-STUB$$memmove` |
| 8 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 7 | 0.1% | `_<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop` |
| 7 | 0.1% | `_RNvCsGIExRX8pES_7___rustc11___rdl_alloc` |
| 6 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 6 | 0.1% | `__floattidf` |
| 6 | 0.1% | `log1p` |
| 6 | 0.1% | `_realloc` |
| 5 | 0.1% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 5 | 0.1% | `_platform_memset_pattern16` |
| 5 | 0.1% | `_pthread_cond_wait` |
| 5 | 0.1% | `_RNvCsGIExRX8pES_7___rustc12___rust_alloc` |
| 5 | 0.1% | `core::str::_<impl str>::split_once` |
| 4 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 4 | 0.1% | `DYLD-STUB$$mach_absolute_time` |
| 4 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 4 | 0.1% | `radiomics::features::distributions::size_statistics` |
| 4 | 0.1% | `ndarray::iterators::Iter<A,D>::new` |
| 4 | 0.1% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 4 | 0.1% | `default_zone_realloc` |
| 3 | 0.1% | `__psynch_cvsignal` |
| 3 | 0.1% | `ndarray::dimension::ndindex::_<impl ndarray::dimension::conversion::IntoDimension for &$u5b$usize$u5d$>::into_dimension` |
| 3 | 0.1% | `crossbeam_epoch::default::with_handle` |
| 3 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2880 | 57.7% | `radiomics::workflows::extraction::extract` |
| 2880 | 57.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2345 | 46.9% | `radiomics::workflows::engine::run_prepared` |
| 2343 | 46.9% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1666 | 33.4% | `radiomics::features::texture::compute` |
| 924 | 18.5% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 535 | 10.7% | `radiomics::imaging::preprocessing::prepare` |
| 330 | 6.6% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 311 | 6.2% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 301 | 6.0% | `radiomics::features::texture::compute::_{{closure}}` |
| 296 | 5.9% | `radiomics::features::distributions::size_statistics` |
| 270 | 5.4% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 265 | 5.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 199 | 4.0% | `radiomics::imaging::preprocessing::gather` |
| 144 | 2.9% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 68 | 1.4% | `radiomics::numerics::sparse_counts::marginals` |
| 47 | 0.9% | `radiomics::numerics::sparse_counts::variance` |
| 27 | 0.5% | `radiomics::imaging::preprocessing::normalize_values` |
| 25 | 0.5% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 24 | 0.5% | `radiomics::imaging::io::HostArray::eq_label` |
| 8 | 0.2% | `radiomics::numerics::sparse_counts::entropy` |
| 3 | 0.1% | `radiomics::features::matrices::Domain::new` |
| 2 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 2 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 1 | 0.0% | `radiomics::voxel::gather::NeighborhoodIndex::valid` |
