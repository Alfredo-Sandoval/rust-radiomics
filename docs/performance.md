# Performance

All timings are warm medians on one machine, an Apple M5 Max with 18 logical CPUs, with
release builds and 18 Rayon threads unless a row says otherwise. Input decoding is outside
the timed region unless stated. The machine was shared with other builds and benchmarks
during most runs, so expect a few percent of noise and more for small parallel jobs. The
measurement records, with commands, sampled profiles and the source revision of each run,
are in
[`benchmarks/results/`](https://github.com/Alfredo-Sandoval/rust-radiomics/tree/main/benchmarks/results).

The main dataset is the NSCLC-Radiomics LUNG1-001 CT, a 512 x 512 x 134 grid with a tumor
mask of 54,639 voxels.

## Against PyRadiomics

PyRadiomics 3.1.1.dev111 (commit `8ed5793`) and the rustradiomics 1.0.0 wheel, published as
`rustrad`, computed the 93 PyRadiomics intensity and texture features from the same float32
CT array, uint8 mask and spacing. The selection is 18 first-order, 24 GLCM, 16 GLRLM, 16
GLSZM, 14 GLDM and 5 NGTDM features, with a 25 HU bin width, distance 1, the original image
only and no resampling or normalization. rustradiomics used the PyRadiomics alternative for
every calculation convention in [Parameters](guide/parameters.md#conventions). PyRadiomics
built its SimpleITK images inside the timed call, and SimpleITK used one thread. A perfkit
parity run found all 93 values within `rtol=1e-6` and `atol=1e-9`. The largest relative
difference was 5.1e-8, in `firstorder_Mean`.

`perfkit benchmark pair` ran six rounds on 2026-11-24, with Python 3.13.18, NumPy 2.5.4 and
SimpleITK 2.5.6. Each round started a fresh process per variant, which made one first call
and seven warm calls. The table gives medians across rounds, and the speedup interval is
perfkit's 97% interval of the warm-time ratio. The 1-minute load average stayed between 3.5
and 4.4.

| Variant | Warm (ms) | First call (ms) | Added peak footprint (MB) | Warm speedup |
| --- | ---: | ---: | ---: | ---: |
| PyRadiomics | 742.39 | 796.03 | 305.8 | 1× |
| rustradiomics, 1 Rayon thread | 14.48 | 32.23 | 12.0 | 51× (51.1–51.6) |
| rustradiomics, 18 Rayon threads | 5.37 | 23.86 | 29.9 | 138× (136–142) |

[`benchmarks/results/pyradiomics.json`](https://github.com/Alfredo-Sandoval/rust-radiomics/blob/main/benchmarks/results/pyradiomics.json)
holds these values, and `docs/assets/speed.py` draws the README figure from it. The adapters
are in
[`benchmarks/pyradiomics_callers.py`](https://github.com/Alfredo-Sandoval/rust-radiomics/blob/main/benchmarks/pyradiomics_callers.py),
which also has an mlx-radiomics adapter with the same settings.
perfkit starts each variant with the interpreter given to `--python`, so each variant
gets a small wrapper script that sets its environment. For example, the 18-thread rustradiomics
wrapper is:

```sh
#!/bin/sh
LUNG1=/path/to/LUNG1-001 PYTHONPATH=/path/to/rust-radiomics/benchmarks RAYON_NUM_THREADS=18 \
    exec /path/to/rustradiomics-env/bin/python "$@"
```

The PyRadiomics wrapper sets `LUNG1` and `PYTHONPATH` the same way and runs an interpreter
with PyRadiomics, NumPy and SimpleITK installed. With wrappers `pyradiomics-python`,
`rustradiomics-1-python` and `rustradiomics-18-python`, run from `benchmarks/`:

```bash
perfkit parity run --reference pyradiomics_callers:pyradiomics \
    --candidate pyradiomics_callers:rustradiomics --inputs pyradiomics_callers:ct \
    --reference-python ./pyradiomics-python --candidate-python ./rustradiomics-18-python \
    --rtol 1e-6 --atol 1e-9 --out parity.json
CALL="--inputs pyradiomics_callers:ct --warm 7 --python"
perfkit benchmark pair init --output spec.json --rounds 6 --max-host-load 6 \
    --variant "pyradiomics=perfkit benchmark call pyradiomics_callers:pyradiomics $CALL ./pyradiomics-python" \
    --variant "rustradiomics_1_thread=perfkit benchmark call pyradiomics_callers:rustradiomics $CALL ./rustradiomics-1-python" \
    --variant "rustradiomics_18_threads=perfkit benchmark call pyradiomics_callers:rustradiomics $CALL ./rustradiomics-18-python" \
    --parity-receipt parity.json
perfkit benchmark pair run --spec spec.json --out pyradiomics-pair
```

## Against mlx-radiomics on Metal

The same arrays were loaded once, and the two implementations ran the 93 default ROI
features. mlx-radiomics is v1.1.0 with MLX 0.32.4 and TF32 disabled. All selected values
agreed at `rtol=2e-5` and `atol=2e-6`.

| What was timed | Rust CPU | mlx-radiomics on Metal | Runs |
| --- | ---: | ---: | --- |
| Crate ROI call against the public in-memory Python call, 93 defaults | 5.99 ms | 117.98 ms | 31 |
| Public Python call, 93 defaults | 9.08 ms | 149.66 ms | 2.5 s blocks, Rust, MLX, MLX, Rust |
| Public Python call, `firstorder_Mean` only | 4.38 ms | 4.42 ms | same |

The first row times the original-image ROI engine called by the public Rust extractor and
the public Python extraction call of mlx-radiomics, which includes its host-side
preparation. The second and third rows time `rustradiomics.extract` against
`mlxradiomics.extract` on the same NumPy inputs.

## ROI extraction

Warm `extract` on LUNG1-001, original image, median of 14 calls:

| Selection | Time (ms) |
| --- | ---: |
| `firstorder_Mean` only | 1.152 |
| First order, 21 features | 2.354 |
| Local intensity, 2 features | 2.368 |
| Default ROI, 93 features | 12.226 |
| All ROI, 181 features | 44.639 |

The extraction calls through the NumPy interface copy the image and mask into Rust-owned
storage. For an int32 image and a uint8 mask on LUNG1-001, a complete `rustradiomics.extract`
call takes 3.95 ms for the mean and 7.60 ms for the 93 defaults with 18 workers. With one
worker the mean takes 6.51 ms.

## Voxel maps

Retained float64 maps on LUNG1-001 at kernel radius 1, with the default 64 MiB scratch
budget. Each entry gives the medians of two blocks of at least five seconds.

| Retained maps | Block medians (ms) |
| --- | --- |
| 93 default features | 79.95, 82.95 |
| 21 first-order features | 11.55, 10.68 |

Each map is a full 512 x 512 x 134 array, and the 93 maps together span 26,134,708,224
logical bytes. Only 54,639 voxels per map are selected. On macOS the zero background uses
demand-zero pages, and an earlier build of the retained-map engine peaked at 0.86 GiB of
resident memory for the 93 maps. That peak was not re-measured on the final build.

## Reproduce

Build the examples, then time a case. Set `LUNG1` to the LUNG1-001 directory that holds
`image.nrrd` and `mask.nrrd`.

```bash
cargo +1.96.0 build --locked --release --examples
RAYON_NUM_THREADS=18 target/release/examples/benchmark_roi "$LUNG1/image.nrrd" "$LUNG1/mask.nrrd" 31
RAYON_NUM_THREADS=18 /usr/bin/time -l target/release/examples/benchmark_voxel \
    "$LUNG1/image.nrrd" "$LUNG1/mask.nrrd" 5 1 defaults
```

The examples in `crates/radiomics/examples/` are `benchmark_roi`, `benchmark_voxel` and
`benchmark_voxel_export`. They use only the public API.

The comparison with mlx-radiomics needs Apple silicon and a Python environment with
mlx-radiomics v1.1.0 installed:

```bash
RAYON_NUM_THREADS=18 python3 benchmarks/roi.py target/release/examples/benchmark_roi \
    "$LUNG1/image.nrrd" "$LUNG1/mask.nrrd" --repeats 31
```

`benchmarks/python_roi.py` times the public NumPy entry point. It accepts `--backend rust`
or `--backend mlx`, and `--layout` to vary the array layout. `benchmarks/profile.py NAME --
COMMAND` records a macOS `sample` profile of any of the examples to
`benchmarks/profiles/NAME.md`. Timings depend on the machine and on what else is running, so
repeat a measurement before comparing it.
