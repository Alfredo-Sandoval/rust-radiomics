# Profile: texture/defaults93

- Command: `target/bench-texture/final/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 916`
- Commit: `aa6355a` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 02:47 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 1840 (idle worker-thread samples excluded: 29491)

## Command output

```text
SmallDependenceEmphasis":0.18926638333660392,"original_gldm_LargeDependenceEmphasis":63.926481084939326,"original_gldm_GrayLevelNonUniformity":7356.928951847582,"original_gldm_DependenceNonUniformity":4120.518915060671,"original_gldm_DependenceNonUniformityNormalized":0.07541351260199987,"original_gldm_GrayLevelVariance":65.88841585097525,"original_gldm_DependenceVariance":20.042736774819456,"original_gldm_DependenceEntropy":7.047659645783571,"original_gldm_LowGrayLevelEmphasis":0.002951469402579746,"original_gldm_HighGrayLevelEmphasis":1570.934753564304,"original_gldm_SmallDependenceLowGrayLevelEmphasis":0.0005612729945968891,"original_gldm_SmallDependenceHighGrayLevelEmphasis":206.95588096262185,"original_gldm_LargeDependenceLowGrayLevelEmphasis":0.11247451941972218,"original_gldm_LargeDependenceHighGrayLevelEmphasis":113459.4919745969,"original_ngtdm_Coarseness":0.00011756926620155445,"original_ngtdm_Contrast":0.056041345217520314,"original_ngtdm_Busyness":1.7340097636445118,"original_ngtdm_Complexity":5670.9224706732375,"original_ngtdm_Strength":0.680281837886726,"original_glrlm_ShortRunEmphasis":0.8466860066657029,"original_glrlm_LongRunEmphasis":2.135824732831354,"original_glrlm_GrayLevelNonUniformity":4637.710874115572,"original_glrlm_GrayLevelNonUniformityNormalized":0.10806989219887199,"original_glrlm_RunLengthNonUniformity":29085.237038438234,"original_glrlm_RunLengthNonUniformityNormalized":0.6750052178210483,"original_glrlm_RunPercentage":0.783673819911672,"original_glrlm_GrayLevelVariance":75.93321316315883,"original_glrlm_RunVariance":0.4821143416653727,"original_glrlm_RunEntropy":4.917357267623147,"original_glrlm_LowGrayLevelRunEmphasis":0.0031531710259638146,"original_glrlm_HighGrayLevelRunEmphasis":1518.3791561676444,"original_glrlm_ShortRunLowGrayLevelEmphasis":0.002788483673992149,"original_glrlm_ShortRunHighGrayLevelEmphasis":1250.2605500773027,"original_glrlm_LongRunLowGrayLevelEmphasis":0.005552685887671641,"original_glrlm_LongRunHighGrayLevelEmphasis":3521.0504151107157,"original_glszm_SmallAreaEmphasis":0.7448190600468144,"original_glszm_LargeAreaEmphasis":30740.146254458978,"original_glszm_GrayLevelNonUniformity":350.7757248696607,"original_glszm_GrayLevelNonUniformityNormalized":0.032084123741851336,"original_glszm_SizeZoneNonUniformity":5616.636879173146,"original_glszm_SizeZoneNonUniformityNormalized":0.5137324503039555,"original_glszm_ZonePercentage":0.2000951701165834,"original_glszm_GrayLevelVariance":103.29083199852047,"original_glszm_ZoneVariance":30715.170030016357,"original_glszm_ZoneEntropy":6.681037981121474,"original_glszm_LowGrayLevelZoneEmphasis":0.003304855925716684,"original_glszm_HighGrayLevelZoneEmphasis":1047.0274398609713,"original_glszm_SmallAreaLowGrayLevelEmphasis":0.002038829050764523,"original_glszm_SmallAreaHighGrayLevelEmphasis":740.0292095505583,"original_glszm_LargeAreaLowGrayLevelEmphasis":17.6007721667729,"original_glszm_LargeAreaHighGrayLevelEmphasis":55103461.85603219},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 298 | 16.2% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 256 | 13.9% | `radiomics::features::matrices::run_matrix` |
| 213 | 11.6% | `_RNvNtNtCs6sq8b9ugfBC_4core5slice6memchr14memchr_aligned` |
| 192 | 10.4% | `radiomics::features::matrices::Runs::join_rows` |
| 178 | 9.7% | `__psynch_mutexwait` |
| 96 | 5.2% | `radiomics::features::cooccurrence::glcm_matrix` |
| 81 | 4.4% | `radiomics::numerics::eigen::maximal_correlation` |
| 40 | 2.2% | `radiomics::features::matrices::Padded::new` |
| 27 | 1.5% | `log1p` |
| 26 | 1.4% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
| 26 | 1.4% | `radiomics::features::glcm::mutual_information_bits` |
| 23 | 1.2% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter` |
| 23 | 1.2% | `radiomics::features::matrices::matrices` |
| 20 | 1.1% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 20 | 1.1% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 20 | 1.1% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 20 | 1.1% | `__psynch_cvsignal` |
| 17 | 0.9% | `__floattidf` |
| 16 | 0.9% | `crossbeam_deque::deque::Stealer<T>::steal` |
| 15 | 0.8% | `madvise` |
| 15 | 0.8% | `radiomics::numerics::sparse_counts::marginals` |
| 14 | 0.8% | `_platform_memmove` |
| 14 | 0.8% | `__psynch_mutexdrop` |
| 13 | 0.7% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 13 | 0.7% | `_<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend` |
| 13 | 0.7% | `_<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold` |
| 10 | 0.5% | `<deduplicated_symbol>` |
| 9 | 0.5% | `radiomics::features::first_order_source::check_samples` |
| 9 | 0.5% | `alloc::collections::btree::map::BTreeMap<K,V,A>::bulk_build_from_sorted_iter` |
| 8 | 0.4% | `rayon_core::registry::WorkerThread::wait_until_cold` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 1649 | 89.6% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 1500 | 81.5% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 1497 | 81.4% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 1172 | 63.7% | `radiomics::features::matrices::matrices` |
| 845 | 45.9% | `radiomics::workflows::engine::run_prepared_at` |
| 842 | 45.8% | `radiomics::workflows::roi::compute_roi` |
| 790 | 42.9% | `radiomics::features::texture::compute` |
| 456 | 24.8% | `radiomics::imaging::preprocessing::prepare` |
| 346 | 18.8% | `radiomics::workflows::roi::bounds::include` |
| 337 | 18.3% | `radiomics::workflows::engine::inputs::input_window` |
| 267 | 14.5% | `radiomics::features::matrices::run_matrix` |
| 242 | 13.2% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 240 | 13.0% | `radiomics::imaging::preprocessing::gather` |
| 226 | 12.3% | `radiomics::features::aggregation::evaluate` |
| 214 | 11.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 204 | 11.1% | `radiomics::features::glcm::glcm_statistics` |
| 192 | 10.4% | `radiomics::features::matrices::Runs::join_rows` |
| 168 | 9.1% | `radiomics::imaging::preprocessing::gather_at` |
| 153 | 8.3% | `radiomics::imaging::preprocessing::lattice_bins` |
| 126 | 6.8% | `radiomics::imaging::preprocessing::true_positions` |
| 126 | 6.8% | `radiomics::features::cooccurrence::glcm_matrix` |
| 104 | 5.7% | `radiomics::numerics::eigen::maximal_correlation` |
| 55 | 3.0% | `radiomics::features::matrices::Padded::new` |
| 46 | 2.5% | `radiomics::features::glcm::mutual_information_bits` |
| 42 | 2.3% | `radiomics::features::matrices::DenseCounts::into_matrix` |
| 39 | 2.1% | `radiomics::features::first_order::compute` |
| 39 | 2.1% | `radiomics::features::first_order::first_order_roi` |
| 39 | 2.1% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 35 | 1.9% | `radiomics::features::first_order::RegionKernel::finish` |
| 35 | 1.9% | `radiomics::workflows::roi::bounds::include::_{{closure}}` |
