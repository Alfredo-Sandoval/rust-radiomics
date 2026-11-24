# Profile: texture/ngtdm

- Command: `target/bench-texture/final/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 1712 ngtdm_Coarseness ngtdm_Contrast ngtdm_Busyness ngtdm_Complexity ngtdm_Strength`
- Commit: `aa6355a` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:46 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2640 (idle worker-thread samples excluded: 11762)

## Command output

```text
.363166,19.530667,19.534167,19.570292,19.638959,19.645583000000002,19.870625,20.166166999999998,20.184875,20.221791999999997,20.431,20.461666,20.481292000000003,20.507958,20.534792,20.55525,20.915959,20.948625,20.9615,20.993499999999997,21.106583,21.191792,21.244959,21.327667,21.415541,21.460791,21.578167,21.584167,21.763666,21.825083,21.869875,21.9155,21.947792,22.241042,22.266583999999998,22.286458,22.348458,22.480542,22.692125,22.720208,22.9195,23.1015,23.137792,23.253958,23.263583,23.410959,23.438083,23.449166,23.4905,23.508167,23.654625,23.761958999999997,23.766625,23.834667,23.908833,24.105292,24.116875,24.117584,24.125,24.31175,24.483125,24.486708,24.598499999999998,24.635125000000002,24.6555,24.690375,24.741167,24.829459,24.927416,25.025125,25.061417,25.126208000000002,25.248167,25.291209,25.29875,25.56875,25.814583,26.138125000000002,26.240583,26.571792000000002,26.635333,26.640917,26.711708,26.830042,26.838417,26.981792000000002,27.058125,27.086833000000002,27.220583,27.272208,27.557042,27.661875,27.70525,27.850333000000003,27.855999999999998,28.123458,28.518,28.518915999999997,28.532291,28.592875,28.666334000000003,28.692124999999997,28.933332999999998,29.031709,29.377208,29.742166,30.311958,30.463916,30.692083,30.809584,30.914334,31.149916,31.181041999999998,31.33175,31.349750000000004,31.749208,31.851458,31.937709,32.010792,32.050959,32.495791000000004,32.663791999999994,32.735542,32.753458,32.802291999999994,32.808917,32.985833,33.048916,33.149959,33.420832999999995,33.464375,33.521708,33.645166999999994,33.646958,34.047374999999995,34.245458,34.28725,34.562125,34.6695,34.767375,35.219208,35.265249999999995,35.370082999999994,35.508792,35.53675,35.700625,35.836833,35.923334000000004,36.150833,36.271042,36.440208,36.582125,36.605125,37.177084,37.599042,38.162124999999996,38.220541,39.064875,39.307583,39.457958,39.491291,39.810708,40.141208999999996,40.588167,40.927583,41.547958,42.206917,42.656292,43.077667,43.253583,44.248999999999995,45.524,45.851333000000004,45.944625,46.253459,47.08525,47.178458,47.732917,48.432,48.509583,48.539832999999994,48.593667,48.911958,49.169208,49.671,50.425584,50.649333999999996,50.65195799999999,50.999542000000005,51.078582999999995,51.208667,51.263584,51.382375,51.593333,52.407666,52.75775,53.5715,54.649083000000005,55.10933300000001,55.415458,56.630374999999994,56.772292,56.869,56.970375,57.101042,58.284917,59.001958,59.68775,60.012457999999995,61.281625,61.612834,61.78025,62.102791999999994,62.57054099999999,63.128209000000005,66.314,67.206125,68.40625,68.545958,69.725083,73.303084,76.712416,77.88983300000001,78.376458,79.983375,82.00766700000001,83.309375,83.68725,85.70029199999999,115.010166,128.800417,147.156583],"median_ms":3.0589375,"features":{"original_ngtdm_Coarseness":0.00011756926620155445,"original_ngtdm_Contrast":0.056041345217520314,"original_ngtdm_Busyness":1.7340097636445118,"original_ngtdm_Complexity":5670.9224706732375,"original_ngtdm_Strength":0.680281837886726},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 623 | 23.6% | `__psynch_mutexwait` |
| 425 | 16.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 395 | 15.0% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 140 | 5.3% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 135 | 5.1% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 104 | 3.9% | `__psynch_mutexdrop` |
| 94 | 3.6% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 89 | 3.4% | `crossbeam_epoch::default::with_handle` |
| 79 | 3.0% | `__psynch_cvsignal` |
| 61 | 2.3% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 56 | 2.1% | `rayon_core::join::join_context::_{{closure}}` |
| 54 | 2.0% | `<deduplicated_symbol>` |
| 47 | 1.8% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 41 | 1.6% | `_tlv_get_addr` |
| 40 | 1.5% | `_platform_memmove` |
| 25 | 0.9% | `radiomics::features::matrices::Padded::new` |
| 25 | 0.9% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 22 | 0.8% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 22 | 0.8% | `crossbeam_deque::deque::Worker<T>::pop` |
| 20 | 0.8% | `crossbeam_epoch::internal::Global::try_advance` |
| 16 | 0.6% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 12 | 0.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 11 | 0.4% | `pthread_mutex_lock` |
| 10 | 0.4% | `rayon_core::sleep::Sleep::sleep` |
| 5 | 0.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 5 | 0.2% | `rayon_core::registry::WorkerThread::find_work` |
| 5 | 0.2% | `__psynch_cvbroad` |
| 5 | 0.2% | `_xzm_xzone_malloc_tiny` |
| 4 | 0.2% | `madvise` |
| 4 | 0.2% | `mach_absolute_time` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 758 | 28.7% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 366 | 13.9% | `radiomics::imaging::preprocessing::prepare` |
| 224 | 8.5% | `radiomics::workflows::engine::run_prepared_at` |
| 216 | 8.2% | `radiomics::workflows::roi::compute_roi` |
| 193 | 7.3% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 173 | 6.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 166 | 6.3% | `radiomics::workflows::roi::bounds::include` |
| 164 | 6.2% | `radiomics::features::texture::compute` |
| 150 | 5.7% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 150 | 5.7% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 144 | 5.5% | `radiomics::imaging::preprocessing::gather` |
| 144 | 5.5% | `radiomics::features::matrices::matrices` |
| 142 | 5.4% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 114 | 4.3% | `radiomics::workflows::engine::inputs::input_window` |
| 112 | 4.2% | `radiomics::imaging::preprocessing::lattice_bins` |
| 109 | 4.1% | `radiomics::imaging::preprocessing::gather_at` |
| 67 | 2.5% | `radiomics::imaging::preprocessing::true_positions` |
| 53 | 2.0% | `radiomics::imaging::io::HostArray::crop` |
| 52 | 2.0% | `radiomics::workflows::roi::occupied_window` |
| 29 | 1.1% | `radiomics::features::matrices::Padded::new` |
| 9 | 0.3% | `radiomics::imaging::preprocessing::normalize_values` |
| 7 | 0.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 6 | 0.2% | `radiomics::features::aggregation::evaluate` |
| 6 | 0.2% | `radiomics::features::distributions::ngtdm_statistics` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 3 | 0.1% | `radiomics::features::matrices::DenseCounts::into_matrix` |
| 3 | 0.1% | `radiomics::config::settings::Settings::validate` |
| 2 | 0.1% | `radiomics::config::validation::unique` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::floor_to_bigint` |
| 1 | 0.0% | `radiomics::workflows::engine::checked_source` |
