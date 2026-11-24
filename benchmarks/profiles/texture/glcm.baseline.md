# Profile: texture/glcm.baseline

- Command: `target/bench-texture/baseline/benchmark_roi $LUNG1/LUNG1-001/image.nrrd $LUNG1/LUNG1-001/mask.nrrd 16 glcm_Autocorrelation glcm_Contrast glcm_Correlation glcm_DifferenceAverage glcm_JointEnergy glcm_JointEntropy glcm_Id glcm_Idm glcm_JointAverage glcm_ClusterProminence glcm_ClusterShade glcm_ClusterTendency glcm_DifferenceEntropy glcm_DifferenceVariance glcm_Imc1 glcm_Imc2 glcm_Idmn glcm_Idn glcm_InverseVariance glcm_MaximumProbability glcm_SumAverage glcm_SumEntropy glcm_SumSquares glcm_MCC glcm_Dissimilarity glcm_SumVariance`
- Commit: `17d817c` (with uncommitted changes)
- Machine: Apple M5 Max, 18 logical CPUs
- Date: 2026-11-22 01:43 UTC
- Sampling: `sample`, 4 s after 1 s warm-up, 1 ms interval, all threads
- Busy samples: 530 (idle worker-thread samples excluded: 0)

## Command output

```text
{"samples_ms":[75.329166,76.964458,77.34375,80.128542,81.229875,81.295834,82.70025,84.947666,87.084584,87.351417,87.61975,87.8385,87.948292,91.944875,93.220792,93.413125],"median_ms":86.016125,"features":{"original_glcm_Autocorrelation":1592.8387107035865,"original_glcm_Contrast":34.84825770092051,"original_glcm_Correlation":0.688783668811451,"original_glcm_DifferenceAverage":3.110845042378862,"original_glcm_JointEnergy":0.033522179311583876,"original_glcm_JointEntropy":6.7208168058740645,"original_glcm_Id":0.49532662441897723,"original_glcm_Idm":0.44699599476286955,"original_glcm_JointAverage":39.40757535228667,"original_glcm_ClusterProminence":449704.3780538093,"original_glcm_ClusterShade":-7644.420769951531,"original_glcm_ClusterTendency":194.17883427130633,"original_glcm_DifferenceEntropy":3.0368680859717774,"original_glcm_DifferenceVariance":24.480649538774134,"original_glcm_Imc1":-0.12844988162404292,"original_glcm_Imc2":0.7266414929796766,"original_glcm_Idmn":0.9951959595644969,"original_glcm_Idn":0.9667179287301619,"original_glcm_InverseVariance":0.3870752452869142,"original_glcm_MaximumProbability":0.08679663920476627,"original_glcm_SumAverage":78.81515070457334,"original_glcm_SumEntropy":4.524789514354813,"original_glcm_SumSquares":57.25677299305672,"original_glcm_MCC":0.7280065443799094,"original_glcm_Dissimilarity":3.110845042378862,"original_glcm_SumVariance":194.17883427130633},"roi_voxels":54639}
```

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 235 | 44.3% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 172 | 32.5% | `radiomics::numerics::eigen::jacobi` |
| 62 | 11.7% | `radiomics::workflows::roi::bounds::include` |
| 12 | 2.3% | `alloc::collections::btree::map::entry::Entry<K,V,A>::or_default` |
| 9 | 1.7% | `radiomics::numerics::eigen::maximal_correlation` |
| 7 | 1.3% | `radiomics::features::glcm::mutual_information_bits` |
| 6 | 1.1% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 4 | 0.8% | `radiomics::numerics::sparse_counts::marginals` |
| 4 | 0.8% | `<deduplicated_symbol>` |
| 3 | 0.6% | `hypot` |
| 3 | 0.6% | `__floattidf` |
| 2 | 0.4% | `_malloc_zone_realloc` |
| 2 | 0.4% | `radiomics::features::glcm::glcm_statistics` |
| 1 | 0.2% | `DYLD-STUB$$hypot` |
| 1 | 0.2% | `_xzm_xzone_find_and_malloc_from_freelist_chunk` |
| 1 | 0.2% | `madvise` |
| 1 | 0.2% | `matrixmultiply::dgemm_kernel::kernel_target_neon` |
| 1 | 0.2% | `log1p` |
| 1 | 0.2% | `_platform_memmove` |
| 1 | 0.2% | `_<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next` |
| 1 | 0.2% | `_platform_memcmp` |
| 1 | 0.2% | `radiomics::imaging::preprocessing::discretize_prepared` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 530 | 100.0% | `radiomics::workflows::engine::inputs::extract_with_shape_mask` |
| 460 | 86.8% | `radiomics::workflows::engine::run_prepared` |
| 460 | 86.8% | `radiomics::workflows::roi::compute_roi` |
| 459 | 86.6% | `radiomics::features::texture::compute` |
| 450 | 84.9% | `radiomics::features::aggregation::aggregated_texture_statistics` |
| 208 | 39.2% | `radiomics::features::aggregation::aggregated_texture_statistics::_{{closure}}` |
| 208 | 39.2% | `radiomics::features::glcm::glcm_statistics` |
| 190 | 35.8% | `radiomics::numerics::eigen::maximal_correlation` |
| 177 | 33.4% | `radiomics::numerics::eigen::jacobi` |
| 62 | 11.7% | `radiomics::workflows::engine::inputs::input_window` |
| 62 | 11.7% | `radiomics::workflows::roi::bounds::include` |
| 8 | 1.5% | `radiomics::features::glcm::mutual_information_bits` |
| 8 | 1.5% | `radiomics::imaging::preprocessing::prepare` |
| 5 | 0.9% | `radiomics::imaging::preprocessing::prepare_sampled_input` |
| 4 | 0.8% | `radiomics::numerics::sparse_counts::marginals` |
| 4 | 0.8% | `radiomics::imaging::preprocessing::gather` |
| 3 | 0.6% | `radiomics::imaging::preprocessing::discretize_prepared` |
| 1 | 0.2% | `radiomics::numerics::sparse_counts::entropy` |
| 1 | 0.2% | `radiomics::workflows::roi::occupied_window` |
| 1 | 0.2% | `radiomics::config::settings::Settings::validate` |
