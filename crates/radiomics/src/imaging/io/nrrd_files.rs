//! Detached NRRD payloads in storage order. The final axis varies slowest.
use crate::{Error, Result};
use regex::Regex;
use std::borrow::Cow;
use std::sync::OnceLock;

pub(super) enum DataFiles {
    Names(Vec<String>),
    Numbered {
        prefix: String,
        suffix: String,
        width: usize,
        zero_pad: bool,
        first: i64,
        step: i64,
        count: usize,
    },
}

impl DataFiles {
    pub(super) fn parse(field: &str, list: &[u8], sizes: &[usize]) -> Result<Self> {
        let invalid = || Error::Execution("invalid NRRD data file specification".into());
        let mut words = field.split_ascii_whitespace();
        let template = words.next().ok_or_else(invalid)?;
        let files = if let Some(rest) = template.strip_prefix("LIST") {
            if !rest.is_empty() {
                return Err(invalid());
            }
            let lines = std::str::from_utf8(list).map_err(|_| invalid())?;
            let names: Vec<_> = lines.lines().map(str::to_owned).collect();
            if names.is_empty() || names.iter().any(String::is_empty) {
                return Err(invalid());
            }
            Self::Names(names)
        } else {
            static DECIMAL: OnceLock<Regex> = OnceLock::new();
            let decimal = DECIMAL
                .get_or_init(|| Regex::new(r"^((?:[^%]|%%)*)%([0-9]*)d((?:[^%]|%%)*)$").unwrap());
            let Some(parts) = decimal.captures(template) else {
                return Ok(Self::Names(vec![field.to_owned()]));
            };
            let mut integer = || {
                words
                    .next()
                    .and_then(|word| word.parse::<i32>().ok())
                    .map(i64::from)
                    .ok_or_else(invalid)
            };
            let (first, last, step) = (integer()?, integer()?, integer()?);
            if step == 0 || (last - first) * step.signum() < 0 {
                return Err(invalid());
            }
            let count = usize::try_from((last - first) / step + 1).map_err(|_| invalid())?;
            let width = if parts[2].is_empty() {
                0
            } else {
                parts[2].parse().map_err(|_| invalid())?
            };
            // A wider field cannot name an accessible file on a supported platform.
            if width > 4096 {
                return Err(invalid());
            }
            Self::Numbered {
                prefix: parts[1].replace("%%", "%"),
                suffix: parts[3].replace("%%", "%"),
                width,
                zero_pad: parts[2].starts_with('0'),
                first,
                step,
                count,
            }
        };
        let slab_dim = match words.next() {
            Some(word) => word.parse().map_err(|_| invalid())?,
            None => sizes.len().saturating_sub(1),
        };
        if words.next().is_some() || !(1..=sizes.len()).contains(&slab_dim) {
            return Err(invalid());
        }
        let count = files.len();
        let valid = if slab_dim == sizes.len() {
            sizes[slab_dim - 1].is_multiple_of(count)
        } else {
            sizes[slab_dim..]
                .iter()
                .try_fold(1usize, |n, &size| n.checked_mul(size))
                == Some(count)
        };
        if !valid {
            return Err(Error::Execution(
                "NRRD data file count does not match the grid".into(),
            ));
        }
        Ok(files)
    }

    pub(super) fn len(&self) -> usize {
        match self {
            Self::Names(names) => names.len(),
            Self::Numbered { count, .. } => *count,
        }
    }

    pub(super) fn name(&self, index: usize) -> Cow<'_, str> {
        match self {
            Self::Names(names) => Cow::Borrowed(&names[index]),
            Self::Numbered {
                prefix,
                suffix,
                width,
                zero_pad,
                first,
                step,
                ..
            } => {
                let i = first + index as i64 * step;
                Cow::Owned(if *zero_pad {
                    format!("{prefix}{i:0width$}{suffix}")
                } else {
                    format!("{prefix}{i:width$}{suffix}")
                })
            }
        }
    }
}
