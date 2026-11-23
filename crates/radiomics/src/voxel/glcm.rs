//! General GLCM evaluation on one local window, mirroring the matrix path in
//! mlx-radiomics. The voxel scheduler calls this for asymmetric, merged and
//! larger windows. Its compact symmetric path uses the same global bin lattice
//! and direction-average rules.

use crate::features::texture;
use crate::{Features, Prepared, Result, Settings};

/// The GLCM statistics of one window, for the selected `glcm_*` keys in input order.
///
/// `window` holds the window's bins on the global lattice (0 outside the valid
/// positions) and `gray_levels` of the whole image.
pub fn local_glcm(
    window: &Prepared,
    spacing: &[f64],
    settings: &Settings,
    names: &[&str],
) -> Result<Features> {
    let glcm: Vec<&str> = names
        .iter()
        .copied()
        .filter(|n| n.starts_with("glcm_"))
        .collect();
    texture::compute(window, spacing, settings, &glcm)
}
