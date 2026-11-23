# Profile: texture/glcm

- Command: `target/bench-texture/final/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 2318 glcm_Autocorrelation glcm_Contrast glcm_Correlation glcm_DifferenceAverage glcm_JointEnergy glcm_JointEntropy glcm_Id glcm_Idm glcm_JointAverage glcm_ClusterProminence glcm_ClusterShade glcm_ClusterTendency glcm_DifferenceEntropy glcm_DifferenceVariance glcm_Imc1 glcm_Imc2 glcm_Idmn glcm_Idn glcm_InverseVariance glcm_MaximumProbability glcm_SumAverage glcm_SumEntropy glcm_SumSquares glcm_MCC glcm_Dissimilarity glcm_SumVariance`
- Commit: `aa6355a` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:44 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2502 (idle worker-thread samples excluded: 14142)

## Command output

```text
67166,12.759917000000002,12.838917,12.8535,13.062834,13.190125,13.21225,13.277709,13.325750000000001,13.4135,13.651125,13.694916999999998,13.706916,13.797500000000001,13.841041,13.899,13.993959,14.086875000000001,14.135166,14.21475,14.370208,14.382291,14.451042000000001,14.503375,14.544375,14.546667,14.557125000000001,14.583541,14.671750000000001,14.689292,14.713875,14.876999999999999,14.90275,14.996084,15.000083,15.106875,15.146667,15.165334000000001,15.318874999999998,15.469459,15.4915,15.554375,15.556042000000001,15.574208,15.734125000000002,15.901625,15.906417,15.909209,15.927,15.984041000000001,15.985125,16.062417,16.111957999999998,16.18025,16.183042,16.240000000000002,16.274292,16.368292,16.483125,16.485708000000002,16.506916999999998,16.593792,16.6215,16.698,16.765291,16.770167,16.827792000000002,16.947625000000002,17.123584,17.147665999999997,17.525834,17.648500000000002,17.678584,18.056875,18.232833,19.981083,19.984167,20.025707999999998,20.442208,21.194042,21.240499999999997,21.3505,21.380083000000003,21.505125,21.528709,21.712042,21.802625,21.991625,22.077084,22.253709,22.709417,22.765875,22.865083000000002,22.911708,23.033208,23.210749999999997,23.934667,24.438625,24.951125,25.185333,26.314458,27.009583,28.191792,28.642667,28.836208,28.965042,29.444792,30.267333,30.65025,30.844417,31.220875,31.332917000000002,33.099875,33.660083,33.886208,34.0295,35.459292000000005,35.728125,36.859708999999995,37.441625,37.855790999999996,38.537666,38.889792,39.120042,40.16125,40.583417,41.023041000000006,41.153208,42.068667,42.462875,46.369958000000004,51.525667,53.640958000000005,57.535833000000004,61.868416,64.439083,79.62374999999999,82.884125,83.7985,84.625792,90.76720900000001,94.049375,98.83491599999999],"median_ms":2.7902495,"features":{"original_glcm_Autocorrelation":1592.8387107035865,"original_glcm_Contrast":34.84825770092051,"original_glcm_Correlation":0.688783668811451,"original_glcm_DifferenceAverage":3.110845042378862,"original_glcm_JointEnergy":0.033522179311583876,"original_glcm_JointEntropy":6.7208168058740645,"original_glcm_Id":0.49532662441897723,"original_glcm_Idm":0.44699599476286955,"original_glcm_JointAverage":39.40757535228667,"original_glcm_ClusterProminence":449704.3780538093,"original_glcm_ClusterShade":-7644.420769951531,"original_glcm_ClusterTendency":194.17883427130633,"original_glcm_DifferenceEntropy":3.0368680859717774,"original_glcm_DifferenceVariance":24.480649538774134,"original_glcm_Imc1":-0.12844988162404292,"original_glcm_Imc2":0.7266414929796766,"original_glcm_Idmn":0.9951959595644969,"original_glcm_Idn":0.9667179287301619,"original_glcm_InverseVariance":0.3870752452869142,"original_glcm_MaximumProbability":0.08679663920476627,"original_glcm_SumAverage":78.81515070457334,"original_glcm_SumEntropy":4.524789514354813,"original_glcm_SumSquares":57.25677299305672,"original_glcm_MCC":0.7280065443799095,"original_glcm_Dissimilarity":3.110845042378862,"original_glcm_SumVariance":194.17883427130633},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 465 | 18.6% | `__psynch_mutexwait` |
| 319 | 12.7% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 230 | 9.2% | `radiomics::numerics::eigen::maximal_correlation` |
| 176 | 7.0% | `radiomics::features::cooccurrence::glcm_matrix` |
| 91 | 3.6% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 81 | 3.2% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 80 | 3.2% | `__psynch_mutexdrop` |
| 67 | 2.7% | `__psynch_cvsignal` |
| 66 | 2.6% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 65 | 2.6% | `radiomics::features::glcm::mutual_information_bits` |
| 61 | 2.4% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 54 | 2.2% | `crossbeam_epoch::default::with_handle` |
| 53 | 2.1% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 50 | 2.0% | `log1p` |
| 43 | 1.7% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 41 | 1.6% | `rayon_core::join::join_context::_{{closure}}` |
| 37 | 1.5% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 37 | 1.5% | `__floattidf` |
| 29 | 1.2% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 29 | 1.2% | `crossbeam_epoch::internal::Global::try_advance` |
| 28 | 1.1% | `<deduplicated_symbol>` |
| 28 | 1.1% | `radiomics::numerics::sparse_counts::marginals` |
| 25 | 1.0% | `radiomics::features::glcm::glcm_statistics` |
| 25 | 1.0% | `_tlv_get_addr` |
| 23 | 0.9% | `crossbeam_deque::deque::Worker<T>::pop` |
| 21 | 0.8% | `radiomics::features::matrices::Padded::new` |
| 19 | 0.8% | `_platform_memmove` |
| 16 | 0.6% | `mach_absolute_time` |
| 12 | 0.5% | `madvise` |
| 11 | 0.4% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 875 | 35.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 596 | 23.8% | `radiomics::features::aggregation::evaluate` |
| 596 | 23.8% | `radiomics::features::glcm::glcm_statistics` |
| 383 | 15.3% | `radiomics::workflows::engine::run_prepared_at` |
| 376 | 15.0% | `radiomics::workflows::roi::compute_roi` |
| 348 | 13.9% | `radiomics::features::texture::compute` |
| 322 | 12.9% | `radiomics::numerics::eigen::maximal_correlation` |
| 314 | 12.5% | `radiomics::imaging::preprocessing::prepare` |
| 311 | 12.4% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 311 | 12.4% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 230 | 9.2% | `radiomics::features::cooccurrence::glcm_matrix` |
| 178 | 7.1% | `radiomics::workflows::roi::bounds::include` |
| 162 | 6.5% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 154 | 6.2% | `radiomics::workflows::engine::inputs::input_window` |
| 152 | 6.1% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 139 | 5.6% | `radiomics::imaging::preprocessing::gather` |
| 138 | 5.5% | `radiomics::features::matrices::matrices` |
| 102 | 4.1% | `radiomics::imaging::preprocessing::lattice_bins` |
| 99 | 4.0% | `radiomics::features::glcm::mutual_information_bits` |
| 90 | 3.6% | `radiomics::imaging::preprocessing::true_positions` |
| 89 | 3.6% | `radiomics::imaging::preprocessing::gather_at` |
| 88 | 3.5% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 52 | 2.1% | `radiomics::features::matrices::DenseCounts::into_matrix` |
| 42 | 1.7% | `radiomics::numerics::sparse_counts::marginals` |
| 31 | 1.2% | `radiomics::features::matrices::Padded::new` |
| 26 | 1.0% | `radiomics::numerics::sparse_counts::entropy` |
| 24 | 1.0% | `radiomics::workflows::roi::occupied_window` |
| 22 | 0.9% | `radiomics::imaging::io::HostArray::crop` |
| 16 | 0.6% | `radiomics::numerics::sparse_counts::Marginal::accumulate` |
| 7 | 0.3% | `radiomics::imaging::io::HostArray::eq_label` |
