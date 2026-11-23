//! Convex-hull topology and area/volume measurements.
//!
//! Mirrors `mlxradiomics/features/geometry/hull.py` (70 lines at mlx-radiomics b58783c).
//! The Python module takes Qhull connectivity from SciPy; this module replaces Qhull with
//! an exact incremental quickhull on integer coordinates. Mesh vertices are half-integer
//! grid points, so doubling them gives exact `i64` coordinates and every orientation test
//! is evaluated without a tolerance, in `i64` when the doubled coordinates are below 2^19
//! (no product can overflow) and in `i128` otherwise. A diagonal spacing scale is an invertible
//! linear map, so the hull connectivity of the grid points equals that of the physical
//! points, as in the Python code.
//!
//! Output: oriented triangles (`simplices`, outward), the extreme vertices (`vertices`,
//! excluding points on a hull edge or facet), and the unique facet planes (`planes`,
//! needed by the OMBB search). Coplanar points never become strict hull vertices.
//!
//! Execution: CPU numerical work, single-threaded. Input contract: `(n, 3)` points that are
//! exact multiples of 0.5.
//!
//! Status: done.

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
use std::ops::{Add, Mul, Sub};

use crate::error::{Error, Result};
use crate::features::geometry::arithmetic::{NeumaierSum, norm3};

/// A convex hull of integer-lattice points.
#[derive(Clone, Debug)]
pub struct ConvexHull {
    /// Input points doubled to exact integers, in input order.
    pub points: Vec<[i64; 3]>,
    /// Outward-oriented hull triangles as indices into the input points.
    pub simplices: Vec<[usize; 3]>,
    /// Extreme vertices (indices into the input points), ascending.
    pub vertices: Vec<usize>,
    /// Unique outward facet normals in doubled-grid space, divided by their gcd.
    pub planes: Vec<[i64; 3]>,
}

/// Multiplicative hasher for the small integer keys of the facet maps (the maps are only
/// probed, never iterated, so the hash does not affect the result).
#[derive(Default)]
struct KeyHasher(u64);

impl Hasher for KeyHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_u64(u64::from(byte));
        }
    }

    fn write_u64(&mut self, value: u64) {
        self.0 = (self.0.rotate_left(5) ^ value).wrapping_mul(0x517c_c1b7_2722_0a95);
    }

    fn write_usize(&mut self, value: usize) {
        self.write_u64(value as u64);
    }

    fn write_i128(&mut self, value: i128) {
        self.write_u64(value as u64);
        self.write_u64((value >> 64) as u64);
    }
}

type KeyMap<K, V> = HashMap<K, V, BuildHasherDefault<KeyHasher>>;
type KeySet<K> = HashSet<K, BuildHasherDefault<KeyHasher>>;

/// Integer type of the orientation tests: `i64` when the doubled coordinates are below
/// [`NARROW_LIMIT`] in magnitude (no test can overflow), `i128` otherwise.
trait Wide:
    Copy + Ord + From<i64> + Into<i128> + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>
{
}

impl Wide for i64 {}
impl Wide for i128 {}

/// Largest doubled-coordinate magnitude (exclusive) for which every facet normal (at most
/// `2 * (2 * 2^19)^2 = 2^41` per component) times a coordinate (`2^19`), summed over three
/// axes and offset by the same bound, fits in `i64`.
const NARROW_LIMIT: i64 = 1 << 19;

type Vec3 = [i128; 3];

fn sub(a: [i64; 3], b: [i64; 3]) -> Vec3 {
    [
        i128::from(a[0] - b[0]),
        i128::from(a[1] - b[1]),
        i128::from(a[2] - b[2]),
    ]
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: Vec3, b: Vec3) -> i128 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn sub_in<T: Wide>(a: [i64; 3], b: [i64; 3]) -> [T; 3] {
    [
        T::from(a[0] - b[0]),
        T::from(a[1] - b[1]),
        T::from(a[2] - b[2]),
    ]
}

