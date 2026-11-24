//! Filter choices and scalar parameter validation.
//!
//! Mirrors `mlxradiomics/config/filtering.py` (137 lines at mlx-radiomics b58783c).
//! Status: enums, Laws taps and `validate_laws_kernel` done.

string_enum! {
    /// Boundary handling for spatial filters. `Mirror` is NumPy `symmetric`.
    FilterPadding { Constant => "constant", Nearest => "nearest", Periodic => "periodic", Mirror => "mirror" }
    default Mirror
}

string_enum! {
    /// Frequency grid used by the spectral filters.
    FourierGrid { Dft => "dft", Centered => "centered" }
    default Dft
}

string_enum! {
    /// Laplacian-of-Gaussian kernel construction.
    LogKernel { Recursive => "recursive", Sampled => "sampled" }
    default Recursive
}

string_enum! {
    /// Base filter of the Riesz transform.
    RieszBase { Log => "log", Simoncelli => "simoncelli" }
    default Log
}

string_enum! {
    /// Which part of the complex Gabor response is kept.
    GaborResponse { Modulus => "modulus", Real => "real", Imaginary => "imaginary" }
    default Modulus
}

string_enum! {
    /// Pooling over rotated filter responses.
    OrientationPooling { None => "none", Average => "average", Max => "max", Min => "min" }
    default None
}

string_enum! {
    /// Local binary pattern coding (scikit-image method names).
    LbpMethod {
        Default => "default",
        RotationInvariant => "ror",
        Uniform => "uniform",
        NonrotationUniform => "nri_uniform",
        Variance => "var",
    }
    default Uniform
}

/// Highest Riesz order accepted with structure-tensor steering.
pub const MAX_STEERING_ORDER: u32 = 16;

/// Laws kernel names, sorted, as listed in the validation message.
const LAWS_NAMES: [&str; 8] = ["E3", "E5", "L3", "L5", "R5", "S3", "S5", "W5"];

/// Normalised IBSI 2 Laws taps for a two-character kernel name such as `"L5"`.
pub fn laws_taps(name: &str) -> Option<Vec<f64>> {
    let (scale, taps): (f64, &[f64]) = match name {
        "L3" => (6.0, &[1.0, 2.0, 1.0]),
        "L5" => (70.0, &[1.0, 4.0, 6.0, 4.0, 1.0]),
        "E3" => (2.0, &[-1.0, 0.0, 1.0]),
        "E5" => (10.0, &[-1.0, -2.0, 0.0, 2.0, 1.0]),
        "S3" => (6.0, &[-1.0, 2.0, -1.0]),
        "S5" => (6.0, &[-1.0, 0.0, 2.0, 0.0, -1.0]),
        "W5" => (10.0, &[-1.0, 2.0, 0.0, -2.0, 1.0]),
        "R5" => (70.0, &[1.0, -4.0, 6.0, -4.0, 1.0]),
        _ => return None,
    };
    let norm = scale.sqrt();
    Some(taps.iter().map(|tap| tap / norm).collect())
}

/// Validate a concatenated Laws kernel name such as `"E5L5S5"`.
///
/// `codes` holds two or three two-character names in IBSI axis order (k1, k2, k3);
/// k1 is the last array axis.
pub fn validate_laws_kernel(codes: &str) -> crate::error::Result<()> {
    let valid = codes.is_ascii()
        && matches!(codes.len(), 4 | 6)
        && (0..codes.len())
            .step_by(2)
            .all(|i| laws_taps(&codes[i..i + 2]).is_some());
    if valid {
        Ok(())
    } else {
        Err(crate::error::Error::InvalidInput(format!(
            "laws_kernel must be two or three names from {:?} concatenated in k1, k2, k3 order",
            LAWS_NAMES
        )))
    }
}
