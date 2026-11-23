//! Marching mesh construction and reusable geometry for shape features.
//!
//! Mirrors `mlxradiomics/features/geometry/mesh.py` (316 lines at mlx-radiomics b58783c).
//! Marching cubes (3-D) and marching squares (2-D) use the PyRadiomics-derived tables in
//! `mesh_tables.json` (embedded with `include_str!`; BSD-3 notices in
//! `THIRD_PARTY_NOTICES.md`).
//!
//! Execution: CPU numerical work in float64, single-threaded. Input contract: a boolean
//! mask with 2 or 3 axes in (z, y, x) order and a positive spacing per axis. There is no
//! GPU path; the float32 chunking, binary exponent scaling and Metal compaction kernel of
//! the Python module are not needed. Cells are visited in C order, but only along the x span
//! of the padded rows that can hold a set corner, so empty space costs nothing.
//!
//! Status: done.

use std::sync::OnceLock;

use ndarray::ArrayD;

use crate::error::{Error, Result};
use crate::features::geometry::arithmetic::{NeumaierSum, cross_product, norm3, validate_spacing};

/// Largest padded cropped extent per axis, as in mlx-radiomics (exact float32 half-grid
/// vertices there; kept as the same refusal here).
pub const MAX_MESH_EXTENT: usize = 1 << 22;

/// Lookup tables from `mesh_tables.json`, named after the tables in `cshape.c`.
#[derive(Clone, Debug)]
pub struct MeshTables {
    /// Cell-corner offsets (`gridAngles`).
    pub corners: Vec<Vec<usize>>,
    /// Full triangle table (`triTable`, 256 codes) or segment table (`lineTable2D`).
    pub facets: Vec<Vec<i32>>,
    /// Edge-midpoint offsets (`vertList`).
    pub vertices: Vec<Vec<f64>>,
}

fn load_tables() -> &'static [MeshTables; 2] {
    static TABLES: OnceLock<[MeshTables; 2]> = OnceLock::new();
    TABLES.get_or_init(|| {
        let data: serde_json::Value = serde_json::from_str(include_str!("mesh_tables.json"))
            .expect("mesh_tables.json is valid JSON");
        let table = |key: &str| -> Vec<Vec<f64>> {
            data[key]
                .as_array()
                .unwrap_or_else(|| panic!("mesh_tables.json lacks {key}"))
                .iter()
                .map(|row| {
                    row.as_array()
                        .expect("table row")
                        .iter()
                        .map(|value| value.as_f64().expect("table value"))
                        .collect()
                })
                .collect()
        };
        let build = |suffix: &str, facets: &str| MeshTables {
            corners: table(&format!("gridAngles{suffix}"))
                .into_iter()
                .map(|row| row.into_iter().map(|value| value as usize).collect())
                .collect(),
            facets: table(facets)
                .into_iter()
                .map(|row| row.into_iter().map(|value| value as i32).collect())
                .collect(),
            vertices: table(&format!("vertList{suffix}")),
        };
        [build("2D", "lineTable2D"), build("", "triTable")]
    })
}

/// The marching tables for a mask with `ndim` axes (2 or 3).
#[must_use]
pub fn tables(ndim: usize) -> &'static MeshTables {
    &load_tables()[usize::from(ndim == 3)]
}

/// Row-major `(count, ndim)` integer coordinates of the set voxels of a mask.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coordinates {
    ndim: usize,
    data: Vec<i64>,
}

impl Coordinates {
    /// Coordinates of every `true` voxel, in C order (`np.argwhere`).
    #[must_use]
    pub fn from_mask(mask: &ArrayD<bool>) -> Self {
        let ndim = mask.ndim();
        let shape = mask.shape();
        let owned;
        let flat: &[bool] = match mask.as_slice() {
            Some(slice) => slice,
            None => {
                owned = mask.iter().copied().collect::<Vec<bool>>();
                &owned
            }
        };
        let count = flat.iter().filter(|&&value| value).count();
        let mut data = Vec::with_capacity(count * ndim);
        let width = shape.last().copied().unwrap_or(1).max(1);
        // Index of the current row over the leading axes, advanced like an odometer.
        let mut lead = vec![0i64; ndim - 1];
        for row in flat.chunks_exact(width) {
            let mut x = 0;
            while let Some(offset) = row[x..].iter().position(|&value| value) {
                x += offset;
                data.extend_from_slice(&lead);
                data.push(x as i64);
                x += 1;
            }
            for axis in (0..ndim - 1).rev() {
                lead[axis] += 1;
                if (lead[axis] as usize) < shape[axis] {
                    break;
                }
                lead[axis] = 0;
            }
        }
        Self { ndim, data }
    }

