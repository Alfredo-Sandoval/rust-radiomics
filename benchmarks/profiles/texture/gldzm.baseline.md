# Profile: texture/gldzm.baseline

- Command: `target/bench-texture/baseline/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 359 gldzm_SmallDistanceEmphasis gldzm_LargeDistanceEmphasis gldzm_LowGrayLevelZoneEmphasis gldzm_HighGrayLevelZoneEmphasis gldzm_SmallDistanceLowGrayLevelEmphasis gldzm_SmallDistanceHighGrayLevelEmphasis gldzm_LargeDistanceLowGrayLevelEmphasis gldzm_LargeDistanceHighGrayLevelEmphasis gldzm_GrayLevelNonUniformity gldzm_GrayLevelNonUniformityNormalized gldzm_ZoneDistanceNonUniformity gldzm_ZoneDistanceNonUniformityNormalized gldzm_ZonePercentage gldzm_GrayLevelVariance gldzm_ZoneDistanceVariance gldzm_ZoneDistanceEntropy`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:44 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3386 (idle worker-thread samples excluded: 0)

## Command output

```text
20.811459,20.81175,20.822042,20.825583,20.847916,20.851000000000003,20.861917000000002,20.8675,20.868417,20.883959,20.901208,20.906916000000002,20.914958000000002,20.9595,20.969624999999997,20.982457999999998,20.9915,20.995707999999997,21.007749999999998,21.02525,21.026542,21.0355,21.04425,21.055292,21.055750000000003,21.056833,21.067875,21.076625,21.081374999999998,21.109958000000002,21.125459,21.129709,21.160792,21.164083,21.166207999999997,21.174166,21.187791,21.187833,21.18925,21.193458,21.209458,21.217458,21.230333,21.233874999999998,21.253417,21.254042000000002,21.276917,21.304,21.316125,21.319208999999997,21.326709,21.339792,21.349292,21.3565,21.360459,21.364833,21.376083,21.378125,21.379708,21.38,21.391750000000002,21.402041999999998,21.417042000000002,21.427041,21.430083,21.453917,21.464833,21.475749999999998,21.508958000000003,21.509041999999997,21.510082999999998,21.579958,21.580624999999998,21.613500000000002,21.623165999999998,21.630833,21.633916,21.671916,21.700666000000002,21.703167,21.718500000000002,21.728084000000003,21.7375,21.740958000000003,21.803625,21.809709,21.847792000000002,21.8805,21.886292,21.896791,21.911042,21.9145,21.923708,21.943917,21.984959,21.988667,22.027292,22.056375,22.070709,22.091417,22.096332999999998,22.119125,22.11975,22.129458,22.1315,22.144458,22.153083,22.161666999999998,22.203542,22.218999999999998,22.249624999999998,22.306791,22.317083,22.322374999999997,22.336624999999998,22.379042,22.382708,22.447958,22.4635,22.502208,22.52325,22.7285,22.761625000000002,22.77375,22.794917,22.826958,22.962333,23.00475,23.072167,23.078083,23.097666999999998,23.176375,23.20375,23.209042,23.330333000000003,23.366667,23.392625,23.4835,23.648583000000002,23.885666999999998,23.984125,24.023042,24.090500000000002,24.116708,24.177625000000003,24.23225,24.50525,24.785792,24.894333,25.014249999999997,25.063042,26.203042,27.073875,27.082124999999998,27.494457999999998,28.938583,29.6425],"median_ms":20.632208,"features":{"original_gldzm_SmallDistanceEmphasis":0.6376309546307348,"original_gldzm_LargeDistanceEmphasis":8.487514863258026,"original_gldzm_LowGrayLevelZoneEmphasis":0.0033048559257166784,"original_gldzm_HighGrayLevelZoneEmphasis":1047.0274398609713,"original_gldzm_SmallDistanceLowGrayLevelEmphasis":0.0015352089387028504,"original_gldzm_SmallDistanceHighGrayLevelEmphasis":610.0135363310085,"original_gldzm_LargeDistanceLowGrayLevelEmphasis":0.07885998921217101,"original_gldzm_LargeDistanceHighGrayLevelEmphasis":9150.351779017654,"original_gldzm_GrayLevelNonUniformity":350.7757248696607,"original_gldzm_GrayLevelNonUniformityNormalized":0.032084123741851336,"original_gldzm_ZoneDistanceNonUniformity":4128.991585109302,"original_gldzm_ZoneDistanceNonUniformityNormalized":0.37766318349120115,"original_gldzm_ZonePercentage":0.2000951701165834,"original_gldzm_GrayLevelVariance":103.29083199852047,"original_gldzm_ZoneDistanceVariance":3.737483396391024,"original_gldzm_ZoneDistanceEntropy":7.1203522374023525},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1596 | 47.1% | `radiomics::workflows::roi::bounds::include` |
| 1328 | 39.2% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 109 | 3.2% | `<deduplicated_symbol>` |
| 92 | 2.7% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 74 | 2.2% | `_<ndarray::indexes::IndicesIter<D> as core::iter::traits::iterator::Iterator>::fold` |
| 52 | 1.5% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 30 | 0.9% | `radiomics::imaging::preprocessing::lattice_bins` |
| 18 | 0.5% | `core::iter::traits::iterator::Iterator::try_fold` |
| 18 | 0.5% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 8 | 0.2% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 6 | 0.2% | `_platform_memmove` |
| 5 | 0.1% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 5 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 4 | 0.1% | `madvise` |
| 3 | 0.1% | `radiomics::numerics::sparse_counts::marginals` |
| 3 | 0.1% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 3 | 0.1% | `_platform_memcmp` |
| 2 | 0.1% | `_malloc_zone_realloc` |
| 2 | 0.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &mut F>::call_mut` |
| 2 | 0.1% | `_<num_bigint::biguint::BigUint as num_integer::Integer>::gcd` |
| 2 | 0.1% | `num_bigint::biguint::shift::_<impl core::ops::bit::ShrAssign<u64> for num_bigint::biguint::BigUint>::shr_assign` |
| 1 | 0.0% | `radiomics::workflows::engine::inputs::input_window` |
| 1 | 0.0% | `_xzm_segment_group_span_mark_smaller` |
| 1 | 0.0% | `malloc` |
| 1 | 0.0% | `_xzm_xzone_malloc_tiny` |
| 1 | 0.0% | `xzm_malloc_zone_try_free_default_main` |
| 1 | 0.0% | `_xzm_reclaim_mark_used` |
| 1 | 0.0% | `radiomics::features::matrices::Domain::new` |
| 1 | 0.0% | `_xzm_chunk_list_pop` |
| 1 | 0.0% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3380 | 99.8% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1597 | 47.2% | `radiomics::workflows::engine::inputs::input_window` |
| 1596 | 47.1% | `radiomics::workflows::roi::bounds::include` |
| 1570 | 46.4% | `radiomics::workflows::engine::run_prepared` |
| 1566 | 46.2% | `radiomics::workflows::roi::compute_roi` |
| 1548 | 45.7% | `radiomics::features::texture::compute` |
| 1424 | 42.1% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 189 | 5.6% | `radiomics::imaging::preprocessing::prepare` |
| 96 | 2.8% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 93 | 2.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 85 | 2.5% | `radiomics::imaging::preprocessing::gather` |
| 39 | 1.2% | `radiomics::imaging::preprocessing::lattice_bins` |
| 21 | 0.6% | `radiomics::imaging::io::HostArray::crop` |
| 16 | 0.5% | `radiomics::workflows::roi::occupied_window` |
| 11 | 0.3% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 8 | 0.2% | `radiomics::features::distributions::size_statistics` |
| 5 | 0.1% | `radiomics::config::settings::Settings::validate` |
| 3 | 0.1% | `radiomics::numerics::sparse_counts::marginals` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 3 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
| 1 | 0.0% | `radiomics::features::matrices::Domain::new` |
| 1 | 0.0% | `radiomics::workflows::engine::aggregation` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::fraction_to_f64` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::validate_exact_float64` |
