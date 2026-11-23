//! Symmetric co-occurrences in small windows. Integer coordinates, cell counts and
//! the numerator of ClusterShade stay exact. Larger and explicitly merged matrices
//! use the general ROI implementation.

use crate::config::settings::DirectionalAggregation;
use crate::voxel::spectrum::spectral_radius;
use crate::voxel::window::{directional_pairs, entropy_table, indices};
use crate::{Error, Result, Settings};

const MAX_LEVEL: usize = 128;
// These indices are private to the compact statistics array. Feature ordering
// at the public entry point is determined entirely by the requested slots.
macro_rules! statistics {
    ($($name:ident),+ $(,)?) => {
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Statistic { $($name),+ }
        impl Statistic {
            fn parse(name:&str)->Option<Self> {
                match name { $(stringify!($name)=>Some(Self::$name),)+ _=>None }
            }
        }
    }
}
statistics! {
    Autocorrelation, Contrast, Correlation, DifferenceAverage, JointEnergy, JointEntropy, Id, Idm, JointAverage, ClusterProminence, ClusterShade, ClusterTendency, DifferenceEntropy, DifferenceVariance, Imc1, Imc2, Idmn, Idn, InverseVariance, MaximumProbability, SumAverage, SumEntropy, SumSquares, MCC, Dissimilarity, SumVariance
}

pub(super) struct WindowGlcm {
    pairs: Vec<Vec<(u8, u8)>>,
    selection: Vec<(usize, Statistic)>,
    levels: usize,
    entropy: Vec<Vec<f64>>,
    inverse: Vec<[f64; 5]>,
    information: bool,
    mcc: bool,
}

impl WindowGlcm {
    pub fn new(shape: &[usize], levels: u32, settings: &Settings, slots: &[usize]) -> Option<Self> {
        let length = shape.iter().product::<usize>();
        if slots.is_empty()
            || length > 125
            || !(1..=MAX_LEVEL as u32).contains(&levels)
            || !settings.symmetrical_glcm
            || settings.force2d_axis.is_some()
            || settings.slice_axis >= shape.len()
            || settings.neighborhood_radius == 0
            || settings.distances.is_empty()
            || settings.distances.contains(&0)
            || settings
                .distances
                .iter()
                .enumerate()
                .any(|(i, d)| settings.distances[..i].contains(d))
            || !(settings.glcm_aggregation == DirectionalAggregation::Native
                || (shape.len() == 3
                    && settings.glcm_aggregation == DirectionalAggregation::Average3D)
                || (shape.len() == 2
                    && settings.glcm_aggregation == DirectionalAggregation::Average2D))
        {
            return None;
        }
        let pairs = directional_pairs(shape, settings)
            .into_iter()
            .map(|(_, pairs)| pairs)
            .collect();
        let selection = slots
            .iter()
            .map(|&slot| {
                let name = settings.features[slot].strip_prefix("glcm_")?;
                Some((slot, Statistic::parse(name)?))
            })
            .collect::<Option<Vec<_>>>()?;
        let entropy = entropy_table(2 * length);
        let inverse = (0..=levels)
            .map(|d| {
                let x = f64::from(d);
                let y = x / f64::from(levels);
                [
                    1. / (1. + x),
                    1. / (1. + x * x),
                    1. / (1. + y * y),
                    1. / (1. + y),
                    if d == 0 { 0. } else { 1. / (x * x) },
                ]
            })
            .collect();
        let information = selection
            .iter()
            .any(|&(_, i)| matches!(i, Statistic::Imc1 | Statistic::Imc2));
        let mcc = selection.iter().any(|&(_, i)| i == Statistic::MCC);
        Some(Self {
            pairs,
            selection,
            levels: levels as usize,
            entropy,
            inverse,
            information,
            mcc,
        })
    }

    pub fn evaluate(&self, bins: &[i32], scratch: &mut PairCounts, out: &mut [f64]) -> Result<()> {
        let mut sum = [0.; 26];
        let mut correction = [0.; 26];
        let mut count = [0u16; 26];
        if bins.iter().any(|&b| b < 0 || b as usize > self.levels) {
            return Err(Error::InvalidInput(
                "bins must be 1..Ng inside sampling_mask and 0 outside".into(),
            ));
        }
        for pairs in &self.pairs {
            for &(p, q) in pairs {
                let (a, b) = (bins[p as usize] as usize, bins[q as usize] as usize);
                if a != 0 && b != 0 {
                    scratch.add(a, b);
                }
            }
            if scratch.occupied.is_empty() {
                continue;
            }
            let values = self.statistics(scratch);
            for &(_, i) in &self.selection {
                let x = values[i as usize];
                if i == Statistic::MCC && x.is_nan() {
                    continue;
                }
                let i = i as usize;
                let next = sum[i] + x;
                let back = next - sum[i];
                correction[i] += (sum[i] - (next - back)) + (x - back);
                sum[i] = next;
                count[i] += 1;
            }
            scratch.clear();
        }
        for &(slot, i) in &self.selection {
            let i = i as usize;
            out[slot] = (sum[i] + correction[i]) / f64::from(count[i]);
        }
        Ok(())
    }