    /// Number of axes.
    #[must_use]
    pub fn ndim(&self) -> usize {
        self.ndim
    }

    /// Number of voxels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len() / self.ndim
    }

    /// Whether there are no voxels.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Coordinates of voxel `index`.
    #[must_use]
    pub fn row(&self, index: usize) -> &[i64] {
        &self.data[index * self.ndim..(index + 1) * self.ndim]
    }

    /// Iterate over voxel coordinates.
    pub fn rows(&self) -> impl Iterator<Item = &[i64]> {
        self.data.chunks_exact(self.ndim)
    }
}

/// Mesh measures plus the grid-space boundary vertices.
///
/// `boundary` (perimeter or area) and `enclosed` (area or volume) are physical. Vertices
/// stay in grid coordinates of the cropped frame (half-integers), flattened with `ndim`
/// values per vertex; the diameter and hull code applies spacing per component.
#[derive(Clone, Debug)]
pub struct MeshGeometry {
    /// Boundary measure: 2-D perimeter or 3-D surface area.
    pub boundary: f64,
    /// Enclosed measure: 2-D area or 3-D volume.
    pub enclosed: f64,
    /// Enclosed measure in grid units times 6 (3-D) or 2 (2-D): `|sum of signed terms|`.
    pub grid_enclosed: f64,
    /// Boundary vertices, `ndim` values each (empty unless requested).
    pub vertices: Vec<f64>,
}

impl MeshGeometry {}

