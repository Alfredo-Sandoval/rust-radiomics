# Profile: texture/gldm.baseline

- Command: `target/bench-texture/baseline/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 445 gldm_SmallDependenceEmphasis gldm_LargeDependenceEmphasis gldm_GrayLevelNonUniformity gldm_DependenceNonUniformity gldm_DependenceNonUniformityNormalized gldm_GrayLevelVariance gldm_DependenceVariance gldm_DependenceEntropy gldm_LowGrayLevelEmphasis gldm_HighGrayLevelEmphasis gldm_SmallDependenceLowGrayLevelEmphasis gldm_SmallDependenceHighGrayLevelEmphasis gldm_LargeDependenceLowGrayLevelEmphasis gldm_LargeDependenceHighGrayLevelEmphasis gldm_GrayLevelNonUniformityNormalized gldm_DependenceCountEnergy gldm_DependencePercentage`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:44 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3377 (idle worker-thread samples excluded: 0)

## Command output

```text
9998,18.421208,18.444499999999998,18.446875000000002,18.453457999999998,18.458458,18.4595,18.483916999999998,18.488083999999997,18.521,18.532584,18.550916,18.596708,18.630417,18.641167,18.649791,18.677167,18.684583,18.706417,18.726208,18.756958,18.758625000000002,18.776874999999997,18.782291999999998,18.789209,18.864375,18.884708,18.891499999999997,18.928375,18.961334,18.969167,18.974708,19.021167,19.028541999999998,19.054499999999997,19.066708000000002,19.070583,19.076542,19.09225,19.111542,19.243459,19.277083,19.328625,19.374833000000002,19.549291999999998,19.555084,19.583958,19.626917000000002,19.715291,19.95075,19.970583,19.9865,20.018583,20.057334,20.087375,20.200583,20.202875,20.222333,20.230125,20.279417000000002,20.313042,20.34075,20.412167,20.514042,20.561374999999998,20.743125000000003,20.752083000000002,20.936667,20.944582999999998,20.990499999999997,21.045209,21.220709,21.488084,21.681583,21.738125,21.834208,21.914459,22.098583,22.590542000000003,22.861083,22.917459,23.130375,23.135333,23.316042,23.318749999999998,23.604584000000003,23.911041,24.018292,24.10425,24.202125000000002,24.246167,24.522125,24.600375,24.7975,24.85275,25.189,25.236333,25.627667,25.666833999999998,26.099584,26.348083000000003,26.472292000000003,26.532792,26.751,26.839792000000003,26.922083,27.503542,27.88375,28.2055,28.222875000000002,28.285,28.418542000000002,28.464416999999997,28.52875,29.15475,29.311500000000002,29.597709,30.663375,30.739292000000003,30.869,31.377625,31.682792000000003,31.962999999999997,32.21575,32.725958,32.863333000000004,32.864583,33.034541,34.156292,34.591667,35.186084,36.800958,36.9135,37.930749999999996,38.535958,39.158125000000005,40.838291999999996,41.29975,43.032083,44.935041000000005,45.041292,45.273958,45.695708,46.474584,46.816125,48.521291,51.014834,51.348083,52.964125,55.388374999999996,55.71,55.766791000000005,67.009625,70.950167,80.98920799999999,92.124375],"median_ms":17.426417,"features":{"original_gldm_SmallDependenceEmphasis":0.18926638333660392,"original_gldm_LargeDependenceEmphasis":63.926481084939326,"original_gldm_GrayLevelNonUniformity":7356.928951847582,"original_gldm_DependenceNonUniformity":4120.518915060671,"original_gldm_DependenceNonUniformityNormalized":0.07541351260199987,"original_gldm_GrayLevelVariance":65.88841585097525,"original_gldm_DependenceVariance":20.042736774819456,"original_gldm_DependenceEntropy":7.047659645783571,"original_gldm_LowGrayLevelEmphasis":0.002951469402579746,"original_gldm_HighGrayLevelEmphasis":1570.934753564304,"original_gldm_SmallDependenceLowGrayLevelEmphasis":0.0005612729945968891,"original_gldm_SmallDependenceHighGrayLevelEmphasis":206.95588096262185,"original_gldm_LargeDependenceLowGrayLevelEmphasis":0.11247451941972218,"original_gldm_LargeDependenceHighGrayLevelEmphasis":113459.4919745969,"original_gldm_GrayLevelNonUniformityNormalized":0.13464611270059082,"original_gldm_DependenceCountEnergy":0.010976642977034273,"original_gldm_DependencePercentage":1.0},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1966 | 58.2% | `radiomics::workflows::roi::bounds::include` |
| 626 | 18.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 313 | 9.3% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 132 | 3.9% | `<deduplicated_symbol>` |
| 123 | 3.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 52 | 1.5% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 30 | 0.9% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 29 | 0.9% | `radiomics::imaging::preprocessing::lattice_bins` |
| 17 | 0.5% | `core::iter::traits::iterator::Iterator::try_fold` |
| 16 | 0.5% | `_platform_memmove` |
| 13 | 0.4% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 12 | 0.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 5 | 0.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 4 | 0.1% | `_xzm_xzone_malloc_tiny` |
| 4 | 0.1% | `_<num_bigint::biguint::BigUint as num_integer::Integer>::gcd` |
| 4 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 3 | 0.1% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 3 | 0.1% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 2 | 0.1% | `num_bigint::bigint::addition::_<impl core::ops::arith::Add for num_bigint::bigint::BigInt>::add` |
| 2 | 0.1% | `mach_absolute_time` |
| 1 | 0.0% | `_xzm_malloc` |
| 1 | 0.0% | `radiomics::numerics::sparse_counts::marginals` |
| 1 | 0.0% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 1 | 0.0% | `radiomics::features::texture::compute` |
| 1 | 0.0% | `_<core::iter::adapters::filter::Filter<I,P> as core::iter::traits::iterator::Iterator>::count` |
| 1 | 0.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &mut F>::call_mut` |
| 1 | 0.0% | `radiomics::config::settings::Settings::validate` |
| 1 | 0.0% | `radiomics::features::morphology::require_pairwise_admission_with` |
| 1 | 0.0% | `_platform_memcmp` |
| 1 | 0.0% | `num_bigint::biguint::subtraction::_<impl core::ops::arith::SubAssign<&num_bigint::biguint::BigUint> for num_bigint::biguint::BigUint>::sub_assign` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3367 | 99.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1966 | 58.2% | `radiomics::workflows::engine::inputs::input_window` |
| 1966 | 58.2% | `radiomics::workflows::roi::bounds::include` |
| 1109 | 32.8% | `radiomics::workflows::engine::run_prepared` |
| 1099 | 32.5% | `radiomics::workflows::roi::compute_roi` |
| 1090 | 32.3% | `radiomics::features::texture::compute` |
| 957 | 28.3% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 246 | 7.3% | `radiomics::imaging::preprocessing::prepare` |
| 140 | 4.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 106 | 3.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 83 | 2.5% | `radiomics::imaging::preprocessing::gather` |
| 43 | 1.3% | `radiomics::imaging::io::HostArray::crop` |
| 41 | 1.2% | `radiomics::imaging::preprocessing::lattice_bins` |
| 8 | 0.2% | `radiomics::workflows::roi::occupied_window` |
| 8 | 0.2% | `radiomics::config::settings::Settings::validate` |
| 7 | 0.2% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 7 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 6 | 0.2% | `radiomics::features::distributions::size_statistics` |
| 6 | 0.2% | `radiomics::config::validation::unique` |
| 6 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 4 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 2 | 0.1% | `radiomics::numerics::sparse_counts::marginals` |
| 1 | 0.0% | `radiomics::features::morphology::require_pairwise_admission` |
| 1 | 0.0% | `radiomics::features::morphology::require_pairwise_admission_with` |
| 1 | 0.0% | `radiomics::workflows::engine::aggregation` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
