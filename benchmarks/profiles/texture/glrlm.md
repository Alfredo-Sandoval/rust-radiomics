# Profile: texture/glrlm

- Command: `target/bench-texture/final/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 1718 glrlm_ShortRunEmphasis glrlm_LongRunEmphasis glrlm_GrayLevelNonUniformity glrlm_GrayLevelNonUniformityNormalized glrlm_RunLengthNonUniformity glrlm_RunLengthNonUniformityNormalized glrlm_RunPercentage glrlm_GrayLevelVariance glrlm_RunVariance glrlm_RunEntropy glrlm_LowGrayLevelRunEmphasis glrlm_HighGrayLevelRunEmphasis glrlm_ShortRunLowGrayLevelEmphasis glrlm_ShortRunHighGrayLevelEmphasis glrlm_LongRunLowGrayLevelEmphasis glrlm_LongRunHighGrayLevelEmphasis`
- Commit: `aa6355a` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:44 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2512 (idle worker-thread samples excluded: 13353)

## Command output

```text
19.603542,19.678292,19.726125,19.828333,19.9135,20.14775,20.187,20.285958,20.305291999999998,20.364083,20.615792000000003,20.63075,20.702416,20.93925,21.011374999999997,21.082333000000002,21.26775,21.313083,21.51375,21.679459,21.7065,21.76875,21.914083,21.979,22.053875,22.105541,22.27375,22.455083000000002,22.494500000000002,22.517417000000002,22.640333,22.677958,22.808083,22.838625,22.90725,22.955584,22.962125,22.964917,23.118917,23.416208,23.682708,23.841207999999998,24.109541,24.136291,24.286625,25.85375,26.333208,26.680290999999997,27.083917,27.108,27.311084,27.622541000000002,27.850291,28.1645,28.205083,28.24775,28.359790999999998,28.5475,28.652292,28.659041,28.773625,28.841833,29.301625,29.313958,29.3395,29.733249999999998,30.276791,31.055875,31.168917,31.229709,31.955833000000002,32.215166,32.3885,32.490959,32.496667,32.635083,32.954541,33.362458999999994,33.522875,34.254958,34.381958,34.461833,34.571458,34.680084,34.773375,35.196875000000006,35.345292,35.360125,35.361291,35.392,35.431458,35.902166,36.137542,36.181041,36.235875,36.262583,36.397334,36.598416,36.679125,36.921583,37.252,37.394458,37.448583,38.4805,39.5755,39.753,40.28337500000001,40.4095,40.687,41.172542,41.276292,41.675042000000005,41.81,42.551167,42.758375,42.835916000000005,44.686416,44.847958,45.0715,45.717375,45.858167,46.441708,46.768499999999996,47.906625,49.134667,49.821999999999996,50.35916700000001,50.464584,50.566708,50.629708,51.476917,51.518875,51.82775,52.084666,53.1385,53.144291,53.598708,53.644999999999996,56.097834,57.583334,58.956792,58.957083,60.272875,60.290583000000005,61.544959000000006,61.819625,63.181374999999996,63.617958,63.84275,64.302084,66.461333,68.440375,70.36637499999999,71.198416,71.657042,72.795875,73.90125,74.476833,74.880375,75.779583,76.73804100000001,77.451167,77.81504100000001,79.45774999999999,79.744708,82.452291,84.431916,88.973333,89.41020800000001,89.56945800000001,90.36616699999999,96.179875,97.326917,108.870959,121.382042,134.790083,139.75233400000002],"median_ms":2.8648545,"features":{"original_glrlm_ShortRunEmphasis":0.8466860066657029,"original_glrlm_LongRunEmphasis":2.135824732831354,"original_glrlm_GrayLevelNonUniformity":4637.710874115572,"original_glrlm_GrayLevelNonUniformityNormalized":0.10806989219887199,"original_glrlm_RunLengthNonUniformity":29085.237038438234,"original_glrlm_RunLengthNonUniformityNormalized":0.6750052178210483,"original_glrlm_RunPercentage":0.783673819911672,"original_glrlm_GrayLevelVariance":75.93321316315883,"original_glrlm_RunVariance":0.4821143416653727,"original_glrlm_RunEntropy":4.917357267623147,"original_glrlm_LowGrayLevelRunEmphasis":0.0031531710259638146,"original_glrlm_HighGrayLevelRunEmphasis":1518.3791561676444,"original_glrlm_ShortRunLowGrayLevelEmphasis":0.002788483673992149,"original_glrlm_ShortRunHighGrayLevelEmphasis":1250.2605500773027,"original_glrlm_LongRunLowGrayLevelEmphasis":0.005552685887671641,"original_glrlm_LongRunHighGrayLevelEmphasis":3521.0504151107157},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 599 | 23.8% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 567 | 22.6% | `radiomics::features::matrices::run_matrix` |
| 364 | 14.5% | `__psynch_mutexwait` |
| 104 | 4.1% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 90 | 3.6% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 85 | 3.4% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 75 | 3.0% | `crossbeam_epoch::default::with_handle` |
| 51 | 2.0% | `rayon_core::join::join_context::_{{closure}}` |
| 50 | 2.0% | `__psynch_mutexdrop` |
| 45 | 1.8% | `__psynch_cvsignal` |
| 39 | 1.6% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 38 | 1.5% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 36 | 1.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 33 | 1.3% | `_platform_memmove` |
| 31 | 1.2% | `<deduplicated_symbol>` |
| 31 | 1.2% | `crossbeam_epoch::internal::Global::try_advance` |
| 31 | 1.2% | `_tlv_get_addr` |
| 20 | 0.8% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 17 | 0.7% | `crossbeam_deque::deque::Worker<T>::pop` |
| 14 | 0.6% | `radiomics::features::matrices::Padded::new` |
| 14 | 0.6% | `rayon_core::registry::WorkerThread::find_work` |
| 12 | 0.5% | `_xzm_xzone_malloc_tiny` |
| 10 | 0.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 10 | 0.4% | `mach_absolute_time` |
| 10 | 0.4% | `rayon_core::sleep::Sleep::sleep` |
| 9 | 0.4% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 9 | 0.4% | `__psynch_cvbroad` |
| 8 | 0.3% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 6 | 0.2% | `pthread_mutex_lock` |
| 5 | 0.2% | `madvise` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 835 | 33.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 597 | 23.8% | `radiomics::features::matrices::run_matrix` |
| 335 | 13.3% | `radiomics::imaging::preprocessing::prepare` |
| 326 | 13.0% | `radiomics::workflows::engine::run_prepared_at` |
| 317 | 12.6% | `radiomics::workflows::roi::compute_roi` |
| 285 | 11.3% | `radiomics::features::texture::compute` |
| 244 | 9.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 244 | 9.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 201 | 8.0% | `radiomics::features::matrices::matrices` |
| 187 | 7.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 178 | 7.1% | `radiomics::workflows::roi::bounds::include` |
| 158 | 6.3% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 148 | 5.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 146 | 5.8% | `radiomics::workflows::engine::inputs::input_window` |
| 136 | 5.4% | `radiomics::imaging::preprocessing::gather` |
| 112 | 4.5% | `radiomics::imaging::preprocessing::lattice_bins` |
| 91 | 3.6% | `radiomics::imaging::preprocessing::gather_at` |
| 90 | 3.6% | `radiomics::imaging::preprocessing::true_positions` |
| 32 | 1.3% | `radiomics::workflows::roi::occupied_window` |
| 31 | 1.2% | `radiomics::features::aggregation::evaluate` |
| 31 | 1.2% | `radiomics::features::distributions::size_statistics` |
| 28 | 1.1% | `radiomics::imaging::io::HostArray::crop` |
| 22 | 0.9% | `radiomics::features::matrices::Padded::new` |
| 22 | 0.9% | `radiomics::features::matrices::DenseCounts::into_matrix` |
| 20 | 0.8% | `radiomics::numerics::sparse_counts::marginals` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 3 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 3 | 0.1% | `radiomics::workflows::engine::aggregation` |
| 3 | 0.1% | `radiomics::numerics::sparse_counts::entropy` |
| 2 | 0.1% | `radiomics::imaging::preprocessing::normalize_values` |
