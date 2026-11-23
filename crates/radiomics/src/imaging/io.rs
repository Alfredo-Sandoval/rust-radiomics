//! NIfTI/NRRD volume I/O with explicit value and geometry contracts.
//!
//! Mirrors `mlxradiomics/imaging/io.py` (385 lines at mlx-radiomics b58783c).
//!
//! Execution: all of this module is CPU I/O and host orchestration (file reads, gzip
//! inflate, byte-order conversion, header parsing) on the calling thread; it is
//! single-threaded and has no numerical stage beyond the NIfTI `scl_slope`/`scl_inter`
//! rescale (float64 arithmetic, one pass). No GPU or MLX backend is involved.
//!
//! mlx-radiomics reads with SimpleITK and re-checks NIfTI values with NiBabel. This port
//! parses the headers itself and follows ITK 5.4 semantics (checked against SimpleITK
//! 2.5.6 in `tests/w1_io.rs`): NIfTI-1 `sform` is preferred when `sform_code == 1`,
//! otherwise `qform` when `qform_code > 0`, otherwise `sform`, otherwise the identity;
//! spacing is `pixdim` (converted to mm); columns of the RAS matrix are normalised and
//! flipped to LPS; `scl_slope`/`scl_inter` produce float32 (float64 stays float64); trailing
//! size-1 dimensions above 3 are dropped. Stored non-finite voxels are never zeroed, so
//! the finiteness re-check applies to the stored (scaled) values directly.
//!
//! Supported: NIfTI-1 (`.nii`, `.nii.gz`, either byte order) and NRRD with attached data
//! or detached single, listed or numbered payloads, encoded as raw, gzip, text or hex.
//! NIfTI-2 and the NRRD bzip2 encoding return [`Error::NotImplemented`].
//!
//! Status: done (read_image, read_mask, read_image_dimension, has_detached_data,
//! write_feature_map for NIfTI-1 and attached raw NRRD).

mod nrrd_axes;
mod nrrd_files;

use crate::error::{Error, Result};
use crate::imaging::io::nrrd_axes::Axes;
use crate::imaging::io::nrrd_files::DataFiles;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use ndarray::{ArrayD, IxDyn};
use std::borrow::Cow;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

/// Accepted file suffixes (`_VOLUME_SUFFIXES`).
pub const VOLUME_SUFFIXES: [&str; 4] = [".nii", ".nii.gz", ".nrrd", ".nhdr"];

/// Orthonormality tolerance of a `Volume` direction matrix (`atol` of `np.allclose`).
pub const DIRECTION_ORTHONORMAL_TOLERANCE: f64 = 1e-5;

/// Value family of a [`HostArray`] (`ndarray.dtype.kind` of mlx-radiomics).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrayKind {
    /// NumPy kind `b`.
    Bool,
    /// NumPy kind `i`.
    Signed,
    /// NumPy kind `u`.
    Unsigned,
    /// NumPy kind `f`.
    Float,
}

/// A host array of any supported pixel type, in NumPy axis order (`HostArray`).
#[derive(Clone, Debug, PartialEq)]
pub enum HostArray {
    /// `bool`
    Bool(ArrayD<bool>),
    /// `uint8`
    U8(ArrayD<u8>),
    /// `int8`
    I8(ArrayD<i8>),
    /// `uint16`
    U16(ArrayD<u16>),
    /// `int16`
    I16(ArrayD<i16>),
    /// `uint32`
    U32(ArrayD<u32>),
    /// `int32`
    I32(ArrayD<i32>),
    /// `uint64`
    U64(ArrayD<u64>),
    /// `int64`
    I64(ArrayD<i64>),
    /// `float32`
    F32(ArrayD<f32>),
    /// `float64`
    F64(ArrayD<f64>),
}

macro_rules! host_dispatch {
    ($value:expr, $array:ident => $body:expr) => {
        match $value {
            HostArray::Bool($array) => $body,
            HostArray::U8($array) => $body,
            HostArray::I8($array) => $body,
            HostArray::U16($array) => $body,
            HostArray::I16($array) => $body,
            HostArray::U32($array) => $body,
            HostArray::I32($array) => $body,
            HostArray::U64($array) => $body,
            HostArray::I64($array) => $body,
            HostArray::F32($array) => $body,
            HostArray::F64($array) => $body,
        }
    };
}

macro_rules! host_from {
    ($($variant:ident($ty:ty)),+ $(,)?) => {$(
        impl From<ArrayD<$ty>> for HostArray {
            fn from(array: ArrayD<$ty>) -> Self {
                Self::$variant(array)
            }
        }
    )+};
}
host_from!(
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
);

impl HostArray {
    /// Array shape.
    pub fn shape(&self) -> &[usize] {
        host_dispatch!(self, a => a.shape())
    }

    /// Number of axes.
    pub fn ndim(&self) -> usize {
        self.shape().len()
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        host_dispatch!(self, a => a.len())
    }

    /// True for an array without elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// NumPy `dtype.kind` family.
    pub fn kind(&self) -> ArrayKind {
        match self {
            Self::Bool(_) => ArrayKind::Bool,
            Self::I8(_) | Self::I16(_) | Self::I32(_) | Self::I64(_) => ArrayKind::Signed,
            Self::U8(_) | Self::U16(_) | Self::U32(_) | Self::U64(_) => ArrayKind::Unsigned,
            Self::F32(_) | Self::F64(_) => ArrayKind::Float,
        }
    }

