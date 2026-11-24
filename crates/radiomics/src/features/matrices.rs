//! Integer texture matrices, mirroring `mlxradiomics/features/matrices.py`.
//! Status: ported. CPU numerical work on 2-D or 3-D masked bin grids. Matrices are counted
//! on a zero-padded flat copy of the grid, so neighbour lookups need no bounds checks. With
//! at most 512 gray levels the counts go into dense integer arrays and the directions of
//! one family, or fixed row chunks of one neighbourhood matrix, run on the Rayon pool and
//! are combined in a fixed order; larger lattices count into the sparse map on one thread.

use crate::config::settings::{Family, NeighborhoodSupport, Settings};
use crate::features::cooccurrence::glcm_matrix;
use crate::numerics::sparse_counts::Counts;
use crate::{Error, Result};
use ndarray::{ArrayView3, ArrayViewD, Axis, Ix2, Ix3};
use rayon::prelude::*;
use std::collections::BTreeMap;

type Position = [usize; 3];
pub(super) type Offset = [isize; 3];

/// A 2-D domain uses a leading singleton axis excluded from all geometry.
#[derive(Clone, Copy)]
pub(super) struct Domain<'a> {
    pub bins: ArrayView3<'a, i32>,
    morphology: ArrayView3<'a, bool>,
    pub ndim: usize,
}

impl<'a> Domain<'a> {
    pub fn new(bins: ArrayViewD<'a, i32>, morphology: ArrayViewD<'a, bool>) -> Self {
        let ndim = bins.ndim();
        fn view3<T>(a: ArrayViewD<'_, T>) -> ArrayView3<'_, T> {
            if a.ndim() == 2 {
                a.into_dimensionality::<Ix2>().unwrap().insert_axis(Axis(0))
            } else {
                a.into_dimensionality::<Ix3>().unwrap()
            }
        }
        Self {
            bins: view3(bins),
            morphology: view3(morphology),
            ndim,
        }
    }

    pub fn plane(self, axis: usize, index: usize) -> Self {
        Self {
            bins: self
                .bins
                .index_axis_move(Axis(axis), index)
                .insert_axis(Axis(0)),
            morphology: self
                .morphology
                .index_axis_move(Axis(axis), index)
                .insert_axis(Axis(0)),
            ndim: 2,
        }
    }

    pub fn shape(self) -> Position {
        self.bins.dim().into()
    }
}

/// [`crate::imaging::preprocessing::directions`] on this module's 3-axis padded grid.
///
/// mlx calls `directions(shape, replace(settings, distances=...))`; here the radius,
/// the kept Chebyshev shells and the `max(s, 2)` run-length extent are explicit
/// arguments. Offsets and their order match the exported function (unit-tested below).
fn directions(
    domain: Domain<'_>,
    force_axis: Option<usize>,
    radius: u32,
    shell: impl Fn(u32) -> bool,
    singleton_runs: bool,
) -> Vec<Offset> {
    let mut limits = [0isize; 3];
    for (axis, limit) in limits.iter_mut().enumerate().skip(3 - domain.ndim) {
        if Some(axis) != force_axis.map(|i| i + 3 - domain.ndim) {
            let extent = domain.shape()[axis]
                .saturating_sub(1)
                .max(usize::from(singleton_runs));
            *limit = extent.min(radius as usize) as isize;
        }
    }
    let [z, y, x] = limits;
    let mut result = Vec::new();
    for k in (0..=z).rev() {
        for j in (-y..=y).rev() {
            for i in (-x..=x).rev() {
                let offset = [k, j, i];
                if offset > [0; 3] && shell(k.max(j.abs()).max(i.abs()) as u32) {
                    result.push(offset);
                }
            }
        }
    }
    result
}

/// Gray-level count above which matrices are counted in the sparse map on one thread.
pub(super) const DENSE_LEVELS: u32 = 512;
/// Largest dense count array in cells.
const DENSE_CELLS: usize = 1 << 19;
/// Domains of this many voxels or more count into the sparse map (u32 cells could overflow).
const DENSE_VOXELS: usize = 1 << 30;
/// Neighbourhood matrices are counted in this many row chunks, whatever the thread count.
const CHUNKS: usize = 32;

#[derive(Default)]
pub(super) struct TextureMatrix {
    pub counts: Counts,
    pub voxels: u64,
    pub differences: BTreeMap<usize, f64>,
}

impl TextureMatrix {
    pub fn count(&mut self, i: usize, j: usize) {
        *self.counts.entry((i, j)).or_default() += 1;
    }