    fn statistics(&self, pairs: &mut PairCounts) -> [f64; 26] {
        let mut joint = [0.; 26];
        let mut rows = [0u16; MAX_LEVEL + 1];
        let mut differences = [0u16; MAX_LEVEL + 1];
        let mut sums = [0u16; 2 * MAX_LEVEL + 1];
        let mut levels = 0u128;
        let mut diff_keys = OccupiedIndices::default();
        let mut sum_keys = OccupiedIndices::default();
        let n = pairs.total * 2;
        let total = f64::from(n);
        let entropy = &self.entropy[n as usize];
        let (mut sx, mut xx, mut xy, mut sd, mut dd) = (0i64, 0i64, 0i64, 0i64, 0i64);
        let mut energy = 0u32;
        let mut maximum = 0;
        for &index in &pairs.occupied {
            let (a, b) = pairs.decode(index);
            let c = pairs.cells[index as usize];
            let w = i64::from(c) * 2;
            rows[a] += c;
            rows[b] += c;
            levels |= bit(a) | bit(b);
            let d = b - a;
            let s = a + b;
            differences[d] += 2 * c;
            sums[s] += 2 * c;
            diff_keys.insert(d);
            sum_keys.insert(s);
            sx += i64::from(c) * s as i64;
            xx += i64::from(c) * (a * a + b * b) as i64;
            xy += w * (a * b) as i64;
            sd += w * d as i64;
            dd += w * (d * d) as i64;
            let mult = if a == b { 1 } else { 2 };
            let actual = if a == b { 2 * c } else { c };
            maximum = maximum.max(actual);
            energy += mult * u32::from(actual).pow(2);
            joint[Statistic::JointEntropy as usize] += mult as f64 * entropy[actual as usize];
        }
        let ni = i64::from(n);
        let n2 = total * total;
        let vx = (ni * xx - sx * sx) as f64 / n2;
        joint[Statistic::Autocorrelation as usize] = xy as f64 / total;
        joint[Statistic::Contrast as usize] = dd as f64 / total;
        joint[Statistic::Correlation as usize] = if vx == 0. {
            1.
        } else {
            (ni * xy - sx * sx) as f64 / n2 / (vx + f64::EPSILON)
        };
        joint[Statistic::DifferenceAverage as usize] = sd as f64 / total;
        joint[Statistic::JointEnergy as usize] = f64::from(energy) / n2;
        joint[Statistic::JointAverage as usize] = sx as f64 / total;
        joint[Statistic::DifferenceVariance as usize] = (ni * dd - sd * sd) as f64 / n2;
        joint[Statistic::MaximumProbability as usize] = f64::from(maximum) / total;
        joint[Statistic::SumAverage as usize] = 2. * joint[Statistic::JointAverage as usize];
        joint[Statistic::SumSquares as usize] = vx;
        joint[Statistic::Dissimilarity as usize] = joint[Statistic::DifferenceAverage as usize];
        let mut inverses = [0.; 5];
        for d in diff_keys.indices() {
            let c = differences[d];
            for (acc, weight) in inverses.iter_mut().zip(self.inverse[d]) {
                *acc += f64::from(c) * weight;
            }
            joint[Statistic::DifferenceEntropy as usize] += entropy[c as usize];
        }
        for (i, v) in [
            Statistic::Id,
            Statistic::Idm,
            Statistic::Idmn,
            Statistic::Idn,
            Statistic::InverseVariance,
        ]
        .into_iter()
        .zip(inverses)
        {
            joint[i as usize] = v / total;
        }
        // At most 250 observations and gray levels <=128 bound this centered
        // integer numerator by 250 * (250 * 254)^3, within signed 64 bits.
        let mut cube = 0i64;
        for s in sum_keys.indices() {
            let c = sums[s];
            let offset = ni * s as i64 - 2 * sx;
            cube += i64::from(c) * offset.pow(3);
            let centered = offset as f64 / total;
            joint[Statistic::ClusterProminence as usize] += f64::from(c) * centered.powi(4);
            joint[Statistic::SumEntropy as usize] += entropy[c as usize];
        }
        joint[Statistic::ClusterProminence as usize] /= total;
        joint[Statistic::ClusterShade as usize] = cube as f64 / n2.powi(2);
        joint[Statistic::ClusterTendency as usize] = (2 * ni * (xx + xy) - 4 * sx * sx) as f64 / n2;
        joint[Statistic::SumVariance as usize] = joint[Statistic::ClusterTendency as usize];
        if self.information {
            let marginal_entropy: f64 = bits(levels).map(|i| entropy[rows[i] as usize]).sum();
            // Near independence the difference of entropies loses significant digits.
            let mut info = 2. * marginal_entropy - joint[Statistic::JointEntropy as usize];
            if info < 1e-4 {
                info = pairs.information(&rows, n);
            }
            joint[Statistic::Imc1 as usize] = if marginal_entropy == 0. {
                0.
            } else {
                -info / marginal_entropy
            };
            joint[Statistic::Imc2 as usize] = (-(-2. * info).exp_m1()).sqrt();
        }
        if self.mcc {
            joint[Statistic::MCC as usize] = pairs.mcc(&rows, levels, n);
        }
        joint
    }
}

