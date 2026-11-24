//! Directional pair counting, mirroring `mlxradiomics/features/cooccurrence.py`.
//! Status: ported. CPU numerical work on a zero-padded flat copy of the masked grid;
//! the caller runs directions in parallel. Native directional averaging shares the
//! reduction in `aggregation`.

use crate::features::matrices::{
    Accumulator, DenseCounts, Offset, Padded, Scratch, TextureMatrix, dense_width,
};

fn glcm_kernel(p: &Padded, delta: isize, symmetric: bool, out: &mut impl Accumulator) {
    p.for_each_center(0..p.centers(), |index, left| {
        let right = p.at(index, delta);
        if right > 0 {
            out.count(left, right as usize);
            if symmetric {
                out.count(right as usize, left);
            }
        }
    });
}

pub(super) fn glcm_matrix(
    p: &Padded,
    offset: Offset,
    symmetric: bool,
    ng: u32,
    voxels: usize,
    scratch: Scratch,
) -> TextureMatrix {
    let delta = p.flat(offset);
    match dense_width(ng, ng as usize + 1, voxels, scratch) {
        Some(width) => {
            let mut dense = DenseCounts::new(ng, width, false);
            glcm_kernel(p, delta, symmetric, &mut dense);
            dense.into_matrix()
        }
        None => {
            let mut sparse = TextureMatrix::default();
            glcm_kernel(p, delta, symmetric, &mut sparse);
            sparse
        }
    }
}