    /// Add `other`'s counts. Small sorted maps are rebuilt in one merge pass.
    pub fn merge(&mut self, other: Self) {
        if self.counts.is_empty() {
            self.counts = other.counts;
        } else if self.counts.len() + other.counts.len() > 4096 {
            // Large maps are updated in place: a rebuild would hold both copies at once.
            for (cell, n) in other.counts {
                *self.counts.entry(cell).or_default() += n;
            }
        } else if !other.counts.is_empty() {
            let left = std::mem::take(&mut self.counts);
            let mut merged = Vec::with_capacity(left.len() + other.counts.len());
            let mut left = left.into_iter().peekable();
            let mut right = other.counts.into_iter().peekable();
            loop {
                let next = match (left.peek(), right.peek()) {
                    (Some(&(a, _)), Some(&(b, _))) => match a.cmp(&b) {
                        std::cmp::Ordering::Less => left.next(),
                        std::cmp::Ordering::Greater => right.next(),
                        std::cmp::Ordering::Equal => {
                            let (cell, n) = left.next().unwrap();
                            Some((cell, n + right.next().unwrap().1))
                        }
                    },
                    (Some(_), None) => left.next(),
                    (None, Some(_)) => right.next(),
                    (None, None) => break,
                };
                merged.extend(next);
            }
            self.counts = merged.into_iter().collect();
        }
        self.voxels += other.voxels;
        for (level, s) in other.differences {
            *self.differences.entry(level).or_default() += s;
        }
    }
}

/// Where the counting kernels put their results.
pub(super) trait Accumulator {
    fn count(&mut self, i: usize, j: usize);
    fn tone_difference(&mut self, level: usize, difference: f64);
    fn voxel(&mut self);
}

impl Accumulator for TextureMatrix {
    fn count(&mut self, i: usize, j: usize) {
        TextureMatrix::count(self, i, j);
    }
    fn tone_difference(&mut self, level: usize, difference: f64) {
        *self.differences.entry(level).or_default() += difference;
    }
    fn voxel(&mut self) {
        self.voxels += 1;
    }
}

/// Dense counts over rows `0..=ng` and columns `0..width`, plus per-level tone differences.
pub(super) struct DenseCounts {
    cells: Vec<u32>,
    width: usize,
    differences: Vec<f64>,
    voxels: u64,
}

impl DenseCounts {
    pub fn new(ng: u32, width: usize, tone: bool) -> Self {
        let rows = ng as usize + 1;
        Self {
            cells: vec![0; rows * width],
            width,
            differences: vec![0.; if tone { rows } else { 0 }],
            voxels: 0,
        }
    }

    fn add(&mut self, other: &Self) {
        for (a, b) in self.cells.iter_mut().zip(&other.cells) {
            *a += b;
        }
        for (a, b) in self.differences.iter_mut().zip(&other.differences) {
            *a += b;
        }
        self.voxels += other.voxels;
    }

    /// Occupied cells in key order build the map in one pass.
    pub fn into_matrix(self) -> TextureMatrix {
        let width = self.width;
        let counts = self
            .cells
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c != 0)
            .map(|(k, &c)| ((k / width, k % width), u64::from(c)))
            .collect();
        let differences = if self.differences.is_empty() {
            BTreeMap::new()
        } else {
            self.cells
                .chunks_exact(width)
                .enumerate()
                .filter(|(_, row)| row.iter().any(|&c| c != 0))
                .map(|(level, _)| (level, self.differences[level]))
                .collect()
        };
        TextureMatrix {
            counts,
            voxels: self.voxels,
            differences,
        }
    }
}

impl Accumulator for DenseCounts {
    #[inline]
    fn count(&mut self, i: usize, j: usize) {
        self.cells[i * self.width + j] += 1;
    }
    #[inline]
    fn tone_difference(&mut self, level: usize, difference: f64) {
        self.differences[level] += difference;
    }
    #[inline]
    fn voxel(&mut self) {
        self.voxels += 1;
    }
}

/// The bin grid copied into a flat array with a zero border, so that stepping by a
/// constant flat offset from any occupied voxel never leaves the allocation or wraps
/// into another row. Zero marks "not in the ROI", so borders never match a level.
pub(super) struct Padded {
    data: Vec<i32>,
    /// Flat indices of the occupied voxels in raster order.
    centers: Vec<usize>,
    shape: Position,
    pad: Position,
    strides: [usize; 3],
}

