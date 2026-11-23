# Profile: voxel/defaults-r1.baseline

- Command: `target/bench-voxel/baseline/benchmark_voxel ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/image.nrrd ../mlx-radiomics/data/nsclc-radiomics/LUNG1-001/mask.nrrd 3 1 defaults`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:48 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 35312 (idle worker-thread samples excluded: 4487)

## Command output

```text
{"median_ms":5368.481250000001,"samples_ms":[5135.21275,5368.481250000001,7592.395084],"roi_voxels":54639,"maps":93,"radius":1}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 9583 | 27.1% | `radiomics::numerics::eigen::jacobi` |
| 3654 | 10.3% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1932 | 5.5% | `radiomics::numerics::eigen::maximal_correlation` |
| 1681 | 4.8% | `__floattidf` |
| 1621 | 4.6% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1358 | 3.8% | `_xzm_xzone_malloc_tiny` |
| 918 | 2.6% | `_xzm_free_main` |
| 913 | 2.6% | `radiomics::numerics::sparse_counts::marginals` |
| 872 | 2.5% | `_platform_memmove` |
| 864 | 2.4% | `log1p` |
| 790 | 2.2% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 707 | 2.0% | `hypot` |
| 685 | 1.9% | `radiomics::features::glcm::mutual_information_bits` |
| 647 | 1.8% | `_platform_memcmp` |
| 606 | 1.7% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 576 | 1.6% | `DYLD-STUB$$hypot` |
| 575 | 1.6% | `radiomics::features::glcm::glcm_statistics` |
| 462 | 1.3% | `<deduplicated_symbol>` |
| 458 | 1.3% | `radiomics::features::texture::compute::_{{closure}}` |
| 456 | 1.3% | `_xzm_free` |
| 415 | 1.2% | `mach_absolute_time` |
| 322 | 0.9% | `radiomics::numerics::sparse_counts::variance` |
| 296 | 0.8% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 283 | 0.8% | `radiomics::features::texture::compute` |
| 270 | 0.8% | `radiomics::numerics::sparse_counts::entropy` |
| 253 | 0.7% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 228 | 0.6% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 224 | 0.6% | `_free` |
| 201 | 0.6% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 192 | 0.5% | `xzm_malloc_zone_malloc_type_realloc` |
| 191 | 0.5% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 189 | 0.5% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 188 | 0.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 182 | 0.5% | `_malloc_zone_realloc` |
| 159 | 0.5% | `_<alloc::collections::btree::map::BTreeMap<K,V,A> as core::ops::drop::Drop>::drop` |
| 156 | 0.4% | `matrixmultiply::gemm::gemm_loop` |
| 149 | 0.4% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 122 | 0.3% | `_xzm_malloc` |
| 111 | 0.3% | `DYLD-STUB$$log1p` |
| 105 | 0.3% | `DYLD-STUB$$memcmp` |
| 86 | 0.2% | `_malloc_zone_malloc` |
| 85 | 0.2% | `radiomics::features::glcm::cluster_shade` |
| 83 | 0.2% | `matrixmultiply::gemm::masked_kernel` |
| 74 | 0.2% | `core::iter::traits::iterator::Iterator::fold` |
| 70 | 0.2% | `xzm_malloc_zone_size` |
| 68 | 0.2% | `_<alloc::collections::btree::map::Iter<K,V> as core::iter::traits::iterator::Iterator>::next` |
| 65 | 0.2% | `alloc::raw_vec::RawVecInner<A>::finish_grow` |
| 63 | 0.2% | `ndarray::iterators::to_vec_mapped` |
| 61 | 0.2% | `_xzm_xzone_malloc_freelist_outlined` |
| 57 | 0.2% | `alloc::raw_vec::RawVec<T,A>::grow_one` |
| 49 | 0.1% | `core::str::_<impl str>::split_once` |
| 47 | 0.1% | `_<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold` |
| 45 | 0.1% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 43 | 0.1% | `expm1` |
| 40 | 0.1% | `ndarray::dimension::dimension_trait::Dimension::default_strides` |
| 38 | 0.1% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 35 | 0.1% | `core::hash::BuildHasher::hash_one` |
| 34 | 0.1% | `_realloc` |
| 33 | 0.1% | `radiomics::features::matrices::directions` |
| 31 | 0.1% | `radiomics::numerics::sparse_counts::weighted_sum` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 33815 | 95.8% | `radiomics::features::texture::compute` |
| 30911 | 87.5% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 25270 | 71.6% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 21659 | 61.3% | `radiomics::features::glcm::glcm_statistics` |
| 15153 | 42.9% | `radiomics::numerics::eigen::maximal_correlation` |
| 11288 | 32.0% | `radiomics::numerics::eigen::jacobi` |
| 2493 | 7.1% | `radiomics::features::distributions::size_statistics` |
| 2355 | 6.7% | `radiomics::workflows::extraction::extract` |
| 2355 | 6.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2355 | 6.7% | `radiomics::workflows::engine::run_prepared` |
| 2355 | 6.7% | `radiomics::voxel::extraction::extract_voxels_with_sink` |
| 1372 | 3.9% | `radiomics::numerics::sparse_counts::marginals` |
| 1190 | 3.4% | `radiomics::features::texture::compute::_{{closure}}` |
| 1023 | 2.9% | `radiomics::numerics::sparse_counts::entropy` |
| 993 | 2.8% | `radiomics::features::glcm::mutual_information_bits` |
| 590 | 1.7% | `radiomics::voxel::extraction::extract_voxels_with_sink::_{{closure}}::_{{closure}}` |
| 567 | 1.6% | `radiomics::numerics::sparse_counts::variance` |
| 231 | 0.7% | `radiomics::features::glcm::cluster_shade` |
| 151 | 0.4% | `radiomics::features::distributions::ngtdm_statistics` |
| 71 | 0.2% | `radiomics::features::matrices::directions` |
| 31 | 0.1% | `radiomics::numerics::sparse_counts::weighted_sum` |
| 28 | 0.1% | `radiomics::workflows::engine::run_prepared::_{{closure}}` |
| 27 | 0.1% | `radiomics::features::aggregation::resolved_aggregation` |
| 9 | 0.0% | `radiomics::voxel::gather::NeighborhoodPlan::index` |
| 8 | 0.0% | `radiomics::features::selection::explicit_texture` |
| 1 | 0.0% | `radiomics::features::matrices::Domain::new` |