#[derive(Default)]
struct OccupiedIndices([u64; 5]);
impl OccupiedIndices {
    fn insert(&mut self, i: usize) {
        self.0[i / 64] |= 1 << (i % 64);
    }
    fn indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(word, &mask)| indices(mask as u128).map(move |i| 64 * word + i))
    }
}

fn bit(level: usize) -> u128 {
    1 << (level - 1)
}
fn bits(mask: u128) -> impl Iterator<Item = usize> {
    indices(mask).map(|i| i + 1)
}
pub(super) struct PairCounts {
    shift: u32,
    mask: usize,
    cells: Vec<u16>,
    residual: Vec<f64>,
    occupied: Vec<u16>,
    total: u16,
}
impl PairCounts {
    pub fn new(levels: usize) -> Self {
        let stride = levels.next_power_of_two();
        Self {
            shift: stride.trailing_zeros(),
            mask: stride - 1,
            cells: vec![0; levels * stride],
            residual: Vec::new(),
            occupied: Vec::new(),
            total: 0,
        }
    }
    fn add(&mut self, a: usize, b: usize) {
        let index = ((a.min(b) - 1) << self.shift) + a.max(b) - 1;
        if self.cells[index] == 0 {
            self.occupied.push(index as u16);
        }
        self.cells[index] += 1;
        self.total += 1;
    }
    fn decode(&self, index: u16) -> (usize, usize) {
        (
            1 + (index as usize >> self.shift),
            1 + (index as usize & self.mask),
        )
    }
    fn clear(&mut self) {
        for i in self.occupied.drain(..) {
            self.cells[i as usize] = 0;
        }
        self.total = 0;
    }
    fn cell(&self, a: usize, b: usize) -> u16 {
        let c = self.cells[((a.min(b) - 1) << self.shift) + a.max(b) - 1];
        if a == b { 2 * c } else { c }
    }
    fn information(&self, rows: &[u16], n: u16) -> f64 {
        let square = i64::from(n).pow(2);
        let mut missing = square;
        let mut sum = 0.;
        for &key in &self.occupied {
            let (i, j) = self.decode(key);
            let expected = i64::from(rows[i]) * i64::from(rows[j]);
            let observed = i64::from(self.cell(i, j)) * i64::from(n);
            let mult = if i == j { 1 } else { 2 };
            missing -= mult * expected;
            let r = (observed - expected) as f64 / expected as f64;
            let divergence = if r.abs() < 0.01 {
                let mut poly = 0.;
                for k in (2..=12).rev() {
                    poly = 1. / f64::from(k * (k - 1)) - r * poly;
                }
                r * r * poly
            } else {
                let q = observed as f64 / expected as f64;
                q * (if q < 0.5 { q.ln() } else { r.ln_1p() }) - r
            };
            sum += (mult * expected) as f64 * divergence;
        }
        (sum + missing as f64) / (square as f64 * std::f64::consts::LN_2)
    }

