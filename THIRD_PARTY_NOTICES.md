# Third-party notices

This repository is licensed under Apache-2.0 (see `LICENSE`). It mirrors mlx-radiomics
(MedMLX, Apache-2.0), whose authors are the authors of this repository. The material below
is adapted from third parties, or is distributed with the Python extension.

## IBSI feature definitions

- Used in: `crates/radiomics/src/config/settings/catalog.rs` (feature identifiers, titles
  and units), generated from mlx-radiomics `docs/ibsi-features.json`.
- Source: [IBSI reference manual](https://github.com/theibsi/ibsi-doc/blob/b952e71e9d40569e8a9ff3c740bc389c04dd6c28/docs/03_Image_features.rst),
  revision b952e71e9d40569e8a9ff3c740bc389c04dd6c28, by the Image Biomarker
  Standardisation Initiative contributors.
- License: [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/).
- Adapted: permanent identifiers, definition titles and units. Implementations are
  independent of the manual text.

## Marching-geometry lookup tables

- Used in: `crates/radiomics/src/features/geometry/mesh_tables.json`, copied from mlx-radiomics v1.1.0.

PyRadiomics-derived marching-geometry lookup tables in
`src/mlxradiomics/features/geometry/mesh_tables.json` (recorded source revision
`6a761c4eb01905cfcb1139b0b158547c55edd4df`, `radiomics/src/cshape.c`).
Source: https://raw.githubusercontent.com/AIM-Harvard/pyradiomics/6a761c4eb01905cfcb1139b0b158547c55edd4df/LICENSE.txt

Copyright 2017 Harvard Medical School

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following
disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products
derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

The 3-D marching table rows 128-255 are derived from scikit-image 0.26.0's
`measure.marching_cubes` classic (`lorensen`) binary-cell geometry, with edge
indices remapped and winding reversed to point outward. Rows 0-127 and the
remaining geometry tables retain the PyRadiomics provenance above.
Source: https://github.com/scikit-image/scikit-image/tree/v0.26.0/skimage/measure
License: https://github.com/scikit-image/scikit-image/blob/v0.26.0/LICENSE.txt

Copyright 2009-2022 the scikit-image team

License: BSD-3-Clause

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.
3. Neither the name of the University nor the names of its contributors
   may be used to endorse or promote products derived from this software
   without specific prior written permission.
.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE HOLDERS OR
CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

## IBSI 1 digital phantom and benchmark values

The Image Biomarker Standardisation Initiative provides the digital phantom
(original design by Alex Zwanenburg) and the IBSI 1 reference table under
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).
`crates/radiomics/tests/data/w5/ibsi_phantom.json` contains the NIfTI image
and mask transposed to NumPy axis order and the morphology rows extracted from
the digital phantom CSV. The MLX outputs were computed for this port.
[IBSI digital phantom](https://github.com/theibsi/data_sets/tree/master/ibsi_1_digital_phantom)
and [IBSI reference data](https://theibsi.github.io/ibsi1/).

## Parity test data

Alex Zwanenburg designed the IBSI 1 digital phantom for the Image Biomarker
Standardisation Initiative. The fixture generator converts its NIfTI image and segmentation to
JSON and includes their physical geometry. The phantom is distributed under
CC BY 4.0. [IBSI digital phantom and attribution](https://github.com/theibsi/data_sets/tree/master/ibsi_1_digital_phantom) · [License](https://creativecommons.org/licenses/by/4.0/)

The digital-phantom table contains factual reference values, tolerances and
tags from the IBSI 1 submission table. The generator adds public feature names and
permanent identifiers from the mlx-radiomics definition catalog credited above.
[IBSI 1 reference values](https://theibsi.github.io/ibsi1/)

The seven synthetic NPZ recordings come from MedMLX's Apache-2.0
mlx-radiomics test suite. They include numerical outputs of PyRadiomics
revision 6a761c4. The additional float64 references were calculated with
PyRadiomics 3.1.1.dev111+g8ed579383 (Harvard Medical School, BSD-3-Clause).
[MedMLX fixture source](https://github.com/MedMLX/mlx-radiomics/tree/7964f3047069aee9dd3da577d91b7ba24d4fa722/tests/fixtures/darwin-arm64) · [PyRadiomics license](https://github.com/AIM-Harvard/pyradiomics/blob/6a761c4eb01905cfcb1139b0b158547c55edd4df/LICENSE.txt)

## IBSI 1 digital phantom and texture benchmarks

The texture fixture `crates/radiomics/tests/data/w4/ibsi.json` contains the masked
digital phantom in NumPy axis order and the 408 texture rows from the IBSI 1
submission table. The generator maps the original benchmark labels to this
crate's feature names. The parity test applies the difference and tolerance
rules from columns H and I of the digital phantom sheet.

The Image Biomarker Standardisation Initiative provides the phantom and
benchmarks under Creative Commons Attribution 4.0 International. Alex
Zwanenburg designed the phantom. [IBSI data sets](https://github.com/theibsi/data_sets)
and [license](https://creativecommons.org/licenses/by/4.0/).

## IBSI 1 digital phantom check

`crates/radiomics/tests/data/w12/` holds the IBSI 1 digital phantom image and mask
(NIfTI) and the digital phantom sheet of the IBSI 1 submission workbook exported as
CSV (`digital_phantom.csv`: reference values, tolerances and tags). The Image Biomarker
Standardisation Initiative provides them under
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); Alex Zwanenburg designed the
phantom. [IBSI data sets](https://github.com/theibsi/data_sets) and
[IBSI reference data](https://theibsi.github.io/ibsi1/). The CT radiomics phantom
(CC BY-NC) is not committed; `w12_ibsi1_ct.rs` reads it from `IBSI1_CT_PHANTOM`.

## PyWavelets filter taps

- Used in: `crates/radiomics/src/imaging/wavelet_tables.json` (read by
  `crates/radiomics/src/config/wavelets.rs`), copied unchanged from mlx-radiomics
  v1.1.0 `src/mlxradiomics/imaging/wavelet_tables.json`.
- Source: [PyWavelets](https://github.com/PyWavelets/pywt) 1.10.0, the `dec_lo` and
  `dec_hi` decomposition filters of the 106 discrete wavelets (all families
  `pywt.wavelist(kind="discrete")` provides).
- Copyright: (c) 2006-2012 Filip Wasilewski <http://en.ig.ma/>; (c) 2012- The PyWavelets
  Developers <https://github.com/PyWavelets/pywt>.
- License: MIT. Full text in [Full license: MIT](#full-license-mit).

The undecimated wavelet in `crates/radiomics/src/imaging/transforms.rs` reproduces the
phase and normalisation of PyWavelets `swtn(norm=False)` (periodic a trous filtering);
the algorithm is written independently from the mlx-radiomics port, not copied.

## ITK recursive Gaussian coefficients

- Used in: `crates/radiomics/src/filters/recursive_gaussian.rs` (Deriche numerator and
  denominator coefficients and their normalisation), ported from mlx-radiomics v1.1.0
  `filters/recursive_gaussian.py`, which follows ITK 5.4.4
  `Modules/Filtering/Smoothing/include/itkRecursiveGaussianImageFilter.hxx`.
- Source: [Insight Toolkit](https://github.com/InsightSoftwareConsortium/ITK) 5.4.4,
  Copyright NumFOCUS.
- License: [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
- Adapted: the coefficient equations (constants `1.3530, 1.8151, -0.3531, 0.0902,
  -1.3563, 5.2318, 0.3446, -2.2355, 0.6681, 2.0787, 1.3932, 1.3732`), evaluated in
  factored form, and the centred causal and anticausal recurrences.

## PyRadiomics image-type definitions

- Used in: `crates/radiomics/src/imaging/transforms.rs` (square, square root, logarithm,
  exponential and gradient definitions and scaling by the maximum absolute intensity;
  wavelet, LoG, LBP2D and LBP3D image-type names), and `filters/lbp2d.rs`, `filters/lbp3d.rs`
  (image-type semantics only; the LBP3D geometry is computed independently).
- Source: [PyRadiomics](https://github.com/AIM-Harvard/pyradiomics) 3.1.x
  `radiomics/imageoperations.py`, Copyright 2017 Harvard Medical School, BSD-3-Clause; the
  license text is reproduced in the PyRadiomics notice of mlx-radiomics and in
  `https://raw.githubusercontent.com/AIM-Harvard/pyradiomics/master/LICENSE.txt`.
- The LBP2D coding methods (`default`, `ror`, `uniform`, `nri_uniform`, `var`) follow the
  scikit-image `feature.local_binary_pattern` definitions (BSD-3-Clause, Copyright
  2009-2022 the scikit-image team).

## IBSI 2 reference data used by the filter tests (not committed)

`crates/radiomics/src/imaging/transforms/ibsi2_maps.rs` reads the IBSI 2 reference response maps
([CC0](https://creativecommons.org/publicdomain/zero/1.0/)) and the IBSI 2 phase-1 digital
phantoms ([CC BY 4.0](https://creativecommons.org/licenses/by/4.0/), IBSI contributors)
from paths in environment variables. Neither is stored in this repository.

## Imaging: file formats, resampling semantics and float helpers

- Used in: `crates/radiomics/src/imaging/{io,spatial,preprocessing}.rs`,
  `crates/radiomics/src/numerics/spline.rs`.
- ITK (Apache-2.0, <https://itk.org>): resampling, interpolation, B-spline decomposition and
  NIfTI/NRRD orientation semantics were implemented from ITK's published behaviour and
  documentation and verified against SimpleITK 2.5.6 (`tests/w1_*.rs`); no ITK source text is
  copied. The B-spline prefilter pole, mirror boundary and causal-horizon rule follow the
  `BSplineDecompositionImageFilter` algorithm (Unser et al.).
- The physical-to-index inverse in `imaging/spatial/inverse.rs` follows G. W. Stewart's
  LINPACK singular-value decomposition (University of Maryland and Argonne National
  Laboratory, 1978). The Rust routine specializes bidiagonalization and QR iteration to
  two and three dimensions. ITK forms the inverse from the singular vectors using VNL.
  [LINPACK DSVDC](https://www.netlib.org/linpack/dsvdc.f) ·
  [ITK 5.4.4 VNL SVD](https://github.com/InsightSoftwareConsortium/ITK/blob/v5.4.4/Modules/ThirdParty/VNL/src/vxl/core/vnl/algo/vnl_svd.hxx)
- PyRadiomics (BSD-3-Clause, <https://github.com/AIM-Harvard/pyradiomics>): the grid and padding
  equations of `spatial::resample` follow `radiomics.imageoperations.resampleImage`, as they do in
  mlx-radiomics.
- NIfTI-1 C library (`nifti1_io.c`, public domain): the quaternion-to-matrix and
  matrix-to-quaternion conversions in `io.rs` follow `nifti_quatern_to_mat44` and
  `nifti_mat44_to_quatern` (header layout from the NIfTI-1 specification).
- NRRD (<https://teem.sourceforge.net/nrrd/format.html>): header fields are read from the file format
  specification; no Teem source is used.
- NumPy (BSD-3-Clause, Copyright (c) 2005-2025 NumPy Developers): `preprocessing::pairwise_sum`
  reproduces NumPy's pairwise summation order so that means and standard deviations agree
  with mlx-radiomics to the last bits.
- musl libc (MIT, Copyright (c) 2005-2020 Rich Felker, et al.): `preprocessing::ldexp` is a port of
  `scalbn`, which rounds once in the subnormal range.
- IBSI 1 digital phantom (CC BY 4.0, Image Biomarker Standardisation Initiative): committed as a
  test fixture at `crates/radiomics/tests/data/w1/io/ibsi_digital_phantom_*.nii.gz` with a notice in
  `NOTICE.md` next to it. The IBSI CT radiomics phantom (CC BY-NC) is never committed; the opt-in
  `tests/w1_ct_phantom.rs` reads it from an environment path.

## IBSI 2 phase 2: consensus values of filtered CT images

- Used in: `crates/radiomics/tests/w13_ibsi2_features.rs`.
- IBSI 2 reference data (CC0, Image Biomarker Standardisation Initiative,
  <https://github.com/theibsi/ibsi_2_reference_data>): `reference_feature_values/reference_values.csv`
  is committed unchanged as `crates/radiomics/tests/data/w13/reference_values.csv` (16 KB).
- The IBSI CT radiomics phantom (CC BY-NC) is never committed; the opt-in test reads it from the
  path in `IBSI1_CT_PHANTOM` and is skipped with a printed notice when the variable is unset.
- Filter configurations follow the IBSI 2 manual (arXiv:2006.05470v9, section 6).

## Rust crates linked into the Python extension

The Python extension `rustrad._native` statically links the crates below, the normal
dependencies of `radiomics-py` for the built target (from `cargo metadata`, locked in
`Cargo.lock`), plus the in-repository `radiomics` crate (Apache-2.0). Where a crate offers
several licenses, any one of them may be chosen; this distribution uses the MIT or
Apache-2.0 option (so the 0BSD, Unlicense, BSL-1.0 and, for `miniz_oxide`, Zlib alternatives
are not relied on). Every license listed is permissive. Texts: [LICENSE](LICENSE)
(Apache-2.0) and the full-license sections below. The Python distribution also
depends on NumPy, which is installed separately and keeps its own license.

| Crate | Version | License | Copyright |
| --- | --- | --- | --- |
| adler2 | 2.0.1 | 0BSD OR MIT OR Apache-2.0 | Copyright (C) Jonas Schievink <jonasschievink@gmail.com> |
| aho-corasick | 1.1.5 | Unlicense OR MIT | Copyright (c) 2015 Andrew Gallant |
| arraydeque | 0.5.1 | MIT/Apache-2.0 | Copyright (c) 2018 Andy Lok <andylokandy@hotmail.com> |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 | Copyright (c) 2014 Alex Crichton |
| crc32fast | 1.5.5 | MIT OR Apache-2.0 | Copyright (c) 2018 Sam Rijs, Alex Crichton and contributors |
| crossbeam-deque | 0.8.8 | MIT OR Apache-2.0 | Copyright (c) 2019 The Crossbeam Project Developers |
| crossbeam-epoch | 0.9.21 | MIT OR Apache-2.0 | Copyright (c) 2019 The Crossbeam Project Developers |
| crossbeam-utils | 0.8.23 | MIT OR Apache-2.0 | Copyright (c) 2019 The Crossbeam Project Developers |
| either | 1.19.0 | MIT OR Apache-2.0 | Copyright (c) 2015 |
| equivalent | 1.0.3 | Apache-2.0 OR MIT | Copyright (c) 2016--2023 |
| flate2 | 1.1.11 | MIT OR Apache-2.0 | Copyright (c) 2014-2026 Alex Crichton |
| foldhash | 0.2.0 | Zlib | Copyright (c) 2024 Orson Peters |
| hashbrown | 0.17.1 | MIT OR Apache-2.0 | Copyright (c) 2016 Amanieu d'Antras |
| hashlink | 0.12.2 | MIT OR Apache-2.0 | not stated in the crate's license file |
| heck | 0.5.0 | MIT OR Apache-2.0 | Copyright (c) 2015 The Rust Project Developers |
| indexmap | 2.14.2 | Apache-2.0 OR MIT | Copyright (c) 2016--2017 |
| itoa | 1.0.19 | MIT OR Apache-2.0 | not stated in the crate's license file |
| libc | 0.2.191 | MIT OR Apache-2.0 | Copyright (c) The Rust Project Developers |
| libyaml-rs | 0.3.0 | MIT | not stated in the crate's license file |
| matrixmultiply | 0.3.11 | MIT/Apache-2.0 | Copyright (c) 2016 - 2023 Ulrik Sverdrup "bluss"; Copyright (c) 2021 DutchGhost [constparse.rs] |
| memchr | 2.8.3 | Unlicense OR MIT | Copyright (c) 2015 Andrew Gallant |
| miniz_oxide | 0.9.1 | MIT OR Zlib OR Apache-2.0 | Copyright 2013-2014 RAD Game Tools and Valve Software; Copyright 2010-2014 Rich Geldreich and Tenacious Software LLC; Copyright (c) 2017 Frommi |
| ndarray | 0.16.1 | MIT OR Apache-2.0 | Copyright (c) 2015 - 2021 Ulrik Sverdrup "bluss", |
| num-bigint | 0.4.8 | MIT OR Apache-2.0 | Copyright (c) 2014 The Rust Project Developers |
| num-complex | 0.4.6 | MIT OR Apache-2.0 | Copyright (c) 2014 The Rust Project Developers |
| num-integer | 0.1.47 | MIT OR Apache-2.0 | Copyright (c) 2014 The Rust Project Developers |
| num-rational | 0.4.2 | MIT OR Apache-2.0 | Copyright (c) 2014 The Rust Project Developers |
| num-traits | 0.2.19 | MIT OR Apache-2.0 | Copyright (c) 2014 The Rust Project Developers |
| numpy | 0.30.0 | BSD-2-Clause | Copyright (c) 2017, Toshiki Teramura |
| once_cell | 1.21.4 | MIT OR Apache-2.0 | not stated in the crate's license file |
| ordered-float | 5.4.0 | MIT | Copyright (c) 2015 Jonathan Reem |
| primal-check | 0.3.4 | MIT OR Apache-2.0 | Copyright (c) 2014 Huon Wilson |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 | not stated in the crate's license file |
| pyo3 | 0.30.1 | MIT OR Apache-2.0 | Copyright (c) 2017-present PyO3 Project and Contributors.  https://github.com/PyO3 |
| pyo3-build-config | 0.30.1 | MIT OR Apache-2.0 | Copyright (c) 2023-present PyO3 Project and Contributors.  https://github.com/PyO3 |
| pyo3-ffi | 0.30.1 | MIT OR Apache-2.0 | Copyright (c) 2023-present PyO3 Project and Contributors.  https://github.com/PyO3 |
| pyo3-macros | 0.30.1 | MIT OR Apache-2.0 | Copyright (c) 2023-present PyO3 Project and Contributors.  https://github.com/PyO3 |
| pyo3-macros-backend | 0.30.1 | MIT OR Apache-2.0 | Copyright (c) 2023-present PyO3 Project and Contributors.  https://github.com/PyO3 |
| pythonize | 0.30.0 | MIT | Copyright (c) 2022-present David Hewitt and Contributors |
| quote | 1.0.47 | MIT OR Apache-2.0 | not stated in the crate's license file |
| rawpointer | 0.2.1 | MIT/Apache-2.0 | Copyright (c) 2015 |
| rayon | 1.12.0 | MIT OR Apache-2.0 | Copyright (c) 2010 The Rust Project Developers |
| rayon-core | 1.13.0 | MIT OR Apache-2.0 | Copyright (c) 2010 The Rust Project Developers |
| regex | 1.13.2 | MIT OR Apache-2.0 | Copyright (c) 2014 The Rust Project Developers |
| regex-automata | 0.4.19 | MIT OR Apache-2.0 | Copyright (c) 2014 The Rust Project Developers |
| regex-syntax | 0.8.12 | MIT OR Apache-2.0 | Copyright (c) 2014 The Rust Project Developers |
| rustc-hash | 2.1.3 | Apache-2.0 OR MIT | not stated in the crate's license file |
| rustfft | 6.4.1 | MIT OR Apache-2.0 | Copyright (c) 2015 The RustFFT Developers |
| ryu | 1.0.23 | Apache-2.0 OR BSL-1.0 | not stated in the crate's license file |
| saphyr | 0.4.0 | MIT OR Apache-2.0 | not stated in the crate's license file |
| saphyr-parser | 0.4.0 | MIT OR Apache-2.0 | not stated in the crate's license file |
| serde | 1.0.229 | MIT OR Apache-2.0 | not stated in the crate's license file |
| serde_core | 1.0.229 | MIT OR Apache-2.0 | not stated in the crate's license file |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 | not stated in the crate's license file |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | not stated in the crate's license file |
| simd-adler32 | 0.3.10 | MIT | Copyright (c) [2021] [Marvin Countryman] |
| strength_reduce | 0.2.4 | MIT OR Apache-2.0 | Copyright (c) 2015 The RustFFT Developers |
| syn | 3.0.10 | MIT OR Apache-2.0 | not stated in the crate's license file |
| target-lexicon | 0.13.6 | Apache-2.0 WITH LLVM-exception | copyright notice that is included in or attached to the work; (c) You must retain, in the Source form of any Derivative Works |
| thiserror | 2.0.21 | MIT OR Apache-2.0 | not stated in the crate's license file |
| thiserror-impl | 2.0.21 | MIT OR Apache-2.0 | not stated in the crate's license file |
| transpose | 0.2.3 | MIT OR Apache-2.0 | Copyright (c) 2022 The transpose Developers |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | COPYRIGHT AND PERMISSION NOTICE; Copyright © 1991-2023 Unicode, Inc |
| yaml_serde | 0.10.7 | MIT OR Apache-2.0 | not stated in the crate's license file |
| zlib-rs | 0.6.8 | Zlib | (C) 2024 Trifecta Tech Foundation |
| zmij | 1.0.23 | MIT | not stated in the crate's license file |

`target-lexicon` is `Apache-2.0 WITH LLVM-exception`; the exception text is in
[Full license: LLVM exception to Apache-2.0](#full-license-llvm-exception-to-apache-20).
Dependencies of the Python package, data and test fixtures keep their own licenses. This
repository does not include the IBSI CT radiomics phantom or other CC BY-NC data.

## Full license: MIT

Applies to the MIT-licensed items above. The copyright holders are those named in the
sections and table above. The Apache-2.0 text is in [LICENSE](LICENSE).

```
Copyright (c) the copyright holders named in this file

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Full license: BSD-2-Clause

Applies to the `numpy` crate.

```
Copyright (c) 2017, Toshiki Teramura
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

## Full license: Zlib

Applies to the `foldhash` and `zlib-rs` crates (copyright lines in the table above).

```
This software is provided 'as-is', without any express or implied
warranty. In no event will the authors be held liable for any damages
arising from the use of this software.

Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it
freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not
   claim that you wrote the original software. If you use this software
   in a product, an acknowledgment in the product documentation would be
   appreciated but is not required.

2. Altered source versions must be plainly marked as such, and must not be
   misrepresented as being the original software.

3. This notice may not be removed or altered from any source distribution.
```

## Full license: Unicode-3.0

Applies to the Unicode data in the `unicode-ident` crate.

```
UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2023 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS.

IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE
BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES,
OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.
```

## Full license: LLVM exception to Apache-2.0

Applies to the `target-lexicon` crate, together with the Apache-2.0 text in
[LICENSE](LICENSE).

```
--- LLVM Exceptions to the Apache 2.0 License ----

As an exception, if, as a result of your compiling your source code, portions
of this Software are embedded into an Object form of such source code, you
may redistribute such embedded portions in such Object form without complying
with the conditions of Sections 4(a), 4(b) and 4(d) of the License.

In addition, if you combine or link compiled forms of this Software with
software that is licensed under the GPLv2 ("Combined Software") and if a
court of competent jurisdiction determines that the patent provision (Section
3), the indemnity provision (Section 9) or other Section of the License
conflicts with the conditions of the GPLv2, you may retroactively and
prospectively choose to deem waived or otherwise exclude such Section(s) of
the License, but only in their entirety and only with respect to the Combined
Software.
```

## Full license: CC-BY-4.0

Applies to the IBSI feature definitions, digital phantom and benchmark tables credited above.

```
Attribution 4.0 International

=======================================================================

Creative Commons Corporation ("Creative Commons") is not a law firm and
does not provide legal services or legal advice. Distribution of
Creative Commons public licenses does not create a lawyer-client or
other relationship. Creative Commons makes its licenses and related
information available on an "as-is" basis. Creative Commons gives no
warranties regarding its licenses, any material licensed under their
terms and conditions, or any related information. Creative Commons
disclaims all liability for damages resulting from their use to the
fullest extent possible.

Using Creative Commons Public Licenses

Creative Commons public licenses provide a standard set of terms and
conditions that creators and other rights holders may use to share
original works of authorship and other material subject to copyright
and certain other rights specified in the public license below. The
following considerations are for informational purposes only, are not
exhaustive, and do not form part of our licenses.

     Considerations for licensors: Our public licenses are
     intended for use by those authorized to give the public
     permission to use material in ways otherwise restricted by
     copyright and certain other rights. Our licenses are
     irrevocable. Licensors should read and understand the terms
     and conditions of the license they choose before applying it.
     Licensors should also secure all rights necessary before
     applying our licenses so that the public can reuse the
     material as expected. Licensors should clearly mark any
     material not subject to the license. This includes other CC-
     licensed material, or material used under an exception or
     limitation to copyright. More considerations for licensors:
    wiki.creativecommons.org/Considerations_for_licensors

     Considerations for the public: By using one of our public
     licenses, a licensor grants the public permission to use the
     licensed material under specified terms and conditions. If
     the licensor's permission is not necessary for any reason--for
     example, because of any applicable exception or limitation to
     copyright--then that use is not regulated by the license. Our
     licenses grant only permissions under copyright and certain
     other rights that a licensor has authority to grant. Use of
     the licensed material may still be restricted for other
     reasons, including because others have copyright or other
     rights in the material. A licensor may make special requests,
     such as asking that all changes be marked or described.
     Although not required by our licenses, you are encouraged to
     respect those requests where reasonable. More considerations
     for the public:
    wiki.creativecommons.org/Considerations_for_licensees

=======================================================================

Creative Commons Attribution 4.0 International Public License

By exercising the Licensed Rights (defined below), You accept and agree
to be bound by the terms and conditions of this Creative Commons
Attribution 4.0 International Public License ("Public License"). To the
extent this Public License may be interpreted as a contract, You are
granted the Licensed Rights in consideration of Your acceptance of
these terms and conditions, and the Licensor grants You such rights in
consideration of benefits the Licensor receives from making the
Licensed Material available under these terms and conditions.


Section 1 -- Definitions.

  a. Adapted Material means material subject to Copyright and Similar
     Rights that is derived from or based upon the Licensed Material
     and in which the Licensed Material is translated, altered,
     arranged, transformed, or otherwise modified in a manner requiring
     permission under the Copyright and Similar Rights held by the
     Licensor. For purposes of this Public License, where the Licensed
     Material is a musical work, performance, or sound recording,
     Adapted Material is always produced where the Licensed Material is
     synched in timed relation with a moving image.

  b. Adapter's License means the license You apply to Your Copyright
     and Similar Rights in Your contributions to Adapted Material in
     accordance with the terms and conditions of this Public License.

  c. Copyright and Similar Rights means copyright and/or similar rights
     closely related to copyright including, without limitation,
     performance, broadcast, sound recording, and Sui Generis Database
     Rights, without regard to how the rights are labeled or
     categorized. For purposes of this Public License, the rights
     specified in Section 2(b)(1)-(2) are not Copyright and Similar
     Rights.

  d. Effective Technological Measures means those measures that, in the
     absence of proper authority, may not be circumvented under laws
     fulfilling obligations under Article 11 of the WIPO Copyright
     Treaty adopted on December 20, 1996, and/or similar international
     agreements.

  e. Exceptions and Limitations means fair use, fair dealing, and/or
     any other exception or limitation to Copyright and Similar Rights
     that applies to Your use of the Licensed Material.

  f. Licensed Material means the artistic or literary work, database,
     or other material to which the Licensor applied this Public
     License.

  g. Licensed Rights means the rights granted to You subject to the
     terms and conditions of this Public License, which are limited to
     all Copyright and Similar Rights that apply to Your use of the
     Licensed Material and that the Licensor has authority to license.

  h. Licensor means the individual(s) or entity(ies) granting rights
     under this Public License.

  i. Share means to provide material to the public by any means or
     process that requires permission under the Licensed Rights, such
     as reproduction, public display, public performance, distribution,
     dissemination, communication, or importation, and to make material
     available to the public including in ways that members of the
     public may access the material from a place and at a time
     individually chosen by them.

  j. Sui Generis Database Rights means rights other than copyright
     resulting from Directive 96/9/EC of the European Parliament and of
     the Council of 11 March 1996 on the legal protection of databases,
     as amended and/or succeeded, as well as other essentially
     equivalent rights anywhere in the world.

  k. You means the individual or entity exercising the Licensed Rights
     under this Public License. Your has a corresponding meaning.


Section 2 -- Scope.

  a. License grant.

       1. Subject to the terms and conditions of this Public License,
          the Licensor hereby grants You a worldwide, royalty-free,
          non-sublicensable, non-exclusive, irrevocable license to
          exercise the Licensed Rights in the Licensed Material to:

            a. reproduce and Share the Licensed Material, in whole or
               in part; and

            b. produce, reproduce, and Share Adapted Material.

       2. Exceptions and Limitations. For the avoidance of doubt, where
          Exceptions and Limitations apply to Your use, this Public
          License does not apply, and You do not need to comply with
          its terms and conditions.

       3. Term. The term of this Public License is specified in Section
          6(a).

       4. Media and formats; technical modifications allowed. The
          Licensor authorizes You to exercise the Licensed Rights in
          all media and formats whether now known or hereafter created,
          and to make technical modifications necessary to do so. The
          Licensor waives and/or agrees not to assert any right or
          authority to forbid You from making technical modifications
          necessary to exercise the Licensed Rights, including
          technical modifications necessary to circumvent Effective
          Technological Measures. For purposes of this Public License,
          simply making modifications authorized by this Section 2(a)
          (4) never produces Adapted Material.

       5. Downstream recipients.

            a. Offer from the Licensor -- Licensed Material. Every
               recipient of the Licensed Material automatically
               receives an offer from the Licensor to exercise the
               Licensed Rights under the terms and conditions of this
               Public License.

            b. No downstream restrictions. You may not offer or impose
               any additional or different terms or conditions on, or
               apply any Effective Technological Measures to, the
               Licensed Material if doing so restricts exercise of the
               Licensed Rights by any recipient of the Licensed
               Material.

       6. No endorsement. Nothing in this Public License constitutes or
          may be construed as permission to assert or imply that You
          are, or that Your use of the Licensed Material is, connected
          with, or sponsored, endorsed, or granted official status by,
          the Licensor or others designated to receive attribution as
          provided in Section 3(a)(1)(A)(i).

  b. Other rights.

       1. Moral rights, such as the right of integrity, are not
          licensed under this Public License, nor are publicity,
          privacy, and/or other similar personality rights; however, to
          the extent possible, the Licensor waives and/or agrees not to
          assert any such rights held by the Licensor to the limited
          extent necessary to allow You to exercise the Licensed
          Rights, but not otherwise.

       2. Patent and trademark rights are not licensed under this
          Public License.

       3. To the extent possible, the Licensor waives any right to
          collect royalties from You for the exercise of the Licensed
          Rights, whether directly or through a collecting society
          under any voluntary or waivable statutory or compulsory
          licensing scheme. In all other cases the Licensor expressly
          reserves any right to collect such royalties.


Section 3 -- License Conditions.

Your exercise of the Licensed Rights is expressly made subject to the
following conditions.

  a. Attribution.

       1. If You Share the Licensed Material (including in modified
          form), You must:

            a. retain the following if it is supplied by the Licensor
               with the Licensed Material:

                 i. identification of the creator(s) of the Licensed
                    Material and any others designated to receive
                    attribution, in any reasonable manner requested by
                    the Licensor (including by pseudonym if
                    designated);

                ii. a copyright notice;

               iii. a notice that refers to this Public License;

                iv. a notice that refers to the disclaimer of
                    warranties;

                 v. a URI or hyperlink to the Licensed Material to the
                    extent reasonably practicable;

            b. indicate if You modified the Licensed Material and
               retain an indication of any previous modifications; and

            c. indicate the Licensed Material is licensed under this
               Public License, and include the text of, or the URI or
               hyperlink to, this Public License.

       2. You may satisfy the conditions in Section 3(a)(1) in any
          reasonable manner based on the medium, means, and context in
          which You Share the Licensed Material. For example, it may be
          reasonable to satisfy the conditions by providing a URI or
          hyperlink to a resource that includes the required
          information.

       3. If requested by the Licensor, You must remove any of the
          information required by Section 3(a)(1)(A) to the extent
          reasonably practicable.

       4. If You Share Adapted Material You produce, the Adapter's
          License You apply must not prevent recipients of the Adapted
          Material from complying with this Public License.


Section 4 -- Sui Generis Database Rights.

Where the Licensed Rights include Sui Generis Database Rights that
apply to Your use of the Licensed Material:

  a. for the avoidance of doubt, Section 2(a)(1) grants You the right
     to extract, reuse, reproduce, and Share all or a substantial
     portion of the contents of the database;

  b. if You include all or a substantial portion of the database
     contents in a database in which You have Sui Generis Database
     Rights, then the database in which You have Sui Generis Database
     Rights (but not its individual contents) is Adapted Material; and

  c. You must comply with the conditions in Section 3(a) if You Share
     all or a substantial portion of the contents of the database.

For the avoidance of doubt, this Section 4 supplements and does not
replace Your obligations under this Public License where the Licensed
Rights include other Copyright and Similar Rights.


Section 5 -- Disclaimer of Warranties and Limitation of Liability.

  a. UNLESS OTHERWISE SEPARATELY UNDERTAKEN BY THE LICENSOR, TO THE
     EXTENT POSSIBLE, THE LICENSOR OFFERS THE LICENSED MATERIAL AS-IS
     AND AS-AVAILABLE, AND MAKES NO REPRESENTATIONS OR WARRANTIES OF
     ANY KIND CONCERNING THE LICENSED MATERIAL, WHETHER EXPRESS,
     IMPLIED, STATUTORY, OR OTHER. THIS INCLUDES, WITHOUT LIMITATION,
     WARRANTIES OF TITLE, MERCHANTABILITY, FITNESS FOR A PARTICULAR
     PURPOSE, NON-INFRINGEMENT, ABSENCE OF LATENT OR OTHER DEFECTS,
     ACCURACY, OR THE PRESENCE OR ABSENCE OF ERRORS, WHETHER OR NOT
     KNOWN OR DISCOVERABLE. WHERE DISCLAIMERS OF WARRANTIES ARE NOT
     ALLOWED IN FULL OR IN PART, THIS DISCLAIMER MAY NOT APPLY TO YOU.

  b. TO THE EXTENT POSSIBLE, IN NO EVENT WILL THE LICENSOR BE LIABLE
     TO YOU ON ANY LEGAL THEORY (INCLUDING, WITHOUT LIMITATION,
     NEGLIGENCE) OR OTHERWISE FOR ANY DIRECT, SPECIAL, INDIRECT,
     INCIDENTAL, CONSEQUENTIAL, PUNITIVE, EXEMPLARY, OR OTHER LOSSES,
     COSTS, EXPENSES, OR DAMAGES ARISING OUT OF THIS PUBLIC LICENSE OR
     USE OF THE LICENSED MATERIAL, EVEN IF THE LICENSOR HAS BEEN
     ADVISED OF THE POSSIBILITY OF SUCH LOSSES, COSTS, EXPENSES, OR
     DAMAGES. WHERE A LIMITATION OF LIABILITY IS NOT ALLOWED IN FULL OR
     IN PART, THIS LIMITATION MAY NOT APPLY TO YOU.

  c. The disclaimer of warranties and limitation of liability provided
     above shall be interpreted in a manner that, to the extent
     possible, most closely approximates an absolute disclaimer and
     waiver of all liability.


Section 6 -- Term and Termination.

  a. This Public License applies for the term of the Copyright and
     Similar Rights licensed here. However, if You fail to comply with
     this Public License, then Your rights under this Public License
     terminate automatically.

  b. Where Your right to use the Licensed Material has terminated under
     Section 6(a), it reinstates:

       1. automatically as of the date the violation is cured, provided
          it is cured within 30 days of Your discovery of the
          violation; or

       2. upon express reinstatement by the Licensor.

     For the avoidance of doubt, this Section 6(b) does not affect any
     right the Licensor may have to seek remedies for Your violations
     of this Public License.

  c. For the avoidance of doubt, the Licensor may also offer the
     Licensed Material under separate terms or conditions or stop
     distributing the Licensed Material at any time; however, doing so
     will not terminate this Public License.

  d. Sections 1, 5, 6, 7, and 8 survive termination of this Public
     License.


Section 7 -- Other Terms and Conditions.

  a. The Licensor shall not be bound by any additional or different
     terms or conditions communicated by You unless expressly agreed.

  b. Any arrangements, understandings, or agreements regarding the
     Licensed Material not stated herein are separate from and
     independent of the terms and conditions of this Public License.


Section 8 -- Interpretation.

  a. For the avoidance of doubt, this Public License does not, and
     shall not be interpreted to, reduce, limit, restrict, or impose
     conditions on any use of the Licensed Material that could lawfully
     be made without permission under this Public License.

  b. To the extent possible, if any provision of this Public License is
     deemed unenforceable, it shall be automatically reformed to the
     minimum extent necessary to make it enforceable. If the provision
     cannot be reformed, it shall be severed from this Public License
     without affecting the enforceability of the remaining terms and
     conditions.

  c. No term or condition of this Public License will be waived and no
     failure to comply consented to unless expressly agreed to by the
     Licensor.

  d. Nothing in this Public License constitutes or may be interpreted
     as a limitation upon, or waiver of, any privileges and immunities
     that apply to the Licensor or You, including from the legal
     processes of any jurisdiction or authority.


=======================================================================

Creative Commons is not a party to its public
licenses. Notwithstanding, Creative Commons may elect to apply one of
its public licenses to material it publishes and in those instances
will be considered the “Licensor.” The text of the Creative Commons
public licenses is dedicated to the public domain under the CC0 Public
Domain Dedication. Except for the limited purpose of indicating that
material is shared under a Creative Commons public license or as
otherwise permitted by the Creative Commons policies published at
creativecommons.org/policies, Creative Commons does not authorize the
use of the trademark "Creative Commons" or any other trademark or logo
of Creative Commons without its prior written consent including,
without limitation, in connection with any unauthorized modifications
to any of its public licenses or any other arrangements,
understandings, or agreements concerning use of licensed material. For
the avoidance of doubt, this paragraph does not form part of the
public licenses.

Creative Commons may be contacted at creativecommons.org.
```