impl Padded {
    /// Pad each axis by the largest offset component, or more where `extra` asks for it.
    pub fn new(domain: Domain<'_>, offsets: &[Offset], extra: Position) -> Self {
        let shape = domain.shape();
        let mut pad = extra;
        for offset in offsets {
            for axis in 0..3 {
                pad[axis] = pad[axis].max(offset[axis].unsigned_abs());
            }
        }
        let dims = [
            shape[0] + 2 * pad[0],
            shape[1] + 2 * pad[1],
            shape[2] + 2 * pad[2],
        ];
        let strides = [dims[1] * dims[2], dims[2], 1];
        let mut data = vec![0; dims[0] * strides[0]];
        let mut centers = Vec::new();
        for (z, plane) in domain.bins.outer_iter().enumerate() {
            for (y, row) in plane.outer_iter().enumerate() {
                let start = (z + pad[0]) * strides[0] + (y + pad[1]) * strides[1] + pad[2];
                let target = &mut data[start..start + shape[2]];
                match row.as_slice() {
                    Some(source) => target.copy_from_slice(source),
                    None => target.iter_mut().zip(row).for_each(|(t, &v)| *t = v),
                }
                centers.extend(
                    target
                        .iter()
                        .enumerate()
                        .filter(|&(_, &level)| level > 0)
                        .map(|(x, _)| start + x),
                );
            }
        }
        Self {
            data,
            centers,
            shape,
            pad,
            strides,
        }
    }

    /// Flat index of the first voxel of row `row` (z-major, then y).
    fn row_start(&self, row: usize) -> usize {
        let (z, y) = (row / self.shape[1], row % self.shape[1]);
        (z + self.pad[0]) * self.strides[0] + (y + self.pad[1]) * self.strides[1] + self.pad[2]
    }

    pub fn centers(&self) -> usize {
        self.centers.len()
    }

    pub fn flat(&self, offset: Offset) -> isize {
        (0..3)
            .map(|axis| offset[axis] * self.strides[axis] as isize)
            .sum()
    }

    /// Visit the occupied voxels numbered `range` (raster order) as (flat index, level).
    #[inline]
    pub fn for_each_center(
        &self,
        range: std::ops::Range<usize>,
        mut visit: impl FnMut(usize, usize),
    ) {
        for &index in &self.centers[range] {
            visit(index, self.data[index] as usize);
        }
    }

    #[inline]
    pub fn at(&self, index: usize, delta: isize) -> i32 {
        self.data[index.wrapping_add_signed(delta)]
    }
}

/// Bytes one count table of `cells` cells needs, with its converted sparse map.
pub(super) fn task_bytes(cells: usize, voxels: usize) -> usize {
    4 * cells + 40 * cells.min(2 * voxels)
}

/// How much scratch a call may use and how many count tables may be live at once.
#[derive(Clone, Copy)]
pub(super) struct Scratch {
    pub budget: usize,
    pub slots: usize,
}

/// The dense width when two tables (an accumulator and a part) fit the scratch budget.
pub(super) fn dense_width(ng: u32, width: usize, voxels: usize, scratch: Scratch) -> Option<usize> {
    let cells = (ng as usize + 1) * width;
    (ng <= DENSE_LEVELS
        && cells <= DENSE_CELLS
        && voxels < DENSE_VOXELS
        && 2 * task_bytes(cells, voxels) <= scratch.budget)
        .then_some(width)
}

/// Count-table cells of one matrix of `family` on `domain`; 0 for the sparse zone families.
pub(super) fn table_cells(
    domain: Domain<'_>,
    ng: u32,
    family: Family,
    settings: &Settings,
    axis: Option<usize>,
) -> (usize, usize) {
    let rows = ng as usize + 1;
    let shells = |d| settings.distances.contains(&d);
    let radius = *settings.distances.iter().max().unwrap();
    match family {
        Family::Glcm => (
            rows * rows,
            directions(domain, axis, radius, shells, false).len(),
        ),
        Family::Glrlm => (
            rows * (domain.shape().into_iter().max().unwrap() + 1),
            directions(domain, axis, 1, |_| true, true).len(),
        ),
        Family::Gldm | Family::Ngtdm => {
            let n = match settings.neighborhood_support {
                NeighborhoodSupport::ClosedRadius => {
                    directions(domain, axis, settings.neighborhood_radius, |_| true, false).len()
                }
                NeighborhoodSupport::Distances => {
                    directions(domain, axis, radius, shells, false).len()
                }
            };
            (rows * if family == Family::Gldm { 2 * n + 2 } else { 2 }, 1)
        }
        _ => (0, 1),
    }
}

fn run_kernel(p: &Padded, delta: isize, out: &mut impl Accumulator) {
    p.for_each_center(0..p.centers(), |index, level| {
        out.voxel();
        if p.at(index, -delta) as usize == level {
            return;
        }
        let mut length = 1;
        let mut current = index;
        while p.at(current, delta) as usize == level {
            current = current.wrapping_add_signed(delta);
            length += 1;
        }
        out.count(level, length);
    });
}