    fn mcc(&mut self, rows: &[u16], levels: u128, n: u16) -> f64 {
        let dim = levels.count_ones();
        if dim == 1 {
            return f64::NAN;
        }
        if dim == 2 {
            let mut active = bits(levels);
            let a = active.next().unwrap();
            let b = active.next().unwrap();
            let determinant = i32::from(self.cell(a, a)) * i32::from(self.cell(b, b))
                - i32::from(self.cell(a, b)).pow(2);
            return f64::from(determinant.abs()) / (f64::from(rows[a]) * f64::from(rows[b]));
        }
        // A symmetric standardized contingency matrix has a unit trivial eigenvalue.
        // For three states the other two follow from its trace and second moment.
        if dim == 3 {
            let mut trace = 0.;
            let mut squares = 0.;
            for a in bits(levels) {
                for b in bits(levels) {
                    let product = i32::from(rows[a]) * i32::from(rows[b]);
                    let delta = i32::from(self.cell(a, b)) * i32::from(n) - product;
                    let residual = f64::from(delta) / (f64::from(n) * f64::from(product).sqrt());
                    if a == b {
                        trace += residual;
                    }
                    squares += residual * residual;
                }
            }
            let gap = (2. * squares - trace * trace).max(0.);
            if gap > squares * 1e-6 || squares == 0. {
                return 0.5 * (trace.abs() + gap.sqrt());
            }
        }
        // A disconnected two-step support graph has another unit singular value.
        let mut adjacent = [0u128; MAX_LEVEL + 1];
        for &key in &self.occupied {
            let (a, b) = self.decode(key);
            adjacent[a] |= bit(b);
            adjacent[b] |= bit(a);
        }
        let neighbors = |mask| bits(mask).fold(0u128, |next, a| next | adjacent[a]);
        let mut seen = bit(levels.trailing_zeros() as usize + 1);
        let mut frontier = seen;
        while seen != levels {
            frontier = neighbors(neighbors(frontier)) & !seen;
            if frontier == 0 {
                return 1.;
            }
            seen |= frontier;
        }
        let mut active = [0usize; 128];
        let dim = dim as usize;
        for (dst, i) in active.iter_mut().zip(bits(levels)) {
            *dst = i;
        }
        // The centered residual R annihilates v_i=sqrt(row_i/n). Reflect v
        // onto the first coordinate and retain the orthogonal (dim-1) block.
        // Since Rv=0, the two-sided Householder update needs only R's first row.
        let pivot = active[0];
        let size = dim - 1;
        self.residual.resize(size * size, 0.);
        let mut reflector = [0.; 128];
        let mut update = [0.; 128];
        let mut inv_root = [0.; 128];
        let total = f64::from(n);
        let root_n = total.sqrt();
        let divisor = root_n + f64::from(rows[pivot]).sqrt();
        let pivot_norm = f64::from(rows[pivot]).sqrt().recip();
        let pivot_diag = f64::from(
            i32::from(self.cell(pivot, pivot)) * i32::from(n) - i32::from(rows[pivot]).pow(2),
        ) / (total * f64::from(rows[pivot]));
        for (i, &g) in active[1..dim].iter().enumerate() {
            let root = f64::from(rows[g]).sqrt();
            reflector[i] = root / divisor;
            inv_root[i] = root.recip();
            let difference = i32::from(self.cell(pivot, g)) * i32::from(n)
                - i32::from(rows[pivot]) * i32::from(rows[g]);
            update[i] = f64::from(difference) * pivot_norm * inv_root[i] / total
                - 0.5 * pivot_diag * reflector[i];
        }
        for (i, &a) in active[1..dim].iter().enumerate() {
            for (j, &b) in active[1..dim].iter().enumerate().skip(i) {
                let difference = i32::from(self.cell(a, b)) * i32::from(n)
                    - i32::from(rows[a]) * i32::from(rows[b]);
                let value = f64::from(difference) * (inv_root[i] * inv_root[j] / total)
                    - (reflector[i] * update[j] + update[i] * reflector[j]);
                self.residual[i * size + j] = value;
                self.residual[j * size + i] = value;
            }
        }
        if size == 2 {
            let a = self.residual[0];
            let b = self.residual[1];
            let c = self.residual[3];
            return 0.5 * ((a + c).abs() + (a - c).hypot(2. * b));
        }
        if size == 3
            && let Some(x) = symmetric_three(&self.residual)
        {
            return x;
        }
        spectral_radius(&mut self.residual, size)
    }
}

// Closed form for the three eigenvalues of a symmetric 3x3 matrix. Near
// repeated roots, the bracketed tridiagonal solver preserves accuracy.
fn symmetric_three(a: &[f64]) -> Option<f64> {
    let mean = (a[0] + a[4] + a[8]) / 3.;
    let (x, y, z) = (a[0] - mean, a[4] - mean, a[8] - mean);
    let (u, v, w) = (a[1], a[2], a[5]);
    let scale = ((x * x + y * y + z * z + 2. * (u * u + v * v + w * w)) / 6.).sqrt();
    if scale == 0. {
        return Some(mean.abs());
    }
    let det = x * y * z + 2. * u * v * w - x * w * w - y * v * v - z * u * u;
    let cos3 = 0.5 * det / scale.powi(3);
    if cos3.abs() > 1. - 1e-6 {
        return None;
    }
    let angle = cos3.acos() / 3.;
    let high = mean + 2. * scale * angle.cos();
    let low = mean + 2. * scale * (angle + std::f64::consts::TAU / 3.).cos();
    Some(high.abs().max(low.abs()))
}