    /// NumPy dtype name, e.g. `int16`.
    pub fn dtype_name(&self) -> &'static str {
        match self {
            Self::Bool(_) => "bool",
            Self::U8(_) => "uint8",
            Self::I8(_) => "int8",
            Self::U16(_) => "uint16",
            Self::I16(_) => "int16",
            Self::U32(_) => "uint32",
            Self::I32(_) => "int32",
            Self::U64(_) => "uint64",
            Self::I64(_) => "int64",
            Self::F32(_) => "float32",
            Self::F64(_) => "float64",
        }
    }

    /// Item size in bytes.
    pub fn itemsize(&self) -> usize {
        match self {
            Self::Bool(_) | Self::U8(_) | Self::I8(_) => 1,
            Self::U16(_) | Self::I16(_) => 2,
            Self::U32(_) | Self::I32(_) | Self::F32(_) => 4,
            Self::U64(_) | Self::I64(_) | Self::F64(_) => 8,
        }
    }

    /// True for the 64-bit integer types that float64 may not represent exactly.
    #[doc(hidden)]
    pub fn is_wide_integer(&self) -> bool {
        matches!(self, Self::U64(_) | Self::I64(_))
    }

    /// Every element converted with `as f64` (lossy for wide integers; see
    /// `preprocessing::exact_float64` for the checked conversion).
    pub fn to_f64_lossy(&self) -> ArrayD<f64> {
        match self {
            Self::Bool(a) => a.mapv(|v| f64::from(u8::from(v))),
            Self::U8(a) => a.mapv(f64::from),
            Self::I8(a) => a.mapv(f64::from),
            Self::U16(a) => a.mapv(f64::from),
            Self::I16(a) => a.mapv(f64::from),
            Self::U32(a) => a.mapv(f64::from),
            Self::I32(a) => a.mapv(f64::from),
            Self::U64(a) => a.mapv(|v| v as f64),
            Self::I64(a) => a.mapv(|v| v as f64),
            Self::F32(a) => a.mapv(f64::from),
            Self::F64(a) => a.clone(),
        }
    }

    /// Mask of the elements equal to the integer `label` (exact for every integer type).
    #[doc(hidden)]
    pub fn eq_label(&self, label: i64) -> ArrayD<bool> {
        match self {
            Self::Bool(a) => a.mapv(|v| i64::from(v) == label),
            Self::U8(a) => a.mapv(|v| i64::from(v) == label),
            Self::I8(a) => a.mapv(|v| i64::from(v) == label),
            Self::U16(a) => a.mapv(|v| i64::from(v) == label),
            Self::I16(a) => a.mapv(|v| i64::from(v) == label),
            Self::U32(a) => a.mapv(|v| i64::from(v) == label),
            Self::I32(a) => a.mapv(|v| i64::from(v) == label),
            Self::U64(a) => a.mapv(|v| i128::from(v) == i128::from(label)),
            Self::I64(a) => a.mapv(|v| v == label),
            Self::F32(a) => a.mapv(|v| f64::from(v) == label as f64),
            Self::F64(a) => a.mapv(|v| v == label as f64),
        }
    }

    /// True when every element is finite (always true for integer and bool types).
    pub fn all_finite(&self) -> bool {
        match self {
            Self::F32(a) => a.iter().all(|v| v.is_finite()),
            Self::F64(a) => a.iter().all(|v| v.is_finite()),
            _ => true,
        }
    }

    /// Smallest and largest element as exact `i128` for integer and bool arrays.
    #[doc(hidden)]
    pub fn integer_range(&self) -> Option<(i128, i128)> {
        macro_rules! range {
            ($a:expr) => {
                $a.iter().fold(None, |acc: Option<(i128, i128)>, &v| {
                    let v = i128::from(v);
                    Some(acc.map_or((v, v), |(lo, hi)| (lo.min(v), hi.max(v))))
                })
            };
        }
        match self {
            Self::Bool(a) => range!(a.mapv(u8::from)),
            Self::U8(a) => range!(a),
            Self::I8(a) => range!(a),
            Self::U16(a) => range!(a),
            Self::I16(a) => range!(a),
            Self::U32(a) => range!(a),
            Self::I32(a) => range!(a),
            Self::U64(a) => range!(a),
            Self::I64(a) => range!(a),
            _ => None,
        }
    }

    /// Gather `self[flat_index[i]]` (row-major) for each output voxel; `None` gives zero.
    /// Keeps the pixel type exactly.
    #[doc(hidden)]
    pub fn gather(&self, shape: &[usize], source: &[Option<usize>]) -> HostArray {
        macro_rules! pick {
            ($a:expr, $zero:expr) => {{
                let flat: Vec<_> = $a.iter().copied().collect();
                let data: Vec<_> = source
                    .iter()
                    .map(|index| index.map_or($zero, |i| flat[i]))
                    .collect();
                ArrayD::from_shape_vec(IxDyn(shape), data)
                    .expect("gather shape equals the number of mapped voxels")
                    .into()
            }};
        }
        match self {
            Self::Bool(a) => pick!(a, false),
            Self::U8(a) => pick!(a, 0),
            Self::I8(a) => pick!(a, 0),
            Self::U16(a) => pick!(a, 0),
            Self::I16(a) => pick!(a, 0),
            Self::U32(a) => pick!(a, 0),
            Self::I32(a) => pick!(a, 0),
            Self::U64(a) => pick!(a, 0),
            Self::I64(a) => pick!(a, 0),
            Self::F32(a) => pick!(a, 0.0),
            Self::F64(a) => pick!(a, 0.0),
        }
    }

    /// Sub-array `[lower, upper)` per axis, keeping the pixel type.
    #[doc(hidden)]
    pub fn crop(&self, lower: &[usize], upper: &[usize]) -> HostArray {
        macro_rules! cut {
            ($a:expr) => {{
                let slices: Vec<ndarray::SliceInfoElem> = lower
                    .iter()
                    .zip(upper)
                    .map(|(&l, &u)| ndarray::SliceInfoElem::Slice {
                        start: l as isize,
                        end: Some(u as isize),
                        step: 1,
                    })
                    .collect();
                $a.slice(slices.as_slice()).to_owned().into()
            }};
        }
        match self {
            Self::Bool(a) => cut!(a),
            Self::U8(a) => cut!(a),
            Self::I8(a) => cut!(a),
            Self::U16(a) => cut!(a),
            Self::I16(a) => cut!(a),
            Self::U32(a) => cut!(a),
            Self::I32(a) => cut!(a),
            Self::U64(a) => cut!(a),
            Self::I64(a) => cut!(a),
            Self::F32(a) => cut!(a),
            Self::F64(a) => cut!(a),
        }
    }
}

/// A 2D/3D volume in this package's array-axis convention (NumPy order).
///
/// `array` is indexed in NumPy order and `spacing` (mm) has one entry per array axis,
/// i.e. it is the reverse of ITK's x-first spacing. `origin` (mm) and `direction` keep the
/// ITK physical x/y/z order; `direction` is the flattened row-major direction matrix and
/// must be orthonormal. Build with [`Volume::new`], which validates the contract.
#[derive(Clone, Debug, PartialEq)]
pub struct Volume {
    /// Voxel values, NumPy order (z, y, x).
    pub array: HostArray,
    /// Spacing in mm per array axis.
    pub spacing: Vec<f64>,
    /// Origin in mm, ITK x, y, z order.
    pub origin: Vec<f64>,
    /// Row-major direction matrix, ITK order.
    pub direction: Vec<f64>,
}

impl Volume {
    /// Construct and validate a volume.
    ///
    /// # Errors
    /// [`Error::InvalidInput`] if the array is empty or not 2-D or 3-D, or if spacing,
    /// origin or direction are non-finite, have the wrong length, or (for spacing) are not
    /// positive, or if the direction is not orthonormal.
    pub fn new(
        array: HostArray,
        spacing: Vec<f64>,
        origin: Vec<f64>,
        direction: Vec<f64>,
    ) -> Result<Self> {
        let volume = Self {
            array,
            spacing,
            origin,
            direction,
        };
        volume.validate()?;
        Ok(volume)
    }

    /// Re-check the construction contract (`Volume.__post_init__`).
    pub fn validate(&self) -> Result<()> {
        let ndim = self.array.ndim();
        if !(ndim == 2 || ndim == 3) || self.array.shape().contains(&0) {
            return invalid("volume must have a nonempty 2D or 3D shape");
        }
        if self.spacing.len() != ndim || self.spacing.iter().any(|v| !v.is_finite() || *v <= 0.0) {
            return invalid("volume requires finite positive spacing per axis");
        }
        if self.origin.len() != ndim || self.origin.iter().any(|v| !v.is_finite()) {
            return invalid("volume requires a finite origin per physical axis");
        }
        if self.direction.len() != ndim * ndim || self.direction.iter().any(|v| !v.is_finite()) {
            return invalid("volume requires a finite square direction matrix");
        }
        if !is_orthonormal(&self.direction, ndim, DIRECTION_ORTHONORMAL_TOLERANCE) {
            return invalid("volume direction must be orthonormal");
        }
        Ok(())
    }
}

fn invalid<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::InvalidInput(message.into()))
}

/// `allclose(DᵀD, I, rtol=0, atol)` for a row-major `n x n` matrix.
pub(crate) fn is_orthonormal(direction: &[f64], n: usize, tolerance: f64) -> bool {
    for a in 0..n {
        for b in 0..n {
            let dot: f64 = (0..n)
                .map(|row| direction[row * n + a] * direction[row * n + b])
                .sum();
            let expected = if a == b { 1.0 } else { 0.0 };
            if (dot - expected).abs() > tolerance {
                return false;
            }
        }
    }
    true
}

