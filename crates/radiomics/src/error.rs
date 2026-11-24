//! Error classes of the mlx-radiomics contract.
//!
//! Mirrors the `medmlx_core.errors` classes that mlx-radiomics raises: `ValueError` and
//! `InvalidInputError` map to [`Error::InvalidInput`], `RuntimeError` and
//! `ModelExecutionError` to [`Error::Execution`], and `MissingDependencyError` to
//! [`Error::MissingDependency`].

use std::fmt;

/// Failure of an extraction, export or configuration step.
///
/// The error is `Send`, `Sync` and `'static`, implements [`std::error::Error`] and
/// carries a human-readable message. New variants may be added, so match with a
/// wildcard arm.
///
/// ```
/// use radiomics::Error;
///
/// fn assert_traits<E: std::error::Error + Send + Sync + 'static>() {}
/// assert_traits::<Error>();
///
/// let error = Error::InvalidInput("bin_width must be positive".into());
/// assert_eq!(error.to_string(), "bin_width must be positive");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Invalid input, settings or parameters (`ValueError`, `InvalidInputError`).
    InvalidInput(String),
    /// A numerical or execution refusal (`RuntimeError`, `ModelExecutionError`).
    Execution(String),
    /// A requested allocation exceeds its memory budget or addressable index range (`MemoryError`).
    Memory(String),
    /// An optional capability is unavailable (`MissingDependencyError`).
    MissingDependency(String),
    /// I/O failure (`OSError`).
    Io(String),
    /// The named mlx-radiomics stage has not been ported yet.
    NotImplemented(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) | Self::Execution(message) | Self::Memory(message) => {
                f.write_str(message)
            }
            Self::MissingDependency(message) => write!(f, "missing dependency: {message}"),
            Self::Io(message) => write!(f, "I/O error: {message}"),
            Self::NotImplemented(stage) => write!(f, "{stage} is not supported"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

/// Result with the crate [`Error`].
pub type Result<T, E = Error> = std::result::Result<T, E>;
