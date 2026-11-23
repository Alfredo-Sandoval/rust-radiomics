# Profile: texture/glcm.try

- Command: `target/bench-texture/cur/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 250 glcm_Autocorrelation glcm_Contrast glcm_Correlation glcm_DifferenceAverage glcm_JointEnergy glcm_JointEntropy glcm_Id glcm_Idm glcm_JointAverage glcm_ClusterProminence glcm_ClusterShade glcm_ClusterTendency glcm_DifferenceEntropy glcm_DifferenceVariance glcm_Imc1 glcm_Imc2 glcm_Idmn glcm_Idn glcm_InverseVariance glcm_MaximumProbability glcm_SumAverage glcm_SumEntropy glcm_SumSquares glcm_MCC glcm_Dissimilarity glcm_SumVariance`
- Commit: `d80f742` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:51 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 1931 (idle worker-thread samples excluded: 26208)

## Command output

```text
96709,6.553708,6.686625,6.7086250000000005,6.753167,6.770875,6.797375,7.300916,7.790417,8.08075,8.082875,8.387458,8.438042,8.495292,8.739916000000001,8.862459,8.884166,9.357249999999999,10.204333,11.364208000000001,11.500833,12.017333,12.366875,12.809458,13.355834,13.391583,13.3955,14.28925,14.35675,16.355375,16.425417,16.684917000000002,16.802083999999997,17.041542,17.499584,18.555333,18.665666,18.967166,19.414041,19.427583,19.790667000000003,21.466167000000002,21.986459,22.14,22.398457999999998,22.682542,24.028542,24.407333,24.591375,25.39525,25.545,25.871499999999997,27.796042,29.493084,30.044042,30.580709000000002,30.974125,32.038125,33.715250000000005,33.961917,35.173875,35.467917,35.701542,35.9655,37.099666000000006,37.147791999999995,38.043333,38.418042,39.204209,39.827374999999996,40.628084,41.131042,41.206042,41.556999999999995,43.419208,46.440292,46.550292,46.801291,46.843542,47.771916,49.728541,50.032709000000004,50.081791,51.123416999999996,51.450207999999996,52.199125,57.912334,60.705166,61.746666999999995,62.966666000000004,65.81020799999999,70.456458,71.067666,72.799458,75.2915,75.299167,76.23866699999999,76.432875,79.514292,79.933875,80.07816700000001,82.07775000000001,86.289042,86.428125,86.46008400000001,89.08675000000001,90.12112499999999,92.893709,93.21870799999999,93.32074999999999,93.58850000000001,96.00341599999999,97.658542,97.979916,98.33129100000001,98.810833,102.06433299999999,103.545625,105.42937500000001,107.859792,110.437375,111.284291,111.33479200000001,113.319834,115.94225,118.939375,119.730209,120.798958,123.440375,125.17658300000001,125.96654199999999,128.07250000000002,130.976292,131.4015,131.83841700000002,134.462459,144.686042,147.535375,160.332709,163.595667,177.524417],"median_ms":8.8733125,"features":{"original_glcm_Autocorrelation":1592.8387107035865,"original_glcm_Contrast":34.84825770092051,"original_glcm_Correlation":0.688783668811451,"original_glcm_DifferenceAverage":3.110845042378862,"original_glcm_JointEnergy":0.033522179311583876,"original_glcm_JointEntropy":6.7208168058740645,"original_glcm_Id":0.49532662441897723,"original_glcm_Idm":0.44699599476286955,"original_glcm_JointAverage":39.40757535228667,"original_glcm_ClusterProminence":449704.3780538093,"original_glcm_ClusterShade":-7644.420769951531,"original_glcm_ClusterTendency":194.17883427130633,"original_glcm_DifferenceEntropy":3.0368680859717774,"original_glcm_DifferenceVariance":24.480649538774134,"original_glcm_Imc1":-0.12844988162404292,"original_glcm_Imc2":0.7266414929796766,"original_glcm_Idmn":0.9951959595644969,"original_glcm_Idn":0.9667179287301619,"original_glcm_InverseVariance":0.3870752452869142,"original_glcm_MaximumProbability":0.08679663920476627,"original_glcm_SumAverage":78.81515070457334,"original_glcm_SumEntropy":4.524789514354813,"original_glcm_SumSquares":57.25677299305672,"original_glcm_MCC":0.7280065443799095,"original_glcm_Dissimilarity":3.110845042378862,"original_glcm_SumVariance":194.17883427130633},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 503 | 26.0% | `radiomics::numerics::eigen::maximal_correlation` |
| 227 | 11.8% | `radiomics::features::cooccurrence::glcm_matrix` |
| 178 | 9.2% | `radiomics::features::glcm::glcm_statistics` |
| 133 | 6.9% | `core::cell::lazy::LazyCell<T,F>::really_init` |
| 131 | 6.8% | `__floattidf` |
| 119 | 6.2% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 87 | 4.5% | `radiomics::numerics::sparse_counts::marginals` |
| 81 | 4.2% | `<deduplicated_symbol>` |
| 64 | 3.3% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 52 | 2.7% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 33 | 1.7% | `hypot` |
| 30 | 1.6% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 23 | 1.2% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 21 | 1.1% | `log1p` |
| 20 | 1.0% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 18 | 0.9% | `radiomics::imaging::preprocessing::lattice_bins` |
| 18 | 0.9% | `DYLD-STUB$$hypot` |
| 17 | 0.9% | `__psynch_mutexwait` |
| 16 | 0.8% | `_platform_memmove` |
| 16 | 0.8% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 14 | 0.7% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 13 | 0.7% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 12 | 0.6% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 12 | 0.6% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 8 | 0.4% | `_<alloc::collections::btree::map::BTreeMap<K,V> as core::iter::traits::collect::FromIterator<$LP$K,V$RP$>>::from_iter` |
| 6 | 0.3% | `core::iter::traits::iterator::Iterator::try_fold` |
| 6 | 0.3% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 5 | 0.3% | `matrixmultiply::threading::RangeChunkParallel<G>::for_each` |
| 5 | 0.3% | `crossbeam_epoch::internal::Global::try_advance` |
| 4 | 0.2% | `_xzm_xzone_malloc_tiny` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1477 | 76.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1243 | 64.4% | `radiomics::features::aggregation::evaluate` |
| 1233 | 63.9% | `radiomics::features::glcm::glcm_statistics` |
| 930 | 48.2% | `radiomics::workflows::engine::run_prepared` |
| 929 | 48.1% | `radiomics::workflows::roi::compute_roi` |
| 914 | 47.3% | `radiomics::features::texture::compute` |
| 863 | 44.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 717 | 37.1% | `radiomics::numerics::eigen::maximal_correlation` |
| 414 | 21.4% | `radiomics::workflows::roi::bounds::include` |
| 400 | 20.7% | `radiomics::workflows::engine::inputs::input_window` |
| 346 | 17.9% | `radiomics::features::matrices::matrices` |
| 260 | 13.5% | `radiomics::features::cooccurrence::glcm_matrix` |
| 140 | 7.3% | `radiomics::imaging::preprocessing::prepare` |
| 98 | 5.1% | `radiomics::numerics::sparse_counts::marginals` |
| 76 | 3.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 64 | 3.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 48 | 2.5% | `radiomics::imaging::preprocessing::gather` |
| 32 | 1.7% | `radiomics::features::matrices::DenseCounts::into_matrix` |
| 21 | 1.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 19 | 1.0% | `radiomics::imaging::io::HostArray::eq_label` |
| 16 | 0.8% | `radiomics::numerics::sparse_counts::entropy` |
| 14 | 0.7% | `radiomics::workflows::roi::occupied_window` |
| 14 | 0.7% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 7 | 0.4% | `radiomics::imaging::io::HostArray::crop` |
| 4 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.2% | `radiomics::numerics::sparse_counts::variance` |
| 1 | 0.1% | `radiomics::features::matrices::Padded::new` |
| 1 | 0.1% | `radiomics::imaging::preprocessing::floor_to_bigint` |