/// Return `path` if it names an existing NIfTI or NRRD file (`_require_volume_path`).
fn require_volume_path(path: &Path) -> Result<PathBuf> {
    if !path.is_file() {
        return invalid(format!("missing input volume: {}", path.display()));
    }
    let name = file_name(path);
    if !VOLUME_SUFFIXES.iter().any(|suffix| name.ends_with(suffix)) {
        return invalid(format!(
            "expected a {} file: {}",
            VOLUME_SUFFIXES.join("/"),
            path.display()
        ));
    }
    Ok(path.to_path_buf())
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn is_nifti_name(name: &str) -> bool {
    name.ends_with(".nii") || name.ends_with(".nii.gz")
}

/// Open `source`, inflating when the name ends in `.gz`, and read up to `limit` bytes
/// (`None` reads everything).
fn read_stream(source: &Path, limit: Option<usize>) -> std::io::Result<Vec<u8>> {
    if limit.is_none() && !file_name(source).ends_with(".gz") {
        return fs::read(source);
    }
    let file = fs::File::open(source)?;
    let mut stream: Box<dyn Read> = if file_name(source).ends_with(".gz") {
        Box::new(GzDecoder::new(file))
    } else {
        Box::new(file)
    };
    let mut bytes = Vec::new();
    match limit {
        Some(n) => {
            stream.take(n as u64).read_to_end(&mut bytes)?;
        }
        None => {
            stream.read_to_end(&mut bytes)?;
        }
    }
    Ok(bytes)
}

/// Reject paired NIfTI/Analyze headers disguised as single-file `.nii` inputs
/// (`_require_single_file_nifti`).
fn require_single_file_nifti(source: &Path, header: &[u8]) -> Result<()> {
    let at = |range: std::ops::Range<usize>| header.get(range).unwrap_or(&[]);
    let (magic1, magic2) = (at(344..348), at(4..8));
    let head = at(0..4);
    let header1 = head == b"\x5c\x01\0\0" || head == b"\0\0\x01\x5c";
    let header2 = head == b"\x1c\x02\0\0" || head == b"\0\0\x02\x1c";
    let analyze = header1 && magic1 != b"n+1\0" && magic1 != b"ni1\0";
    if (header1 && magic1 == b"ni1\0") || (header2 && magic2 == b"ni2\0") || analyze {
        return invalid(format!(
            "detached NIfTI/Analyze input is unsupported: {}",
            source.display()
        ));
    }
    Ok(())
}

/// Detect detached NRRD and NIfTI data by header, including compressed headers
/// (`has_detached_data`).
///
/// # Errors
/// [`Error::InvalidInput`] if the file header cannot be inspected or recognized.
pub fn has_detached_data(source: &Path) -> Result<bool> {
    let cannot = || {
        Error::InvalidInput(format!(
            "cannot inspect volume header: {}",
            source.display()
        ))
    };
    let prefix = read_stream(source, Some(348)).map_err(|_| cannot())?;
    if prefix.starts_with(b"NRRD") {
        let header = read_nrrd_header(source).map_err(|_| cannot())?;
        for line in header.split(|&b| b == b'\n') {
            if line.iter().all(u8::is_ascii_whitespace) {
                break;
            }
            if let Some(colon) = line.iter().position(|&b| b == b':') {
                let (field, value) = (&line[..colon], &line[colon + 1..]);
                let field = String::from_utf8_lossy(field).trim().to_lowercase();
                if !value.starts_with(b"=") && (field == "data file" || field == "datafile") {
                    return Ok(true);
                }
            }
        }
        return Ok(false);
    }
    let at = |range: std::ops::Range<usize>| prefix.get(range).unwrap_or(&[]);
    if at(344..348) == b"n+1\0" || at(344..348) == b"ni1\0" {
        return Ok(at(344..348) == b"ni1\0");
    }
    if at(4..8) == b"n+2\0" || at(4..8) == b"ni2\0" {
        return Ok(at(4..8) == b"ni2\0");
    }
    let head = at(0..4);
    if head == b"\x5c\x01\0\0" || head == b"\0\0\x01\x5c" {
        return Ok(true);
    }
    Err(cannot())
}

// ---------------------------------------------------------------------------------------
// Decoded file contents
// ---------------------------------------------------------------------------------------

/// What a file header plus data decode to, in ITK terms.
struct Decoded {
    array: HostArray,
    /// ITK x-first spacing.
    spacing: Vec<f64>,
    origin: Vec<f64>,
    /// Row-major `n x n` direction.
    direction: Vec<f64>,
}

/// Header facts needed before any pixels are decoded.
struct Header {
    /// ITK dimension of the image.
    dimension: usize,
    components: usize,
    /// Complex pixels, which the volume contract rejects.
    complex: bool,
}

fn execution<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::Execution(message.into()))
}

// --------------------------------- NIfTI-1 ---------------------------------------------

struct Nifti {
    big_endian: bool,
    dim: [i64; 8],
    datatype: i16,
    pixdim: [f64; 8],
    vox_offset: usize,
    slope: f64,
    inter: f64,
    units: u8,
    qform_code: i16,
    sform_code: i16,
    quatern: [f64; 3],
    qoffset: [f64; 3],
    srow: [[f64; 4]; 3],
}

struct Cursor<'a> {
    bytes: &'a [u8],
    big: bool,
}

impl Cursor<'_> {
    fn raw<const N: usize>(&self, at: usize) -> [u8; N] {
        let mut out = [0u8; N];
        out.copy_from_slice(&self.bytes[at..at + N]);
        if self.big {
            out.reverse();
        }
        out
    }
    fn i16(&self, at: usize) -> i16 {
        i16::from_le_bytes(self.raw(at))
    }
    fn f32(&self, at: usize) -> f64 {
        f64::from(f32::from_le_bytes(self.raw(at)))
    }
}

fn parse_nifti(bytes: &[u8], source: &Path) -> Result<Nifti> {
    if bytes.len() < 348 {
        return execution(format!("not a NIfTI file: {}", source.display()));
    }
    let little = i32::from_le_bytes(bytes[0..4].try_into().expect("4 bytes"));
    let big = i32::from_be_bytes(bytes[0..4].try_into().expect("4 bytes"));
    let big_endian = match (little, big) {
        (348, _) => false,
        (_, 348) => true,
        (540, _) | (_, 540) => return Err(Error::NotImplemented("NIfTI-2 reading")),
        _ => return execution(format!("unrecognised NIfTI header: {}", source.display())),
    };
    if &bytes[344..348] != b"n+1\0" {
        return execution(format!(
            "not a single-file NIfTI-1 image: {}",
            source.display()
        ));
    }
    let c = Cursor {
        bytes,
        big: big_endian,
    };
    let mut dim = [0i64; 8];
    for (i, slot) in dim.iter_mut().enumerate() {
        *slot = i64::from(c.i16(40 + 2 * i));
    }
    let mut pixdim = [0f64; 8];
    for (i, slot) in pixdim.iter_mut().enumerate() {
        *slot = c.f32(76 + 4 * i);
    }
    let mut srow = [[0f64; 4]; 3];
    for (r, row) in srow.iter_mut().enumerate() {
        for (k, slot) in row.iter_mut().enumerate() {
            *slot = c.f32(280 + 16 * r + 4 * k);
        }
    }
    Ok(Nifti {
        big_endian,
        dim,
        datatype: c.i16(70),
        pixdim,
        vox_offset: c.f32(108).max(0.0) as usize,
        slope: c.f32(112),
        inter: c.f32(116),
        units: bytes[123],
        qform_code: c.i16(252),
        sform_code: c.i16(254),
        quatern: [c.f32(256), c.f32(260), c.f32(264)],
        qoffset: [c.f32(268), c.f32(272), c.f32(276)],
        srow,
    })
}

impl Nifti {
    /// ITK dimension: the declared one, dropping trailing size-1 axes above three.
    fn dimension(&self) -> Result<usize> {
        let declared = self.dim[0];
        if !(1..=7).contains(&declared) || self.dim[1] < 1 {
            return execution("invalid NIfTI dimension");
        }
        let mut ndim = declared as usize;
        while ndim > 3 && self.dim[ndim] == 1 {
            ndim -= 1;
        }
        if ndim < 2 {
            return execution("ITK cannot read a 1-D NIfTI image");
        }
        Ok(ndim)
    }