/// Build the mesh of a boolean mask and return its measures and vertices.
///
/// The mask is cropped to the ROI bounding box and padded by one voxel. The 3-D volume
/// integral is evaluated in that cropped frame; the closed surface makes it independent of
/// the ROI's position in the source array. `coordinates` optionally supplies the
/// precomputed [`Coordinates::from_mask`] rows.
///
/// # Errors
///
/// [`Error::InvalidInput`] if the spacing is invalid, the mask is empty, the mesh has no
/// boundary vertices, or the padded cropped extent exceeds [`MAX_MESH_EXTENT`] per axis.
pub fn mesh_geometry(
    mask: &ArrayD<bool>,
    spacing: &[f64],
    include_vertices: bool,
    coordinates: Option<&Coordinates>,
) -> Result<MeshGeometry> {
    validate_spacing(spacing)?;
    let ndim = mask.ndim();
    if spacing.len() != ndim {
        return Err(Error::InvalidInput(
            "spacing must contain one value per mask axis".into(),
        ));
    }
    let owned;
    let roi = match coordinates {
        Some(rows) => rows,
        None => {
            owned = Coordinates::from_mask(mask);
            &owned
        }
    };
    if roi.is_empty() {
        return Err(Error::InvalidInput("shape mask is empty".into()));
    }
    let mut lower = vec![i64::MAX; ndim];
    let mut upper = vec![i64::MIN; ndim];
    for row in roi.rows() {
        for axis in 0..ndim {
            lower[axis] = lower[axis].min(row[axis]);
            upper[axis] = upper[axis].max(row[axis]);
        }
    }
    let extent: Vec<usize> = (0..ndim)
        .map(|axis| (upper[axis] - lower[axis] + 1) as usize)
        .collect();
    if extent.iter().max().copied().unwrap_or(0) + 2 > MAX_MESH_EXTENT {
        return Err(Error::InvalidInput(format!(
            "shape mesh requires a cropped ROI extent below {MAX_MESH_EXTENT} voxels per axis \
             for exact float32 half-grid vertices"
        )));
    }
    // Padded cropped occupancy, row-major, and the occupied x span of every padded row.
    let padded_shape: Vec<usize> = extent.iter().map(|size| size + 2).collect();
    let mut strides = vec![1usize; ndim];
    for axis in (0..ndim.saturating_sub(1)).rev() {
        strides[axis] = strides[axis + 1] * padded_shape[axis + 1];
    }
    let width = padded_shape[ndim - 1];
    let mut padded = vec![false; padded_shape.iter().product()];
    let mut spans: Vec<(usize, usize)> = vec![(usize::MAX, 0); padded.len() / width];
    for row in roi.rows() {
        let offset: usize = (0..ndim)
            .map(|axis| (row[axis] - lower[axis]) as usize + 1)
            .zip(&strides)
            .map(|(position, stride)| position * stride)
            .sum();
        padded[offset] = true;
        let span = &mut spans[offset / width];
        let x = offset % width;
        span.0 = span.0.min(x);
        span.1 = span.1.max(x);
    }
    let tables = tables(ndim);
    let corner_offsets: Vec<usize> = tables
        .corners
        .iter()
        .map(|corner| corner.iter().zip(&strides).map(|(c, s)| c * s).sum())
        .collect();
    let full_code = (1usize << corner_offsets.len()) - 1;
    // Cells: every origin in the padded array except the last index along each axis.
    let cells: Vec<usize> = padded_shape.iter().map(|size| size - 1).collect();
    let (edge_ends, edges): (&[(usize, usize)], &[usize]) = if ndim == 3 {
        (&[(7, 6), (7, 4), (7, 3)], &[6, 7, 11])
    } else {
        (&[(3, 0), (3, 2)], &[3, 2])
    };
    let mut boundary = NeumaierSum::new();
    let mut measure = NeumaierSum::new();
    let mut vertices: Vec<f64> = Vec::new();
    // Cell rows in C order (all leading axes, then x). A cell has a boundary only when a
    // corner voxel is set, so each row is scanned over the span of its four (2-D: two)
    // contributing padded rows, widened by one cell on the low side.
    let lead_axes = ndim - 1;
    let mut lead = [0usize; 2];
    let lead_rows: usize = cells[..lead_axes].iter().product();
    let row_strides: Vec<usize> = (0..lead_axes).map(|axis| strides[axis] / width).collect();
    for _ in 0..lead_rows {
        let (mut first, mut last) = (usize::MAX, 0usize);
        for delta in 0..(1usize << lead_axes) {
            let row: usize = (0..lead_axes)
                .map(|axis| (lead[axis] + ((delta >> axis) & 1)) * row_strides[axis])
                .sum();
            let (low, high) = spans[row];
            if low <= high {
                first = first.min(low);
                last = last.max(high);
            }
        }
        if first <= last {
            let row_base: usize = (0..lead_axes).map(|axis| lead[axis] * strides[axis]).sum();
            for x in first.saturating_sub(1)..=last.min(cells[ndim - 1] - 1) {
                let base = row_base + x;
                let mut code = 0usize;
                for (bit, offset) in corner_offsets.iter().enumerate() {
                    if padded[base + offset] {
                        code |= 1 << bit;
                    }
                }
                if code == 0 || code == full_code {
                    continue;
                }
                let facets = &tables.facets[code];
                let mut origin = [0.0f64; 3];
                for axis in 0..lead_axes {
                    origin[axis] = lead[axis] as f64;
                }
                origin[lead_axes] = x as f64;
                let point = |edge: i32| -> [f64; 3] {
                    let offsets = &tables.vertices[edge as usize];
                    let mut p = [0.0f64; 3];
                    for axis in 0..ndim {
                        p[axis] = origin[axis] + offsets[axis];
                    }
                    p
                };
                if ndim == 3 {
                    for triangle in facets[..15].chunks_exact(3) {
                        if triangle[0] < 0 {
                            continue;
                        }
                        let ia = point(triangle[0]);
                        let ib = point(triangle[1]);
                        let ic = point(triangle[2]);
                        let a = [ia[0] - ic[0], ia[1] - ic[1], ia[2] - ic[2]];
                        let b = [ib[0] - ic[0], ib[1] - ic[1], ib[2] - ic[2]];
                        let c = cross_product(a, b);
                        measure.add(c[0] * ic[0] + c[1] * ic[1] + c[2] * ic[2]);
                        let physical = cross_product(
                            [a[0] * spacing[0], a[1] * spacing[1], a[2] * spacing[2]],
                            [b[0] * spacing[0], b[1] * spacing[1], b[2] * spacing[2]],
                        );
                        boundary.add(0.5 * norm3(physical));
                    }
                } else {
                    for segment in facets[..4].chunks_exact(2) {
                        if segment[0] < 0 {
                            continue;
                        }
                        let ia = point(segment[0]);
                        let ib = point(segment[1]);
                        let delta = [ib[0] - ia[0], ib[1] - ia[1]];
                        measure.add(ia[0] * delta[1] - ia[1] * delta[0]);
                        let (dx, dy) = (delta[0] * spacing[0], delta[1] * spacing[1]);
                        boundary.add(dx.hypot(dy));
                    }
                }
                if include_vertices {
                    for (&(start, end), &edge) in edge_ends.iter().zip(edges) {
                        if (code >> start) & 1 != (code >> end) & 1 {
                            for axis in 0..ndim {
                                vertices.push(origin[axis] + tables.vertices[edge][axis]);
                            }
                        }
                    }
                }
            }
        }
        for axis in (0..lead_axes).rev() {
            lead[axis] += 1;
            if lead[axis] < cells[axis] {
                break;
            }
            lead[axis] = 0;
        }
    }
    if include_vertices && vertices.is_empty() {
        return Err(Error::InvalidInput(
            "mask has no mesh boundary vertices".into(),
        ));
    }
    let grid_enclosed = measure.value().abs();
    let divisor = if ndim == 3 { 6.0 } else { 2.0 };
    let enclosed = grid_enclosed * spacing.iter().product::<f64>() / divisor;
    Ok(MeshGeometry {
        boundary: boundary.value(),
        enclosed,
        grid_enclosed,
        vertices,
    })
}

