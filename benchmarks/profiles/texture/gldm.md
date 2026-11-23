# Profile: texture/gldm

- Command: `target/bench-texture/final/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 3460 gldm_SmallDependenceEmphasis gldm_LargeDependenceEmphasis gldm_GrayLevelNonUniformity gldm_DependenceNonUniformity gldm_DependenceNonUniformityNormalized gldm_GrayLevelVariance gldm_DependenceVariance gldm_DependenceEntropy gldm_LowGrayLevelEmphasis gldm_HighGrayLevelEmphasis gldm_SmallDependenceLowGrayLevelEmphasis gldm_SmallDependenceHighGrayLevelEmphasis gldm_LargeDependenceLowGrayLevelEmphasis gldm_LargeDependenceHighGrayLevelEmphasis gldm_GrayLevelNonUniformityNormalized gldm_DependenceCountEnergy gldm_DependencePercentage`
- Commit: `aa6355a` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:45 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3932 (idle worker-thread samples excluded: 12123)

## Command output

```text
000000001,9.756499999999999,9.774415999999999,9.777084,9.782083,9.785791,9.792542,9.794458,9.802,9.805625000000001,9.812292,9.812792,9.81475,9.821916,9.828667,9.831292,9.836084,9.841958,9.843208,9.847625,9.852292,9.852542000000001,9.8595,9.861082999999999,9.865374999999998,9.869542000000001,9.887459,9.891834000000001,9.892833,9.894584,9.902542,9.912209,9.921084,9.925209,9.935541,9.946167,9.948041,9.948459,9.958584,9.963792000000002,9.965125,9.966209000000001,9.972167,9.982417,9.983166,9.983749999999999,9.993375,10.003541,10.0045,10.005167,10.015708,10.016375,10.030375,10.047667,10.048,10.050542,10.05525,10.056542,10.058375,10.062916,10.065665999999998,10.080667,10.103833,10.104375000000001,10.114167,10.128417,10.129666,10.130167,10.132083,10.132208,10.149792,10.152916,10.160917,10.161750000000001,10.171083,10.180208,10.189458,10.191167,10.193417,10.204417000000001,10.207083,10.208542,10.208958,10.211708,10.213375000000001,10.2325,10.233042,10.243542,10.245916000000001,10.250209,10.26,10.267333,10.268374999999999,10.281041,10.284666,10.290000000000001,10.292542,10.309584000000001,10.311167,10.324791999999999,10.33325,10.337166,10.3535,10.354333,10.362292,10.378375,10.379166,10.382833,10.390416,10.399917,10.418708,10.428292,10.428375,10.434542,10.447458,10.45075,10.455334,10.465292,10.475292000000001,10.490457999999999,10.50625,10.514584000000001,10.523207999999999,10.537833,10.571375,10.579625,10.596584,10.598165999999999,10.633042,10.63975,10.650417000000001,10.653083,10.656374999999999,10.668042000000002,10.677624999999999,10.685208999999999,10.694166999999998,10.726667,10.859041999999999,10.878374999999998,10.906625,10.952791999999999,10.956791,10.963959000000001,10.970917,11.107458000000001,11.154333000000001,11.275,11.278583,11.857208,12.213959,12.374875000000001,12.575875,12.797416,13.160834,13.339458,13.455,14.43575,14.882583,16.319750000000003,16.594041999999998,17.395042],"median_ms":1.913875,"features":{"original_gldm_SmallDependenceEmphasis":0.18926638333660392,"original_gldm_LargeDependenceEmphasis":63.926481084939326,"original_gldm_GrayLevelNonUniformity":7356.928951847582,"original_gldm_DependenceNonUniformity":4120.518915060671,"original_gldm_DependenceNonUniformityNormalized":0.07541351260199987,"original_gldm_GrayLevelVariance":65.88841585097525,"original_gldm_DependenceVariance":20.042736774819456,"original_gldm_DependenceEntropy":7.047659645783571,"original_gldm_LowGrayLevelEmphasis":0.002951469402579746,"original_gldm_HighGrayLevelEmphasis":1570.934753564304,"original_gldm_SmallDependenceLowGrayLevelEmphasis":0.0005612729945968891,"original_gldm_SmallDependenceHighGrayLevelEmphasis":206.95588096262185,"original_gldm_LargeDependenceLowGrayLevelEmphasis":0.11247451941972218,"original_gldm_LargeDependenceHighGrayLevelEmphasis":113459.4919745969,"original_gldm_GrayLevelNonUniformityNormalized":0.13464611270059082,"original_gldm_DependenceCountEnergy":0.010976642977034273,"original_gldm_DependencePercentage":1.0},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 932 | 23.7% | `__psynch_mutexwait` |
| 804 | 20.4% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 431 | 11.0% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 193 | 4.9% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 154 | 3.9% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 141 | 3.6% | `__psynch_mutexdrop` |
| 136 | 3.5% | `crossbeam_epoch::default::with_handle` |
| 125 | 3.2% | `__psynch_cvsignal` |
| 122 | 3.1% | `rayon_core::join::join_context::_{{closure}}` |
| 118 | 3.0% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 108 | 2.7% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 73 | 1.9% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 57 | 1.4% | `_tlv_get_addr` |
| 55 | 1.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |
| 45 | 1.1% | `<deduplicated_symbol>` |
| 43 | 1.1% | `_platform_memmove` |
| 42 | 1.1% | `radiomics::features::matrices::Padded::new` |
| 41 | 1.0% | `crossbeam_deque::deque::Worker<T>::pop` |
| 41 | 1.0% | `crossbeam_epoch::internal::Global::try_advance` |
| 28 | 0.7% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 26 | 0.7% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 21 | 0.5% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 18 | 0.5% | `rayon_core::job::StackJob<L,F,R>::run_inline` |
| 17 | 0.4% | `rayon_core::registry::WorkerThread::find_work` |
| 16 | 0.4% | `pthread_mutex_lock` |
| 15 | 0.4% | `_<rayon_core::job::StackJob<L,F,R> as rayon_core::job::Job>::execute` |
| 12 | 0.3% | `rayon_core::sleep::Sleep::sleep` |
| 10 | 0.3% | `rayon_core::sleep::Sleep::wake_specific_thread` |
| 8 | 0.2% | `rayon_core::sleep::Sleep::wake_any_threads` |
| 8 | 0.2% | `__psynch_cvbroad` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 845 | 21.5% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 413 | 10.5% | `radiomics::imaging::preprocessing::prepare` |
| 264 | 6.7% | `radiomics::workflows::engine::run_prepared_at` |
| 255 | 6.5% | `radiomics::workflows::roi::compute_roi` |
| 224 | 5.7% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 207 | 5.3% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 197 | 5.0% | `radiomics::features::texture::compute` |
| 189 | 4.8% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 181 | 4.6% | `radiomics::workflows::roi::bounds::include` |
| 170 | 4.3% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 170 | 4.3% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 167 | 4.2% | `radiomics::features::matrices::matrices` |
| 158 | 4.0% | `radiomics::imaging::preprocessing::gather` |
| 141 | 3.6% | `radiomics::imaging::preprocessing::lattice_bins` |
| 124 | 3.2% | `radiomics::workflows::engine::inputs::input_window` |
| 122 | 3.1% | `radiomics::imaging::preprocessing::gather_at` |
| 76 | 1.9% | `radiomics::imaging::preprocessing::true_positions` |
| 57 | 1.4% | `radiomics::workflows::roi::occupied_window` |
| 53 | 1.3% | `radiomics::features::matrices::Padded::new` |
| 43 | 1.1% | `radiomics::imaging::io::HostArray::crop` |
| 11 | 0.3% | `radiomics::imaging::io::HostArray::eq_label` |
| 10 | 0.3% | `radiomics::imaging::preprocessing::normalize_values` |
| 4 | 0.1% | `radiomics::imaging::preprocessing::fraction_of_f64` |
| 4 | 0.1% | `radiomics::config::settings::Settings::validate` |
| 3 | 0.1% | `radiomics::features::matrices::DenseCounts::into_matrix` |
| 3 | 0.1% | `radiomics::features::aggregation::evaluate` |
| 3 | 0.1% | `radiomics::features::distributions::size_statistics` |
| 3 | 0.1% | `radiomics::numerics::sparse_counts::marginals` |
| 2 | 0.1% | `radiomics::config::validation::unique` |
| 1 | 0.0% | `radiomics::workflows::engine::aggregation` |