fn run_matrix(
    p: &Padded,
    offset: Offset,
    ng: u32,
    voxels: usize,
    scratch: Scratch,
) -> TextureMatrix {
    let delta = p.flat(offset);
    let longest = *p.shape.iter().max().unwrap() + 1;
    match dense_width(ng, longest, voxels, scratch) {
        Some(width) => {
            let mut dense = DenseCounts::new(ng, width, false);
            run_kernel(p, delta, &mut dense);
            dense.into_matrix()
        }
        None => {
            let mut sparse = TextureMatrix::default();
            run_kernel(p, delta, &mut sparse);
            sparse
        }
    }
}

fn neighborhood_kernel(
    p: &Padded,
    range: std::ops::Range<usize>,
    deltas: &[isize],
    gldm: bool,
    alpha: u32,
    out: &mut impl Accumulator,
) {
    p.for_each_center(range, |index, i| {
        if gldm {
            let mut size = 1;
            for &delta in deltas {
                let j = p.at(index, delta);
                size += usize::from(j > 0 && (i as i32).abs_diff(j) <= alpha);
            }
            out.count(i, size);
        } else {
            let (mut count, mut sum) = (0i64, 0i64);
            for &delta in deltas {
                let j = i64::from(p.at(index, delta));
                if j > 0 {
                    count += 1;
                    sum += j;
                }
            }
            if count == 0 {
                return;
            }
            out.count(i, 1);
            out.tone_difference(i, (count * i as i64 - sum).abs() as f64 / count as f64);
        }
        out.voxel();
    });
}

fn neighborhood_matrix(
    domain: Domain<'_>,
    offsets: &[Offset],
    family: Family,
    alpha: u32,
    ng: u32,
    scratch: Scratch,
) -> TextureMatrix {
    let p = Padded::new(domain, offsets, [0; 3]);
    let deltas: Vec<isize> = offsets
        .iter()
        .flat_map(|&d| [p.flat(d), p.flat(d.map(|v| -v))])
        .collect();
    let gldm = family == Family::Gldm;
    let width = if gldm { deltas.len() + 2 } else { 2 };
    let voxels = domain.bins.len();
    let total = p.centers();
    let per_chunk = total.div_ceil(CHUNKS).max(1);
    let chunks = total.div_ceil(per_chunk);
    let range = |chunk: usize| chunk * per_chunk..((chunk + 1) * per_chunk).min(total);
    // Chunk results are always added in chunk order, whether dense or sparse, one wave of
    // parallel chunks at a time: at most `slots` tables are live, the running sum included.
    match dense_width(ng, width, voxels, scratch) {
        Some(width) => {
            let wave = scratch.slots.saturating_sub(1).max(1);
            let mut sum: Option<DenseCounts> = None;
            for first in (0..chunks).step_by(wave) {
                let parts: Vec<DenseCounts> = (first..(first + wave).min(chunks))
                    .into_par_iter()
                    .map(|chunk| {
                        let mut dense = DenseCounts::new(ng, width, !gldm);
                        neighborhood_kernel(&p, range(chunk), &deltas, gldm, alpha, &mut dense);
                        dense
                    })
                    .collect();
                for part in parts {
                    match &mut sum {
                        Some(sum) => sum.add(&part),
                        None => sum = Some(part),
                    }
                }
            }
            sum.map_or_else(TextureMatrix::default, DenseCounts::into_matrix)
        }
        None => {
            let mut sparse = TextureMatrix::default();
            for chunk in 0..chunks {
                let mut part = TextureMatrix::default();
                neighborhood_kernel(&p, range(chunk), &deltas, gldm, alpha, &mut part);
                sparse.merge(part);
            }
            sparse
        }
    }
}

/// One forward and one backward minimum pass of `d[k] = min(d[k], d[k - step] + 1)` along an
/// axis whose neighbouring elements are `step` apart, in each block of `block` elements.
fn chamfer_pass(dist: &mut [u32], block: usize, step: usize) {
    let rows = block / step;
    let relax = |lower: &[u32], upper: &mut [u32]| {
        for (u, &l) in upper.iter_mut().zip(lower) {
            *u = (*u).min(l + 1);
        }
    };
    let group = block * if step == 1 { 64 } else { 1 };
    dist.par_chunks_mut(group).for_each(|group| {
        for b in group.chunks_mut(block) {
            if step == 1 {
                for k in 1..b.len() {
                    b[k] = b[k].min(b[k - 1] + 1);
                }
                for k in (1..b.len()).rev() {
                    b[k - 1] = b[k - 1].min(b[k] + 1);
                }
                continue;
            }
            for y in 1..rows {
                let (head, tail) = b.split_at_mut(y * step);
                relax(&head[(y - 1) * step..], &mut tail[..step]);
            }
            for y in (0..rows - 1).rev() {
                let (head, tail) = b.split_at_mut((y + 1) * step);
                relax(&tail[..step], &mut head[y * step..]);
            }
        }
    });
}

