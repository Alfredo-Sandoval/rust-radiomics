# Profile: texture/glrlm.baseline

- Command: `target/bench-texture/baseline/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 154 glrlm_ShortRunEmphasis glrlm_LongRunEmphasis glrlm_GrayLevelNonUniformity glrlm_GrayLevelNonUniformityNormalized glrlm_RunLengthNonUniformity glrlm_RunLengthNonUniformityNormalized glrlm_RunPercentage glrlm_GrayLevelVariance glrlm_RunVariance glrlm_RunEntropy glrlm_LowGrayLevelRunEmphasis glrlm_HighGrayLevelRunEmphasis glrlm_ShortRunLowGrayLevelEmphasis glrlm_ShortRunHighGrayLevelEmphasis glrlm_LongRunLowGrayLevelEmphasis glrlm_LongRunHighGrayLevelEmphasis`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:43 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3344 (idle worker-thread samples excluded: 0)

## Command output

```text
2,26.977583,27.065291,27.117416,27.419249999999998,27.573166999999998,28.013916,28.028707999999998,28.063583,28.185291,28.264583,28.280875,28.3825,28.389166,28.404291,28.484125,28.499291,28.583750000000002,28.594958000000002,28.605708,28.643458,28.733791,28.77375,28.7825,28.783209,28.817667,28.833375,28.986375000000002,28.999959,29.117167,29.127083000000002,29.136792000000003,29.209875,29.215333,29.241291999999998,29.278708,29.297083,29.313083,29.3875,29.407292,29.413124999999997,29.445583,29.520709,29.799875,29.804167,29.834916999999997,29.846833999999998,29.882458,29.923666,30.047250000000002,30.126375,30.243875,30.272582999999997,30.346709,30.383958,30.425957999999998,30.431542,30.490625,30.513292,30.745583,30.779999999999998,30.833167000000003,30.842499999999998,30.874917000000003,30.9655,31.112917,31.127458999999998,31.131542000000003,31.148540999999998,31.244541,31.249,31.343000000000004,31.34575,31.349875,31.383292,31.414040999999997,31.579334,31.611583999999997,31.630291999999997,31.671625000000002,31.709834,31.754042000000002,31.759625,31.806167000000002,31.815999999999995,31.818541999999997,31.825542,31.848957999999996,31.901791999999997,31.939500000000002,31.960666,32.000042,32.004834,32.014041999999996,32.030583,32.068041,32.203625,32.206125,32.217416,32.24375,32.292874999999995,32.321709,32.3255,32.346584,32.349791,32.392125,32.435958,32.456125,32.486709000000005,32.584624999999996,32.695375,32.701208,32.725875,32.738167,32.758250000000004,32.813500000000005,32.844125000000005,32.896375,32.934290999999995,32.946000000000005,33.007333,33.04125,33.210917,33.366708,33.387750000000004,33.49525,33.836999999999996,34.08625,34.099542,34.254917000000006,34.696290999999995,34.812708,35.0885,35.611375,36.203083,39.010375,42.478541,43.175834,43.388416,44.224166000000004,44.692583,44.854084,46.480291,47.956667,49.382374999999996,58.204125000000005,59.606041999999995,62.613625000000006,93.00224999999999,101.955333,103.535708,104.258458,110.165417,114.990458],"median_ms":31.595458999999998,"features":{"original_glrlm_ShortRunEmphasis":0.8466860066657029,"original_glrlm_LongRunEmphasis":2.135824732831354,"original_glrlm_GrayLevelNonUniformity":4637.710874115572,"original_glrlm_GrayLevelNonUniformityNormalized":0.10806989219887199,"original_glrlm_RunLengthNonUniformity":29085.237038438234,"original_glrlm_RunLengthNonUniformityNormalized":0.6750052178210483,"original_glrlm_RunPercentage":0.783673819911672,"original_glrlm_GrayLevelVariance":75.93321316315883,"original_glrlm_RunVariance":0.4821143416653727,"original_glrlm_RunEntropy":4.917357267623147,"original_glrlm_LowGrayLevelRunEmphasis":0.0031531710259638146,"original_glrlm_HighGrayLevelRunEmphasis":1518.3791561676444,"original_glrlm_ShortRunLowGrayLevelEmphasis":0.002788483673992149,"original_glrlm_ShortRunHighGrayLevelEmphasis":1250.2605500773027,"original_glrlm_LongRunLowGrayLevelEmphasis":0.005552685887671641,"original_glrlm_LongRunHighGrayLevelEmphasis":3521.0504151107157},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1834 | 54.8% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1150 | 34.4% | `radiomics::workflows::roi::bounds::include` |
| 79 | 2.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 62 | 1.9% | `<deduplicated_symbol>` |
| 57 | 1.7% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 44 | 1.3% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 37 | 1.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 14 | 0.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 12 | 0.4% | `core::iter::traits::iterator::Iterator::try_fold` |
| 11 | 0.3% | `_malloc_zone_realloc` |
| 10 | 0.3% | `_platform_memmove` |
| 6 | 0.2% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 3 | 0.1% | `radiomics::workflows::engine::inputs::input_window` |
| 3 | 0.1% | `_<num_bigint::biguint::BigUint as num_integer::Integer>::gcd` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 2 | 0.1% | `_xzm_xzone_malloc_tiny` |
| 2 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 1 | 0.0% | `__floattidf` |
| 1 | 0.0% | `radiomics::numerics::sparse_counts::marginals` |
| 1 | 0.0% | `radiomics::features::distributions::size_statistics` |
| 1 | 0.0% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 1 | 0.0% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 1 | 0.0% | `radiomics::config::settings::Family::parse` |
| 1 | 0.0% | `_<hashbrown::map::HashMap<K,V,S,A> as core::iter::traits::collect::Extend<$LP$K,V$RP$>>::extend` |
| 1 | 0.0% | `num_bigint::biguint::BigUint::trailing_zeros` |
| 1 | 0.0% | `num_bigint::biguint::shift::biguint_shr2` |
| 1 | 0.0% | `_xzm_xzone_malloc_small_freelist` |
| 1 | 0.0% | `_xzm_malloc` |
| 1 | 0.0% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3333 | 99.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 2019 | 60.4% | `radiomics::workflows::engine::run_prepared` |
| 2018 | 60.3% | `radiomics::workflows::roi::compute_roi` |
| 2012 | 60.2% | `radiomics::features::texture::compute` |
| 1897 | 56.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1153 | 34.5% | `radiomics::workflows::engine::inputs::input_window` |
| 1150 | 34.4% | `radiomics::workflows::roi::bounds::include` |
| 134 | 4.0% | `radiomics::imaging::preprocessing::prepare` |
| 94 | 2.8% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 44 | 1.3% | `radiomics::imaging::preprocessing::lattice_bins` |
| 40 | 1.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 35 | 1.0% | `radiomics::imaging::preprocessing::gather` |
| 25 | 0.7% | `radiomics::imaging::io::HostArray::crop` |
| 23 | 0.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 21 | 0.6% | `radiomics::features::distributions::size_statistics` |
| 4 | 0.1% | `radiomics::workflows::roi::occupied_window` |
| 2 | 0.1% | `radiomics::numerics::sparse_counts::marginals` |
| 1 | 0.0% | `radiomics::numerics::sparse_counts::variance` |
| 1 | 0.0% | `radiomics::config::settings::Family::parse` |
| 1 | 0.0% | `radiomics::config::settings::Settings::validate` |
| 1 | 0.0% | `radiomics::config::validation::unique` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::normalize_values` |
| 1 | 0.0% | `radiomics::imaging::io::HostArray::eq_label` |
