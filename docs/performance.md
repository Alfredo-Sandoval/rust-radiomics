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
preparation. The second and third rows time `rustrad.extract` against
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
storage. For an int32 image and a uint8 mask on LUNG1-001, a complete `rustrad.extract`
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
