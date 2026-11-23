# CPU performance of ROI extraction and core kernels

Apple M5 Max, 18 logical CPUs. These measurements compare release builds from `17d817c` through `4f42922` on the same Mac. The full-feature results include the texture and shape changes that landed between those commits.

## Warm ROI extraction

Each revision ran two blocks of seven timed extractions in baseline–final–final–baseline order per feature set. Each block loads the inputs and warms the extractor before starting its timer. The table gives the median of 14 unsampled calls per revision with 18 Rayon threads. Builds and other benchmarks shared the Mac.

| LUNG1-001 selection | Before (ms) | After (ms) | Speedup |
|---|---:|---:|---:|
| Mean only | 9.936 | 1.152 | 8.62× |
| First order, 21 | 11.919 | 2.354 | 5.06× |
| Local intensity, 2 | 16.005 | 2.368 | 6.76× |
| Default ROI, 93 | 105.866 | 12.226 | 8.66× |
| All ROI, 181 | 671.101 | 44.639 | 15.03× |

A separate 31-run head-to-head on the 93 defaults measured **5.99 ms on the Rust CPU and 117.98 ms on Metal**. The Rust driver times the original-image ROI engine called by the public extractor. The MLX driver times its public in-memory Python extraction call, including host-side preparation. The MLX checkout was `b58783c` (tag `v1.1.0`, MLX runtime 0.32.4). The editable installation reported stale distribution metadata `0.2.0.dev1`. The two implementations agreed on all 93 outputs at the comparison driver’s tolerances (`rtol=2e-5`, `atol=2e-6`).

The earlier reference medians supplied for this Mac were 10.5 ms for first order, 12.7 ms for local intensity, 91.5 ms for the defaults and 422.1 ms for all 181. The reported Metal reference was 17.47 ms. The two Rust timing tables use measurements from this run for both sides of every speedup.

## Sampled calculations

Each profile records a one-second warm-up followed by four seconds of macOS stack sampling. The driver collects its own median over at least six seconds of repeated work. The lock serializes cooperating profiling commands. CPU load from separate processes varied, especially for small parallel jobs. The six profiles that touch the full-grid bounds scan use `4f42922`. The remaining nine use `3ab52d3`, which has identical code for those paths. Each profile records its timing, sample count, command and measured source revision. An early trial reused a cached baseline executable and was discarded. Separate build directories and executable checksums distinguish the measurements included here.

| Calculation | Before (ms) | After (ms) | Speedup | Change | Profiles |
|---|---:|---:|---:|---|---|
| Mean only | 20.234 | 4.214 | 4.80× | The search skips empty mask blocks. Mean bypasses unrelated statistics. | [before](../profiles/core/mean.baseline.md) · [after](../profiles/core/mean.md) |
| First order, 21 | 20.995 | 2.385 | 8.80× | The compact integral CT range permits counting sort for exact quantiles. | [before](../profiles/core/firstorder.baseline.md) · [after](../profiles/core/firstorder.md) |
| Local intensity, 2 | 43.499 | 7.655 | 5.68× | Row-prefix sums serve neighboring physical spheres in one pass across centers. | [before](../profiles/core/localintensity.baseline.md) · [after](../profiles/core/localintensity.md) |
| Default ROI, 93 | 191.071 | 6.279 | 30.43× | Includes parallel run-length, co-occurrence, dependence and zone counting from the texture work. | [before](../profiles/core/defaults93.baseline.md) · [after](../profiles/core/defaults93.md) |
| All ROI, 181 | 1,250.022 | 50.130 | 24.94× | Also includes geometry pruning and shared spatial weights for the shape features. | [before](../profiles/core/all181.baseline.md) · [after](../profiles/core/all181.md) |

LUNG1-001 is a 512 × 512 × 134 grid with 54,639 selected voxels. The bounds driver places two selected labels at one corner of that grid and calls the public ROI engine. Its timing includes the remaining fixed work for a two-voxel extraction. The prepared stages use the real cropped mask, with a physical sphere halo for local intensity. The full-map call allocates 21 output grids in the existing f32 map format. The batch driver visits the same centers in groups of 2,048 and releases each group of f64 columns.

The read timings include opening the file, decoding it and constructing the returned image or mask. Repeated reads use the filesystem cache. NRRD uses LUNG1-001, and NIfTI uses the 204 × 201 × 60 IBSI CT phantom. Extraction medians above exclude file reads.

## Where the CPU time goes

Percentages are self-time shares of busy stack samples from the final profiles. Optimized builds sometimes attribute inlined code to a closure or runtime symbol. See the inclusive stacks for the radiomics callers.

