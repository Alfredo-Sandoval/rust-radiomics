# Voxel feature map performance

Apple M5 Max, 18 logical CPUs. Each LUNG1-001 extraction uses the full 512 × 512 × 134 source grid, 54,639 VOI centres, the original image, default IBSI conventions, and float64 retained maps through `radiomics::extract`. Radius 1 is the mlx-radiomics default (27 window sites). Radius 2 has 125 sites. Input I/O, the first extraction, and destruction of the retained result are outside each timed interval. Zero-background allocation, map writes and feature-validity accounting are inside it. The existing engine delivers float32 samples before widening them to its float64 maps. The new texture and spectral arithmetic is float64 with integer counts.

## Warm medians

Times are milliseconds. Each final timing links to its stack profile. The matching baseline profile has the suffix `.baseline.md`. Independent full-volume measurements overlapped other CPU and memory-intensive work. The paired reruns below quantify variability at radius 1.

| Calculation | R1 baseline | R1 final | Speedup | R2 baseline | R2 final | Speedup | Change |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 93 defaults | 5,368.48 | [4,188.44](../profiles/voxel/defaults-r1.md) | 1.28× | 17,203.38 | [2,289.73](../profiles/voxel/defaults-r2.md) | 7.51× | All default families share ordered blocks and reusable local storage. |
| All 147 | 8,488.07 | [4,536.51](../profiles/voxel/all-r1.md) | 1.87× | 22,180.51 | [3,352.40](../profiles/voxel/all-r2.md) | 6.62× | The two zone families share one connected-component traversal. |
| GLCM | 1,357.12 | [1,354.21](../profiles/voxel/glcm-r1.md) | 1.00× | 14,886.40 | [1,751.76](../profiles/voxel/glcm-r2.md) | 8.50× | Exact compact cell counts and symmetric MCC on occupied gray levels. |
| GLRLM | 961.44 | [1,565.47](../profiles/voxel/glrlm-r1.md) | 0.61× | 2,487.00 | [908.02](../profiles/voxel/glrlm-r2.md) | 2.74× | Shifted bit sets count runs along each precomputed direction. |
| GLSZM | 1,075.96 | [1,490.50](../profiles/voxel/glszm-r1.md) | 0.72× | 5,099.36 | [830.73](../profiles/voxel/glszm-r2.md) | 6.14× | Bit-set flood fill finds connected gray-level zones. |
| GLDZM | 1,121.46 | [552.83](../profiles/voxel/gldzm-r1.md) | 2.03× | 2,798.88 | [726.45](../profiles/voxel/gldzm-r2.md) | 3.85× | Nested face erosions give boundary depth for each connected zone. |
| GLDM | 1,041.66 | [724.58](../profiles/voxel/gldm-r1.md) | 1.44× | 2,540.28 | [732.95](../profiles/voxel/gldm-r2.md) | 3.47× | Population counts give the number of equal-level neighbours. |
| NGTDM | 1,065.54 | [626.12](../profiles/voxel/ngtdm-r1.md) | 1.70× | 1,826.77 | [644.97](../profiles/voxel/ngtdm-r2.md) | 2.83× | Precomputed adjacency and occupied-tone accumulators. |
| First order | 1,659.85 | [862.84](../profiles/voxel/firstorder-r1.md) | 1.92× | 3,545.51 | [2,886.15](../profiles/voxel/firstorder-r2.md) | 1.23× | The existing first-order batch kernel supplies output columns directly. |
| Intensity histogram | 1,066.82 | [1,222.71](../profiles/voxel/intensityhistogram-r1.md) | 0.87× | 2,686.76 | [1,270.88](../profiles/voxel/intensityhistogram-r2.md) | 2.11× | Reusable windows feed the existing histogram statistics. |
| IVH | 3,110.16 | [514.07](../profiles/voxel/ivh-r1.md) | 6.05× | 2,665.41 | [805.47](../profiles/voxel/ivh-r2.md) | 3.31× | Reusable buffers gather IVH bins from the global lattice. |
| Neighbourhood gather | 22.31 | [3.04](../profiles/voxel/gather-r1.md) | 7.33× | 47.98 | [16.59](../profiles/voxel/gather-r2.md) | 2.89× | Interior centres share precomputed flat strides for every offset. |
| Six texture families on prepared input | 3,248.39 | [608.83](../profiles/voxel/windows-r1.md) | 5.34× | 15,485.75 | [1,309.52](../profiles/voxel/windows-r2.md) | 11.83× | Each worker reuses compact matrices and window buffers across a block of centres. |
| Map storage | 2,689.98 | [613.00](../profiles/voxel/store-r1.md) | 4.39× | — | — | — | Validate flat offsets once and distribute independent maps across threads. |

### Paired radius-1 reruns

Each selection was queued with the baseline binary first, followed by the final binary, with at least five timed extractions per binary. Other jobs used the shared profiler between some runs. Selection covered the three apparent regressions, GLCM and both combined map sets. The profiles above preserve the complete first pass.