    /// `nifti_quatern_to_mat44` rotation-and-scale block (RAS, 3x3, row-major).
    fn qform_matrix(&self) -> [[f64; 3]; 3] {
        let [mut b, mut c, mut d] = self.quatern;
        let mut a = 1.0 - (b * b + c * c + d * d);
        if a < 1e-7 {
            let scale = 1.0 / (b * b + c * c + d * d).sqrt();
            b *= scale;
            c *= scale;
            d *= scale;
            a = 0.0;
        } else {
            a = a.sqrt();
        }
        let qfac = if self.pixdim[0] < 0.0 { -1.0 } else { 1.0 };
        let (dx, dy, dz) = (
            if self.pixdim[1] > 0.0 {
                self.pixdim[1]
            } else {
                1.0
            },
            if self.pixdim[2] > 0.0 {
                self.pixdim[2]
            } else {
                1.0
            },
            if self.pixdim[3] > 0.0 {
                self.pixdim[3]
            } else {
                1.0
            },
        );
        // nifti1_io.c stores the matrix as float32; ITK reads those rounded entries.
        let single = |v: f64| f64::from(v as f32);
        [
            [
                single((a * a + b * b - c * c - d * d) * dx),
                single(2.0 * (b * c - a * d) * dy),
                single(2.0 * (b * d + a * c) * dz * qfac),
            ],
            [
                single(2.0 * (b * c + a * d) * dx),
                single((a * a + c * c - b * b - d * d) * dy),
                single(2.0 * (c * d - a * b) * dz * qfac),
            ],
            [
                single(2.0 * (b * d - a * c) * dx),
                single(2.0 * (c * d + a * b) * dy),
                single((a * a + d * d - c * c - b * b) * dz * qfac),
            ],
        ]
    }

    /// ITK geometry: x-first spacing, LPS origin, row-major LPS direction.
    fn geometry(&self, ndim: usize) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>)> {
        let scale = match self.units & 7 {
            1 => 1000.0,
            3 => 0.001,
            _ => 1.0,
        };
        let spacing: Vec<f64> = (0..ndim)
            .map(|i| {
                let value = self.pixdim[i + 1];
                (if value == 0.0 { 1.0 } else { value }) * scale
            })
            .collect();
        let use_sform = self.sform_code == 1 || (self.sform_code > 0 && self.qform_code <= 0);
        let use_qform = !use_sform && self.qform_code > 0;
        let (matrix, offset) = if use_sform {
            let m = [
                [self.srow[0][0], self.srow[0][1], self.srow[0][2]],
                [self.srow[1][0], self.srow[1][1], self.srow[1][2]],
                [self.srow[2][0], self.srow[2][1], self.srow[2][2]],
            ];
            (m, [self.srow[0][3], self.srow[1][3], self.srow[2][3]])
        } else if use_qform {
            (self.qform_matrix(), self.qoffset)
        } else {
            return Ok((
                spacing,
                vec![0.0; ndim],
                (0..ndim * ndim)
                    .map(|k| if k / ndim == k % ndim { 1.0 } else { 0.0 })
                    .collect(),
            ));
        };
        // Columns of the leading ndim x ndim block are normalised (for 2-D files after
        // dropping the z row, which is what ITK does).
        let mut columns = [[0f64; 3]; 3];
        for (j, column) in columns.iter_mut().enumerate().take(ndim) {
            let norm = (0..ndim)
                .map(|i| matrix[i][j] * matrix[i][j])
                .sum::<f64>()
                .sqrt();
            if norm == 0.0 {
                return execution(
                    "ITK only supports orthonormal direction cosines.  No orthonormal definition found!",
                );
            }
            for i in 0..ndim {
                column[i] = matrix[i][j] / norm;
            }
        }
        let flip = [-1.0, -1.0, 1.0];
        let mut direction = vec![0.0; ndim * ndim];
        for i in 0..ndim {
            for j in 0..ndim {
                direction[i * ndim + j] = flip[i] * columns[j][i];
            }
        }
        if !is_orthonormal(&direction, ndim, DIRECTION_ORTHONORMAL_TOLERANCE) {
            return execution(
                "ITK only supports orthonormal direction cosines.  No orthonormal definition found!",
            );
        }
        let origin = (0..ndim).map(|i| flip[i] * offset[i] * scale).collect();
        Ok((spacing, origin, direction))
    }
}

macro_rules! decode_elements {
    ($bytes:expr, $big:expr, $ty:ty) => {{
        const SIZE: usize = std::mem::size_of::<$ty>();
        $bytes
            .chunks_exact(SIZE)
            .map(|chunk| {
                let mut raw = [0u8; SIZE];
                raw.copy_from_slice(chunk);
                if $big {
                    <$ty>::from_be_bytes(raw)
                } else {
                    <$ty>::from_le_bytes(raw)
                }
            })
            .collect::<Vec<$ty>>()
    }};
}

/// Element type of a decoded raw buffer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Element {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
}

impl Element {
    fn size(self) -> usize {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 => 2,
            Self::U32 | Self::I32 | Self::F32 => 4,
            Self::U64 | Self::I64 | Self::F64 => 8,
        }
    }
}

fn host_from_bytes(
    element: Element,
    bytes: &[u8],
    big: bool,
    numpy_shape: &[usize],
) -> Result<HostArray> {
    macro_rules! build {
        ($variant:ident, $ty:ty) => {{
            let data = decode_elements!(bytes, big, $ty);
            HostArray::$variant(
                ArrayD::from_shape_vec(IxDyn(numpy_shape), data)
                    .map_err(|e| Error::Execution(e.to_string()))?,
            )
        }};
    }
    Ok(match element {
        Element::U8 => build!(U8, u8),
        Element::I8 => build!(I8, i8),
        Element::U16 => build!(U16, u16),
        Element::I16 => build!(I16, i16),
        Element::U32 => build!(U32, u32),
        Element::I32 => build!(I32, i32),
        Element::U64 => build!(U64, u64),
        Element::I64 => build!(I64, i64),
        Element::F32 => build!(F32, f32),
        Element::F64 => build!(F64, f64),
    })
}

fn nifti_element(datatype: i16) -> Result<Element> {
    Ok(match datatype {
        2 => Element::U8,
        4 => Element::I16,
        8 => Element::I32,
        16 => Element::F32,
        64 => Element::F64,
        256 => Element::I8,
        512 => Element::U16,
        768 => Element::U32,
        1024 => Element::I64,
        1280 => Element::U64,
        32 | 1792 | 2048 => {
            return invalid("volume must contain real numeric values");
        }
        128 | 2304 => return invalid("expected a single-channel volume"),
        other => return execution(format!("unsupported NIfTI datatype {other}")),
    })
}

fn nifti_header(source: &Path) -> Result<(Nifti, Vec<u8>)> {
    let bytes = read_stream(source, Some(352))
        .map_err(|e| Error::Io(format!("{}: {e}", source.display())))?;
    require_single_file_nifti(source, &bytes)?;
    let header = parse_nifti(&bytes, source)?;
    Ok((header, bytes))
}

fn decode_nifti(source: &Path) -> Result<Decoded> {
    let bytes =
        read_stream(source, None).map_err(|e| Error::Io(format!("{}: {e}", source.display())))?;
    require_single_file_nifti(source, &bytes)?;
    let header = parse_nifti(&bytes, source)?;
    let ndim = header.dimension()?;
    if ndim != 2 && ndim != 3 {
        return invalid(format!("expected a 2D or 3D volume: {}", source.display()));
    }
    let element = nifti_element(header.datatype)?;
    let sizes: Vec<usize> = (1..=ndim).map(|i| header.dim[i].max(1) as usize).collect();
    let count: usize = sizes.iter().product();
    let offset = header.vox_offset.max(352);
    let payload = bytes
        .get(offset..)
        .and_then(|data| data.get(..count.checked_mul(element.size())?))
        .ok_or_else(|| {
            Error::Execution(format!("NIfTI data are truncated: {}", source.display()))
        })?;
    let numpy_shape: Vec<usize> = sizes.iter().rev().copied().collect();
    let mut array = host_from_bytes(element, payload, header.big_endian, &numpy_shape)?;
    let slope = if header.slope == 0.0 {
        1.0
    } else {
        header.slope
    };
    let rescale = header.slope.is_finite()
        && header.inter.is_finite()
        && (slope != 1.0 || header.inter != 0.0);
    if rescale {
        let (slope, inter) = (slope, header.inter);
        // ITK converts the stored value to the output type first (float32 unless the
        // file holds float64), then applies `value * slope + inter` in float64.
        let raw = array.to_f64_lossy();
        array = if element == Element::F64 {
            HostArray::F64(raw.mapv(|v| v * slope + inter))
        } else {
            HostArray::F32(raw.mapv(|v| (f64::from(v as f32) * slope + inter) as f32))
        };
    }
    let (spacing, origin, direction) = header.geometry(ndim)?;
    Ok(Decoded {
        array,
        spacing,
        origin,
        direction,
    })
}