| Calculation | Largest self-time entries | Remaining work |
|---|---|---|
| Mean only | `aligned byte search` 36.8%, `__psynch_mutexwait` 23.5% | The end-to-end search still reads label rows and constructs a prepared ROI. |
| First order, 21 | `aligned byte search` 30.6%, `__psynch_mutexwait` 20.9% | Bins, samples and the selected statistical moments still need to be calculated. |
| Local intensity, 2 | `aligned byte search` 31.6%, `local_intensity_statistics closure` 28.6% | Every occupied center must be compared with the local and global peaks. |
| Default ROI, 93 | `matrices` 12.2%, `run_matrix` 12.1% | Independent CPU workers still build texture matrices and reduce counts. |
| All ROI, 181 | `inlined closure call` 85.5%, `pruned_max_square closure` 2.6% | Spatial statistics and geometry include work outside the core kernels. |

A full-grid mask must be searched on each new extraction. Earlier measurements used a parallel row scan. A second change checks each 128 Ki-element block for the label before inspecting its rows. A 28-sample interleaved microbenchmark of the real mask measured 4.95 ms before and 2.19 ms after at the same block size.

For streaming batches, the intermediate parallel implementation increased scheduling costs. Its sampled median was 183.75 ms. Returning each batch to the caller’s thread and iterating occupied histogram levels from a bit mask brought the next trial to 49.96 ms. Full-grid map workers still partition contiguous output.

## Validation

- Rust 1.96.0 formatting, Clippy across the workspace and all targets with warnings denied, and the locked release suite passed on the measured source. The suite passed 272 tests with the external phantom and Python comparison data enabled.
- IBSI 1 passed 482/482 digital-phantom rows and 1,322/1,322 CT rows across A–E under the repository’s declared rules. Of the CT rows, 1,321 met the literal reference tolerance. The remaining row is the previously documented three-significant-digit comparison.
- IBSI 2 phase 1 passed all 33 response maps. Phase 2 passed all 323 consensus-grid rows. The additional DFT-grid checks passed 69/71, with the existing Simoncelli exceptions for 8.A quartile coefficient of dispersion and 9.B minimum.
- The export parity check passed 88 MLX reports and 617,456 values. The CT interpolation and binning check also passed.
- Integration at `a324330`, including the texture update `9a7a2d5`, also passed the same 272 release tests and both bit-level comparisons. Its timings are outside the tables above.
- All 181 scalar IEEE words on LUNG1-001 matched across 1 and 18 Rayon threads. The 21 first-order and two local-intensity scalars also matched the baseline. The permanent synthetic tests compare all 21 voxel maps across thread counts, compare every selected map center to the f64 streaming result after output conversion, and exercise clipped spheres and sparse masks.

## Reproducing a profile

Build the example at the measured revision. The recorded commands use copies of the executable under `target/bench-core/`. The `baseline` directory holds `17d817c`, `final` holds `3ab52d3`, and `mask` holds `4f42922`. The old revision uses the same two benchmark drivers from `3ab52d3`. On macOS, set `LUNG1` to the LUNG1-001 directory and run the prepared local-intensity calculation as follows.

```sh
cargo +1.96.0 build --locked --release --example benchmark_roi
python3 benchmarks/profile.py core/defaults93 -- target/release/examples/benchmark_roi "$LUNG1/image.nrrd" "$LUNG1/mask.nrrd" profile
```

From the Rust repository root, the same 31-run defaults comparison can be repeated with a Python environment containing mlx-radiomics and its Metal runtime. The comparison driver reads the arrays once per backend and checks the returned values.

```sh
RAYON_NUM_THREADS=18 python3 benchmarks/roi.py target/release/examples/benchmark_roi "$LUNG1/image.nrrd" "$LUNG1/mask.nrrd" --repeats 31
```

## Retained voxel maps and NIfTI export

Computing the 93 default radius-1 float64 maps on LUNG1-001 took **63 ms** with **0.86 GiB peak process RSS** on an 18-core Apple M5 Max. The previous engine took **670 ms** and peaked at **16.95 GiB**. Each map still exposes a writable 512 × 512 × 134 array with its original geometry. Together the maps span 26,134,708,224 logical bytes. Only 54,639 voxels per map are selected.

### Timing and memory