/// Lazily share one mask's coordinates and mesh within an extraction call.
///
/// The bound mask and spacing must remain unchanged while the context is in use. Nothing
/// is retained across public calls. A scalar-only consumer can omit boundary-vertex
/// collection; a later vertex consumer upgrades the cached geometry.
#[derive(Debug)]
pub struct MeshGeometryContext<'a> {
    mask: &'a ArrayD<bool>,
    spacing: Vec<f64>,
    coordinates: Option<Coordinates>,
    geometry: Option<MeshGeometry>,
    has_vertices: bool,
}

impl<'a> MeshGeometryContext<'a> {
    /// Bind a mask and its per-axis spacing.
    #[must_use]
    pub fn new(mask: &'a ArrayD<bool>, spacing: &[f64]) -> Self {
        Self {
            mask,
            spacing: spacing.to_vec(),
            coordinates: None,
            geometry: None,
            has_vertices: false,
        }
    }

    /// The `(n, ndim)` mask coordinates, computed once.
    pub fn coordinates(&mut self) -> &Coordinates {
        self.coordinates
            .get_or_insert_with(|| Coordinates::from_mask(self.mask))
    }

    /// The cached coordinates and geometry, without building anything (call
    /// [`Self::coordinates`] and [`Self::get`] first).
    #[must_use]
    pub fn cached(&self) -> (Option<&Coordinates>, Option<&MeshGeometry>) {
        (self.coordinates.as_ref(), self.geometry.as_ref())
    }

    /// The mask's mesh geometry, built on first use. A cached geometry built without
    /// vertices is rebuilt when `include_vertices` is requested.
    ///
    /// # Errors
    ///
    /// As [`mesh_geometry`].
    pub fn get(&mut self, include_vertices: bool) -> Result<&MeshGeometry> {
        if self.geometry.is_none() || (include_vertices && !self.has_vertices) {
            self.coordinates();
            let geometry = mesh_geometry(
                self.mask,
                &self.spacing,
                include_vertices,
                self.coordinates.as_ref(),
            )?;
            self.geometry = Some(geometry);
            self.has_vertices = include_vertices;
        }
        Ok(self.geometry.as_ref().expect("geometry was just built"))
    }
}