/// Distance of every voxel of the morphological mask to the nearest voxel outside it or
/// outside the image, in steps along the active axes (0 outside the mask), laid out like
/// `p.data`. The border of `p` supplies the outside of the image on every active axis.
fn boundary_distances(p: &Padded, morphology: ArrayView3<'_, bool>, active: [bool; 3]) -> Vec<u32> {
    const FAR: u32 = u32::MAX / 2;
    let mut dist = vec![0u32; p.data.len()];
    for (z, plane) in morphology.outer_iter().enumerate() {
        for (y, row) in plane.outer_iter().enumerate() {
            let start = (z + p.pad[0]) * p.strides[0] + (y + p.pad[1]) * p.strides[1] + p.pad[2];
            for (d, &inside) in dist[start..start + p.shape[2]].iter_mut().zip(row) {
                *d = if inside { FAR } else { 0 };
            }
        }
    }
    let dims = [
        p.shape[0] + 2 * p.pad[0],
        p.shape[1] + 2 * p.pad[1],
        p.shape[2] + 2 * p.pad[2],
    ];
    // Each pass is the min-plus transform along one axis; together the exact L1 distance.
    // Everything after the first-axis passes in the original scheme is the same transform,
    // since both start from the same background set.
    for axis in (0..3).filter(|&a| active[a]) {
        let (block, step) = match axis {
            0 => (dist.len(), p.strides[0]),
            1 => (p.strides[0], p.strides[1]),
            _ => (dims[2], 1),
        };
        chamfer_pass(&mut dist, block, step);
    }
    dist
}

fn zone_matrix(
    domain: Domain<'_>,
    axis: Option<usize>,
    distance_zone: bool,
) -> Result<TextureMatrix> {
    let active: [bool; 3] = std::array::from_fn(|a| {
        a >= 3 - domain.ndim && Some(a) != axis.map(|i| i + 3 - domain.ndim)
    });
    let shape = domain.shape();
    let limits: [usize; 3] =
        std::array::from_fn(|a| if active[a] { (shape[a] - 1).min(1) } else { 0 });
    // GLDZM needs the boundary distance of every voxel, on a grid padded where it is active.
    let padded = distance_zone.then(|| Padded::new(domain, &[], active.map(usize::from)));
    let depth = match &padded {
        Some(p) => {
            let depth = boundary_distances(p, domain.morphology, active);
            if p.centers.iter().any(|&index| depth[index] == 0) {
                return Err(Error::InvalidInput(
                    "GLDZM intensity mask must lie inside its morphological mask".into(),
                ));
            }
            depth
        }
        None => Vec::new(),
    };
    let runs = Runs::new(domain.bins, limits[2] > 0);
    let zone_of = runs.zones(limits);
    let mut size = vec![0u32; runs.runs.len()];
    let mut shallowest = vec![u32::MAX; if distance_zone { runs.runs.len() } else { 0 }];
    for (k, run) in runs.runs.iter().enumerate() {
        let root = zone_of[k] as usize;
        size[root] += run.len;
        if let Some(p) = &padded {
            let start = p.row_start(run.row as usize) + run.first as usize;
            let deepest = depth[start..start + run.len as usize].iter().min().unwrap();
            shallowest[root] = shallowest[root].min(*deepest);
        }
    }
    // One (level, size or depth) key per zone; equal keys are counted after sorting.
    let mut keys = Vec::new();
    let mut voxels = 0u64;
    for (k, run) in runs.runs.iter().enumerate() {
        if zone_of[k] as usize == k {
            let zone = size[k];
            let value = if distance_zone { shallowest[k] } else { zone };
            keys.push(u64::from(run.level) << 32 | u64::from(value));
            voxels += u64::from(zone);
        }
    }
    keys.sort_unstable();
    let mut cells: Vec<((usize, usize), u64)> = Vec::new();
    for key in keys {
        let cell = ((key >> 32) as usize, (key & u64::from(u32::MAX)) as usize);
        match cells.last_mut() {
            Some((last, n)) if *last == cell => *n += 1,
            _ => cells.push((cell, 1)),
        }
    }
    Ok(TextureMatrix {
        counts: cells.into_iter().collect(),
        voxels,
        differences: BTreeMap::new(),
    })
}

/// Union-find root of `x` with path halving. `part` holds the parents of the run indices
/// from `base`; the smaller index is always the root.
fn find(part: &mut [u32], base: usize, mut x: usize) -> usize {
    while part[x - base] as usize != x {
        let grandparent = part[part[x - base] as usize - base];
        part[x - base] = grandparent;
        x = grandparent as usize;
    }
    x
}