Release builds of `aa6355a` and `df67aec` ran in baseline–final–final–baseline order on the same Mac with 18 Rayon threads and the shared benchmark lock. Each process loaded the CT once, warmed the public extractor, then timed at least five seconds of extractions. Input I/O and result destruction are outside the timer. Peak RSS from macOS `/usr/bin/time -l` covers the whole process. The **10.6×** speedup uses the average of the two block medians for each build.

| 93 retained maps | First block median | Second block median | Peak RSS across blocks |
|---|---:|---:|---:|
| Before | 673.14 ms | 666.62 ms | 16.95 GiB |
| After | 62.87 ms | 63.34 ms | 0.86 GiB |

The following runs include macOS stack sampling. The export case writes three compressed float32 NIfTI maps and their report through the public exporter on a synthetic 48 × 256 × 256 grid with 6,776 selected voxels.

| Sampled calculation | Before (ms) | After (ms) | Speedup | Profiles |
|---|---:|---:|---:|---|
| LUNG1-001, 93 retained maps, radius 1 | 920.05 | 180.76 | 5.09× | [before](../profiles/core/retained-defaults-r1.baseline.md) · [after](../profiles/core/retained-defaults-r1.md) |
| LUNG1-001, 16 retained GLRLM maps, radius 1 | 319.40 | 21.98 | 14.53× | [before](../profiles/core/retained-glrlm-r1.baseline.md) · [after](../profiles/core/retained-glrlm-r1.md) |
| Sparse NIfTI export, three first-order maps | 53.56 | 11.07 | 4.84× | [before](../profiles/core/sparse-nifti-export.baseline.md) · [after](../profiles/core/sparse-nifti-export.md) |

In the 93-map profiles, `__bzero` fell from 33.6% to 0.2% of busy self-time samples. In the GLRLM profiles, preparation fell from 25.4% to 0.4% of inclusive samples. The full-size arrays use demand-zero pages for large buffers on supported macOS kernels. Other platforms use the ordinary zero-initialized ndarray allocation. The measured memory reduction applies to macOS. [Allocation implementation](../../crates/radiomics/src/numerics/runtime.rs)

### Layout and precision

