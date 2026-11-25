//! Split one mask label into connected lesions.
//!
//! Mirrors `mlxradiomics/imaging/lesions.py`. Voxels of the label that touch by a face,
//! an edge or a corner belong to one lesion (26-connectivity in 3D, 8-connectivity in
//! 2D), as in PySERA. Lesions are numbered from 1 by decreasing voxel count; equal
//! counts keep the order of their first voxel in C order.

use crate::imaging::io::{ArrayKind, HostArray};
use crate::{Error, Result};
use ndarray::{ArrayD, IxDyn};

/// Which connected lesions of a mask label are kept.
///
/// Lesions smaller than `min_voxels` are dropped before `max_lesions` keeps the largest
/// ones. The default keeps every lesion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LesionOptions {
    /// Smallest lesion to keep, in voxels. Must be at least 1.
    pub min_voxels: usize,
    /// Keep only this many of the largest lesions; `None` keeps all.
    pub max_lesions: Option<usize>,
}

impl Default for LesionOptions {
    fn default() -> Self {
        Self {
            min_voxels: 1,
            max_lesions: None,
        }
    }
}

impl LesionOptions {
    /// Reject a size floor or count limit of zero.
    pub(crate) fn validate(&self) -> Result<()> {
        if self.min_voxels == 0 {
            return Err(Error::InvalidInput(
                "min_voxels must be a positive integer".into(),
            ));
        }
        if self.max_lesions == Some(0) {
            return Err(Error::InvalidInput(
                "max_lesions must be a positive integer or None".into(),
            ));
        }
        Ok(())
    }
}

/// Lesion labels with each kept lesion's voxel count and index bounds, largest first.
/// `labels` is 0 outside the kept lesions; entry `i` of the vectors describes lesion
/// `i + 1`, with `upper` exclusive.
pub(crate) struct LesionMap {
    pub labels: ArrayD<i32>,
    pub voxels: Vec<usize>,
    pub lower: Vec<Vec<usize>>,
    pub upper: Vec<Vec<usize>>,
}

/// Label the connected lesions of `label` and describe the ones `options` keeps.
pub(crate) fn find_lesions(
    mask: &HostArray,
    label: i64,
    options: &LesionOptions,
) -> Result<LesionMap> {
    options.validate()?;
    let shape = mask.shape().to_vec();
    if !(2..=3).contains(&shape.len()) || shape.contains(&0) {
        return Err(Error::InvalidInput(
            "mask must be a nonempty 2D or 3D array".into(),
        ));
    }
    if mask.kind() == ArrayKind::Float {
        return Err(Error::InvalidInput(
            "mask must contain integer labels".into(),
        ));
    }
    // As in `Settings`: label 0 would split the background into lesions.
    if label <= 0 {
        return Err(Error::InvalidInput(
            "label must be a positive integer".into(),
        ));
    }
    let selected = mask.eq_label(label);
    let selected = selected.as_standard_layout();
    let selected = selected.as_slice().expect("standard layout");
    if !selected.contains(&true) {
        return Err(Error::InvalidInput(
            "selected label is absent from mask".into(),
        ));
    }
    // A 2D mask is one slice of a 3D grid; its bounds drop the leading axis again.
    let lead = 3 - shape.len();
    let mut dims = [1usize; 3];
    dims[lead..].copy_from_slice(&shape);
    let mut component = vec![0u32; selected.len()];
    let mut sizes = Vec::new();
    let mut bounds: Vec<([usize; 3], [usize; 3])> = Vec::new();
    let mut stack = Vec::new();
    // Components are numbered in the C order of their first voxel.
    for start in 0..selected.len() {
        if !selected[start] || component[start] != 0 {
            continue;
        }
        let id = sizes.len() as u32 + 1;
        let (mut size, mut lower, mut upper) = (0usize, [usize::MAX; 3], [0usize; 3]);
        component[start] = id;
        stack.push(start);
        while let Some(flat) = stack.pop() {
            size += 1;
            let at = [
                flat / (dims[1] * dims[2]),
                flat / dims[2] % dims[1],
                flat % dims[2],
            ];
            for axis in 0..3 {
                lower[axis] = lower[axis].min(at[axis]);
                upper[axis] = upper[axis].max(at[axis] + 1);
            }
            for z in at[0].saturating_sub(1)..(at[0] + 2).min(dims[0]) {
                for y in at[1].saturating_sub(1)..(at[1] + 2).min(dims[1]) {
                    for x in at[2].saturating_sub(1)..(at[2] + 2).min(dims[2]) {
                        let next = (z * dims[1] + y) * dims[2] + x;
                        if selected[next] && component[next] == 0 {
                            component[next] = id;
                            stack.push(next);
                        }
                    }
                }
            }
        }
        sizes.push(size);
        bounds.push((lower, upper));
    }
    let mut order: Vec<usize> = (0..sizes.len()).collect();
    // The sort is stable, so equal sizes keep their first-voxel order.
    order.sort_by_key(|&id| std::cmp::Reverse(sizes[id]));
    order.retain(|&id| sizes[id] >= options.min_voxels);
    order.truncate(options.max_lesions.unwrap_or(usize::MAX));
    let mut renumber = vec![0i32; sizes.len() + 1];
    for (rank, &id) in order.iter().enumerate() {
        renumber[id + 1] = rank as i32 + 1;
    }
    let labels = component.iter().map(|&id| renumber[id as usize]).collect();
    Ok(LesionMap {
        labels: ArrayD::from_shape_vec(IxDyn(&shape), labels).expect("mask shape"),
        voxels: order.iter().map(|&id| sizes[id]).collect(),
        lower: order
            .iter()
            .map(|&id| bounds[id].0[lead..].to_vec())
            .collect(),
        upper: order
            .iter()
            .map(|&id| bounds[id].1[lead..].to_vec())
            .collect(),
    })
}

/// Number the connected lesions of one mask label, largest first.
///
/// The result has the mask's shape: 0 outside the kept lesions, 1 for the largest lesion,
/// 2 for the next. Voxels that touch by a face, an edge or a corner belong to one lesion.
/// Equal-sized lesions keep the order of their first voxel in C order. Wrap the map in a
/// [`HostArray`] and pass it as `mask` with `Settings { label: k, .. }` to extract or
/// export lesion `k`. The map is all zero when no lesion reaches `options.min_voxels`.
///
/// # Errors
/// [`Error::InvalidInput`] if the mask is not a nonempty 2D or 3D boolean or integer
/// array, the label is not positive or is absent, or an option is zero.
///
/// # Examples
///
/// ```
/// use radiomics::ndarray::ArrayD;
/// use radiomics::{label_lesions, HostArray, LesionOptions};
///
/// # fn main() -> radiomics::Result<()> {
/// let mut mask = ArrayD::from_elem(vec![1, 4, 6], 0_u8);
/// mask[[0, 0, 0]] = 1;
/// for x in 2..5 {
///     mask[[0, 3, x]] = 1;
/// }
/// let lesions = label_lesions(&HostArray::from(mask), 1, &LesionOptions::default())?;
/// assert_eq!(lesions[[0, 3, 2]], 1);
/// assert_eq!(lesions[[0, 0, 0]], 2);
/// # Ok(())
/// # }
/// ```
pub fn label_lesions(mask: &HostArray, label: i64, options: &LesionOptions) -> Result<ArrayD<i32>> {
    Ok(find_lesions(mask, label, options)?.labels)
}