fn nifti_info(source: &Path) -> Result<Header> {
    let (header, _) = nifti_header(source)?;
    let components = match header.datatype {
        128 => 3,
        2304 => 4,
        _ => 1,
    };
    let dimension = header.dimension()?;
    if dimension == 2 || dimension == 3 {
        // ITK derives the orientation while reading the information, and rejects
        // non-orthonormal direction cosines there.
        header.geometry(dimension)?;
    }
    Ok(Header {
        dimension,
        components,
        complex: matches!(header.datatype, 32 | 1792 | 2048),
    })
}

// ---------------------------------- NRRD -----------------------------------------------

struct Nrrd {
    element: Element,
    sizes: Vec<usize>,
    big_endian: bool,
    encoding: String,
    axes: Axes,
    space: Option<String>,
    axis_origin: Option<Vec<f64>>,
    space_directions: Vec<Option<Vec<f64>>>,
    space_origin: Option<Vec<f64>>,
    spacings: Vec<f64>,
    data_files: Option<DataFiles>,
    byte_skip: i64,
    line_skip: usize,
    /// Offset of the data after the header block (attached files).
    body_start: usize,
}

fn parse_nrrd_type(text: &str) -> Result<Element> {
    Ok(match text.trim().to_lowercase().as_str() {
        "signed char" | "int8" | "int8_t" => Element::I8,
        "uchar" | "unsigned char" | "uint8" | "uint8_t" => Element::U8,
        "short" | "short int" | "signed short" | "signed short int" | "int16" | "int16_t" => {
            Element::I16
        }
        "ushort" | "unsigned short" | "unsigned short int" | "uint16" | "uint16_t" => Element::U16,
        "int" | "signed int" | "int32" | "int32_t" => Element::I32,
        "uint" | "unsigned int" | "uint32" | "uint32_t" => Element::U32,
        "longlong"
        | "long long"
        | "long long int"
        | "signed long long"
        | "signed long long int"
        | "int64"
        | "int64_t" => Element::I64,
        "ulonglong" | "unsigned long long" | "unsigned long long int" | "uint64" | "uint64_t" => {
            Element::U64
        }
        "float" => Element::F32,
        "double" => Element::F64,
        other => return execution(format!("unsupported NRRD type {other:?}")),
    })
}

fn parse_vector(text: &str) -> Result<Vec<f64>> {
    let inner = text.trim().trim_start_matches('(').trim_end_matches(')');
    inner
        .split(',')
        .map(|part| {
            part.trim()
                .parse::<f64>()
                .map_err(|_| Error::Execution(format!("invalid NRRD vector {text:?}")))
        })
        .collect()
}

fn parse_vector_list(text: &str) -> Result<Vec<Option<Vec<f64>>>> {
    let mut out = Vec::new();
    let mut rest = text.trim();
    while !rest.is_empty() {
        if let Some(stripped) = rest.strip_prefix("none") {
            out.push(None);
            rest = stripped.trim_start();
        } else if rest.starts_with('(') {
            let close = rest
                .find(')')
                .ok_or_else(|| Error::Execution(format!("invalid NRRD vector list {text:?}")))?;
            out.push(Some(parse_vector(&rest[..=close])?));
            rest = rest[close + 1..].trim_start();
        } else {
            return execution(format!("invalid NRRD vector list {text:?}"));
        }
    }
    Ok(out)
}

fn parse_nrrd(bytes: &[u8]) -> Result<Nrrd> {
    if !bytes.starts_with(b"NRRD") {
        return execution("not an NRRD file");
    }
    let mut fields: Vec<(String, String)> = Vec::new();
    let mut position = 0usize;
    let mut listed_names: &[u8] = &[];
    let mut first = true;
    loop {
        let end = bytes[position..]
            .iter()
            .position(|&b| b == b'\n')
            .map_or(bytes.len(), |n| position + n);
        let line = String::from_utf8_lossy(&bytes[position..end])
            .trim_end_matches('\r')
            .to_string();
        position = (end + 1).min(bytes.len());
        if first {
            first = false;
            continue;
        }
        if line.trim().is_empty() {
            break;
        }
        if line.starts_with('#') {
            if end >= bytes.len() {
                break;
            }
            continue;
        }
        if let Some(colon) = line.find(':') {
            let (key, value) = (&line[..colon], &line[colon + 1..]);
            if !value.starts_with('=') {
                let key = key.trim().to_lowercase();
                let value = value.trim();
                let list = matches!(key.as_str(), "data file" | "datafile")
                    && value.split_ascii_whitespace().next() == Some("LIST");
                fields.push((key, value.to_owned()));
                if list {
                    listed_names = &bytes[position..];
                    break;
                }
            }
        }
        if end >= bytes.len() {
            break;
        }
    }
    let get = |name: &str| {
        fields
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    };
    let Some(dimension) = get("dimension").and_then(|v| v.parse::<usize>().ok()) else {
        return execution("NRRD header lacks dimension");
    };
    let sizes: Vec<usize> = get("sizes")
        .ok_or_else(|| Error::Execution("NRRD header lacks sizes".into()))?
        .split_whitespace()
        .map(|v| v.parse::<usize>())
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| Error::Execution("invalid NRRD sizes".into()))?;
    if sizes.len() != dimension || dimension == 0 || sizes.contains(&0) {
        return execution("NRRD sizes do not match dimension");
    }
    let element = parse_nrrd_type(
        get("type").ok_or_else(|| Error::Execution("NRRD header lacks type".into()))?,
    )?;
    let encoding = get("encoding").unwrap_or("raw").to_lowercase();
    let big_endian = match get("endian").map(str::to_lowercase).as_deref() {
        Some("big") => true,
        Some("little") | None => false,
        Some(other) => return execution(format!("invalid NRRD endian {other:?}")),
    };
    let space = get("space").map(|v| v.to_lowercase().replace([' ', '-'], ""));
    let space_dimension = match space.as_deref() {
        Some(
            "rightanteriorsuperior"
            | "ras"
            | "leftanteriorsuperior"
            | "las"
            | "leftposteriorsuperior"
            | "lps"
            | "scannerxyz"
            | "3drighthanded"
            | "3dlefthanded",
        ) => 3,
        Some(
            "rightanteriorsuperiortime"
            | "rast"
            | "leftanteriorsuperiortime"
            | "last"
            | "leftposteriorsuperiortime"
            | "lpst"
            | "scannerxyztime"
            | "scannerxyzt"
            | "3drighthandedtime"
            | "3dlefthandedtime",
        ) => 4,
        Some(other) => return execution(format!("unsupported NRRD space {other:?}")),
        None => get("space dimension")
            .map(str::parse::<usize>)
            .transpose()
            .map_err(|_| Error::Execution("invalid NRRD space dimension".into()))?
            .unwrap_or(0),
    };
    let axes = Axes::new(&sizes, get("kinds"), space_dimension)?;
    let space_directions = get("space directions")
        .map(parse_vector_list)
        .transpose()?
        .unwrap_or_default();
    let space_origin = get("space origin").map(parse_vector).transpose()?;
    let numbers = |field: Option<&str>| -> Result<Vec<f64>> {
        let values: Vec<_> = field
            .into_iter()
            .flat_map(str::split_whitespace)
            .map(str::parse::<f64>)
            .collect::<std::result::Result<_, _>>()
            .map_err(|_| Error::Execution("invalid NRRD axis coordinates".into()))?;
        if field.is_some() && values.len() != dimension {
            return execution("NRRD axis coordinates do not match dimension");
        }
        Ok(values)
    };
    let spacings = numbers(get("spacings"))?;
    let axis_origin = if space_dimension == 0 {
        let min = numbers(get("axis mins").or_else(|| get("axismins")))?;
        let max = numbers(get("axis maxs").or_else(|| get("axismaxs")))?;
        let center: Vec<_> = get("centers")
            .or_else(|| get("centerings"))
            .into_iter()
            .flat_map(str::split_whitespace)
            .collect();
        axis_limits_origin(&axes, &sizes, &min, &max, &spacings, &center)
    } else {
        None
    };
    let data_files = get("data file")
        .or_else(|| get("datafile"))
        .map(|field| DataFiles::parse(field, listed_names, &sizes))
        .transpose()?;
    Ok(Nrrd {
        element,
        sizes,
        big_endian,
        encoding,
        axes,
        space,
        axis_origin,
        space_directions,
        space_origin,
        spacings,
        data_files,
        byte_skip: get("byte skip")
            .or_else(|| get("byteskip"))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
        line_skip: get("line skip")
            .or_else(|| get("lineskip"))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
        body_start: position,
    })
}

