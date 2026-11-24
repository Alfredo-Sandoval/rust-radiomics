# Features

`rustradiomics.FEATURES` (Python) and `radiomics::features()` (Rust) list all 200
selectable names, including all 116 non-deprecated PyRadiomics features. Without a
`features` selection, extraction returns the 93 default features: the PyRadiomics
intensity and texture features. Shape, moment invariants, local intensity, IVH, intensity
histogram and GLDZM features are selected by name.

| Family | Features | Defaults | Modes |
| --- | ---: | ---: | --- |
| `firstorder` | 21 | 18 | ROI, voxel |
| `intensityhistogram` | 23 | 0 | ROI, voxel |
| `glcm` | 26 | 24 | ROI, voxel |
| `gldm` (NGLDM) | 17 | 14 | ROI, voxel |
| `ngtdm` | 5 | 5 | ROI, voxel |
| `glrlm` | 16 | 16 | ROI, voxel |
| `glszm` | 16 | 16 | ROI, voxel |
| `gldzm` | 16 | 0 | ROI, voxel |
| `ivh` | 7 | 0 | ROI, voxel |
| `localintensity` | 2 | 0 | 3D ROI |
| `shape` | 32 | 0 | 3D ROI |
| `momentinvariant` | 10 | 0 | 3D ROI |
| `shape2D` | 9 | 0 | 2D ROI, or one slice of a 3D ROI with `force2d_axis` |

The counts come from the catalog in the code. `list_features` returns 191 names for 3D
input, 156 for 2D input and 147 in voxel mode:

```python
from rustradiomics import Settings, list_features

len(list_features(ndim=3))                # 191
len(list_features(ndim=2))                # 156
len(list_features(mode="voxel"))          # 147
Settings(features=list_features("firstorder", "glcm"))
```

