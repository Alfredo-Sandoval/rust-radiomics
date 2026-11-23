//! Exact integer indexing of local neighbourhoods around a batch of voxel centres.
//!
//! Mirrors `mlxradiomics/voxel/gather.py` (90 lines at mlx-radiomics b58783c).
//! Status: ported. CPU index arithmetic on the calling thread; the callers fan out over
//! centres with rayon. Coordinates, bounds tests and edge clamping use exact integers,
//! as in the Metal kernel; positions outside the image are clamped for lookup and
//! reported invalid by [`NeighborhoodIndex::inside`].

use crate::{Error, Result};
use ndarray::{Array2, ArrayView2};

/// Largest supported voxel count of one grid, as in mlx-radiomics (int32 indexing).
const MAX_VOXELS: usize = 1 << 30;

/// Clamped flat positions and the original, unclamped image-bounds mask, both `(C, K)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeighborhoodIndex {
    /// Row-major flat index of each position, clamped to the image.
    pub flat: Array2<i32>,
    /// Whether each unclamped position lies inside the image.
    pub inside: Array2<bool>,
}

impl NeighborhoodIndex {}

/// Grid shape of one voxel extraction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeighborhoodPlan {
    shape: Vec<usize>,
}

impl NeighborhoodPlan {
    /// Plan for a 2-D or 3-D grid of fewer than 2**30 voxels.
    pub fn new(shape: &[usize]) -> Result<Self> {
        if !(2..=3).contains(&shape.len())
            || shape.contains(&0)
            || shape.iter().product::<usize>() >= MAX_VOXELS
        {
            return Err(Error::InvalidInput(
                "image exceeds supported integer indexing/count range".into(),
            ));
        }
        Ok(Self {
            shape: shape.to_vec(),
        })
    }

    /// Build one `(C, K)` lookup for int32 centres `(C, D)` and offsets `(K, D)`.
    pub fn index(
        &self,
        centers: ArrayView2<'_, i32>,
        offsets: ArrayView2<'_, i32>,
    ) -> NeighborhoodIndex {
        let (c, k, ndim) = (centers.nrows(), offsets.nrows(), self.shape.len());
        assert!(centers.ncols() == ndim && offsets.ncols() == ndim);
        let mut flat = Array2::<i32>::zeros((c, k));
        let mut inside = Array2::from_elem((c, k), true);
        // Most windows lie wholly inside the image. A single bounds test per
        // centre then suffices for all offsets, including arbitrary offset order.
        let mut low = [0i64; 3];
        let mut high = [0i64; 3];
        let mut deltas = Vec::with_capacity(k);
        for offset in offsets.rows() {
            let mut delta = 0i64;
            for a in 0..ndim {
                let d = i64::from(offset[a]);
                low[a] = low[a].min(d);
                high[a] = high[a].max(d);
                delta = delta * self.shape[a] as i64 + d;
            }
            deltas.push(delta);
        }
        for (r, center) in centers.rows().into_iter().enumerate() {
            let interior = (0..ndim).all(|a| {
                i64::from(center[a]) + low[a] >= 0
                    && i64::from(center[a]) + high[a] < self.shape[a] as i64
            });
            if interior {
                let origin = center
                    .iter()
                    .zip(&self.shape)
                    .fold(0i64, |i, (&v, &n)| i * n as i64 + i64::from(v));
                for (out, &delta) in flat.row_mut(r).iter_mut().zip(&deltas) {
                    *out = (origin + delta) as i32;
                }
            } else {
                for (col, offset) in offsets.rows().into_iter().enumerate() {
                    let mut position = 0i64;
                    let mut in_image = true;
                    for a in 0..ndim {
                        let extent = self.shape[a] as i64;
                        let at = i64::from(center[a]) + i64::from(offset[a]);
                        in_image &= (0..extent).contains(&at);
                        position = position * extent + at.clamp(0, extent - 1);
                    }
                    flat[[r, col]] = position as i32;
                    inside[[r, col]] = in_image;
                }
            }
        }
        NeighborhoodIndex { flat, inside }
    }
}

#[cfg(test)]
mod tests {
    use crate::voxel::gather::*;

    #[test]
    fn oversized_grids_are_refused() {
        assert!(NeighborhoodPlan::new(&[1 << 15, 1 << 15]).is_err());
        assert!(NeighborhoodPlan::new(&[4]).is_err());
    }
}