/// ITK uses the first sample's centre as origin. Without a space origin, cell
/// boundaries give a half-spacing offset and node coordinates give the sample itself.
fn axis_limits_origin(
    axes: &Axes,
    sizes: &[usize],
    min: &[f64],
    max: &[f64],
    spacings: &[f64],
    centers: &[&str],
) -> Option<Vec<f64>> {
    let spatial = &axes.image[..axes.spatial_count];
    if !min.get(*spatial.first()?).is_some_and(|v| v.is_finite()) {
        return None;
    }
    let mut origin = vec![0.; axes.image.len()];
    for (i, &axis) in spatial.iter().enumerate() {
        let node = centers
            .get(axis)
            .is_some_and(|s| s.eq_ignore_ascii_case("node"));
        let step = spacings
            .get(axis)
            .copied()
            .filter(|s| s.is_finite())
            .or_else(|| {
                max.get(axis)
                    .filter(|m| m.is_finite())
                    .map(|m| (m - min[axis]) / (sizes[axis] - usize::from(node)) as f64)
            })?;
        origin[i] = if node {
            min[axis]
        } else {
            min[axis] + step / 2.
        };
    }
    Some(origin)
}

impl Nrrd {
    fn components(&self) -> usize {
        self.axes.component.map_or(1, |axis| self.sizes[axis])
    }

    fn geometry(&self) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>)> {
        let n = self.axes.image.len();
        let spatial = &self.axes.image[..self.axes.spatial_count];
        let signs = match self.space.as_deref() {
            Some("rightanteriorsuperior" | "ras") => [-1., -1.],
            Some("leftanteriorsuperior" | "las") => [1., -1.],
            _ => [1., 1.],
        };
        let sign = |i: usize| signs.get(i).copied().unwrap_or(1.);
        let mut spacing = vec![1.0; n];
        let mut direction: Vec<f64> = (0..n * n)
            .map(|k| if k / n == k % n { 1.0 } else { 0.0 })
            .collect();
        for (i, &axis) in spatial.iter().enumerate() {
            if let Some(Some(vector)) = self.space_directions.get(axis) {
                if vector.len() != spatial.len() {
                    return execution("NRRD space direction has the wrong dimension");
                }
                let norm = vector.iter().map(|v| v * v).sum::<f64>().sqrt();
                if norm == 0.0 {
                    return execution("NRRD space direction has zero length");
                }
                spacing[i] = norm;
                for (r, component) in vector.iter().enumerate() {
                    direction[r * n + i] = sign(r) * (component * (1.0 / norm));
                }
            } else if let Some(&s) = self.spacings.get(axis)
                && s.is_finite()
            {
                spacing[i] = s;
            }
        }
        let mut origin = self.axis_origin.clone().unwrap_or_else(|| vec![0.; n]);
        if let Some(values) = &self.space_origin {
            if values.len() != spatial.len() {
                return execution("NRRD space origin has the wrong dimension");
            }
            for (i, v) in values.iter().enumerate() {
                origin[i] = sign(i) * v;
            }
        }
        Ok((spacing, origin, direction))
    }
}

/// A LIST header may exceed one MiB. Stop at the header delimiter for attached data.
fn read_nrrd_header(source: &Path) -> std::io::Result<Vec<u8>> {
    let file = fs::File::open(source)?;
    let reader: Box<dyn Read> = if file_name(source).ends_with(".gz") {
        Box::new(GzDecoder::new(file))
    } else {
        Box::new(file)
    };
    let mut stream = BufReader::new(reader);
    let mut bytes = Vec::new();
    loop {
        let start = bytes.len();
        if stream.read_until(b'\n', &mut bytes)? == 0 || matches!(&bytes[start..], b"\n" | b"\r\n")
        {
            break;
        }
    }
    Ok(bytes)
}

fn nrrd_open(source: &Path, whole: bool) -> Result<(Nrrd, Vec<u8>)> {
    let bytes = if whole {
        read_stream(source, None)
    } else {
        read_nrrd_header(source)
    }
    .map_err(|e| Error::Io(format!("{}: {e}", source.display())))?;
    let header = parse_nrrd(&bytes)?;
    Ok((header, bytes))
}

fn nrrd_info(source: &Path) -> Result<Header> {
    let (nrrd, _) = nrrd_open(source, false)?;
    Ok(Header {
        dimension: nrrd.axes.image.len(),
        components: if nrrd.axes.complex {
            1
        } else {
            nrrd.components()
        },
        complex: nrrd.axes.complex,
    })
}

fn decode_nrrd_hex(bytes: &[u8], wanted: usize) -> Result<Vec<u8>> {
    let mut digits = bytes
        .iter()
        .copied()
        .filter(|&ch| !matches!(ch, b' ' | b'\t'..=b'\r'));
    let mut digit = || {
        let ch = digits
            .next()
            .ok_or_else(|| Error::Execution("NRRD hex data are truncated".into()))?;
        char::from(ch)
            .to_digit(16)
            .map(|n| n as u8)
            .ok_or_else(|| Error::Execution("invalid character in NRRD hex data".into()))
    };
    let mut decoded = Vec::with_capacity(wanted.min(bytes.len() / 2));
    for _ in 0..wanted {
        decoded.push((digit()? << 4) | digit()?);
    }
    Ok(decoded)
}

fn nrrd_piece<'a>(
    nrrd: &Nrrd,
    source: &Path,
    mut payload: &'a [u8],
    wanted: usize,
) -> Result<Cow<'a, [u8]>> {
    for _ in 0..nrrd.line_skip {
        if payload.is_empty() {
            break;
        }
        let cut = payload
            .iter()
            .position(|&b| b == b'\n')
            .map_or(payload.len(), |p| p + 1);
        payload = &payload[cut..];
    }
    let skip = nrrd.byte_skip.max(0) as usize;
    let data: Cow<'_, [u8]> = match nrrd.encoding.as_str() {
        "raw" => {
            let start = if nrrd.byte_skip == -1 {
                payload
                    .len()
                    .checked_sub(wanted)
                    .ok_or_else(|| Error::Execution("NRRD data are truncated".into()))?
            } else {
                skip.min(payload.len())
            };
            Cow::Borrowed(&payload[start..])
        }
        "gzip" | "gz" => {
            // Bound the initial reservation by both the requested pixels and the
            // compressed input. Tiny corrupt files cannot reserve an image-sized buffer.
            let capacity = wanted.min(payload.len().saturating_mul(1024)).min(64 << 20);
            let mut out = Vec::with_capacity(capacity);
            flate2::bufread::GzDecoder::new(payload)
                .read_to_end(&mut out)
                .map_err(|e| Error::Io(format!("{}: {e}", source.display())))?;
            if skip != 0 {
                out.drain(..skip.min(out.len()));
            }
            Cow::Owned(out)
        }
        "text" | "txt" | "ascii" => {
            let text = String::from_utf8_lossy(&payload[skip.min(payload.len())..]);
            let mut out = Vec::with_capacity(wanted.min(payload.len()));
            for token in text.split_whitespace().take(wanted / nrrd.element.size()) {
                macro_rules! push {
                    ($ty:ty) => {{
                        let value: $ty = token
                            .parse()
                            .map_err(|_| Error::Execution("invalid NRRD text value".into()))?;
                        out.extend_from_slice(&value.to_le_bytes());
                    }};
                }
                match nrrd.element {
                    Element::U8 => push!(u8),
                    Element::I8 => push!(i8),
                    Element::U16 => push!(u16),
                    Element::I16 => push!(i16),
                    Element::U32 => push!(u32),
                    Element::I32 => push!(i32),
                    Element::U64 => push!(u64),
                    Element::I64 => push!(i64),
                    Element::F32 => push!(f32),
                    Element::F64 => push!(f64),
                }
            }
            Cow::Owned(out)
        }
        "hex" => {
            if nrrd.byte_skip < 0 {
                return execution("negative NRRD byte skip requires raw encoding");
            }
            Cow::Owned(decode_nrrd_hex(
                &payload[skip.min(payload.len())..],
                wanted,
            )?)
        }
        "bzip2" | "bz2" => return Err(Error::NotImplemented("NRRD bzip2 encoding")),
        other => return execution(format!("unsupported NRRD encoding {other:?}")),
    };
    if data.len() < wanted {
        return execution(format!("NRRD data are truncated: {}", source.display()));
    }
    Ok(data)
}

