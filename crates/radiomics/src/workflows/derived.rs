//! Source-precision admission and undefined LBP3D kurtosis handling for CPU filters.

use crate::filters::lbp3d::SourceDtype;
use crate::imaging::io::HostArray;
use crate::{Error, ImageType, Result, Settings};
use ndarray::{ArrayD, Axis, Zip};

pub(super) fn lbp_dtype(image: &HostArray, normalized: bool) -> Result<SourceDtype> {
    if normalized {
        return Ok(SourceDtype::Float);
    }
    let (min, max) = match image {
        HostArray::I8(_) => (i8::MIN.into(), i8::MAX.into()),
        HostArray::U8(_) => (0, u8::MAX.into()),
        HostArray::I16(_) => (i16::MIN.into(), i16::MAX.into()),
        HostArray::U16(_) => (0, u16::MAX.into()),
        HostArray::F32(_) | HostArray::F64(_) => return Ok(SourceDtype::Float),
        _ => {
            return Err(Error::InvalidInput(
                "LBP3D integer dtype exceeds exact float32 integer range".into(),
            ));
        }
    };
    Ok(SourceDtype::Integer { min, max })
}

pub(super) fn validate_filter_source(image: &ArrayD<f64>, kinds: &[ImageType]) -> Result<()> {
    let needs_tail = kinds.contains(&ImageType::Lbp2D);
    for &x in image {
        let high = x as f32;
        if !high.is_finite() {
            return Err(Error::InvalidInput(
                "feature image intensities must be finite".into(),
            ));
        }
        if needs_tail {
            let rest = x - f64::from(high);
            let rest = rest - f64::from(rest as f32);
            if rest != f64::from(rest as f32) {
                return Err(Error::InvalidInput("LBP2D source sign is unresolved: source precision cannot be represented by three float32 words".into()));
            }
        }
    }
    Ok(())
}

/// Replace undefined samples with the smallest finite ROI intensity for binning.
/// Report undefined only at centers whose sampled kernel includes an undefined voxel.
/// Returns the affected centers and whether the lattice had no finite ROI samples.
pub(super) fn prepare_kurtosis(
    image: &mut ArrayD<f64>,
    roi: &ArrayD<bool>,
    settings: &Settings,
    voxel: bool,
) -> (Option<ArrayD<bool>>, bool) {
    let mut invalid = ArrayD::from_elem(image.raw_dim(), false);
    let mut min = f64::INFINITY;
    let mut has_invalid = false;
    Zip::from(&mut invalid)
        .and(&*image)
        .and(roi)
        .for_each(|u, &x, &inside| {
            *u = inside && !x.is_finite();
            has_invalid |= *u;
            if inside && x.is_finite() {
                min = min.min(x);
            }
        });
    if !has_invalid {
        return (None, false);
    }
    let empty = !min.is_finite();
    let fill = if empty {
        settings.bin_anchor.unwrap_or(0.)
    } else {
        min
    };
    Zip::from(image).and(&invalid).for_each(|v, &bad| {
        if bad {
            *v = fill;
        }
    });
    if voxel {
        let radius = settings.kernel_radius as usize;
        for a in 0..invalid.ndim() {
            if Some(a) == settings.force2d_axis {
                continue;
            }
            for mut line in invalid.lanes_mut(Axis(a)) {
                let mut prefix = Vec::with_capacity(line.len() + 1);
                prefix.push(0usize);
                let mut sum = 0;
                for &bad in &line {
                    sum += usize::from(bad);
                    prefix.push(sum);
                }
                let n = line.len();
                for (i, val) in line.iter_mut().enumerate() {
                    *val = prefix[i.saturating_add(radius).saturating_add(1).min(n)]
                        != prefix[i.saturating_sub(radius)];
                }
            }
        }
    }
    (Some(invalid), empty)
}
