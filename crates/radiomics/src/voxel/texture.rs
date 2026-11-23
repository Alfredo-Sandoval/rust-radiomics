//! General GLRLM, GLSZM, GLDZM, GLDM and NGTDM evaluation on one local
//! window. The voxel scheduler uses the ROI formulas here for explicit
//! aggregations and windows outside the compact bit-set implementation.

use crate::features::texture;
use crate::{Features, Prepared, Result, Settings};

const FAMILIES: [&str; 5] = ["glrlm_", "glszm_", "gldzm_", "gldm_", "ngtdm_"];

/// The GLRLM, GLSZM, GLDZM, GLDM and NGTDM statistics of one window, for the selected
/// keys in input order. Arguments as for [`crate::voxel::glcm::local_glcm`]; for GLDZM the
/// window's `shape_roi` is the morphological mask.
pub fn local_texture(
    window: &Prepared,
    spacing: &[f64],
    settings: &Settings,
    names: &[&str],
) -> Result<Features> {
    let selected: Vec<&str> = names
        .iter()
        .copied()
        .filter(|n| FAMILIES.iter().any(|f| n.starts_with(f)))
        .collect();
    texture::compute(window, spacing, settings, &selected)
}