fn cross_in<T: Wide>(a: [T; 3], b: [T; 3]) -> [T; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot_in<T: Wide>(a: [T; 3], b: [T; 3]) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn degenerate() -> Error {
    Error::Execution("convex hull is degenerate for the requested shape features".into())
}

struct Face<T> {
    v: [usize; 3],
    normal: [T; 3],
    offset: T,
    outside: Vec<usize>,
    alive: bool,
}

impl<T: Wide> Face<T> {
    fn new(points: &[[i64; 3]], v: [usize; 3]) -> Self {
        let normal = cross_in(
            sub_in::<T>(points[v[1]], points[v[0]]),
            sub_in::<T>(points[v[2]], points[v[0]]),
        );
        let a = points[v[0]];
        let offset = dot_in(normal, a.map(T::from));
        Self {
            v,
            normal,
            offset,
            outside: Vec::new(),
            alive: true,
        }
    }

    fn distance(&self, p: [i64; 3]) -> T {
        dot_in(self.normal, p.map(T::from)) - self.offset
    }
}

/// Quickhull from the initial tetrahedron `tetra`; returns the outward triangles with their
/// (unnormalised) facet normals.
fn hull_faces<T: Wide>(grid: &[[i64; 3]], tetra: [usize; 4]) -> Vec<([usize; 3], [i128; 3])> {
    let count = grid.len();
    let mut faces: Vec<Face<T>> = Vec::new();
    for (triple, opposite) in [
        ([0, 1, 2], 3),
        ([0, 3, 1], 2),
        ([0, 2, 3], 1),
        ([1, 3, 2], 0),
    ] {
        let mut v = [tetra[triple[0]], tetra[triple[1]], tetra[triple[2]]];
        if Face::<T>::new(grid, v).distance(grid[tetra[opposite]]) > T::from(0) {
            v.swap(1, 2);
        }
        faces.push(Face::new(grid, v));
    }
    let mut edges: KeyMap<(usize, usize), usize> = KeyMap::default();
    for (index, face) in faces.iter().enumerate() {
        for k in 0..3 {
            edges.insert((face.v[k], face.v[(k + 1) % 3]), index);
        }
    }
    let in_tetra = |i: usize| tetra.contains(&i);
    for point in (0..count).filter(|&i| !in_tetra(i)) {
        if let Some(face) = faces
            .iter_mut()
            .find(|f| f.distance(grid[point]) > T::from(0))
        {
            face.outside.push(point);
        }
    }
    let mut pending: Vec<usize> = (0..faces.len()).collect();
    while let Some(start) = pending.pop() {
        if !faces[start].alive || faces[start].outside.is_empty() {
            continue;
        }
        let eye = *faces[start]
            .outside
            .iter()
            .max_by(|&&a, &&b| {
                faces[start]
                    .distance(grid[a])
                    .cmp(&faces[start].distance(grid[b]))
            })
            .expect("outside is non-empty");
        // Visible faces: connected strictly-positive faces from `start`.
        let mut visible = vec![start];
        let mut seen = KeySet::from_iter([start]);
        let mut cursor = 0;
        while cursor < visible.len() {
            let face = visible[cursor];
            cursor += 1;
            for k in 0..3 {
                let (a, b) = (faces[face].v[k], faces[face].v[(k + 1) % 3]);
                let neighbour = edges[&(b, a)];
                if seen.contains(&neighbour) || !faces[neighbour].alive {
                    continue;
                }
                if faces[neighbour].distance(grid[eye]) > T::from(0) {
                    seen.insert(neighbour);
                    visible.push(neighbour);
                }
            }
        }
        let mut horizon: Vec<(usize, usize)> = Vec::new();
        for &face in &visible {
            for k in 0..3 {
                let (a, b) = (faces[face].v[k], faces[face].v[(k + 1) % 3]);
                if !seen.contains(&edges[&(b, a)]) {
                    horizon.push((a, b));
                }
            }
        }
        let mut orphans: Vec<usize> = Vec::new();
        for &face in &visible {
            for k in 0..3 {
                edges.remove(&(faces[face].v[k], faces[face].v[(k + 1) % 3]));
            }
            faces[face].alive = false;
            orphans.append(&mut faces[face].outside);
        }
        let first_new = faces.len();
        for &(a, b) in &horizon {
            let index = faces.len();
            faces.push(Face::new(grid, [a, b, eye]));
            for k in 0..3 {
                let v = faces[index].v;
                edges.insert((v[k], v[(k + 1) % 3]), index);
            }
        }
        for point in orphans.into_iter().filter(|&p| p != eye) {
            if let Some(face) = faces[first_new..]
                .iter_mut()
                .find(|f| f.distance(grid[point]) > T::from(0))
            {
                face.outside.push(point);
            }
        }
        for index in first_new..faces.len() {
            if !faces[index].outside.is_empty() {
                pending.push(index);
            }
        }
    }
    faces
        .iter()
        .filter(|f| f.alive)
        .map(|f| (f.v, f.normal.map(Into::into)))
        .collect()
}

/// Build the convex hull of points that are exact multiples of 0.5.
///
/// Replaces `scipy_hull` of the Python module.
///
/// # Errors
///
/// [`Error::Execution`] if the point set is planar or collinear (a degenerate hull), or a
/// coordinate is not an exact multiple of 0.5.
pub fn convex_hull(points: &[[f64; 3]]) -> Result<ConvexHull> {
    let mut grid = Vec::with_capacity(points.len());
    for point in points {
        let mut doubled = [0i64; 3];
        for axis in 0..3 {
            let scaled = point[axis] * 2.0;
            if !scaled.is_finite() || scaled != scaled.round() || scaled.abs() > 1e15 {
                return Err(Error::Execution(
                    "convex hull requires half-grid mesh coordinates".into(),
                ));
            }
            doubled[axis] = scaled as i64;
        }
        grid.push(doubled);
    }
    let count = grid.len();
    if count < 4 {
        return Err(degenerate());
    }
    // Initial tetrahedron from exact extremes.
    let p0 = (0..count).min_by_key(|&i| grid[i]).expect("non-empty");
    let sq = |i: usize, j: usize| {
        let d = sub(grid[i], grid[j]);
        dot(d, d)
    };
    let p1 = (0..count).max_by_key(|&i| sq(i, p0)).expect("non-empty");
    if sq(p1, p0) == 0 {
        return Err(degenerate());
    }
    let axis = sub(grid[p1], grid[p0]);
    let area = |i: usize| {
        let c = cross(axis, sub(grid[i], grid[p0]));
        dot(c, c)
    };
    let p2 = (0..count).max_by_key(|&i| area(i)).expect("non-empty");
    if area(p2) == 0 {
        return Err(degenerate());
    }
    let normal = cross(axis, sub(grid[p2], grid[p0]));
    let volume = |i: usize| dot(normal, sub(grid[i], grid[p0])).abs();
    let p3 = (0..count).max_by_key(|&i| volume(i)).expect("non-empty");
    if volume(p3) == 0 {
        return Err(degenerate());
    }
    let tetra = [p0, p1, p2, p3];
    let faces = if grid.iter().flatten().all(|c| c.abs() < NARROW_LIMIT) {
        hull_faces::<i64>(&grid, tetra)
    } else {
        hull_faces::<i128>(&grid, tetra)
    };
    let simplices: Vec<[usize; 3]> = faces.iter().map(|f| f.0).collect();
    // Unique planes and extreme vertices.
    let mut plane_ids: KeyMap<Vec3, usize> = KeyMap::default();
    let mut planes: Vec<[i64; 3]> = Vec::new();
    let mut incident: Vec<Vec<usize>> = vec![Vec::new(); count];
    for (face_vertices, n) in &faces {
        let g = gcd(gcd(n[0], n[1]), n[2]);
        if g == 0 {
            return Err(Error::Execution(
                "convex hull contains a zero-area face".into(),
            ));
        }
        let key = [n[0] / g, n[1] / g, n[2] / g];
        let id = *plane_ids.entry(key).or_insert_with(|| {
            planes.push([key[0] as i64, key[1] as i64, key[2] as i64]);
            planes.len() - 1
        });
        for &vertex in face_vertices {
            if !incident[vertex].contains(&id) {
                incident[vertex].push(id);
            }
        }
    }
    let independent = |ids: &[usize]| -> bool {
        let n = |id: usize| -> Vec3 {
            [
                i128::from(planes[id][0]),
                i128::from(planes[id][1]),
                i128::from(planes[id][2]),
            ]
        };
        for (i, &a) in ids.iter().enumerate() {
            for (j, &b) in ids.iter().enumerate().skip(i + 1) {
                let c = cross(n(a), n(b));
                if c == [0; 3] {
                    continue;
                }
                if ids.iter().skip(j + 1).any(|&d| dot(c, n(d)) != 0) {
                    return true;
                }
            }
        }
        false
    };
    let vertices: Vec<usize> = (0..count)
        .filter(|&i| incident[i].len() >= 3 && independent(&incident[i]))
        .collect();
    Ok(ConvexHull {
        points: grid,
        simplices,
        vertices,
        planes,
    })
}

/// Convex-hull surface area (physical) and volume in grid units.
///
/// `spacing` is the voxel spacing per axis. The area is the sum of the physical triangle
/// areas; the grid volume is the exact sum of signed tetrahedra from one hull vertex, which
/// does not depend on facet triangulation. Mirrors `hull_measures`, which returns the area
/// normalised by the boundary exponent and the grid volume; here both are plain float64.
#[must_use]
pub fn hull_measures(hull: &ConvexHull, spacing: &[f64]) -> (f64, f64) {
    let mut area = NeumaierSum::new();
    let mut volume: i128 = 0;
    let origin = hull.points[hull.simplices[0][0]];
    for triangle in &hull.simplices {
        let [a, b, c] = triangle.map(|i| hull.points[i]);
        let edge = |p: [i64; 3], q: [i64; 3]| {
            [
                (p[0] - q[0]) as f64 * 0.5 * spacing[0],
                (p[1] - q[1]) as f64 * 0.5 * spacing[1],
                (p[2] - q[2]) as f64 * 0.5 * spacing[2],
            ]
        };
        let (ea, eb) = (edge(b, a), edge(c, a));
        let triangle_cross = [
            ea[1] * eb[2] - ea[2] * eb[1],
            ea[2] * eb[0] - ea[0] * eb[2],
            ea[0] * eb[1] - ea[1] * eb[0],
        ];
        area.add(0.5 * norm3(triangle_cross));
        volume += dot(sub(a, origin), cross(sub(b, origin), sub(c, origin)));
    }
    // Doubled coordinates scale a determinant by 8; the tetrahedron factor is 1/6.
    (area.value(), volume as f64 / 48.0)
}

#[cfg(test)]
mod tests {
    use crate::features::geometry::hull::*;

    #[test]
    fn cube_hull_measures() {
        let mut points = Vec::new();
        for x in [0.0, 1.0] {
            for y in [0.0, 1.0] {
                for z in [0.0, 1.0] {
                    points.push([x, y, z]);
                }
            }
        }
        points.push([0.5, 0.5, 0.5]);
        points.push([0.5, 0.0, 0.0]);
        points.push([0.5, 1.0, 0.5]);
        let hull = convex_hull(&points).unwrap();
        assert_eq!(hull.vertices.len(), 8);
        assert_eq!(hull.planes.len(), 6);
        let (area, volume) = hull_measures(&hull, &[1.0, 1.0, 1.0]);
        assert!((area - 6.0).abs() < 1e-12);
        assert!((volume - 1.0).abs() < 1e-12);
    }

    #[test]
    fn planar_points_are_degenerate() {
        let points = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ];
        assert!(convex_hull(&points).is_err());
    }
}
