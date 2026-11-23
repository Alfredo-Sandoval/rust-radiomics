# Profile: texture/glszm.baseline

- Command: `target/bench-texture/baseline/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 370 glszm_SmallAreaEmphasis glszm_LargeAreaEmphasis glszm_GrayLevelNonUniformity glszm_GrayLevelNonUniformityNormalized glszm_SizeZoneNonUniformity glszm_SizeZoneNonUniformityNormalized glszm_ZonePercentage glszm_GrayLevelVariance glszm_ZoneVariance glszm_ZoneEntropy glszm_LowGrayLevelZoneEmphasis glszm_HighGrayLevelZoneEmphasis glszm_SmallAreaLowGrayLevelEmphasis glszm_SmallAreaHighGrayLevelEmphasis glszm_LargeAreaLowGrayLevelEmphasis glszm_LargeAreaHighGrayLevelEmphasis`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:43 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3351 (idle worker-thread samples excluded: 0)

## Command output

```text
19.653042,19.664541,19.671583,19.675334,19.676417,19.6805,19.748791,19.803958,19.824416,19.843334000000002,19.858917,19.873708,19.887083,19.939167,19.945708,19.948833999999998,19.955375,19.966125,19.975875000000002,19.989916,19.993582999999997,20.007625,20.014542,20.032,20.075625,20.077,20.089416999999997,20.090291999999998,20.118084,20.123708,20.133958,20.175167000000002,20.179166,20.217416,20.221667,20.239167,20.243083,20.266209,20.270125,20.282708,20.2865,20.292959,20.301333,20.306458,20.349458000000002,20.357584000000003,20.361041,20.362958,20.390459,20.396417,20.396625,20.422124999999998,20.426917,20.430666,20.432334,20.448125,20.450333,20.462541,20.501834,20.526374999999998,20.582,20.5825,20.582832999999997,20.584834,20.619374999999998,20.629208,20.633334,20.643458,20.644917,20.646833,20.664458,20.675458,20.698999999999998,20.718166,20.718166999999998,20.729999999999997,20.744374999999998,20.749292,20.806042,20.814875,20.821208000000002,20.867375,20.867834,20.869125,20.878500000000003,20.878833,20.879042000000002,20.885709,20.895291,20.895875,20.902875,20.952958000000002,20.954583,20.974708000000003,21.001709,21.006833,21.026333,21.053708,21.059082999999998,21.092834,21.09825,21.103791,21.130166,21.188041000000002,21.207290999999998,21.208875,21.25925,21.268709,21.288375,21.304416,21.306291,21.335875,21.348125,21.367250000000002,21.37125,21.396917000000002,21.403541,21.404583000000002,21.421709,21.424708,21.427083,21.450167,21.478167,21.490624999999998,21.549583000000002,21.559124999999998,21.601042,21.627667,21.678917,21.686375,21.721916,21.740790999999998,21.765375,21.796416999999998,21.827625,22.083458,22.198083,22.496624999999998,22.580833000000002,22.784959,23.017,23.421958,23.718625,24.463584,25.44625,26.904958,27.182291,30.830334,31.415875000000003,32.127207999999996,32.925583,40.334958,41.3205,43.008667,43.576625,45.214375000000004,45.224292,45.3825,45.653,46.368334,47.821833,62.557708000000005,68.92075,84.505625,91.121209,91.202875,133.496791],"median_ms":19.4060625,"features":{"original_glszm_SmallAreaEmphasis":0.7448190600468144,"original_glszm_LargeAreaEmphasis":30740.146254458978,"original_glszm_GrayLevelNonUniformity":350.7757248696607,"original_glszm_GrayLevelNonUniformityNormalized":0.032084123741851336,"original_glszm_SizeZoneNonUniformity":5616.636879173146,"original_glszm_SizeZoneNonUniformityNormalized":0.5137324503039555,"original_glszm_ZonePercentage":0.2000951701165834,"original_glszm_GrayLevelVariance":103.29083199852047,"original_glszm_ZoneVariance":30715.170030016357,"original_glszm_ZoneEntropy":6.681037981121474,"original_glszm_LowGrayLevelZoneEmphasis":0.003304855925716684,"original_glszm_HighGrayLevelZoneEmphasis":1047.0274398609713,"original_glszm_SmallAreaLowGrayLevelEmphasis":0.002038829050764523,"original_glszm_SmallAreaHighGrayLevelEmphasis":740.0292095505583,"original_glszm_LargeAreaLowGrayLevelEmphasis":17.6007721667729,"original_glszm_LargeAreaHighGrayLevelEmphasis":55103461.85603219},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1768 | 52.8% | `radiomics::workflows::roi::bounds::include` |
| 1187 | 35.4% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 113 | 3.4% | `<deduplicated_symbol>` |
| 107 | 3.2% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 34 | 1.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 29 | 0.9% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 23 | 0.7% | `radiomics::imaging::preprocessing::lattice_bins` |
| 19 | 0.6% | `core::iter::traits::iterator::Iterator::try_fold` |
| 12 | 0.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 9 | 0.3% | `_platform_memmove` |
| 7 | 0.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 5 | 0.1% | `num_rational::Ratio<T>::reduce` |
| 5 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 4 | 0.1% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 3 | 0.1% | `radiomics::numerics::sparse_counts::marginals` |
| 2 | 0.1% | `_xzm_segment_group_find_and_allocate_chunk` |
| 2 | 0.1% | `_xzm_xzone_malloc_from_freelist_chunk` |
| 2 | 0.1% | `_platform_memcmp` |
| 1 | 0.0% | `radiomics::workflows::engine::inputs::input_window` |
| 1 | 0.0% | `madvise` |
| 1 | 0.0% | `radiomics::workflows::roi_memory::check_roi_count_buffers` |
| 1 | 0.0% | `core::ops::function::FnOnce::call_once` |
| 1 | 0.0% | `radiomics::workflows::engine::checked_source` |
| 1 | 0.0% | `radiomics::config::settings::Settings::validate` |
| 1 | 0.0% | `_<&T as core::fmt::Display>::fmt` |
| 1 | 0.0% | `hashbrown::raw::RawTable<T,A>::reserve_rehash` |
| 1 | 0.0% | `num_bigint::biguint::shift::biguint_shr2` |
| 1 | 0.0% | `num_bigint::big_digit::BigDigits::normalize` |
| 1 | 0.0% | `num_bigint::biguint::subtraction::_<impl core::ops::arith::SubAssign<&num_bigint::biguint::BigUint> for num_bigint::biguint::BigUint>::sub_assign` |
| 1 | 0.0% | `num_bigint::bigint::division::_<impl core::ops::arith::Div for num_bigint::bigint::BigInt>::div` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3347 | 99.9% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1769 | 52.8% | `radiomics::workflows::engine::inputs::input_window` |
| 1768 | 52.8% | `radiomics::workflows::roi::bounds::include` |
| 1324 | 39.5% | `radiomics::workflows::engine::run_prepared` |
| 1319 | 39.4% | `radiomics::workflows::roi::compute_roi` |
| 1302 | 38.9% | `radiomics::features::texture::compute` |
| 1197 | 35.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 216 | 6.4% | `radiomics::imaging::preprocessing::prepare` |
| 130 | 3.9% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 86 | 2.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 65 | 1.9% | `radiomics::imaging::preprocessing::gather` |
| 36 | 1.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 36 | 1.1% | `radiomics::imaging::io::HostArray::crop` |
| 14 | 0.4% | `radiomics::workflows::roi::occupied_window` |
| 8 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 4 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 3 | 0.1% | `radiomics::features::distributions::size_statistics` |
| 3 | 0.1% | `radiomics::numerics::sparse_counts::marginals` |
| 3 | 0.1% | `radiomics::config::settings::Settings::validate` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 1 | 0.0% | `radiomics::workflows::roi_memory::check_roi_count_buffers` |
| 1 | 0.0% | `radiomics::workflows::engine::checked_source` |
| 1 | 0.0% | `radiomics::workflows::engine::aggregation` |