MLX v1.1.0 allocates a zero-background array at the original image shape for each retained feature. It bounds the working neighborhood to the selected centers plus kernel reach. It delivers float32 samples at source coordinates, including for float64 output arrays. Rust follows the same public precision boundary and checks the retained-output limit against the full logical byte count. [MLX working region](https://github.com/MedMLX/mlx-radiomics/blob/v1.1.0/src/mlxradiomics/voxel/extraction.py#L432-L452) · [Retained shape](https://github.com/MedMLX/mlx-radiomics/blob/v1.1.0/src/mlxradiomics/voxel/extraction.py#L515-L520) · [Sample precision and coordinates](https://github.com/MedMLX/mlx-radiomics/blob/v1.1.0/src/mlxradiomics/voxel/extraction.py#L695-L712)

For masked original-image kernels, Rust prepares the label bounds plus the exact kernel reach when normalization is off. It translates selected centers back to source coordinates for retained and streamed outputs. Unmasked calibration, global normalization and filtered images still use the complete source where required. For file export, MLX spools selected values to disk. Rust buffers float32 columns and shared source offsets for one image in RAM, then writes each NIfTI in source order, emitting zeros for background runs. The export buffer scales with the number of selected voxels and features. [MLX export storage](https://github.com/MedMLX/mlx-radiomics/blob/v1.1.0/src/mlxradiomics/voxel/storage.py) · [Rust export sink](../../crates/radiomics/src/workflows/reporting.rs)

### Verification

On LUNG1-001, all 5,081,427 selected float64 bit patterns in the 93 maps matched the baseline with both 1 and 18 Rayon threads. Every one of the 3,261,757,101 background values was positive zero. Synthetic tests also check source coordinates, clipped neighborhoods, independent morphology masks, normalized and unmasked inputs, and exported float32 bit patterns including signed zero and NaN payloads.

Rust 1.96.0 formatting, strict workspace Clippy and all 278 release tests passed at `df67aec`. The W7 voxel goldens and W8 export tests passed, including the parity comparison of 88 reports and 617,456 values. The rebuilt Python extension passed 254 tests. The existing IBSI gates passed all 482 digital phantom rows, all 1,322 CT rows under the declared rules (1,321 literal), all 33 phase-1 response maps and all 323 phase-2 consensus-grid rows. The supplemental DFT grids reproduced the two documented exceptions (69/71).

To repeat the timing and RSS measurement on macOS, set `LUNG1` to the LUNG1-001 input directory. The export example generates its own volume.

```sh
cargo +1.96.0 build --locked --release --example benchmark_voxel --example benchmark_voxel_export
RAYON_NUM_THREADS=18 /usr/bin/time -l target/release/examples/benchmark_voxel "$LUNG1/image.nrrd" "$LUNG1/mask.nrrd" 5 1 defaults
RAYON_NUM_THREADS=18 python3 benchmarks/profile.py core/sparse-nifti-export -- target/release/examples/benchmark_voxel_export
```

## Additional MLX v1.1.0 compatibility checks

We ran 164 cases from the upstream API, voxel scheduling, first-order, local-intensity, input-domain, moment and texture tests against both packages. The real-image cases used the HCC-TACE-Seg HCC_063 contrast CT and its aligned liver and tumor masks. The Rust run routed extraction through its public Python binding and kept the upstream numerical assertions and tolerances. [Upstream reference tests](https://github.com/MedMLX/mlx-radiomics/tree/v1.1.0/tests)

The upstream sparse-lattice cases (a 25,001-level lattice with a 1 MiB calculation budget) are refused exactly as MLX refuses them, because the native CPU planner uses MLX's working-set estimator. The test `upstream_sparse_lattice_refusals_hold_through_public_extraction` covers them. [Upstream sparse GLCM cases](https://github.com/MedMLX/mlx-radiomics/blob/v1.1.0/tests/test_sparse_glcm_voxels.py) · [CPU batch estimate](../../crates/radiomics/src/voxel/extraction.rs)

Export admission charges the retained float32 rows, the u32 offsets and a 459,104-byte writer allowance. A public export on a 56 × 56 × 56 source grid with two selected voxels and two first-order maps therefore completes under a 1 MiB export limit (459,128 bytes charged). This limit is independent of the calculation budget above. [MLX export storage](https://github.com/MedMLX/mlx-radiomics/blob/v1.1.0/src/mlxradiomics/voxel/storage.py) · [Rust export admission](../../crates/radiomics/src/workflows/reporting.rs)

## Voxel scratch limits and delivery batches

On LUNG1-001, the 93 default radius-one maps cover 54,639 tumor voxels on a 512 × 512 × 134 source grid. With a 64 MiB scratch limit, mlx-radiomics v1.1.0 delivers 75 coordinates per callback and groups 975 first-order rows. Rust at `4bc9a28` reproduces both sizes, preserves source coordinates and computes independent texture batches concurrently within the budget.

| Warm retained maps on Apple M5 Max, 18 Rayon threads | First block median | Second block median |
|---|---:|---:|
| `faa6ef9` | 62.92 ms | 65.17 ms |
| `4bc9a28` | 76.63 ms | 76.57 ms |

The release benchmark loaded the image and mask once per process. It ran each block for at least five seconds in baseline, changed, changed, baseline order. The mean of the two block medians increased from 64.04 to 76.60 ms (19.6%). Separate sampling runs recorded 71.16 and 76.54 ms. [Baseline CPU profile](../profiles/core/voxel-admission-r1.baseline.md) · [CPU profile with reference batching](../profiles/core/voxel-admission-r1.md)

All 1,512 scheduler fixtures matched MLX's memory estimates or refusal, including 42 grouped cases and 306 refusals. Six streamed extractions spanning the IBSI and PyRadiomics conventions and 1, 8 and 256 MiB reproduced the reference callback lengths and memory fields. All 10,944 selected map values met the existing numerical tolerances. Native streamed and retained values matched bit for bit at one and 18 threads. The Rust regression suite also checks sparse 25,001-level lattices and a radius-six neighborhood that requires 1,945,728 bytes at a 2 MiB limit. [Voxel tests](../../crates/radiomics/tests/w7_voxel.rs)

## Retained voxel map delivery

The CPU engine at `2178bcb` writes consecutive selected coordinates directly into source-grid map slices. It rounds each sample to float32 as it writes, including for float64 maps. It counts validity from the rounded values. Retaining maps no longer needs a separate float32 column for each feature. Callback sizes still follow the scheduler at `4bc9a28`.

These LUNG1-001 radius-one measurements used 18 Rayon threads on the M5 Max. Each release executable loaded the image and mask before timing. The benchmark ran each feature set in baseline, changed, changed, baseline order under the shared CPU and GPU benchmark locks. Each block lasted at least five seconds. The percentage compares the means of the two block medians. [Benchmark entry point](../../crates/radiomics/examples/benchmark_voxel.rs)

| Retained float64 maps | Baseline block medians, ms (`bc7a192`) | Changed block medians, ms (`2178bcb`) | Reduction in time | CPU samples |
|---|---:|---:|---:|---|
| 93 defaults | 89.61, 84.63 | 79.95, 82.95 | 6.5% | [Before](../profiles/core/voxel-delivery-r1.baseline.md) · [After](../profiles/core/voxel-delivery-r1.md) |
| 21 first-order features | 11.82, 13.54 | 11.55, 10.68 | 12.3% | [Before](../profiles/core/firstorder-delivery-r1.baseline.md) · [After](../profiles/core/firstorder-delivery-r1.md) |

Across 54,639 tumor voxels and 93 features, all 5,081,427 selected samples matched the preceding engine bit for bit for both retained dtypes and for streamed output. All 729 callbacks preserved coordinate order and batch lengths at one and 18 threads. The per-feature validity counts matched in every run. Separate sparse 2D and 3D cases matched across all 147 voxel features, including NaNs. A derived LBP3D case matched across six maps and 1,724 undefined feature values. [Map delivery](../../crates/radiomics/src/workflows/engine/delivery.rs)

## NumPy array extraction

On LUNG1-001 the Python binding copies a 134 × 512 × 512 int32 image and its label mask into Rust-owned storage for each array extraction. At `2cfa716` copies of at least 8 MiB can use the Rayon workers. A boolean mask is normalized directly into the owned Rust allocation. The GIL stays held until both snapshots are ready. [Binding implementation](../../crates/radiomics-py/src/arrays.rs)

The table measures the complete warm Python `extract` call on the M5 Max. Each entry is the mean of two block medians, with at least 15 calls and 2.5 seconds per block. The baseline (`c23929c`) and changed wheels ran in baseline, changed, changed, baseline order on identical NumPy inputs. File decoding, layout changes and imports finished before timing. The unchanged Rust entry point, timed with already-owned inputs in two 300-call blocks, took 1.89 ms for the mean and 5.88 ms for the defaults with 18 workers. [Public Python benchmark](../python_roi.py) · [Native Rust benchmark](../../crates/radiomics/examples/benchmark_roi.rs)

| Workers | Selection | NumPy inputs | Before, ms | After, ms | Speedup | CPU samples |
|---:|---|---|---:|---:|---:|---|
| 18 | Mean | int32 image, uint8 mask | 5.63 | 3.95 | 1.42× | [Before](../profiles/core/python-mean.baseline.md) · [After](../profiles/core/python-mean.md) |
| 18 | 93 defaults | int32 image, uint8 mask | 9.83 | 7.60 | 1.29× | [Before](../profiles/core/python-defaults.baseline.md) · [After](../profiles/core/python-defaults.md) |
| 18 | Mean | int32 image, bool mask | 8.04 | 4.72 | 1.70× | [Before](../profiles/core/python-mean-bool.baseline.md) · [After](../profiles/core/python-mean-bool.md) |
| 18 | 93 defaults | int32 image, bool mask | 12.16 | 8.81 | 1.38× | [Before](../profiles/core/python-defaults-bool.baseline.md) · [After](../profiles/core/python-defaults-bool.md) |
| 18 | Mean | Reversed first axis, both arrays | 9.50 | 7.73 | 1.23× |  |
| 18 | Mean | Byte-swapped int32 image | 9.28 | 7.27 | 1.28× |  |
| 1 | Mean | int32 image, uint8 mask | 6.42 | 6.51 | 0.99× |  |
| 1 | Mean | int32 image, bool mask | 15.87 | 14.23 | 1.12× |  |

A separate public-API comparison loaded the same native arrays into one Python process and timed Rust, MLX, MLX, Rust in 2.5-second blocks. For the 93 defaults, Rust CPU took **9.08 ms** and mlx-radiomics v1.1.0 on Metal took **149.66 ms** (16.5×). Mean alone took 4.38 ms and 4.42 ms, respectively. All selected scalars agreed at `rtol=2e-5`, `atol=2e-6`. MLX ran with `MLX_ENABLE_TF32=0`, and Rust used 18 Rayon workers. The Python benchmark accepts `--backend rust` and `--backend mlx` to time either entry point.

Native arrays, read-only masks with noncanonical true bytes, and byte-swapped images with reversed strides all reproduced the previous wheel's 181 ROI scalars and 5,081,427 selected samples for the 93 default maps bit for bit. The checks covered both retained dtypes, stream order and validity metadata at one and 18 workers. A separate four-caller check completed 12 streaming extractions whose callbacks each ran a nested ROI extraction. [Array layout regressions](../../crates/radiomics-py/tests/test_parity.py)