fn decode_nrrd(source: &Path) -> Result<Decoded> {
    let (nrrd, bytes) = nrrd_open(source, true)?;
    let n = nrrd.axes.image.len();
    if n != 2 && n != 3 {
        return invalid(format!("expected a 2D or 3D volume: {}", source.display()));
    }
    if nrrd.components() != 1 {
        return invalid(format!(
            "expected a single-channel volume: {}",
            source.display()
        ));
    }
    let wanted = nrrd
        .sizes
        .iter()
        .try_fold(nrrd.element.size(), |bytes, &n| bytes.checked_mul(n))
        .ok_or_else(|| Error::Execution("NRRD data length exceeds addressable memory".into()))?;
    let storage_axes: Vec<_> = (0..nrrd.sizes.len())
        .rev()
        .filter(|&axis| Some(axis) != nrrd.axes.component)
        .collect();
    let numpy_shape: Vec<_> = storage_axes.iter().map(|&axis| nrrd.sizes[axis]).collect();
    let detached;
    let data = match &nrrd.data_files {
        None => nrrd_piece(&nrrd, source, &bytes[nrrd.body_start..], wanted)?,
        Some(files) => {
            let directory = source.parent().unwrap_or_else(|| Path::new("."));
            let read = |index| {
                let path = directory.join(files.name(index).as_ref());
                fs::read(&path).map_err(|e| Error::Io(format!("{}: {e}", path.display())))
            };
            if files.len() == 1 {
                detached = read(0)?;
                nrrd_piece(&nrrd, source, &detached, wanted)?
            } else {
                let size = wanted / files.len();
                let mut pixels = Vec::new();
                for i in 0..files.len() {
                    let file = read(i)?;
                    let slab = nrrd_piece(&nrrd, source, &file, size)?;
                    pixels.extend_from_slice(&slab[..size]);
                }
                Cow::Owned(pixels)
            }
        }
    };
    let little_text = matches!(nrrd.encoding.as_str(), "text" | "txt" | "ascii");
    let big = nrrd.big_endian && !little_text && nrrd.element.size() > 1;
    let array = match (nrrd.element, data) {
        (Element::U8, Cow::Owned(mut bytes)) => {
            bytes.truncate(wanted);
            HostArray::U8(
                ArrayD::from_shape_vec(IxDyn(&numpy_shape), bytes)
                    .map_err(|e| Error::Execution(e.to_string()))?,
            )
        }
        (_, bytes) => host_from_bytes(nrrd.element, &bytes[..wanted], big, &numpy_shape)?,
    };
    // A singleton channel has no effect on pixel order. Additional list axes
    // can be interleaved with spatial axes in the file and move to the front in NumPy.
    let array = if nrrd
        .axes
        .image
        .iter()
        .rev()
        .copied()
        .eq(storage_axes.iter().copied())
    {
        array
    } else {
        let order: Vec<_> = nrrd
            .axes
            .image
            .iter()
            .rev()
            .map(|axis| {
                storage_axes
                    .iter()
                    .position(|a| a == axis)
                    .expect("image axis is in storage")
            })
            .collect();
        host_dispatch!(array, a => HostArray::from(a.permuted_axes(IxDyn(&order)).as_standard_layout().into_owned()))
    };
    let (spacing, origin, direction) = nrrd.geometry()?;
    Ok(Decoded {
        array,
        spacing,
        origin,
        direction,
    })
}

// ------------------------------- public readers ------------------------------------------

fn header_info(source: &Path) -> Result<Header> {
    if is_nifti_name(&file_name(source)) {
        nifti_info(source)
    } else {
        nrrd_info(source)
    }
}

/// Read a volume under mask (`integer`) or image rules (`_read`).
///
/// Masks must hold integer labels. Images must be finite; stored non-finite values are
/// rejected (the NiBabel re-check of mlx-radiomics).
fn read(path: &Path, integer: bool) -> Result<Volume> {
    let source = require_volume_path(path)?;
    let info = header_info(&source)?;
    if info.dimension != 2 && info.dimension != 3 {
        return invalid(format!("expected a 2D or 3D volume: {}", source.display()));
    }
    if info.components != 1 {
        return invalid(format!(
            "expected a single-channel volume: {}",
            source.display()
        ));
    }
    if info.complex {
        return invalid(if integer {
            format!("mask must contain integer labels: {}", source.display())
        } else {
            "volume must contain real numeric values".to_string()
        });
    }
    let decoded = if is_nifti_name(&file_name(&source)) {
        decode_nifti(&source)?
    } else {
        decode_nrrd(&source)?
    };
    let integer_kind = matches!(
        decoded.array.kind(),
        ArrayKind::Bool | ArrayKind::Signed | ArrayKind::Unsigned
    );
    if integer && !integer_kind {
        return invalid(format!(
            "mask must contain integer labels: {}",
            source.display()
        ));
    }
    if !integer && !decoded.array.all_finite() {
        return invalid(format!(
            "image contains non-finite voxels: {}",
            source.display()
        ));
    }
    let spacing = decoded.spacing.iter().rev().copied().collect();
    Volume::new(decoded.array, spacing, decoded.origin, decoded.direction)
}

/// Read a 2D/3D NIfTI or NRRD image preserving pixel type and physical geometry
/// (`read_image`).
///
/// # Errors
/// [`Error::InvalidInput`] if the file is missing, has an unsupported suffix or detached
/// NIfTI header, is not a single-channel 2-D or 3-D volume, or contains non-finite values.
pub fn read_image(path: &Path) -> Result<Volume> {
    read(path, false)
}

/// Read a 2D/3D integer label mask preserving pixel type and physical geometry
/// (`read_mask`).
///
/// # Errors
/// [`Error::InvalidInput`] if the file is missing, has an unsupported suffix or detached
/// NIfTI header, is not a single-channel 2-D or 3-D volume, or has a non-integer pixel
/// type.
pub fn read_mask(path: &Path) -> Result<Volume> {
    read(path, true)
}

// ------------------------------- feature-map writer --------------------------------------

