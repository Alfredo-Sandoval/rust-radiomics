//! CPU first-order statistics and the conventions that select them.

mod statistics;

pub use crate::native_first_order::statistics::{FirstOrderFeature, FirstOrderSettings, Kurtosis};

pub(crate) use crate::native_first_order::statistics::Calculation;
use std::{error::Error, fmt};

/// Invalid input to a first-order extraction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtractionError {
    /// No features were requested.
    EmptyFeatures,
    /// A sampled intensity is NaN or infinite.
    NonFiniteIntensity,
    /// The histogram bin width is outside the finite positive range.
    InvalidBinWidth,
    /// A bin index is too large for precise discretization.
    BinIndexOutOfRange,
    /// The spacing or its product is outside the finite positive range.
    InvalidSpacing,
    /// The intensity shift is NaN or infinite.
    InvalidIntensityShift,
}

impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFeatures => f.write_str("select at least one first-order feature"),
            Self::NonFiniteIntensity => f.write_str("sampled intensities must be finite"),
            Self::InvalidBinWidth => f.write_str("bin_width must be positive and finite"),
            Self::BinIndexOutOfRange => {
                f.write_str("intensity bin index exceeds the supported range")
            }
            Self::InvalidSpacing => {
                f.write_str("spacing and voxel volume must be positive and finite")
            }
            Self::InvalidIntensityShift => f.write_str("voxel_array_shift must be finite"),
        }
    }
}

impl Error for ExtractionError {}
