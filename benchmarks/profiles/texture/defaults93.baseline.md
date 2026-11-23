# Profile: texture/defaults93.baseline

- Command: `target/bench-texture/baseline/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 64`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:45 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 3381 (idle worker-thread samples excluded: 0)

## Command output

```text
m_SmallDependenceEmphasis":0.18926638333660392,"original_gldm_LargeDependenceEmphasis":63.926481084939326,"original_gldm_GrayLevelNonUniformity":7356.928951847582,"original_gldm_DependenceNonUniformity":4120.518915060671,"original_gldm_DependenceNonUniformityNormalized":0.07541351260199987,"original_gldm_GrayLevelVariance":65.88841585097525,"original_gldm_DependenceVariance":20.042736774819456,"original_gldm_DependenceEntropy":7.047659645783571,"original_gldm_LowGrayLevelEmphasis":0.002951469402579746,"original_gldm_HighGrayLevelEmphasis":1570.934753564304,"original_gldm_SmallDependenceLowGrayLevelEmphasis":0.0005612729945968891,"original_gldm_SmallDependenceHighGrayLevelEmphasis":206.95588096262185,"original_gldm_LargeDependenceLowGrayLevelEmphasis":0.11247451941972218,"original_gldm_LargeDependenceHighGrayLevelEmphasis":113459.4919745969,"original_ngtdm_Coarseness":0.00011756926620155437,"original_ngtdm_Contrast":0.056041345217520314,"original_ngtdm_Busyness":1.7340097636445129,"original_ngtdm_Complexity":5670.92247067324,"original_ngtdm_Strength":0.680281837886726,"original_glrlm_ShortRunEmphasis":0.8466860066657029,"original_glrlm_LongRunEmphasis":2.135824732831354,"original_glrlm_GrayLevelNonUniformity":4637.710874115572,"original_glrlm_GrayLevelNonUniformityNormalized":0.10806989219887199,"original_glrlm_RunLengthNonUniformity":29085.237038438234,"original_glrlm_RunLengthNonUniformityNormalized":0.6750052178210483,"original_glrlm_RunPercentage":0.783673819911672,"original_glrlm_GrayLevelVariance":75.93321316315883,"original_glrlm_RunVariance":0.4821143416653727,"original_glrlm_RunEntropy":4.917357267623147,"original_glrlm_LowGrayLevelRunEmphasis":0.0031531710259638146,"original_glrlm_HighGrayLevelRunEmphasis":1518.3791561676444,"original_glrlm_ShortRunLowGrayLevelEmphasis":0.002788483673992149,"original_glrlm_ShortRunHighGrayLevelEmphasis":1250.2605500773027,"original_glrlm_LongRunLowGrayLevelEmphasis":0.005552685887671641,"original_glrlm_LongRunHighGrayLevelEmphasis":3521.0504151107157,"original_glszm_SmallAreaEmphasis":0.7448190600468144,"original_glszm_LargeAreaEmphasis":30740.146254458978,"original_glszm_GrayLevelNonUniformity":350.7757248696607,"original_glszm_GrayLevelNonUniformityNormalized":0.032084123741851336,"original_glszm_SizeZoneNonUniformity":5616.636879173146,"original_glszm_SizeZoneNonUniformityNormalized":0.5137324503039555,"original_glszm_ZonePercentage":0.2000951701165834,"original_glszm_GrayLevelVariance":103.29083199852047,"original_glszm_ZoneVariance":30715.170030016357,"original_glszm_ZoneEntropy":6.681037981121474,"original_glszm_LowGrayLevelZoneEmphasis":0.003304855925716684,"original_glszm_HighGrayLevelZoneEmphasis":1047.0274398609713,"original_glszm_SmallAreaLowGrayLevelEmphasis":0.002038829050764523,"original_glszm_SmallAreaHighGrayLevelEmphasis":740.0292095505583,"original_glszm_LargeAreaLowGrayLevelEmphasis":17.6007721667729,"original_glszm_LargeAreaHighGrayLevelEmphasis":55103461.85603219},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 1796 | 53.1% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 752 | 22.2% | `radiomics::numerics::eigen::jacobi` |
| 310 | 9.2% | `radiomics::workflows::roi::bounds::include` |
| 162 | 4.8% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold` |
| 74 | 2.2% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 45 | 1.3% | `radiomics::numerics::eigen::maximal_correlation` |
| 32 | 0.9% | `radiomics::features::glcm::glcm_statistics` |
| 26 | 0.8% | `radiomics::features::glcm::mutual_information_bits` |
| 24 | 0.7% | `<deduplicated_symbol>` |
| 19 | 0.6% | `__floattidf` |
| 15 | 0.4% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 13 | 0.4% | `_platform_memmove` |
| 12 | 0.4% | `hypot` |
| 9 | 0.3% | `radiomics::numerics::sparse_counts::marginals` |
| 9 | 0.3% | `core::slice::sort::unstable::quicksort::quicksort` |
| 8 | 0.2% | `radiomics::imaging::preprocessing::lattice_bins` |
| 6 | 0.2% | `log1p` |
| 6 | 0.2% | `_<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold` |
| 6 | 0.2% | `core::iter::traits::iterator::Iterator::try_fold` |
| 5 | 0.1% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 5 | 0.1% | `alloc::collections::btree::map::IntoIter<K,V,A>::dying_next` |
| 5 | 0.1% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 5 | 0.1% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 5 | 0.1% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 4 | 0.1% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 3 | 0.1% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::map` |
| 2 | 0.1% | `DYLD-STUB$$hypot` |
| 2 | 0.1% | `_xzm_free_main` |
| 2 | 0.1% | `radiomics::features::first_order::RegionKernel::finish` |
| 1 | 0.0% | `madvise` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 3381 | 100.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 3029 | 89.6% | `radiomics::workflows::engine::run_prepared` |
| 3025 | 89.5% | `radiomics::workflows::roi::compute_roi` |
| 2995 | 88.6% | `radiomics::features::texture::compute` |
| 2953 | 87.3% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 928 | 27.4% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 917 | 27.1% | `radiomics::features::glcm::glcm_statistics` |
| 827 | 24.5% | `radiomics::numerics::eigen::maximal_correlation` |
| 767 | 22.7% | `radiomics::numerics::eigen::jacobi` |
| 310 | 9.2% | `radiomics::workflows::engine::inputs::input_window` |
| 310 | 9.2% | `radiomics::workflows::roi::bounds::include` |
| 38 | 1.1% | `radiomics::imaging::preprocessing::prepare` |
| 32 | 0.9% | `radiomics::features::glcm::mutual_information_bits` |
| 24 | 0.7% | `radiomics::features::first_order::compute` |
| 24 | 0.7% | `radiomics::features::first_order::first_order_roi` |
| 24 | 0.7% | `radiomics::features::first_order::RegionKernel::evaluate` |
| 19 | 0.6% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 19 | 0.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 17 | 0.5% | `radiomics::features::first_order::RegionKernel::finish` |
| 15 | 0.4% | `radiomics::native_first_order::statistics::Calculation::measure` |
| 15 | 0.4% | `radiomics::imaging::preprocessing::gather` |
| 10 | 0.3% | `radiomics::numerics::sparse_counts::marginals` |
| 8 | 0.2% | `radiomics::imaging::preprocessing::lattice_bins` |
| 7 | 0.2% | `radiomics::features::first_order::RegionKernel::gather_flat` |
| 5 | 0.1% | `radiomics::workflows::roi::occupied_window` |
| 3 | 0.1% | `radiomics::numerics::sparse_counts::entropy` |
| 3 | 0.1% | `radiomics::features::distributions::size_statistics` |
| 3 | 0.1% | `radiomics::imaging::io::HostArray::crop` |
| 2 | 0.1% | `radiomics::config::settings::Settings::validate` |
| 2 | 0.1% | `radiomics::config::validation::unique` |