/// `nifti_mat44_to_quatern` for an orthonormal-column RAS matrix; returns `(b, c, d, qfac)`.
fn quaternion(rotation: &[[f64; 3]; 3]) -> (f64, f64, f64, f64) {
    let mut r = *rotation;
    for j in 0..3 {
        let norm = (0..3).map(|i| r[i][j] * r[i][j]).sum::<f64>().sqrt();
        if norm > 0.0 {
            for row in &mut r {
                row[j] /= norm;
            }
        }
    }
    let determinant = r[0][0] * (r[1][1] * r[2][2] - r[1][2] * r[2][1])
        - r[0][1] * (r[1][0] * r[2][2] - r[1][2] * r[2][0])
        + r[0][2] * (r[1][0] * r[2][1] - r[1][1] * r[2][0]);
    let mut qfac = 1.0;
    if determinant < 0.0 {
        for row in &mut r {
            row[2] = -row[2];
        }
        qfac = -1.0;
    }
    let (r11, r12, r13) = (r[0][0], r[0][1], r[0][2]);
    let (r21, r22, r23) = (r[1][0], r[1][1], r[1][2]);
    let (r31, r32, r33) = (r[2][0], r[2][1], r[2][2]);
    let a0 = r11 + r22 + r33 + 1.0;
    let (mut a, mut b, mut c, mut d);
    if a0 > 0.5 {
        a = 0.5 * a0.sqrt();
        b = 0.25 * (r32 - r23) / a;
        c = 0.25 * (r13 - r31) / a;
        d = 0.25 * (r21 - r12) / a;
    } else {
        let xd = 1.0 + r11 - (r22 + r33);
        let yd = 1.0 + r22 - (r11 + r33);
        let zd = 1.0 + r33 - (r11 + r22);
        if xd > 1.0 {
            b = 0.5 * xd.sqrt();
            a = 0.25 * (r32 - r23) / b;
            c = 0.25 * (r12 + r21) / b;
            d = 0.25 * (r13 + r31) / b;
        } else if yd > 1.0 {
            c = 0.5 * yd.sqrt();
            a = 0.25 * (r13 - r31) / c;
            b = 0.25 * (r12 + r21) / c;
            d = 0.25 * (r23 + r32) / c;
        } else {
            d = 0.5 * zd.sqrt();
            a = 0.25 * (r21 - r12) / d;
            b = 0.25 * (r13 + r31) / d;
            c = 0.25 * (r23 + r32) / d;
        }
        if a < 0.0 {
            a = -a;
            b = -b;
            c = -c;
            d = -d;
        }
    }
    let _ = a;
    (b, c, d, qfac)
}

/// NIfTI-1 float32 header including the four-byte extender.
fn nifti_map_header(reference: &Volume, shape: &[usize]) -> Result<Vec<u8>> {
    let ndim = shape.len();
    let signs = [-1.0, -1.0, 1.0];
    // Affine: diag(-1,-1,1) * D * spacing (columns), translation diag(-1,-1,1) * origin.
    let mut affine = [[0f64; 4]; 4];
    affine[3][3] = 1.0;
    for k in 0..3 {
        affine[k][k] = 1.0;
    }
    for i in 0..ndim {
        for j in 0..ndim {
            affine[i][j] =
                signs[i] * reference.direction[i * ndim + j] * reference.spacing[ndim - 1 - j];
        }
        affine[i][3] = signs[i] * reference.origin[i];
    }
    let rotation = [
        [affine[0][0], affine[0][1], affine[0][2]],
        [affine[1][0], affine[1][1], affine[1][2]],
        [affine[2][0], affine[2][1], affine[2][2]],
    ];
    let zooms: Vec<f64> = (0..3)
        .map(|j| {
            (0..3)
                .map(|i| rotation[i][j] * rotation[i][j])
                .sum::<f64>()
                .sqrt()
        })
        .collect();
    let (b, c, d, qfac) = quaternion(&rotation);
    let mut header = vec![0u8; 352];
    header[0..4].copy_from_slice(&348i32.to_le_bytes());
    let mut dims = [1i16; 8];
    dims[0] = ndim as i16;
    // NiBabel uses the FreeSurfer conventions for a long one-dimensional
    // surface stored in a 3-D NIfTI-1. Other shapes must fit signed 16-bit axes.
    match shape {
        [1, 1, 163842] => dims[1..4].copy_from_slice(&[27307, 1, 6]),
        [1, 1, n] if *n > i16::MAX as usize => {
            let length = i32::try_from(*n)
                .map_err(|_| Error::InvalidInput("surface length exceeds NIfTI-1 range".into()))?;
            dims[1] = -1;
            header[144..148].copy_from_slice(&length.to_le_bytes());
        }
        _ => {
            for (i, &n) in shape.iter().rev().enumerate() {
                dims[i + 1] = i16::try_from(n).map_err(|_| {
                    Error::InvalidInput("feature map shape exceeds NIfTI-1 dimensions".into())
                })?;
            }
        }
    }
    for (i, v) in dims.iter().enumerate() {
        header[40 + 2 * i..42 + 2 * i].copy_from_slice(&v.to_le_bytes());
    }
    header[70..72].copy_from_slice(&16i16.to_le_bytes());
    header[72..74].copy_from_slice(&32i16.to_le_bytes());
    let pixdim = [
        qfac as f32,
        zooms[0] as f32,
        zooms[1] as f32,
        zooms[2] as f32,
        1.0,
        1.0,
        1.0,
        1.0,
    ];
    for (i, v) in pixdim.iter().enumerate() {
        header[76 + 4 * i..80 + 4 * i].copy_from_slice(&v.to_le_bytes());
    }
    header[108..112].copy_from_slice(&352f32.to_le_bytes());
    header[112..116].copy_from_slice(&1f32.to_le_bytes());
    header[116..120].copy_from_slice(&0f32.to_le_bytes());
    header[123] = 2;
    header[252..254].copy_from_slice(&1i16.to_le_bytes());
    header[254..256].copy_from_slice(&1i16.to_le_bytes());
    for (k, v) in [b, c, d, affine[0][3], affine[1][3], affine[2][3]]
        .iter()
        .enumerate()
    {
        header[256 + 4 * k..260 + 4 * k].copy_from_slice(&(*v as f32).to_le_bytes());
    }
    for (r, row) in affine.iter().take(3).enumerate() {
        for (k, v) in row.iter().enumerate() {
            let at = 280 + 16 * r + 4 * k;
            header[at..at + 4].copy_from_slice(&(*v as f32).to_le_bytes());
        }
    }
    header[344..348].copy_from_slice(b"n+1\0");
    Ok(header)
}

/// Emit an exported map in source-grid order. Only the selected samples need backing storage.
pub(crate) fn write_sparse_nifti_map(
    path: &Path,
    reference: &Volume,
    samples: impl Iterator<Item = (u32, f32)>,
) -> Result<()> {
    let shape = reference.array.shape();
    let length = reference.array.len();
    let header = nifti_map_header(reference, shape)?;
    let mut encoder = GzEncoder::new(fs::File::create(path)?, Compression::new(1));
    encoder.write_all(&header)?;
    let mut writer = std::io::BufWriter::with_capacity(64 * 1024, &mut encoder);
    write_sparse_pixels(&mut writer, length, samples)?;
    writer.flush()?;
    drop(writer);
    encoder.finish()?;
    Ok(())
}

fn write_sparse_pixels(
    writer: &mut impl Write,
    length: usize,
    samples: impl Iterator<Item = (u32, f32)>,
) -> Result<()> {
    let mut next = 0;
    for (offset, value) in samples {
        let offset = offset as usize;
        if offset < next || offset >= length {
            return invalid("voxel export centers must be in source-grid order");
        }
        write_zero_voxels(writer, offset - next)?;
        writer.write_all(&value.to_le_bytes())?;
        next = offset + 1;
    }
    write_zero_voxels(writer, length - next)?;
    Ok(())
}

fn write_zero_voxels(writer: &mut impl Write, mut count: usize) -> std::io::Result<()> {
    static ZERO: [u8; 65536] = [0; 65536];
    while count > 0 {
        let chunk = count.min(ZERO.len() / 4);
        writer.write_all(&ZERO[..4 * chunk])?;
        count -= chunk;
    }
    Ok(())
}

#[cfg(test)]
mod sparse_export_tests {
    use crate::imaging::io::write_sparse_pixels;

    #[test]
    fn sparse_nifti_has_exact_float_words_and_full_length_across_background_runs() {
        let samples = [
            (3, -0.0),
            (4, f32::from_bits(0x7fc01001)),
            (50001, f32::NEG_INFINITY),
        ];
        let mut bytes = Vec::new();
        write_sparse_pixels(&mut bytes, 80000, samples.into_iter()).unwrap();
        let mut expected = vec![0u8; 4 * 80000];
        for (at, value) in samples {
            let i = at as usize * 4;
            expected[i..i + 4].copy_from_slice(&value.to_le_bytes());
        }
        assert!(bytes == expected);
    }
}
