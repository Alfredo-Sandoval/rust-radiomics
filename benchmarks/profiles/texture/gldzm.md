# Profile: texture/gldzm

- Command: `target/bench-texture/final/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 1747 gldzm_SmallDistanceEmphasis gldzm_LargeDistanceEmphasis gldzm_LowGrayLevelZoneEmphasis gldzm_HighGrayLevelZoneEmphasis gldzm_SmallDistanceLowGrayLevelEmphasis gldzm_SmallDistanceHighGrayLevelEmphasis gldzm_LargeDistanceLowGrayLevelEmphasis gldzm_LargeDistanceHighGrayLevelEmphasis gldzm_GrayLevelNonUniformity gldzm_GrayLevelNonUniformityNormalized gldzm_ZoneDistanceNonUniformity gldzm_ZoneDistanceNonUniformityNormalized gldzm_ZonePercentage gldzm_GrayLevelVariance gldzm_ZoneDistanceVariance gldzm_ZoneDistanceEntropy`
- Commit: `aa6355a` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:45 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 2631 (idle worker-thread samples excluded: 11486)

## Command output

```text
99,15.926124999999999,16.080166,16.109167,16.13625,16.144959,16.167,16.283875,16.326083,16.369374999999998,16.466291000000002,16.593958,16.63875,16.683125,16.695625,16.749292,16.79175,16.822083000000003,16.840833,16.867375,16.971083,16.982792,16.984167000000003,17.15875,17.246000000000002,17.318042,17.367375,17.371084,17.413,17.519042,17.538249999999998,17.747292,17.762041,17.816416,17.818875000000002,17.819791000000002,17.860042,17.886625,17.963459,18.022707999999998,18.028292,18.029833,18.117375000000003,18.135125,18.173583999999998,18.185292,18.199458,18.350833,18.422667,18.458458,18.469125,18.473542,18.586875,18.709875,18.733,18.765375000000002,18.808333,18.846459,18.852125,18.891791,18.983875,18.997291999999998,19.080625,19.187917,19.265207999999998,19.425375000000003,19.43825,19.51925,19.541458000000002,19.643957999999998,19.680583000000002,19.714417,19.717083,19.773,19.777124999999998,19.781291,19.857625,19.890958,20.1345,20.1785,20.2925,20.339125,20.863542,20.915333,20.923041,20.976916000000003,20.986,21.093958999999998,21.099,21.111417,21.129084,21.150208,21.211750000000002,21.229791000000002,21.278667000000002,21.297625,21.620875,21.661707999999997,21.783792,21.921291,21.956417,21.989416000000002,21.999125000000003,22.002625000000002,22.035667,22.078542,22.091042,22.101375,22.131999999999998,22.192125,22.282166999999998,22.303083,22.335207999999998,22.354041000000002,22.446333,22.472292000000003,22.543792,22.551917,22.560333,22.590334,22.598958,22.747833,22.773125,22.915582999999998,22.937959,23.19075,23.252709,23.258833,23.366959,23.484042,23.637999999999998,23.639917,23.669958,23.779042,23.792583999999998,23.831792,24.040708000000002,24.129916,24.220708000000002,24.293916,24.553458,24.715709,25.258209,25.654125,25.655625,25.741540999999998,25.835,26.408208,26.63975,27.13825,27.470542000000002,28.90875,29.945875,30.734,31.144709,31.242458,32.864875000000005,34.158417,37.060875,49.619208,60.912875],"median_ms":3.430167,"features":{"original_gldzm_SmallDistanceEmphasis":0.6376309546307348,"original_gldzm_LargeDistanceEmphasis":8.487514863258026,"original_gldzm_LowGrayLevelZoneEmphasis":0.0033048559257166784,"original_gldzm_HighGrayLevelZoneEmphasis":1047.0274398609713,"original_gldzm_SmallDistanceLowGrayLevelEmphasis":0.0015352089387028504,"original_gldzm_SmallDistanceHighGrayLevelEmphasis":610.0135363310085,"original_gldzm_LargeDistanceLowGrayLevelEmphasis":0.07885998921217101,"original_gldzm_LargeDistanceHighGrayLevelEmphasis":9150.351779017654,"original_gldzm_GrayLevelNonUniformity":350.7757248696607,"original_gldzm_GrayLevelNonUniformityNormalized":0.032084123741851336,"original_gldzm_ZoneDistanceNonUniformity":4128.991585109302,"original_gldzm_ZoneDistanceNonUniformityNormalized":0.37766318349120115,"original_gldzm_ZonePercentage":0.2000951701165834,"original_gldzm_GrayLevelVariance":103.29083199852047,"original_gldzm_ZoneDistanceVariance":3.737483396391024,"original_gldzm_ZoneDistanceEntropy":7.1203522374023525},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 604 | 23.0% | `__psynch_mutexwait` |
| 326 | 12.4% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 321 | 12.2% | `radiomics::features::matrices::Runs::join_rows` |
| 200 | 7.6% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 123 | 4.7% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 114 | 4.3% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 105 | 4.0% | `__psynch_mutexdrop` |
| 86 | 3.3% | `__psynch_cvsignal` |
| 74 | 2.8% | `radiomics::features::matrices::matrices` |
| 70 | 2.7% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 70 | 2.7% | `crossbeam_epoch::default::with_handle` |
| 67 | 2.5% | `rayon_core::join::join_context::_{{closure}}` |
| 58 | 2.2% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 43 | 1.6% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 38 | 1.4% | `_platform_memmove` |
| 32 | 1.2% | `_tlv_get_addr` |
| 26 | 1.0% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 25 | 1.0% | `<deduplicated_symbol>` |
| 23 | 0.9% | `crossbeam_deque::deque::Worker<T>::pop` |
| 22 | 0.8% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 20 | 0.8% | `crossbeam_epoch::internal::Global::try_advance` |
| 15 | 0.6% | `radiomics::features::matrices::Padded::new` |
| 13 | 0.5% | `pthread_mutex_lock` |
| 12 | 0.5% | `mach_absolute_time` |
| 11 | 0.4% | `rayon_core::sleep::Sleep::wake_specific_thread` |
| 10 | 0.4% | `madvise` |
| 9 | 0.3% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 8 | 0.3% | `_xzm_free` |
| 8 | 0.3% | `rayon_core::registry::WorkerThread::find_work` |
| 7 | 0.3% | `radiomics::imaging::preprocessing::discretize_prepared` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 743 | 28.2% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 423 | 16.1% | `radiomics::workflows::engine::run_prepared_at` |
| 420 | 16.0% | `radiomics::workflows::roi::compute_roi` |
| 394 | 15.0% | `radiomics::features::texture::compute` |
| 372 | 14.1% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 371 | 14.1% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 367 | 13.9% | `radiomics::features::matrices::matrices` |
| 321 | 12.2% | `radiomics::features::matrices::Runs::join_rows` |
| 216 | 8.2% | `radiomics::imaging::preprocessing::prepare` |
| 133 | 5.1% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 115 | 4.4% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 103 | 3.9% | `radiomics::workflows::roi::bounds::include` |
| 101 | 3.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 82 | 3.1% | `radiomics::imaging::preprocessing::gather` |
| 79 | 3.0% | `radiomics::workflows::engine::inputs::input_window` |
| 78 | 3.0% | `radiomics::imaging::preprocessing::lattice_bins` |
| 67 | 2.5% | `radiomics::imaging::preprocessing::gather_at` |
| 36 | 1.4% | `radiomics::imaging::preprocessing::true_positions` |
| 25 | 1.0% | `radiomics::imaging::io::HostArray::crop` |
| 24 | 0.9% | `radiomics::workflows::roi::occupied_window` |
| 22 | 0.8% | `radiomics::features::matrices::Padded::new` |
| 5 | 0.2% | `radiomics::imaging::io::HostArray::eq_label` |
| 4 | 0.2% | `radiomics::imaging::preprocessing::normalize_values` |
| 3 | 0.1% | `radiomics::config::settings::Settings::validate` |
| 3 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 2 | 0.1% | `radiomics::features::aggregation::evaluate` |
| 2 | 0.1% | `radiomics::features::distributions::size_statistics` |
| 2 | 0.1% | `radiomics::features::texture::compute::_{{closure}}` |
| 1 | 0.0% | `radiomics::workflows::engine::aggregation` |
| 1 | 0.0% | `radiomics::imaging::preprocessing::resegment_samples` |