fn join(part: &mut [u32], base: usize, a: usize, b: usize) {
    let (ra, rb) = (find(part, base, a), find(part, base, b));
    if ra != rb {
        part[ra.max(rb) - base] = ra.min(rb) as u32;
    }
}

/// A maximal stretch of equal gray level along the last axis of one row.
struct Run {
    level: u32,
    row: u32,
    first: u32,
    last: u32,
    len: u32,
}

/// The runs of the grid in raster order, with the range of runs of every row.
struct Runs {
    shape: Position,
    runs: Vec<Run>,
    row_runs: Vec<u32>,
}

impl Runs {
    /// `along_last` is false when no offset steps along the last axis: each voxel is then a run.
    /// Blocks of rows are scanned in parallel and concatenated in row order.
    fn new(bins: ArrayView3<'_, i32>, along_last: bool) -> Self {
        let (nz, ny, nx) = bins.dim();
        let rows = nz * ny;
        let per_block = rows.div_ceil(CHUNKS).max(1);
        let blocks: Vec<(Vec<Run>, Vec<u32>)> = (0..rows.div_ceil(per_block))
            .into_par_iter()
            .map(|block| {
                let mut runs = Vec::new();
                let mut starts = Vec::new();
                for row in block * per_block..((block + 1) * per_block).min(rows) {
                    starts.push(runs.len() as u32);
                    let plane = bins.index_axis(Axis(0), row / ny);
                    let view = plane.index_axis(Axis(0), row % ny);
                    let owned;
                    let cells = match view.as_slice() {
                        Some(slice) => slice,
                        None => {
                            owned = view.to_vec();
                            &owned
                        }
                    };
                    let mut x = 0;
                    while x < nx {
                        let level = cells[x];
                        if level <= 0 {
                            x += 1;
                            continue;
                        }
                        let mut end = x + 1;
                        while along_last && end < nx && cells[end] == level {
                            end += 1;
                        }
                        runs.push(Run {
                            level: level as u32,
                            row: row as u32,
                            first: x as u32,
                            last: (end - 1) as u32,
                            len: (end - x) as u32,
                        });
                        x = end;
                    }
                }
                (runs, starts)
            })
            .collect();
        let mut runs = Vec::new();
        let mut row_runs = Vec::with_capacity(rows + 1);
        for (block_runs, starts) in blocks {
            let offset = runs.len() as u32;
            row_runs.extend(starts.into_iter().map(|s| s + offset));
            runs.extend(block_runs);
        }
        row_runs.push(runs.len() as u32);
        Self {
            shape: [nz, ny, nx],
            runs,
            row_runs,
        }
    }

    /// Join the runs of the rows `rows` with the same-level runs of their earlier neighbouring
    /// rows that are not before row `floor`; `part` holds the parents of the runs from `base`.
    fn join_rows(
        &self,
        rows: std::ops::Range<usize>,
        floor: usize,
        limits: [usize; 3],
        part: &mut [u32],
        base: usize,
    ) {
        let ny = self.shape[1];
        let margin = limits[2] as u32;
        for row in rows {
            let (z, y) = (row / ny, row % ny);
            let own = self.row_runs[row] as usize..self.row_runs[row + 1] as usize;
            if own.is_empty() {
                continue;
            }
            // Neighbouring rows that come earlier in raster order.
            for dz in -(limits[0] as isize)..=0 {
                for dy in -(limits[1] as isize)..=limits[1] as isize {
                    if dz == 0 && dy >= 0 {
                        break;
                    }
                    let (z2, y2) = (z as isize + dz, y as isize + dy);
                    if z2 < 0 || y2 < 0 || y2 >= ny as isize {
                        continue;
                    }
                    let other = z2 as usize * ny + y2 as usize;
                    if other < floor {
                        continue;
                    }
                    let end = self.row_runs[other + 1] as usize;
                    let mut j = self.row_runs[other] as usize;
                    for i in own.clone() {
                        let run = &self.runs[i];
                        while j < end && self.runs[j].last + margin < run.first {
                            j += 1;
                        }
                        let mut k = j;
                        while k < end && self.runs[k].first <= run.last + margin {
                            if self.runs[k].level == run.level {
                                join(part, base, i, k);
                            }
                            k += 1;
                        }
                    }
                }
            }
        }
    }

