"""Public-API adapters that time rustradiomics against other radiomics packages with perfkit.

Every adapter receives the same float32 CT array, uint8 mask and spacing from `ct`, computes
the same 93 PyRadiomics intensity and texture features with a 25 HU bin width, distance 1
and no resampling or normalization, and returns them keyed by PyRadiomics name.
rustradiomics and mlx-radiomics take the same settings, with the PyRadiomics alternative for
every calculation convention. Set `LUNG1` to the LUNG1-001 directory that holds `image.nrrd`
and `mask.nrrd`. Reading the files happens in `ct`, outside the timed calls. SimpleITK uses
one thread; set `RAYON_NUM_THREADS` to choose how many CPU workers rustradiomics uses.
"""

import logging
import os
from pathlib import Path

import numpy as np
import SimpleITK as sitk

sitk.ProcessObject.SetGlobalDefaultNumberOfThreads(1)

SELECTION = {
    "firstorder": (
        "Mean", "Minimum", "Maximum", "Range", "Variance", "Skewness", "Kurtosis", "Energy",
        "TotalEnergy", "RootMeanSquared", "Entropy", "Uniformity", "10Percentile",
        "90Percentile", "Median", "InterquartileRange", "MeanAbsoluteDeviation",
        "RobustMeanAbsoluteDeviation",
    ),
    "glcm": (
        "Autocorrelation", "Contrast", "Correlation", "DifferenceAverage", "JointEnergy",
        "JointEntropy", "Id", "Idm", "JointAverage", "ClusterProminence", "ClusterShade",
        "ClusterTendency", "DifferenceEntropy", "DifferenceVariance", "Idmn", "Idn", "Imc1",
        "Imc2", "InverseVariance", "MaximumProbability", "SumEntropy", "SumSquares", "MCC",
        "SumAverage",
    ),
    "glrlm": (
        "GrayLevelNonUniformity", "GrayLevelNonUniformityNormalized", "GrayLevelVariance",
        "HighGrayLevelRunEmphasis", "LongRunEmphasis", "LongRunHighGrayLevelEmphasis",
        "LongRunLowGrayLevelEmphasis", "LowGrayLevelRunEmphasis", "RunEntropy",
        "RunLengthNonUniformity", "RunLengthNonUniformityNormalized", "RunPercentage",
        "RunVariance", "ShortRunEmphasis", "ShortRunHighGrayLevelEmphasis",
        "ShortRunLowGrayLevelEmphasis",
    ),
    "glszm": (
        "GrayLevelNonUniformity", "GrayLevelNonUniformityNormalized", "GrayLevelVariance",
        "HighGrayLevelZoneEmphasis", "LargeAreaEmphasis", "LargeAreaHighGrayLevelEmphasis",
        "LargeAreaLowGrayLevelEmphasis", "LowGrayLevelZoneEmphasis", "SizeZoneNonUniformity",
        "SizeZoneNonUniformityNormalized", "SmallAreaEmphasis",
        "SmallAreaHighGrayLevelEmphasis", "SmallAreaLowGrayLevelEmphasis", "ZoneEntropy",
        "ZonePercentage", "ZoneVariance",
    ),
    "gldm": (
        "DependenceEntropy", "DependenceNonUniformity", "DependenceNonUniformityNormalized",
        "DependenceVariance", "GrayLevelNonUniformity", "GrayLevelVariance",
        "HighGrayLevelEmphasis", "LargeDependenceEmphasis",
        "LargeDependenceHighGrayLevelEmphasis", "LargeDependenceLowGrayLevelEmphasis",
        "LowGrayLevelEmphasis", "SmallDependenceEmphasis",
        "SmallDependenceHighGrayLevelEmphasis", "SmallDependenceLowGrayLevelEmphasis",
    ),
    "ngtdm": ("Busyness", "Coarseness", "Complexity", "Contrast", "Strength"),
}
NAMES = tuple(f"{family}_{name}" for family, names in SELECTION.items() for name in names)


def ct():
    """Return the LUNG1-001 CT and GTV-1 mask as one perfkit case."""
    directory = Path(os.environ["LUNG1"])
    volumes = [sitk.ReadImage(str(directory / name)) for name in ("image.nrrd", "mask.nrrd")]
    geometry = [(v.GetSize(), v.GetSpacing(), v.GetOrigin(), v.GetDirection()) for v in volumes]
    if geometry[0] != geometry[1]:
        raise ValueError("image and mask geometry differ")
    image = sitk.GetArrayFromImage(volumes[0]).astype(np.float32)
    mask = sitk.GetArrayFromImage(volumes[1]).astype(np.uint8)
    return {"name": "LUNG1-001", "args": [image, mask, tuple(reversed(volumes[0].GetSpacing()))]}


def _extract(package, image, mask, spacing):
    """Extract the 93 features through a package with the rustradiomics settings API."""
    settings = package.Settings(
        features=NAMES,
        bin_width=25.0,
        label=1,
        distances=(1,),
        covariance_divisor="n",
        ellipsoid_area="exact",
        kurtosis="raw",
        zero_variance_moments="nan",
        ngtdm_singular_cases="unadjusted",
        neighborhood_support="distances",
        bin_origin="zero-lattice",
        constant_roi_bin="middle",
        mask_extrapolation="nearest",
        spline_boundary="mirror",
    )
    result = package.extract(image, mask, settings=settings, spacing=spacing)
    values = {name.removeprefix("original_"): float(v) for name, v in result.features.items()}
    if set(values) != set(NAMES):
        raise ValueError(f"{package.__name__} returned a different feature selection")
    return dict(sorted(values.items())), result


def rustradiomics(image, mask, spacing):
    """Extract the 93 features with rustradiomics on the CPU."""
    import rustradiomics as package

    return _extract(package, image, mask, spacing)[0]


def mlxradiomics(image, mask, spacing):
    """Extract the 93 features with mlx-radiomics, which must report GPU execution."""
    import mlxradiomics as package

    values, result = _extract(package, image, mask, spacing)
    if result.metadata.get("device") != "gpu":
        raise RuntimeError("mlx-radiomics did not report GPU execution")
    return values


def pyradiomics(image, mask, spacing):
    """Extract the 93 features with PyRadiomics from in-memory SimpleITK images."""
    import radiomics
    from radiomics.featureextractor import RadiomicsFeatureExtractor

    radiomics.logger.setLevel(logging.ERROR)
    image_sitk, mask_sitk = sitk.GetImageFromArray(image), sitk.GetImageFromArray(mask)
    image_sitk.SetSpacing(tuple(reversed(spacing)))
    mask_sitk.CopyInformation(image_sitk)
    extractor = RadiomicsFeatureExtractor(
        binWidth=25.0, label=1, additionalInfo=False, normalize=False, distances=[1],
        voxelArrayShift=0,
    )
    extractor.disableAllFeatures()
    extractor.enableFeaturesByName(**{family: list(names) for family, names in SELECTION.items()})
    result = extractor.execute(image_sitk, mask_sitk)
    values = {name.removeprefix("original_"): float(v) for name, v in result.items()}
    if set(values) != set(NAMES):
        raise ValueError("PyRadiomics returned a different feature selection")
    return dict(sorted(values.items()))
