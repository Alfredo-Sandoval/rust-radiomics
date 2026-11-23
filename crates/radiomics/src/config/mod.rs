//! Extraction settings and PyRadiomics parameter parsing.
//!
//! Mirrors the `mlxradiomics/config` package.

/// Declares a closed set of string choices that round-trips the exact mlx-radiomics spelling.
macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        $name:ident { $($(#[$vmeta:meta])* $variant:ident => $text:literal),+ $(,)? }
        default $default:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $($(#[$vmeta])* #[doc = concat!("`", $text, "`")] $variant),+
        }

        impl $name {
            /// Every choice, in mlx-radiomics declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            /// The mlx-radiomics spelling of this choice.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $text),+
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::error::Error;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                match text {
                    $($text => Ok(Self::$variant),)+
                    _ => Err(crate::error::Error::InvalidInput(format!(
                        concat!(stringify!($name), " must be one of {:?}, got {:?}"),
                        [$($text),+],
                        text,
                    ))),
                }
            }
        }
    };
}

pub mod filtering;
pub mod parameters;
pub mod settings;
pub mod spatial;
pub mod transforms;
pub mod validation;
pub mod wavelets;