    /// The first run (smallest index) of the zone of every run. `limits` is the reach of
    /// the 26- (or 8-) neighbourhood along each axis, 0 or 1. Blocks of rows are joined
    /// in parallel among themselves; the links across block starts follow serially. Zones
    /// are a property of the grid, so the result does not depend on the schedule.
    fn zones(&self, limits: [usize; 3]) -> Vec<u32> {
        let ny = self.shape[1];
        let rows = self.shape[0] * ny;
        let mut parent: Vec<u32> = (0..self.runs.len() as u32).collect();
        let per_block = rows.div_ceil(CHUNKS).max(4 * (ny + 1));
        let blocks: Vec<usize> = (0..rows).step_by(per_block).collect();
        let first_run = |row: usize| self.row_runs[row] as usize;
        let mut parts = Vec::with_capacity(blocks.len());
        let mut rest = parent.as_mut_slice();
        for (k, &start) in blocks.iter().enumerate() {
            let end = blocks.get(k + 1).copied().unwrap_or(rows);
            let (part, tail) = rest.split_at_mut(first_run(end) - first_run(start));
            parts.push((start..end, part));
            rest = tail;
        }
        parts.into_par_iter().for_each(|(range, part)| {
            let (floor, base) = (range.start, first_run(range.start));
            self.join_rows(range, floor, limits, part, base);
        });
        for &start in blocks.iter().skip(1) {
            let end = (start + ny + 1).min(rows);
            self.join_rows(start..end, 0, limits, &mut parent, 0);
        }
        for k in 0..parent.len() {
            let root = find(&mut parent, 0, k);
            parent[k] = root as u32;
        }
        parent
    }
}

/// The matrices of one family on one domain, one per direction for GLCM and GLRLM and a
/// single one otherwise, handed to `sink` in direction order. Up to `scratch.slots`
/// directions are counted in parallel when the counts are dense; none outlives its group.
pub(super) fn matrices(
    domain: Domain<'_>,
    ng: u32,
    family: Family,
    settings: &Settings,
    axis: Option<usize>,
    scratch: Scratch,
) -> Result<Vec<TextureMatrix>> {
    let mut all = Vec::new();
    matrices_into(domain, ng, family, settings, axis, scratch, |m| all.push(m))?;
    Ok(all)
}

pub(super) fn matrices_into(
    domain: Domain<'_>,
    ng: u32,
    family: Family,
    settings: &Settings,
    axis: Option<usize>,
    scratch: Scratch,
    mut sink: impl FnMut(TextureMatrix),
) -> Result<()> {
    let shells = |d| settings.distances.contains(&d);
    let radius = *settings.distances.iter().max().unwrap();
    match family {
        Family::Glcm | Family::Glrlm => {
            let runs = family == Family::Glrlm;
            let offsets = if runs {
                directions(domain, axis, 1, |_| true, true)
            } else {
                directions(domain, axis, radius, shells, false)
            };
            let p = Padded::new(domain, &offsets, [0; 3]);
            let voxels = domain.bins.len();
            let build = |&offset: &Offset| {
                if runs {
                    run_matrix(&p, offset, ng, voxels, scratch)
                } else {
                    glcm_matrix(&p, offset, settings.symmetrical_glcm, ng, voxels, scratch)
                }
            };
            let width = if ng <= DENSE_LEVELS { scratch.slots } else { 1 };
            for group in offsets.chunks(width.max(1)) {
                if width > 1 {
                    let built: Vec<TextureMatrix> = group.par_iter().map(build).collect();
                    built.into_iter().for_each(&mut sink);
                } else {
                    group.iter().map(build).for_each(&mut sink);
                }
            }
            Ok(())
        }
        Family::Gldm | Family::Ngtdm => {
            let offsets = match settings.neighborhood_support {
                NeighborhoodSupport::ClosedRadius => {
                    directions(domain, axis, settings.neighborhood_radius, |_| true, false)
                }
                NeighborhoodSupport::Distances => directions(domain, axis, radius, shells, false),
            };
            sink(neighborhood_matrix(
                domain,
                &offsets,
                family,
                settings.gldm_alpha,
                ng,
                scratch,
            ));
            Ok(())
        }
        Family::Glszm | Family::Gldzm => {
            sink(zone_matrix(domain, axis, family == Family::Gldzm)?);
            Ok(())
        }
        _ => unreachable!("selection restricts texture families"),
    }
}

#[cfg(test)]
mod tests {
    use crate::features::matrices::*;
    use crate::imaging::preprocessing;
    use ndarray::{ArrayD, IxDyn};

