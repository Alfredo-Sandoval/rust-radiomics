"""Compare warm, in-memory Rust CPU and mlx-radiomics GPU ROI extraction on one case."""

import argparse
from importlib.metadata import version
import json
from pathlib import Path
from statistics import median
import subprocess
from time import perf_counter

from mlxradiomics import Settings, extract
from mlxradiomics.imaging.io import read_image, read_mask
import numpy as np


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path, help="release benchmark_roi example")
    parser.add_argument("image", type=Path)
    parser.add_argument("mask", type=Path)
    parser.add_argument("--repeats", type=int, default=5)
    parser.add_argument("--features", nargs="+", help="full feature keys, default is Settings()")
    args = parser.parse_args()
    if args.repeats < 1:
        parser.error("--repeats must be positive")
    settings = Settings(features=tuple(args.features)) if args.features else Settings()
    image, mask = read_image(args.image), read_mask(args.mask)
    rust = json.loads(subprocess.check_output([
        str(args.binary.resolve()), str(args.image), str(args.mask), str(args.repeats), *settings.features
    ], text=True))

    def run():
        return extract(image.array, mask.array, spacing=image.spacing, settings=settings).features

    values = run()
    samples = []
    for _ in range(args.repeats):
        start = perf_counter()
        result = run()
        samples.append(1000 * (perf_counter() - start))
        del result

    print(f"{args.image.name}, {rust['roi_voxels']} ROI voxels, {len(values)} features, "
          f"{args.repeats} warm runs per backend")
    print(f"Rust CPU median {rust['median_ms']:.2f} ms; "
          f"mlx-radiomics {version('mlxradiomics')} GPU median {median(samples):.2f} ms")
    assert list(values) == list(rust["features"]), "feature names or order differ"
    expected = np.array(list(values.values()), dtype=float)
    actual = np.array(list(rust["features"].values()), dtype=float)
    close = np.isclose(actual, expected, rtol=2e-5, atol=2e-6, equal_nan=True)
    if not close.all():
        for name, x, y, ok in zip(values, actual, expected, close, strict=True):
            if not ok:
                print(f"{name}: Rust {x:g}, MLX {y:g}")
        raise SystemExit(f"{np.count_nonzero(~close)} features outside tolerance")
    print("All feature values agree at rtol=2e-5 and atol=2e-6.")


if __name__ == "__main__":
    main()
