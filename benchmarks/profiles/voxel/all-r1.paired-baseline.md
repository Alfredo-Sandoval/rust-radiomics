# Profile: voxel/all-r1.paired-baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 5 1 all`
- Commit: `3ab52d3` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:27 UTC
- Sampling: `sample`, 4 s after 2 s warm-up, 1 ms interval, all threads
- Busy samples: 27725 (idle worker-thread samples excluded: 10788)

## Command output

```text
{"median_ms":4784.912625000001,"samples_ms":[4621.112333,4718.506916,4784.912625000001,5588.659875,6230.892917],"roi_voxels":54639,"maps":147,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 6397 | 23.1% | `radiomics::numerics::eigen::jacobi` |
| 3508 | 12.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1446 | 5.2% | `radiomics::numerics::eigen::maximal_correlation` |
| 1239 | 4.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1108 | 4.0% | `_platform_memmove` |
| 1063 | 3.8% | `__floattidf` |
| 1047 | 3.8% | `_xzm_xzone_malloc_tiny` |
| 771 | 2.8% | `_platform_memcmp` |
| 723 | 2.6% | `_xzm_free_main` |
| 622 | 2.2% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 609 | 2.2% | `mach_absolute_time` |
| 535 | 1.9% | `radiomics::numerics::sparse_counts::marginals` |
| 525 | 1.9% | `log1p` |
| 495 | 1.8% | `hypot` |
| 470 | 1.7% | `radiomics::features::glcm::mutual_information_bits` |
| 436 | 1.6% | `<deduplicated_symbol>` |
| 426 | 1.5% | `radiomics::features::texture::compute::_{{closure}}` |
| 419 | 1.5% | `radiomics::features::glcm::glcm_statistics` |
| 418 | 1.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 373 | 1.3% | `DYLD-STUB$$hypot` |
| 363 | 1.3% | `__psynch_mutexwait` |
| 345 | 1.2% | `_xzm_free` |
| 282 | 1.0% | `radiomics::features::texture::compute` |
| 279 | 1.0% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 204 | 0.7% | `xzm_malloc_zone_size` |
| 194 | 0.7% | `radiomics::numerics::sparse_counts::entropy` |
| 185 | 0.7% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 170 | 0.6% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 163 | 0.6% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 142 | 0.5% | `_malloc_zone_malloc` |
| 130 | 0.5% | `radiomics::numerics::sparse_counts::variance` |
| 128 | 0.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 122 | 0.4% | `DYLD-STUB$$memcmp` |
| 120 | 0.4% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 117 | 0.4% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 114 | 0.4% | `rayon_core::join::join_context::_{{closure}}` |
| 102 | 0.4% | `_realloc` |
| 91 | 0.3% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 88 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 86 | 0.3% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 83 | 0.3% | `_xzm_malloc` |
| 75 | 0.3% | `xzm_malloc_zone_malloc_type_realloc` |
| 74 | 0.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 71 | 0.3% | `core::iter::traits::iterator::Iterator::fold` |
| 70 | 0.3% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 69 | 0.2% | `radiomics::features::ivh::ivh_statistics` |
| 65 | 0.2% | `matrixmultiply::gemm::gemm_loop` |
| 63 | 0.2% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 51 | 0.2% | `radiomics::features::intensity_histogram::compute` |
| 50 | 0.2% | `core::hash::BuildHasher::hash_one` |
| 46 | 0.2% | `_<ndarray::indexes::IndicesIter<D> as core::iter::traits::iterator::Iterator>::fold` |
| 45 | 0.2% | `_malloc_zone_realloc` |
| 44 | 0.2% | `DYLD-STUB$$memmove` |
| 41 | 0.1% | `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle` |
| 36 | 0.1% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 34 | 0.1% | `radiomics::features::glcm::cluster_shade` |
| 34 | 0.1% | `xzm_malloc_zone_try_free_default_main` |
| 31 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 31 | 0.1% | `_free` |
| 31 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 25494 | 92.0% | `radiomics::features::texture::compute` |
| 22855 | 82.4% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 17062 | 61.5% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 13902 | 50.1% | `radiomics::features::glcm::glcm_statistics` |
| 9785 | 35.3% | `radiomics::numerics::eigen::maximal_correlation` |
| 7587 | 27.4% | `radiomics::numerics::eigen::jacobi` |
| 2263 | 8.2% | `radiomics::features::distributions::size_statistics` |
| 2000 | 7.2% | `radiomics::workflows::extraction::extract` |
| 2000 | 7.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1736 | 6.3% | `radiomics::workflows::engine::run_prepared` |
| 1736 | 6.3% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1291 | 4.7% | `radiomics::features::texture::compute::_{{closure}}` |
| 859 | 3.1% | `radiomics::features::glcm::mutual_information_bits` |
| 856 | 3.1% | `radiomics::numerics::sparse_counts::marginals` |
| 531 | 1.9% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 525 | 1.9% | `radiomics::numerics::sparse_counts::entropy` |
| 411 | 1.5% | `radiomics::numerics::sparse_counts::variance` |
| 280 | 1.0% | `radiomics::features::intensity_histogram::compute` |
| 263 | 0.9% | `radiomics::imaging::preprocessing::prepare` |
| 248 | 0.9% | `radiomics::features::distributions::ngtdm_statistics` |
| 190 | 0.7% | `radiomics::features::ivh::ivh_statistics` |
| 139 | 0.5% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 124 | 0.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 93 | 0.3% | `radiomics::imaging::preprocessing::gather` |
| 83 | 0.3% | `radiomics::features::matrices::directions` |
| 61 | 0.2% | `radiomics::features::glcm::cluster_shade` |
| 32 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 27 | 0.1% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 23 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 19 | 0.1% | `radiomics::features::matrices::Domain::new` |
| 10 | 0.0% | `radiomics::features::intensity_histogram::histogram_gradients` |
| 8 | 0.0% | `radiomics::imaging::io::HostArray::eq_label` |
| 4 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 3 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
| 3 | 0.0% | `radiomics::features::aggregation::resolved_aggregation` |
| 2 | 0.0% | `radiomics::features::intensity_histogram::robust_mean_absolute_deviation` |
| 2 | 0.0% | `radiomics::features::intensity_histogram::linear_percentile` |