    #[test]
    fn padded_directions_match_the_exported_directions() {
        let shapes: [&[usize]; 7] = [
            &[3, 4],
            &[1, 5],
            &[2, 2],
            &[2, 2, 2],
            &[4, 3, 5],
            &[1, 3, 3],
            &[5, 1, 4],
        ];
        let distance_sets: [&[u32]; 5] = [&[1], &[2], &[1, 2], &[1, 3], &[2, 3]];
        let mut compared = 0;
        for shape in shapes {
            let bins = ArrayD::<i32>::zeros(IxDyn(shape));
            let morphology = ArrayD::<bool>::from_elem(IxDyn(shape), false);
            let domain = Domain::new(bins.view(), morphology.view());
            for distances in distance_sets {
                for force2d_axis in std::iter::once(None).chain((0..shape.len()).map(Some)) {
                    let settings = Settings {
                        distances: distances.to_vec(),
                        force2d_axis,
                        ..Settings::default()
                    };
                    let radius = *distances.iter().max().unwrap();
                    let ours: Vec<Vec<i64>> = directions(
                        domain,
                        force2d_axis,
                        radius,
                        |d| distances.contains(&d),
                        false,
                    )
                    .into_iter()
                    .map(|o| o[3 - shape.len()..].iter().map(|&v| v as i64).collect())
                    .collect();
                    let exported = preprocessing::directions(shape, &settings);
                    assert_eq!(
                        ours, exported,
                        "shape {shape:?} distances {distances:?} force2d {force2d_axis:?}"
                    );
                    compared += 1;
                }
            }
        }
        assert_eq!(compared, 5 * (3 * 3 + 4 * 4));
    }

    /// Flood-fill zones and brute-force boundary distances, straight from the definitions.
    fn reference_zones(
        domain: Domain<'_>,
        axis: Option<usize>,
        distance_zone: bool,
    ) -> Vec<(usize, usize)> {
        let shape = domain.shape();
        let active: Vec<usize> = (3 - domain.ndim..3)
            .filter(|&a| Some(a) != axis.map(|i| i + 3 - domain.ndim))
            .collect();
        let offsets: Vec<Offset> = directions(domain, axis, 1, |_| true, false)
            .into_iter()
            .flat_map(|d| [d, d.map(|v| -v)])
            .collect();
        let depth_of = |pos: Position| {
            let faces = active
                .iter()
                .map(|&a| (pos[a] + 1).min(shape[a] - pos[a]))
                .min()
                .unwrap();
            let mut best = faces;
            for (q, &inside) in domain.morphology.indexed_iter() {
                let q: Position = q.into();
                let same_inactive = (0..3).all(|a| active.contains(&a) || q[a] == pos[a]);
                if !inside && same_inactive {
                    best = best.min(active.iter().map(|&a| pos[a].abs_diff(q[a])).sum());
                }
            }
            best
        };
        let mut seen = ndarray::Array3::from_elem(shape, false);
        let mut cells = Vec::new();
        for (seed, &level) in domain.bins.indexed_iter() {
            let seed: Position = seed.into();
            if level <= 0 || seen[seed] {
                continue;
            }
            seen[seed] = true;
            let mut stack = vec![seed];
            let (mut size, mut depth) = (0, usize::MAX);
            while let Some(pos) = stack.pop() {
                size += 1;
                depth = depth.min(depth_of(pos));
                for delta in &offsets {
                    let mut next = pos;
                    let inside = (0..3).all(|a| {
                        next[a] = pos[a].wrapping_add_signed(delta[a]);
                        next[a] < shape[a]
                    });
                    if inside && !seen[next] && domain.bins[next] == level {
                        seen[next] = true;
                        stack.push(next);
                    }
                }
            }
            cells.push((level as usize, if distance_zone { depth } else { size }));
        }
        cells.sort_unstable();
        cells
    }

    #[test]
    fn zone_matrices_match_flood_fill_and_brute_force_distances() {
        let mut state = 0x1234_5678_9abc_def1_u64;
        let mut next = move |bound: u64| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % bound
        };
        let shapes: [&[usize]; 8] = [
            &[5, 6, 7],
            &[1, 6, 7],
            &[4, 1, 5],
            &[4, 5, 1],
            &[2, 2, 2],
            &[6, 7],
            &[1, 5],
            &[8, 1],
        ];
        let mut compared = 0;
        for shape in shapes {
            for levels in [1u64, 2, 4] {
                let bins = ArrayD::<i32>::from_shape_fn(IxDyn(shape), |_| {
                    if next(10) < 3 {
                        0
                    } else {
                        1 + next(levels) as i32
                    }
                });
                let morphology = bins.mapv(|b| b > 0 || next(3) == 0);
                for force in std::iter::once(None).chain((0..shape.len()).map(Some)) {
                    let domain = Domain::new(bins.view(), morphology.view());
                    for distance_zone in [false, true] {
                        let got = zone_matrix(domain, force, distance_zone).unwrap().counts;
                        let mut expected = std::collections::BTreeMap::new();
                        for cell in reference_zones(domain, force, distance_zone) {
                            *expected.entry(cell).or_insert(0u64) += 1;
                        }
                        assert_eq!(
                            got, expected,
                            "shape {shape:?} levels {levels} force {force:?} dzm {distance_zone}"
                        );
                        compared += 1;
                    }
                }
            }
        }
        assert_eq!(compared, 2 * 3 * (5 * 4 + 3 * 3));
    }
}
