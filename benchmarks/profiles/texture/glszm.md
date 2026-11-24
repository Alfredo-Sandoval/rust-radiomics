# Profile: texture/glszm

- Command: `target/bench-texture/final/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 1210 glszm_SmallAreaEmphasis glszm_LargeAreaEmphasis glszm_GrayLevelNonUniformity glszm_GrayLevelNonUniformityNormalized glszm_SizeZoneNonUniformity glszm_SizeZoneNonUniformityNormalized glszm_ZonePercentage glszm_GrayLevelVariance glszm_ZoneVariance glszm_ZoneEntropy glszm_LowGrayLevelZoneEmphasis glszm_HighGrayLevelZoneEmphasis glszm_SmallAreaLowGrayLevelEmphasis glszm_SmallAreaHighGrayLevelEmphasis glszm_LargeAreaLowGrayLevelEmphasis glszm_LargeAreaHighGrayLevelEmphasis`
- Commit: `aa6355a` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:44 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2568 (idle worker-thread samples excluded: 10656)

## Command output

```text
9,17.885417,17.941167,17.995792,18.0415,18.13975,18.155959,18.158292,18.168166,18.177417000000002,18.182333,18.235417,18.261542,18.380333,18.385709,18.406958,18.489583,18.546583,18.549875,18.663167,18.664333000000003,18.683333,18.791042,18.84925,18.859333,18.936292,18.964458999999998,18.9675,18.969875000000002,19.045459,19.076292000000002,19.085333,19.376167,19.379959,19.380000000000003,19.397042,19.407167,19.40725,19.428125,19.496167,19.535792,19.668584,19.697083,19.820125,19.888666,20.056417,20.319834,20.349209,20.395917,20.396041,20.402041999999998,20.431125,20.458667,20.751625,20.753542,20.761958,20.947792,21.019083000000002,21.409000000000002,21.494374999999998,21.508958000000003,21.562834,21.703709,21.818917,22.156625,22.204292,22.227417,22.267625,22.347833,22.575542000000002,22.619125,22.620667,22.748,22.763042000000002,22.894458,22.905792,23.012583,23.019792000000002,23.172124999999998,23.249291,23.373624999999997,23.449125000000002,23.583625,23.6795,24.118416,24.285500000000003,25.246958,25.511167,25.627458999999998,25.828958,26.037582999999998,26.05325,26.115833000000002,26.673916,26.723084,26.873125,26.994584,27.432583,28.473958,28.929666,28.944292,28.958417,29.329958,29.457416,29.470917,29.684375,29.765458000000002,29.909417,29.910958,29.917708,29.921792,30.085708,30.178957999999998,30.336624999999998,30.479416,30.934625,31.282542000000003,31.287083,31.766958999999996,32.401542,32.441084000000004,33.787,33.907292,34.196333,34.735542,34.880250000000004,35.503750000000004,35.861583,36.01775,37.068916,37.208209,37.441708,37.612583,37.830292,37.944584000000006,38.111332999999995,38.335041,38.551958,38.900125,39.262417,40.162416,40.691916,42.583999999999996,43.348834000000004,44.819916,47.09025,47.976583000000005,48.280792000000005,48.305125,49.634042,49.800208,50.051541,50.889875,53.451834,55.506666,55.514458000000005,56.727209,56.925541,58.252625,58.458666,58.585583,61.469958999999996,61.962584,61.993375,65.9445,68.069083,78.326459,100.97687499999999],"median_ms":4.009687,"features":{"original_glszm_SmallAreaEmphasis":0.7448190600468144,"original_glszm_LargeAreaEmphasis":30740.146254458978,"original_glszm_GrayLevelNonUniformity":350.7757248696607,"original_glszm_GrayLevelNonUniformityNormalized":0.032084123741851336,"original_glszm_SizeZoneNonUniformity":5616.636879173146,"original_glszm_SizeZoneNonUniformityNormalized":0.5137324503039555,"original_glszm_ZonePercentage":0.2000951701165834,"original_glszm_GrayLevelVariance":103.29083199852047,"original_glszm_ZoneVariance":30715.170030016357,"original_glszm_ZoneEntropy":6.681037981121474,"original_glszm_LowGrayLevelZoneEmphasis":0.003304855925716684,"original_glszm_HighGrayLevelZoneEmphasis":1047.0274398609713,"original_glszm_SmallAreaLowGrayLevelEmphasis":0.002038829050764523,"original_glszm_SmallAreaHighGrayLevelEmphasis":740.0292095505583,"original_glszm_LargeAreaLowGrayLevelEmphasis":17.6007721667729,"original_glszm_LargeAreaHighGrayLevelEmphasis":55103461.85603219},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 618 | 24.1% | `__psynch_mutexwait` |
| 406 | 15.8% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 313 | 12.2% | `radiomics::features::matrices::Runs::join_rows` |
| 128 | 5.0% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 122 | 4.8% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 115 | 4.5% | `__psynch_mutexdrop` |
| 81 | 3.2% | `__psynch_cvsignal` |
| 76 | 3.0% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 76 | 3.0% | `crossbeam_epoch::default::with_handle` |
| 70 | 2.7% | `rayon_core::join::join_context::_{{closure}}` |
| 56 | 2.2% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 55 | 2.1% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 54 | 2.1% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 38 | 1.5% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 34 | 1.3% | `_platform_memmove` |
| 30 | 1.2% | `<deduplicated_symbol>` |
| 28 | 1.1% | `radiomics::features::matrices::matrices` |
| 28 | 1.1% | `_tlv_get_addr` |
| 24 | 0.9% | `crossbeam_epoch::internal::Global::try_advance` |
| 23 | 0.9% | `crossbeam_deque::deque::Worker<T>::pop` |
| 17 | 0.7% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 17 | 0.7% | `pthread_mutex_lock` |
| 14 | 0.5% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 11 | 0.4% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 10 | 0.4% | `mach_absolute_time` |
| 10 | 0.4% | `rayon_core::sleep::Sleep::sleep` |
| 10 | 0.4% | `rayon_core::sleep::Sleep::wake_specific_thread` |
| 9 | 0.4% | `__psynch_cvbroad` |
| 9 | 0.4% | `rayon_core::registry::WorkerThread::find_work` |
| 8 | 0.3% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 696 | 27.1% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 329 | 12.8% | `radiomics::workflows::engine::run_prepared_at` |
| 325 | 12.7% | `radiomics::workflows::roi::compute_roi` |
| 313 | 12.2% | `radiomics::features::matrices::Runs::join_rows` |
| 285 | 11.1% | `radiomics::features::texture::compute` |
| 269 | 10.5% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 269 | 10.5% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 264 | 10.3% | `radiomics::features::matrices::matrices` |
| 246 | 9.6% | `radiomics::imaging::preprocessing::prepare` |
| 139 | 5.4% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 126 | 4.9% | `radiomics::workflows::roi::bounds::include` |
| 125 | 4.9% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 121 | 4.7% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 107 | 4.2% | `radiomics::imaging::preprocessing::gather` |
| 92 | 3.6% | `radiomics::workflows::engine::inputs::input_window` |
| 73 | 2.8% | `radiomics::imaging::preprocessing::gather_at` |
| 71 | 2.8% | `radiomics::imaging::preprocessing::lattice_bins` |
| 58 | 2.3% | `radiomics::imaging::preprocessing::true_positions` |
| 34 | 1.3% | `radiomics::workflows::roi::occupied_window` |
| 29 | 1.1% | `radiomics::imaging::io::HostArray::crop` |
| 8 | 0.3% | `radiomics::imaging::preprocessing::normalize_values` |
| 5 | 0.2% | `radiomics::features::aggregation::evaluate` |
| 5 | 0.2% | `radiomics::features::distributions::size_statistics` |
| 4 | 0.2% | `radiomics::numerics::sparse_counts::marginals` |
| 3 | 0.1% | `radiomics::imaging::io::HostArray::eq_label` |
| 2 | 0.1% | `radiomics::workflows::roi_memory::check_roi_count_buffers` |
| 2 | 0.1% | `radiomics::config::settings::Settings::validate` |
| 1 | 0.0% | `radiomics::config::validation::unique` |
