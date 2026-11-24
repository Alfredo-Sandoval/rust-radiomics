"""Time warm ROI extraction through the public NumPy API.

Read an aligned image and label mask with SimpleITK before starting the timer.
Use RAYON_NUM_THREADS to select how many CPU workers the binding can use.
"""

import argparse
import json
from importlib import import_module
from pathlib import Path
from statistics import median
from time import perf_counter

import numpy as np
import SimpleITK as sitk


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("image", type=Path)
    parser.add_argument("mask", type=Path)
    parser.add_argument("--backend", choices=("rust", "mlx"), default="rust")
    parser.add_argument("--features", nargs="+", help="full feature names (default: 93 ROI features)")
    parser.add_argument("--layout", choices=("native", "bool-mask", "reversed", "swapped"),
                        default="native")
    parser.add_argument("--seconds", type=float, default=5.0, help="minimum time per block")
    args = parser.parse_args()
    if not np.isfinite(args.seconds) or args.seconds <= 0:
        parser.error("--seconds must be finite and positive")

    package = import_module("rustradiomics" if args.backend == "rust" else "mlxradiomics")
    volume = sitk.ReadImage(str(args.image))
    spacing = tuple(reversed(volume.GetSpacing()))
    image = sitk.GetArrayFromImage(volume)
    mask = sitk.GetArrayFromImage(sitk.ReadImage(str(args.mask)))
    settings = (package.Settings(features=tuple(args.features))
                if args.features else package.Settings())
    if args.layout == "bool-mask":
        mask = mask == settings.label
    elif args.layout == "reversed":
        image, mask = image[::-1], mask[::-1]
    elif args.layout == "swapped":
        image = image.astype(image.dtype.newbyteorder("S"))

    def extract():
        return package.extract(image, mask, settings=settings, spacing=spacing)

    warm = extract()
    samples = []
    start = perf_counter()
    while len(samples) < 15 or perf_counter() - start < args.seconds:
        before = perf_counter()
        result = extract()
        samples.append((perf_counter() - before) * 1000)
        del result
    print(json.dumps({
        "backend": args.backend,
        "layout": args.layout,
        "shape": image.shape,
        "image_dtype": str(image.dtype),
        "mask_dtype": str(mask.dtype),
        "features": len(warm.features),
        "roi_voxels": warm.metadata["roi_voxels"],
        "runs": len(samples),
        "median_ms": median(samples),
    }))


if __name__ == "__main__":
    main()
