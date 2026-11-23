# Profile: texture/ngtdm.baseline

- Command: `target/bench-texture/baseline/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 203 ngtdm_Coarseness ngtdm_Contrast ngtdm_Busyness ngtdm_Complexity ngtdm_Strength`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:45 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2306 (idle worker-thread samples excluded: 0)

## Command output

```text
{"samples_ms":[15.549957999999998,15.785916,15.790709,15.800125000000001,15.823042,15.905209,15.924707999999999,16.004375,16.019958000000003,16.031875,16.04925,16.064791,16.06925,16.086416999999997,16.092292,16.120874999999998,16.12325,16.129084,16.129208,16.193792,16.195542,16.205542,16.223625,16.235416999999998,16.2825,16.286166,16.317584,16.338083,16.385042,16.401208,16.413667,16.449624999999997,16.460917,16.473875,16.486166,16.546917,16.558833,16.559041,16.562708,16.57675,16.586541,16.591874999999998,16.595167,16.598916,16.619834,16.645791,16.647792000000003,16.650624999999998,16.664915999999998,16.677083,16.682542,16.693334,16.693791,16.728292,16.745334,16.753833999999998,16.755917,16.756792,16.758292,16.768459,16.770041,16.798833,16.815333,16.817709,16.859833000000002,16.909292,16.915875,16.933083,16.947625000000002,16.947833999999997,16.948791,16.95675,16.962083,16.969209,16.97025,16.973875,16.978208000000002,16.979791000000002,16.989417,17.022665999999997,17.034958,17.055334,17.058875,17.069666,17.086000000000002,17.103042,17.105708,17.114542,17.116959,17.119,17.12025,17.138666999999998,17.173083,17.180917,17.180999999999997,17.189709,17.220042000000003,17.233792,17.243250000000003,17.258208,17.262166,17.266334,17.284208,17.287125,17.290333,17.295792000000002,17.296709,17.319375,17.327583,17.330541,17.337459,17.344125,17.359125,17.365499999999997,17.370125,17.375167,17.378375000000002,17.381625,17.395999999999997,17.4085,17.434417,17.445541,17.452749999999998,17.458125000000003,17.460625,17.46125,17.466208,17.472917000000002,17.474415999999998,17.482125,17.508665999999998,17.50975,17.53125,17.531625,17.556416,17.56375,17.5805,17.607333999999998,17.613042,17.623709,17.637,17.65175,17.661792,17.664875,17.670708,17.706791,17.712459,17.716458,17.728125000000002,17.747166,17.753791,17.75825,17.790958999999997,17.803,17.834792,17.847917000000002,17.864792,17.899584,17.926000000000002,17.966458,17.986375,17.999208,18.008792,18.031917,18.032167,18.036165999999998,18.048166000000002,18.053917,18.060666,18.077416999999997,18.096500000000002,18.126917,18.231541,18.240334,18.296416999999998,18.306542,18.352,18.358125,18.433334,18.434165999999998,18.448167,18.45025,18.4555,18.468667,18.469791999999998,18.490083000000002,18.493708,18.554917,18.60475,18.617708,18.667334,18.675375000000003,18.699417,18.957,19.017375,19.044959,19.063,19.069416,19.337042,19.364583,19.62075,19.711583,21.519541999999998],"median_ms":17.266334,"features":{"original_ngtdm_Coarseness":0.00011756926620155437,"original_ngtdm_Contrast":0.056041345217520314,"original_ngtdm_Busyness":1.7340097636445129,"original_ngtdm_Complexity":5670.92247067324,"original_ngtdm_Strength":0.680281837886726},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1420 | 61.6% | `radiomics::workflows::roi::bounds::include` |
| 395 | 17.1% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 136 | 5.9% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 87 | 3.8% | `<deduplicated_symbol>` |
| 84 | 3.6% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 32 | 1.4% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 27 | 1.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 23 | 1.0% | `__floattidf` |
| 23 | 1.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 21 | 0.9% | `core::iter::traits::iterator::Iterator::try_fold` |
| 11 | 0.5% | `_platform_memmove` |
| 10 | 0.4% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 8 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 2 | 0.1% | `_xzm_xzone_malloc_tiny` |
| 2 | 0.1% | `mach_absolute_time` |
| 2 | 0.1% | `num_bigint::biguint::shift::biguint_shr2` |
| 2 | 0.1% | `_<num_bigint::biguint::BigUint as num_integer::Integer>::gcd` |
| 2 | 0.1% | `num_bigint::biguint::subtraction::_<impl core::ops::arith::SubAssign<&num_bigint::biguint::BigUint> for num_bigint::biguint::BigUint>::sub_assign` |
| 1 | 0.0% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 1 | 0.0% | `xzm_malloc_zone_size` |
| 1 | 0.0% | `_malloc_zone_realloc` |
| 1 | 0.0% | `radiomics::features::matrices::Domain::new` |
| 1 | 0.0% | `_<core::hash::sip::Hasher<S> as core::hash::Hasher>::write` |
| 1 | 0.0% | `xzm_malloc_zone_malloc_type_realloc` |
| 1 | 0.0% | `_RNvXsZ_NtCs8dnTdrJsiec_5alloc6stringNtB5_6StringNtNtCs6sq8b9ugfBC_4core3fmt5Write9write_str` |
| 1 | 0.0% | `hashbrown::raw::RawTable<T,A>::reserve_rehash` |
| 1 | 0.0% | `_<num_bigint::bigint::BigInt as num_integer::Integer>::div_rem` |
| 1 | 0.0% | `num_bigint::biguint::shift::biguint_shl2` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2299 | 99.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1420 | 61.6% | `radiomics::workflows::engine::inputs::input_window` |
| 1420 | 61.6% | `radiomics::workflows::roi::bounds::include` |
| 684 | 29.7% | `radiomics::workflows::engine::run_prepared` |
| 680 | 29.5% | `radiomics::workflows::roi::compute_roi` |
| 671 | 29.1% | `radiomics::features::texture::compute` |
| 566 | 24.5% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 169 | 7.3% | `radiomics::imaging::preprocessing::prepare` |
| 99 | 4.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 70 | 3.0% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 58 | 2.5% | `radiomics::imaging::preprocessing::gather` |
| 31 | 1.3% | `radiomics::imaging::preprocessing::lattice_bins` |
| 25 | 1.1% | `radiomics::imaging::io::HostArray::crop` |
| 10 | 0.4% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 10 | 0.4% | `radiomics::features::distributions::ngtdm_statistics` |
| 8 | 0.3% | `radiomics::workflows::roi::occupied_window` |
| 5 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 3 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 1 | 0.0% | `radiomics::features::matrices::Domain::new` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::floor_to_bigint` |
| 1 | 0.0% | `radiomics::config::settings::Settings::validate` |
| 1 | 0.0% | `radiomics::config::validation::unique` |