| Calculation | Paired baseline, ms | Paired final, ms | Speedup |
| --- | ---: | ---: | ---: |
| 93 defaults | [3,939.66](../profiles/voxel/defaults-r1.paired-baseline.md) | [1,746.34](../profiles/voxel/defaults-r1.paired-final.md) | 2.26× |
| All 147 | [4,784.91](../profiles/voxel/all-r1.paired-baseline.md) | [1,291.85](../profiles/voxel/all-r1.paired-final.md) | 3.70× |
| GLCM | [2,594.18](../profiles/voxel/glcm-r1.paired-baseline.md) | [899.32](../profiles/voxel/glcm-r1.paired-final.md) | 2.88× |
| GLRLM | [2,888.52](../profiles/voxel/glrlm-r1.paired-baseline.md) | [875.69](../profiles/voxel/glrlm-r1.paired-final.md) | 3.30× |
| GLSZM | [1,634.47](../profiles/voxel/glszm-r1.paired-baseline.md) | [698.06](../profiles/voxel/glszm-r1.paired-final.md) | 2.34× |
| Intensity histogram | [2,896.93](../profiles/voxel/intensityhistogram-r1.paired-baseline.md) | [772.01](../profiles/voxel/intensityhistogram-r1.paired-final.md) | 3.75× |

The gather microbenchmark indexes the real centres and reads mask and bin values. The prepared-window measurement includes gathering, per-window matrices, feature arithmetic and column assembly, with preprocessing and dense output allocation excluded. Map storage allocates 93 full-grid float64 W7 maps and scatters 54,639 coordinates per map. Its input is independent of radius, so the W7 storage timing applies to both. The retained engine uses its own map writer.

## The 10.1-million-voxel comparison

The first full radius-1 default run took 4,188.44 ms on LUNG1-001, or 76.66 µs per VOI voxel. Linear scaling to 10,100,000 VOI voxels gives **774.23 s**. The paired rerun gives **322.81 s** at 31.96 µs per voxel. On LUNG1, just 0.16% of the CT is in the VOI, yet 93 full-grid float64 maps occupy 26.13 GB. Per-VOI scaling therefore multiplies whole-CT preparation and map allocation as well as feature computation. [First run](../profiles/voxel/defaults-r1.md) · [Paired run](../profiles/voxel/defaults-r1.paired-final.md)

The direct synthetic volume was 101 × 200 × 500 with an all-ones mask, a deterministic integer intensity field and 7.51 GB of retained map payload. The first three timed extractions after warm-up had a **28.05 s** median (range 24.78–33.95 s). A second three-extraction run had a **13.78 s** median (range 10.26–14.14 s). The final five-extraction median was **11.02 s** (range 7.73–18.24 s), or 0.92 million VOI voxels per second. The supplied Metal time was **24.15 s** for 93 maps at the same VOI count. The final CPU median was 0.46× that time. Concurrent CT benchmarking and builds changed the available CPU across runs. The Metal input volume was unavailable for a same-image comparison. [First synthetic run](../profiles/voxel/synthetic-10m-defaults-r1.md) · [Synthetic repeat](../profiles/voxel/synthetic-10m-defaults-r1-repeat.md) · [Five-extraction confirmation](../profiles/voxel/synthetic-10m-defaults-r1-confirm.md)

## Remaining self time

The two largest self-time entries are percentages of busy thread samples in the first pass. `ndarray logical iteration` denotes `Iter::next` during full-grid traversal. The sampler reports identical machine code shared by several functions as `<deduplicated_symbol>`. Its source-level identity is unresolved. Dense map initialization appears as `__bzero`.