`list_features(ndim=3)` omits `shape2D`, because 3D use needs a single-slice ROI and an
explicit `force2d_axis`. Selecting a family does not change binning or numerical
conventions; those are `Settings` fields (see [Parameters](parameters.md#settings)).

## Status

All 200 features are implemented in ROI mode, and the 147 voxel-capable features are
implemented in voxel mode. Their values were compared with exports from mlx-radiomics
v1.1.0 for every family and mode, and the moment invariants with mlx-radiomics on the IBSI
digital phantom. The IBSI 1 digital-phantom and CT-phantom tables cover the features they
publish values for. [Technical notes](../technical.md#validation) give the counts.
Twenty-six features have no IBSI identifier: `firstorder_TotalEnergy`,
`firstorder_Entropy`, `firstorder_Uniformity`, `glcm_MCC`, the three
`shape_Maximum2DDiameter*` features, all nine `shape2D` features and all ten
`momentinvariant` features.

## Catalog

Identifiers, titles and units come from the IBSI reference manual (CC BY 4.0). Units
marked `1*` are dimensionless, with discrete index powers retained by the formula. A
blank IBSI column marks a PyRadiomics feature or extension with no IBSI definition.
Names are shown without the image prefix; extraction keys add it, for example
`original_glcm_Contrast` or `wavelet-HHL_glcm_Contrast`.

### `firstorder` (21)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `firstorder_Mean` | Q4LE | Mean intensity | `intensity` | yes | ROI, voxel |
| `firstorder_Minimum` | 1GSF | Minimum intensity | `intensity` | yes | ROI, voxel |
| `firstorder_Maximum` | 84IY | Maximum intensity | `intensity` | yes | ROI, voxel |
| `firstorder_Range` | 2OJQ | Intensity range | `intensity` | yes | ROI, voxel |
| `firstorder_Variance` | ECT3 | Intensity variance | `intensity^2` | yes | ROI, voxel |
| `firstorder_Skewness` | KE2A | Intensity skewness | `1` | yes | ROI, voxel |
| `firstorder_Kurtosis` | IPH6 | (Excess) intensity kurtosis | `1` | yes | ROI, voxel |
| `firstorder_Energy` | N8CA | Intensity-based energy | `intensity^2` | yes | ROI, voxel |
| `firstorder_TotalEnergy` |  |  |  | yes | ROI, voxel |
| `firstorder_RootMeanSquared` | 5ZWQ | Root mean square intensity | `intensity` | yes | ROI, voxel |
| `firstorder_Entropy` |  |  |  | yes | ROI, voxel |
| `firstorder_Uniformity` |  |  |  | yes | ROI, voxel |
| `firstorder_10Percentile` | QG58 | 10th intensity percentile | `intensity` | yes | ROI, voxel |
| `firstorder_90Percentile` | 8DWT | 90th intensity percentile | `intensity` | yes | ROI, voxel |
| `firstorder_Median` | Y12H | Median intensity | `intensity` | yes | ROI, voxel |
| `firstorder_InterquartileRange` | SALO | Intensity interquartile range | `intensity` | yes | ROI, voxel |
| `firstorder_MeanAbsoluteDeviation` | 4FUA | Intensity-based mean absolute deviation | `intensity` | yes | ROI, voxel |
| `firstorder_RobustMeanAbsoluteDeviation` | 1128 | Intensity-based robust mean absolute deviation | `intensity` | yes | ROI, voxel |
| `firstorder_MedianAbsoluteDeviation` | N72L | Intensity-based median absolute deviation | `intensity` |  | ROI, voxel |
| `firstorder_CoefficientOfVariation` | 7TET | Intensity-based coefficient of variation | `1` |  | ROI, voxel |
| `firstorder_QuartileCoefficientOfDispersion` | 9S40 | Intensity-based quartile coefficient of dispersion | `1` |  | ROI, voxel |

### `glcm` (26)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `glcm_Autocorrelation` | QWB0 | Autocorrelation | `1*` | yes | ROI, voxel |
| `glcm_Contrast` | ACUI | Contrast | `1*` | yes | ROI, voxel |
| `glcm_Correlation` | NI2N | Correlation | `1*` | yes | ROI, voxel |
| `glcm_DifferenceAverage` | TF7R | Difference average | `1*` | yes | ROI, voxel |
| `glcm_JointEnergy` | 8ZQL | Angular second moment | `1*` | yes | ROI, voxel |
| `glcm_JointEntropy` | TU9B | Joint entropy | `bit` | yes | ROI, voxel |
| `glcm_Id` | IB1Z | Inverse difference | `1*` | yes | ROI, voxel |
| `glcm_Idm` | WF0Z | Inverse difference moment | `1*` | yes | ROI, voxel |
| `glcm_JointAverage` | 60VM | Joint average | `1*` | yes | ROI, voxel |
| `glcm_ClusterProminence` | AE86 | Cluster prominence | `1*` | yes | ROI, voxel |
| `glcm_ClusterShade` | 7NFM | Cluster shade | `1*` | yes | ROI, voxel |
| `glcm_ClusterTendency` | DG8W | Cluster tendency | `1*` | yes | ROI, voxel |
| `glcm_DifferenceEntropy` | NTRS | Difference entropy | `bit` | yes | ROI, voxel |
| `glcm_DifferenceVariance` | D3YU | Difference variance | `1*` | yes | ROI, voxel |
| `glcm_Imc1` | R8DG | Information correlation 1 | `1*` | yes | ROI, voxel |
| `glcm_Imc2` | JN9H | Information correlation 2 | `1*` | yes | ROI, voxel |
| `glcm_Idmn` | 1QCO | Normalised inverse difference moment | `1*` | yes | ROI, voxel |
| `glcm_Idn` | NDRX | Normalised inverse difference | `1*` | yes | ROI, voxel |
| `glcm_InverseVariance` | E8JP | Inverse variance | `1*` | yes | ROI, voxel |
| `glcm_MaximumProbability` | GYBY | Joint maximum | `1*` | yes | ROI, voxel |
| `glcm_SumAverage` | ZGXS | Sum average | `1*` | yes | ROI, voxel |
| `glcm_SumEntropy` | P6QZ | Sum entropy | `bit` | yes | ROI, voxel |
| `glcm_SumSquares` | UR99 | Joint variance | `1*` | yes | ROI, voxel |
| `glcm_MCC` |  |  |  | yes | ROI, voxel |
| `glcm_Dissimilarity` | 8S9J | Dissimilarity | `1*` |  | ROI, voxel |
| `glcm_SumVariance` | OEEB | Sum variance | `1*` |  | ROI, voxel |

### `gldm` (17)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `gldm_SmallDependenceEmphasis` | SODN | Low dependence emphasis | `1*` | yes | ROI, voxel |
| `gldm_LargeDependenceEmphasis` | IMOQ | High dependence emphasis | `1*` | yes | ROI, voxel |
| `gldm_GrayLevelNonUniformity` | FP8K | Grey level non-uniformity | `1*` | yes | ROI, voxel |
| `gldm_DependenceNonUniformity` | Z87G | Dependence count non-uniformity | `1*` | yes | ROI, voxel |
| `gldm_DependenceNonUniformityNormalized` | OKJI | Normalised dependence count non-uniformity | `1*` | yes | ROI, voxel |
| `gldm_GrayLevelVariance` | 1PFV | Grey level variance | `1*` | yes | ROI, voxel |
| `gldm_DependenceVariance` | DNX2 | Dependence count variance | `1*` | yes | ROI, voxel |
| `gldm_DependenceEntropy` | FCBV | Dependence count entropy | `bit` | yes | ROI, voxel |
| `gldm_LowGrayLevelEmphasis` | TL9H | Low grey level count emphasis | `1*` | yes | ROI, voxel |
| `gldm_HighGrayLevelEmphasis` | OAE7 | High grey level count emphasis | `1*` | yes | ROI, voxel |
| `gldm_SmallDependenceLowGrayLevelEmphasis` | EQ3F | Low dependence low grey level emphasis | `1*` | yes | ROI, voxel |
| `gldm_SmallDependenceHighGrayLevelEmphasis` | JA6D | Low dependence high grey level emphasis | `1*` | yes | ROI, voxel |
| `gldm_LargeDependenceLowGrayLevelEmphasis` | NBZI | High dependence low grey level emphasis | `1*` | yes | ROI, voxel |
| `gldm_LargeDependenceHighGrayLevelEmphasis` | 9QMG | High dependence high grey level emphasis | `1*` | yes | ROI, voxel |
| `gldm_GrayLevelNonUniformityNormalized` | 5SPA | Normalised grey level non-uniformity | `1*` |  | ROI, voxel |
| `gldm_DependenceCountEnergy` | CAS9 | Dependence count energy | `1*` |  | ROI, voxel |
| `gldm_DependencePercentage` | 6XV8 | Dependence count percentage | `1*` |  | ROI, voxel |

### `ngtdm` (5)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `ngtdm_Coarseness` | QCDE | Coarseness | `gray-level index^-1` | yes | ROI, voxel |
| `ngtdm_Contrast` | 65HE | Contrast | `gray-level index^3` | yes | ROI, voxel |
| `ngtdm_Busyness` | NQ30 | Busyness | `1` | yes | ROI, voxel |
| `ngtdm_Complexity` | HDEZ | Complexity | `gray-level index^2` | yes | ROI, voxel |
| `ngtdm_Strength` | 1X9X | Strength | `gray-level index` | yes | ROI, voxel |

### `glrlm` (16)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `glrlm_ShortRunEmphasis` | 22OV | Short runs emphasis | `1*` | yes | ROI, voxel |
| `glrlm_LongRunEmphasis` | W4KF | Long runs emphasis | `1*` | yes | ROI, voxel |
| `glrlm_GrayLevelNonUniformity` | R5YN | Grey level non-uniformity | `1*` | yes | ROI, voxel |
| `glrlm_GrayLevelNonUniformityNormalized` | OVBL | Normalised grey level non-uniformity | `1*` | yes | ROI, voxel |
| `glrlm_RunLengthNonUniformity` | W92Y | Run length non-uniformity | `1*` | yes | ROI, voxel |
| `glrlm_RunLengthNonUniformityNormalized` | IC23 | Normalised run length non-uniformity | `1*` | yes | ROI, voxel |
| `glrlm_RunPercentage` | 9ZK5 | Run percentage | `1*` | yes | ROI, voxel |
| `glrlm_GrayLevelVariance` | 8CE5 | Grey level variance | `1*` | yes | ROI, voxel |
| `glrlm_RunVariance` | SXLW | Run length variance | `1*` | yes | ROI, voxel |
| `glrlm_RunEntropy` | HJ9O | Run entropy | `bit` | yes | ROI, voxel |
| `glrlm_LowGrayLevelRunEmphasis` | V3SW | Low grey level run emphasis | `1*` | yes | ROI, voxel |
| `glrlm_HighGrayLevelRunEmphasis` | G3QZ | High grey level run emphasis | `1*` | yes | ROI, voxel |
| `glrlm_ShortRunLowGrayLevelEmphasis` | HTZT | Short run low grey level emphasis | `1*` | yes | ROI, voxel |
| `glrlm_ShortRunHighGrayLevelEmphasis` | GD3A | Short run high grey level emphasis | `1*` | yes | ROI, voxel |
| `glrlm_LongRunLowGrayLevelEmphasis` | IVPO | Long run low grey level emphasis | `1*` | yes | ROI, voxel |
| `glrlm_LongRunHighGrayLevelEmphasis` | 3KUM | Long run high grey level emphasis | `1*` | yes | ROI, voxel |

### `glszm` (16)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `glszm_SmallAreaEmphasis` | 5QRC | Small zone emphasis | `1*` | yes | ROI, voxel |
| `glszm_LargeAreaEmphasis` | 48P8 | Large zone emphasis | `1*` | yes | ROI, voxel |
| `glszm_GrayLevelNonUniformity` | JNSA | Grey level non-uniformity | `1*` | yes | ROI, voxel |
| `glszm_GrayLevelNonUniformityNormalized` | Y1RO | Normalised grey level non-uniformity | `1*` | yes | ROI, voxel |
| `glszm_SizeZoneNonUniformity` | 4JP3 | Zone size non-uniformity | `1*` | yes | ROI, voxel |
| `glszm_SizeZoneNonUniformityNormalized` | VB3A | Normalised zone size non-uniformity | `1*` | yes | ROI, voxel |
| `glszm_ZonePercentage` | P30P | Zone percentage | `1*` | yes | ROI, voxel |
| `glszm_GrayLevelVariance` | BYLV | Grey level variance | `1*` | yes | ROI, voxel |
| `glszm_ZoneVariance` | 3NSA | Zone size variance | `1*` | yes | ROI, voxel |
| `glszm_ZoneEntropy` | GU8N | Zone size entropy | `bit` | yes | ROI, voxel |
| `glszm_LowGrayLevelZoneEmphasis` | XMSY | Low grey level zone emphasis | `1*` | yes | ROI, voxel |
| `glszm_HighGrayLevelZoneEmphasis` | 5GN9 | High grey level zone emphasis | `1*` | yes | ROI, voxel |
| `glszm_SmallAreaLowGrayLevelEmphasis` | 5RAI | Small zone low grey level emphasis | `1*` | yes | ROI, voxel |
| `glszm_SmallAreaHighGrayLevelEmphasis` | HW1V | Small zone high grey level emphasis | `1*` | yes | ROI, voxel |
| `glszm_LargeAreaLowGrayLevelEmphasis` | YH51 | Large zone low grey level emphasis | `1*` | yes | ROI, voxel |
| `glszm_LargeAreaHighGrayLevelEmphasis` | J17V | Large zone high grey level emphasis | `1*` | yes | ROI, voxel |

### `shape` (32)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `shape_MeshVolume` | RNU0 | Volume (mesh) | `mm^3` |  | 3D ROI |
| `shape_VoxelVolume` | YEKZ | Volume (voxel counting) | `mm^3` |  | 3D ROI |
| `shape_SurfaceArea` | C0JK | Surface area (mesh) | `mm^2` |  | 3D ROI |
| `shape_SurfaceVolumeRatio` | 2PR5 | Surface to volume ratio | `mm^-1` |  | 3D ROI |
| `shape_Sphericity` | QCFX | Sphericity | `1` |  | 3D ROI |
| `shape_Maximum3DDiameter` | L0JK | Maximum 3D diameter | `mm` |  | 3D ROI |
| `shape_Maximum2DDiameterSlice` |  |  |  |  | 3D ROI |
| `shape_Maximum2DDiameterColumn` |  |  |  |  | 3D ROI |
| `shape_Maximum2DDiameterRow` |  |  |  |  | 3D ROI |
| `shape_MajorAxisLength` | TDIC | Major axis length | `mm` |  | 3D ROI |
| `shape_MinorAxisLength` | P9VJ | Minor axis length | `mm` |  | 3D ROI |
| `shape_LeastAxisLength` | 7J51 | Least axis length | `mm` |  | 3D ROI |
| `shape_Elongation` | Q3CK | Elongation | `1` |  | 3D ROI |
| `shape_Flatness` | N17B | Flatness | `1` |  | 3D ROI |
| `shape_Compactness1` | SKGS | Compactness 1 | `1` |  | 3D ROI |
| `shape_Compactness2` | BQWJ | Compactness 2 | `1` |  | 3D ROI |
| `shape_SphericalDisproportion` | KRCK | Spherical disproportion | `1` |  | 3D ROI |
| `shape_Asphericity` | 25C7 | Asphericity | `1` |  | 3D ROI |
| `shape_VolumeDensityAxisAlignedBoundingBox` | PBX1 | Volume density (axis-aligned bounding box) | `1` |  | 3D ROI |
| `shape_AreaDensityAxisAlignedBoundingBox` | R59B | Area density (axis-aligned bounding box) | `1` |  | 3D ROI |
| `shape_VolumeDensityApproximateEnclosingEllipsoid` | 6BDE | Volume density (approximate enclosing ellipsoid) | `1` |  | 3D ROI |
| `shape_AreaDensityApproximateEnclosingEllipsoid` | RDD2 | Area density (approximate enclosing ellipsoid) | `1` |  | 3D ROI |
| `shape_VolumeDensityOrientedMinimumBoundingBox` | ZH1A | Volume density (oriented minimum bounding box) | `1` |  | 3D ROI |
| `shape_AreaDensityOrientedMinimumBoundingBox` | IQYR | Area density (oriented minimum bounding box) | `1` |  | 3D ROI |
| `shape_VolumeDensityMinimumVolumeEnclosingEllipsoid` | SWZ1 | Volume density (minimum volume enclosing ellipsoid) | `1` |  | 3D ROI |
| `shape_AreaDensityMinimumVolumeEnclosingEllipsoid` | BRI8 | Area density (minimum volume enclosing ellipsoid) | `1` |  | 3D ROI |
| `shape_VolumeDensityConvexHull` | R3ER | Volume density (convex hull) | `1` |  | 3D ROI |
| `shape_AreaDensityConvexHull` | 7T7F | Area density (convex hull) | `1` |  | 3D ROI |
| `shape_CentreOfMassShift` | KLMA | Centre of mass shift | `mm` |  | 3D ROI |
| `shape_IntegratedIntensity` | 99N0 | Integrated intensity | `intensity*mm^3` |  | 3D ROI |
| `shape_MoransI` | N365 | Moran’s I index | `1` |  | 3D ROI |
| `shape_GearysC` | NPT7 | Geary’s C measure | `1` |  | 3D ROI |

### `shape2D` (9)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `shape2D_MeshSurface` |  |  |  |  | 2D ROI |
| `shape2D_PixelSurface` |  |  |  |  | 2D ROI |
| `shape2D_Perimeter` |  |  |  |  | 2D ROI |
| `shape2D_PerimeterSurfaceRatio` |  |  |  |  | 2D ROI |
| `shape2D_Sphericity` |  |  |  |  | 2D ROI |
| `shape2D_MaximumDiameter` |  |  |  |  | 2D ROI |
| `shape2D_MajorAxisLength` |  |  |  |  | 2D ROI |
| `shape2D_MinorAxisLength` |  |  |  |  | 2D ROI |
| `shape2D_Elongation` |  |  |  |  | 2D ROI |

### `gldzm` (16)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `gldzm_SmallDistanceEmphasis` | 0GBI | Small distance emphasis | `1*` |  | ROI, voxel |
| `gldzm_LargeDistanceEmphasis` | MB4I | Large distance emphasis | `1*` |  | ROI, voxel |
| `gldzm_LowGrayLevelZoneEmphasis` | S1RA | Low grey level zone emphasis | `1*` |  | ROI, voxel |
| `gldzm_HighGrayLevelZoneEmphasis` | K26C | High grey level zone emphasis | `1*` |  | ROI, voxel |
| `gldzm_SmallDistanceLowGrayLevelEmphasis` | RUVG | Small distance low grey level emphasis | `1*` |  | ROI, voxel |
| `gldzm_SmallDistanceHighGrayLevelEmphasis` | DKNJ | Small distance high grey level emphasis | `1*` |  | ROI, voxel |
| `gldzm_LargeDistanceLowGrayLevelEmphasis` | A7WM | Large distance low grey level emphasis | `1*` |  | ROI, voxel |
| `gldzm_LargeDistanceHighGrayLevelEmphasis` | KLTH | Large distance high grey level emphasis | `1*` |  | ROI, voxel |
| `gldzm_GrayLevelNonUniformity` | VFT7 | Grey level non-uniformity | `1*` |  | ROI, voxel |
| `gldzm_GrayLevelNonUniformityNormalized` | 7HP3 | Normalised grey level non-uniformity | `1*` |  | ROI, voxel |
| `gldzm_ZoneDistanceNonUniformity` | V294 | Zone distance non-uniformity | `1*` |  | ROI, voxel |
| `gldzm_ZoneDistanceNonUniformityNormalized` | IATH | Normalised zone distance non-uniformity | `1*` |  | ROI, voxel |
| `gldzm_ZonePercentage` | VIWW | Zone percentage | `1*` |  | ROI, voxel |
| `gldzm_GrayLevelVariance` | QK93 | Grey level variance | `1*` |  | ROI, voxel |
| `gldzm_ZoneDistanceVariance` | 7WT1 | Zone distance variance | `1*` |  | ROI, voxel |
| `gldzm_ZoneDistanceEntropy` | GBDU | Zone distance entropy | `bit` |  | ROI, voxel |

### `intensityhistogram` (23)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `intensityhistogram_Mean` | X6K6 | Mean discretised intensity | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_Variance` | CH89 | Discretised intensity variance | `gray-level index^2` |  | ROI, voxel |
| `intensityhistogram_Skewness` | 88K1 | Discretised intensity skewness | `1` |  | ROI, voxel |
| `intensityhistogram_Kurtosis` | C3I7 | (Excess) discretised intensity kurtosis | `1` |  | ROI, voxel |
| `intensityhistogram_Median` | WIFQ | Median discretised intensity | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_Minimum` | 1PR8 | Minimum discretised intensity | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_10Percentile` | GPMT | 10th discretised intensity percentile | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_90Percentile` | OZ0C | 90th discretised intensity percentile | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_Maximum` | 3NCY | Maximum discretised intensity | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_Mode` | AMMC | Intensity histogram mode | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_InterquartileRange` | WR0O | Discretised intensity interquartile range | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_Range` | 5Z3W | Discretised intensity range | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_MeanAbsoluteDeviation` | D2ZX | Intensity histogram mean absolute deviation | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_RobustMeanAbsoluteDeviation` | WRZB | Intensity histogram robust mean absolute deviation | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_MedianAbsoluteDeviation` | 4RNL | Intensity histogram median absolute deviation | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_CoefficientOfVariation` | CWYJ | Intensity histogram coefficient of variation | `1` |  | ROI, voxel |
| `intensityhistogram_QuartileCoefficientOfDispersion` | SLWD | Intensity histogram quartile coefficient of dispersion | `1` |  | ROI, voxel |
| `intensityhistogram_Entropy` | TLU2 | Discretised intensity entropy | `bit` |  | ROI, voxel |
| `intensityhistogram_Uniformity` | BJ5W | Discretised intensity uniformity | `1` |  | ROI, voxel |
| `intensityhistogram_MaximumGradient` | 12CE | Maximum histogram gradient | `voxel count` |  | ROI, voxel |
| `intensityhistogram_MaximumGradientGrayLevel` | 8E6O | Maximum histogram gradient intensity | `gray-level index` |  | ROI, voxel |
| `intensityhistogram_MinimumGradient` | VQB3 | Minimum histogram gradient | `voxel count` |  | ROI, voxel |
| `intensityhistogram_MinimumGradientGrayLevel` | RHQZ | Minimum histogram gradient intensity | `gray-level index` |  | ROI, voxel |

### `localintensity` (2)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `localintensity_LocalIntensityPeak` | VJGA | Local intensity peak | `intensity` |  | 3D ROI |
| `localintensity_GlobalIntensityPeak` | 0F91 | Global intensity peak | `intensity` |  | 3D ROI |

### `ivh` (7)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `ivh_VolumeAtIntensityFraction10` | BC2M | Volume at intensity fraction | `1` |  | ROI, voxel |
| `ivh_VolumeAtIntensityFraction90` | BC2M | Volume at intensity fraction | `1` |  | ROI, voxel |
| `ivh_IntensityAtVolumeFraction10` | GBPN | Intensity at volume fraction | `calibrated intensity (unit/fixed width) or bin index (fixed count)` |  | ROI, voxel |
| `ivh_IntensityAtVolumeFraction90` | GBPN | Intensity at volume fraction | `calibrated intensity (unit/fixed width) or bin index (fixed count)` |  | ROI, voxel |
| `ivh_VolumeFractionDifference` | DDTU | Volume fraction difference between intensity fractions | `1` |  | ROI, voxel |
| `ivh_IntensityFractionDifference` | CNV2 | Intensity fraction difference between volume fractions | `calibrated intensity (unit/fixed width) or bin index (fixed count)` |  | ROI, voxel |
| `ivh_AreaUnderIVHCurve` | 9CMM | Area under the IVH curve | `1` |  | ROI, voxel |

### `momentinvariant` (10)

| Feature | IBSI ID | Definition | Units | Default | Modes |
| --- | --- | --- | --- | :---: | --- |
| `momentinvariant_IntensityI1` |  | Trace of the second-order moment tensor of the intensity | `1` |  | 3D ROI |
| `momentinvariant_IntensityF2` |  | Sum of the squared second-order moments of the intensity | `1` |  | 3D ROI |
| `momentinvariant_IntensityI2` |  | Second principal invariant of the second-order moment tensor of the intensity | `1` |  | 3D ROI |
| `momentinvariant_IntensityI3` |  | Determinant of the second-order moment tensor of the intensity | `1` |  | 3D ROI |
| `momentinvariant_IntensityF3` |  | Sum of the squared third-order moments of the intensity | `1` |  | 3D ROI |
| `momentinvariant_ShapeI1` |  | Trace of the second-order moment tensor of the shape | `1` |  | 3D ROI |
| `momentinvariant_ShapeF2` |  | Sum of the squared second-order moments of the shape | `1` |  | 3D ROI |
| `momentinvariant_ShapeI2` |  | Second principal invariant of the second-order moment tensor of the shape | `1` |  | 3D ROI |
| `momentinvariant_ShapeI3` |  | Determinant of the second-order moment tensor of the shape | `1` |  | 3D ROI |
| `momentinvariant_ShapeF3` |  | Sum of the squared third-order moments of the shape | `1` |  | 3D ROI |

The moment invariants follow PySERA. They are dimensionless and do not change when the ROI
is moved, rotated or uniformly scaled. Each set is computed from a density at the voxel
centers. The shape density is 1 on the morphological mask. The intensity density is
`(I - min) / (mean - min)` on the intensity mask, so the intensity invariants also ignore
an intensity offset or positive gain, and they are NaN for a constant ROI. With mass `m000`
and central moments `mu_pqr`, the normalized moments are
`eta_pqr = mu_pqr / m000^((p + q + r) / 3 + 1)`, and the second principal invariant is
`eta200 eta020 + eta200 eta002 + eta020 eta002 - eta110^2 - eta101^2 - eta011^2`. Moments
use physical coordinates, so anisotropic spacing is honored. PySERA uses voxel indices, and
the two agree on an isotropic grid. On an HCC tumor at 1 mm spacing, all ten values match
PySERA 2.1.5 within 1e-14 relative. PySERA reports each connected lesion of a mask
separately, and here the whole mask is one ROI. Like shape, the family is computed once on
the original image.
