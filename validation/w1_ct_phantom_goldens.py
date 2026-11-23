"""Generate goldens for the IBSI 1 CT radiomics phantom (CC BY-NC, never committed).

Usage::

    validation/w1_ct_phantom_goldens.py <ct_radiomics_phantom/nifti dir> <output.json>

The output stays outside the repository. The opt-in Rust test ``tests/w1_ct_phantom.rs``
reads both paths from the environment variables ``W1_IBSI_CT_PHANTOM`` and
``W1_IBSI_CT_GOLDENS``.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).parent))
from w1_common import array_record  # noqa: E402

from mlxradiomics.config.settings import Settings  # noqa: E402
from mlxradiomics.config.spatial import IbsiInterpolation, ImageSettings, Interpolator  # noqa: E402
from mlxradiomics.imaging.preprocessing import prepare  # noqa: E402
from mlxradiomics.imaging.spatial import load_image_pair  # noqa: E402


def main() -> None:
    root, output = Path(sys.argv[1]), Path(sys.argv[2])
    image_path, mask_path = root / "image" / "phantom.nii.gz", root / "mask" / "mask.nii.gz"
    cases = []
    # IBSI 1 configurations C-E: 3-D, trilinear interpolation to 2 mm, rounding, ROI threshold
    # 0.5, resegmentation [-500, 400]; C/D use fixed bin size 25 and E fixed bin count 32.
    configurations = {
        "ibsi_3d_linear_fbs25": ({"bin_width": 25.0, "resegment_range": [-500.0, 400.0]}, Interpolator.LINEAR),
        "ibsi_3d_linear_fbn32": ({"bin_count": 32, "resegment_range": [-500.0, 400.0]}, Interpolator.LINEAR),
        "ibsi_3d_bspline_fbs25": ({"bin_width": 25.0, "resegment_range": [-500.0, 400.0]}, Interpolator.BSPLINE),
    }
    for name, (options, interpolator) in configurations.items():
        settings = Settings(**options)
        spatial = ImageSettings(ibsi_interpolation=IbsiInterpolation(
            spacing=(2.0, 2.0, 2.0), round_intensities=True, mask_threshold=0.5, image_interpolator=interpolator,
        ))
        pair = load_image_pair(image_path, mask_path, settings=settings, image_settings=spatial)
        prepared = prepare(pair.image.array, pair.mask.array, settings, voxel=False)
        cases.append({
            "name": name, "settings": options, "interpolator": interpolator.value,
            "spacing": list(pair.image.spacing), "origin": list(pair.image.origin),
            "image": array_record(pair.image.array), "mask": array_record(pair.mask.array),
            "roi": array_record(prepared.roi), "bins": array_record(prepared.bins), "gray_levels": int(prepared.gray_levels),
        })
        print(name, pair.image.array.shape, int(prepared.roi.sum()), prepared.gray_levels)
    output.write_text(json.dumps(cases, allow_nan=False))


if __name__ == "__main__":
    main()