| Calculation | Radius 1 | Radius 2 | Function work |
| --- | --- | --- | --- |
| 93 defaults | `__bzero` 86.6%, `WindowGlcm::evaluate` 6.3% | `WindowGlcm::evaluate` 61.7%, `__bzero` 7.9% | MCC spectra, local feature evaluation and full-grid map allocation. |
| All 147 | `__bzero` 48.2%, `WindowGlcm::evaluate` 19.4% | `WindowGlcm::evaluate` 50.1%, `__bzero` 20.5% | Texture spectra, histogram statistics and full-grid map allocation. |
| GLCM | `WindowGlcm::evaluate` 52.6%, `__bzero` 14.4% | `WindowGlcm::evaluate` 81.3%, `__bzero` 5.1% | Symmetric spectral reduction and the co-occurrence statistics per direction. |
| GLRLM | `ndarray logical iteration` 22.6%, `__bzero` 20.7% | `ndarray logical iteration` 24.5%, `WindowTexture::evaluate` 16.5% | Directional runs, size statistics and full-grid preparation and storage. |
| GLSZM | `__bzero` 42.5%, `ndarray logical iteration` 20% | `ndarray logical iteration` 36%, `<deduplicated_symbol>` 15.4% | Connected components and full-grid preparation and storage. |
| GLDZM | `ndarray logical iteration` 30.4%, `__bzero` 21.7% | `ndarray logical iteration` 33.5%, `<deduplicated_symbol>` 14.3% | Component depths and full-grid preparation and storage. |
| GLDM | `ndarray logical iteration` 36.8%, `__bzero` 15.9% | `ndarray logical iteration` 28.8%, `WindowTexture::evaluate` 16.2% | Neighbour populations and full-grid preparation and storage. |
| NGTDM | `ndarray logical iteration` 44.7%, `<deduplicated_symbol>` 14.9% | `ndarray logical iteration` 28.8%, `WindowTexture::evaluate` 23.5% | Per-level neighbour sums and full-grid preparation and storage. |
| First order | `ndarray logical iteration` 31.8%, `__bzero` 23.9% | `__bzero` 30.8%, `ndarray logical iteration` 28.1% | Full-grid preparation and map clearing accompany the existing first-order kernel. |
| Intensity histogram | `__bzero` 33.4%, `ndarray logical iteration` 16.6% | `__bzero` 16.6%, `ndarray logical iteration` 12.8% | Full-grid preparation and map clearing accompany bin counts and moments. |
| IVH | `<deduplicated_symbol>` 19.8%, `ndarray logical iteration` 15.5% | `<deduplicated_symbol>` 19.2%, `ivh::prepare_ivh` 14.2% | Global IVH preparation, local IVH statistics and full-grid traversal. |
| Neighbourhood gather | `ndarray::map` 47.2%, `NeighborhoodPlan::index` 42.3% | `ndarray::map` 50.9%, `NeighborhoodPlan::index` 42.3% | Reading source bins and masks at the computed indices. |
| Six texture families on prepared input | `WindowGlcm::evaluate` 58.3%, `WindowTexture::evaluate` 18% | `WindowGlcm::evaluate` 75.5%, `WindowTexture::evaluate` 10.9% | Matrix counts and feature arithmetic, chiefly MCC. |
| Map storage | `__bzero` 97.7%, `_kernelrpc_mach_vm_deallocate_trap` 0.9% | — | Allocate zero-background maps and scatter each independent feature column. |
| Synthetic 10.1M | `WindowGlcm::evaluate` 38.6%, `WindowTexture::evaluate` 21.4% | — | Per-window co-occurrences, spectra and counts on a dense VOI. |
| Synthetic repeat | `WindowGlcm::evaluate` 40.7%, `WindowTexture::evaluate` 18.8% | — | Per-window co-occurrences, spectra and counts on a dense VOI. |
| Synthetic confirmation | `WindowGlcm::evaluate` 38.9%, `WindowTexture::evaluate` 18.8% | — | Per-window co-occurrences, spectra and counts on a dense VOI. |

## Measurement and validation notes

The three voxel commits were `b144fec` (reusable parallel windows and compact matrices), `f6a4d68` (MCC root finding and parallel W7 storage) and `3982280` (remove the trivial MCC eigenvector). Both baseline drivers were built from an archive of `9e334b7` with the benchmark example added. The profile harness records the checkout HEAD at the end of each run. The final driver was built from the source tree committed as `3982280`. The before/after extraction numbers include concurrent contributors’ committed work between the two revisions. The prepared-window and W7 storage cases directly exercise the voxel code. Timings include sampling overhead. The profiler serialized benchmark processes, while other sessions also compiled, ran correctness tests and timed a separate IBSI CT filter process outside the lock. The min/max or individual repetitions in each profile show how much wall time varied.

All nine voxel families share the globally prepared image and masks. Discretized features use global bins. The compact paths cover native small windows with at most 125 sites and 128 gray levels. The general ROI algorithms handle other settings, including explicit merged matrices, forced planes and asymmetric GLCMs. The direct bit comparison covers all 147 maps at both radii across one and default Rayon pools. A second test checks selected radius-2 windows against the general ROI formulas at 128 levels with clipped faces and an independent morphological mask.

The workspace format check, warnings-as-errors Clippy across all targets, absolute-import checks and the full release suite passed on the final voxel source. All W7 fixtures passed unchanged. The opt-in suites printed their tables using the original settings. W8 passed 88 reports and 617,456 values. W12 passed 1,322/1,322 under the declared IBSI rules, including 1,321 within literal tolerances. All 33 W6 response maps passed. W13 passed its test with 392/394 numerical comparisons inside consensus. Its existing Simoncelli DFT exceptions are 8.A `stat_qcod` and 9.B `stat_min`. The final voxel tree passed 268 release tests, and integration with committed main at `3ab52d3` passed 272 release tests and the same four opt-in suites. An intermediate shared-tree W8 run found an empty-domain panic in an in-progress ROI matrix change. After the correction, the shared checkout at `4f42922` plus the other contributors’ local changes passed format, Clippy, all 272 release tests and all four opt-in suites. All voxel tolerances and expected values are the originals.

Outside the voxel modules, the retained engine allocates zero-background maps over the entire CT and runs preparation over the source grid. First-order, intensity-histogram and IVH statistics come from the respective feature modules. On the radius-1 GLRLM run, image preparation accounted for 46.6% of busy samples inclusively. On the default run, zero fill accounted for 86.6% of self time. [GLRLM](../profiles/voxel/glrlm-r1.md) · [93 defaults](../profiles/voxel/defaults-r1.md) · [All maps](../profiles/voxel/all-r1.md) · [Prepared windows](../profiles/voxel/windows-r1.md) · [Storage](../profiles/voxel/store-r1.md)
