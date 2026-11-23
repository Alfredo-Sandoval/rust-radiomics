//! Bit sets encode the membership of at most 125 sites in a 2-D or 3-D window.
//! A direction shifts a set of in-bounds source sites to their neighbours. Runs
//! use consecutive intersections, zones use flood fill, and dependences use
//! neighbourhood population counts. The same components feed both zone families.
use crate::config::settings::{DirectionalAggregation, MatrixAggregation, NgtdmSingularCases};
use crate::voxel::window::{directional_pairs, entropy_table, indices};
use crate::{Error, Result, Settings};

#[derive(Clone, Copy)]
enum SizeStatistic {
    Short,
    Long,
    Low,
    High,
    ShortLow,
    ShortHigh,
    LongLow,
    LongHigh,
    GrayNonuniformity,
    GrayNormalized,
    SizeNonuniformity,
    SizeNormalized,
    GrayVariance,
    SizeVariance,
    Entropy,
    Energy,
    Percentage,
}
impl SizeStatistic {
    fn parse(name: &str) -> Option<Self> {
        Some(match name {
            "SmallDependenceEmphasis"
            | "ShortRunEmphasis"
            | "SmallAreaEmphasis"
            | "SmallDistanceEmphasis" => Self::Short,
            "LargeDependenceEmphasis"
            | "LongRunEmphasis"
            | "LargeAreaEmphasis"
            | "LargeDistanceEmphasis" => Self::Long,
            "LowGrayLevelEmphasis" | "LowGrayLevelRunEmphasis" | "LowGrayLevelZoneEmphasis" => {
                Self::Low
            }
            "HighGrayLevelEmphasis" | "HighGrayLevelRunEmphasis" | "HighGrayLevelZoneEmphasis" => {
                Self::High
            }
            "SmallDependenceLowGrayLevelEmphasis"
            | "ShortRunLowGrayLevelEmphasis"
            | "SmallAreaLowGrayLevelEmphasis"
            | "SmallDistanceLowGrayLevelEmphasis" => Self::ShortLow,
            "SmallDependenceHighGrayLevelEmphasis"
            | "ShortRunHighGrayLevelEmphasis"
            | "SmallAreaHighGrayLevelEmphasis"
            | "SmallDistanceHighGrayLevelEmphasis" => Self::ShortHigh,
            "LargeDependenceLowGrayLevelEmphasis"
            | "LongRunLowGrayLevelEmphasis"
            | "LargeAreaLowGrayLevelEmphasis"
            | "LargeDistanceLowGrayLevelEmphasis" => Self::LongLow,
            "LargeDependenceHighGrayLevelEmphasis"
            | "LongRunHighGrayLevelEmphasis"
            | "LargeAreaHighGrayLevelEmphasis"
            | "LargeDistanceHighGrayLevelEmphasis" => Self::LongHigh,
            "GrayLevelNonUniformity" => Self::GrayNonuniformity,
            "GrayLevelNonUniformityNormalized" => Self::GrayNormalized,
            "DependenceNonUniformity"
            | "RunLengthNonUniformity"
            | "SizeZoneNonUniformity"
            | "ZoneDistanceNonUniformity" => Self::SizeNonuniformity,
            "DependenceNonUniformityNormalized"
            | "RunLengthNonUniformityNormalized"
            | "SizeZoneNonUniformityNormalized"
            | "ZoneDistanceNonUniformityNormalized" => Self::SizeNormalized,
            "GrayLevelVariance" => Self::GrayVariance,
            "DependenceVariance" | "RunVariance" | "ZoneVariance" | "ZoneDistanceVariance" => {
                Self::SizeVariance
            }
            "DependenceEntropy" | "RunEntropy" | "ZoneEntropy" | "ZoneDistanceEntropy" => {
                Self::Entropy
            }
            "DependenceCountEnergy" => Self::Energy,
            "DependencePercentage" | "RunPercentage" | "ZonePercentage" => Self::Percentage,
            _ => return None,
        })
    }
}
struct Shift {
    src: u128,
    offset: usize,
}
impl Shift {
    fn forward(&self, set: u128) -> u128 {
        (set & self.src) << self.offset
    }
}

pub(super) struct WindowTexture {
    runs: Vec<(usize, SizeStatistic)>,
    areas: Vec<(usize, SizeStatistic)>,
    depths: Vec<(usize, SizeStatistic)>,
    dependences: Vec<(usize, SizeStatistic)>,
    tones: Vec<(usize, usize)>,
    caps: bool,
    directions: Vec<Shift>,
    faces: Vec<usize>,
    neighbors: Vec<u128>,
    nearest: Vec<Vec<u8>>,
    entropy: Vec<Vec<f64>>,
    levels: usize,
    sites: usize,
}

impl WindowTexture {
    pub fn new(shape: &[usize], levels: u32, settings: &Settings, slots: &[usize]) -> Option<Self> {
        let sites = shape.iter().product::<usize>();
        if slots.is_empty()
            || sites > 125
            || !(1..=128).contains(&levels)
            || settings.force2d_axis.is_some()
            || settings.slice_axis >= shape.len()
            || settings.glrlm_aggregation != DirectionalAggregation::Native
            || [
                settings.glszm_aggregation,
                settings.gldzm_aggregation,
                settings.ngtdm_aggregation,
                settings.gldm_aggregation,
            ]
            .iter()
            .any(|&v| v != MatrixAggregation::Native)
            || settings.gldm_alpha != 0
            || settings.distances.as_slice() != [1]
            || settings.neighborhood_radius != 1
        {
            return None;
        }
        let mut plan = Self {
            runs: Vec::new(),
            areas: Vec::new(),
            depths: Vec::new(),
            dependences: Vec::new(),
            tones: Vec::new(),
            caps: settings.ngtdm_singular_cases == NgtdmSingularCases::IbsiCaps,
            directions: Vec::new(),
            faces: Vec::new(),
            neighbors: vec![0; sites],
            nearest: vec![Vec::new(); sites],
            entropy: entropy_table(sites),
            levels: levels as usize,
            sites,
        };
        for &slot in slots {
            let (family, name) = settings.features[slot].split_once('_')?;
            let target = match family {
                "glrlm" => &mut plan.runs,
                "glszm" => &mut plan.areas,
                "gldzm" => &mut plan.depths,
                "gldm" => &mut plan.dependences,
                "ngtdm" => {
                    let id = [
                        "Coarseness",
                        "Contrast",
                        "Busyness",
                        "Complexity",
                        "Strength",
                    ]
                    .iter()
                    .position(|&n| n == name)?;
                    plan.tones.push((slot, id));
                    continue;
                }
                _ => return None,
            };
            target.push((slot, SizeStatistic::parse(name)?));
        }
        for (face, pairs) in directional_pairs(shape, settings) {
            let mut shift = Shift { src: 0, offset: 0 };
            for (p, q) in pairs {
                let (a, b) = (usize::from(p), usize::from(q));
                shift.src |= 1 << a;
                shift.offset = b - a;
                plan.neighbors[a] |= 1 << b;
                plan.neighbors[b] |= 1 << a;
                plan.nearest[a].push(q);
                plan.nearest[b].push(p);
            }
            if face {
                plan.faces.push(plan.directions.len());
            }
            plan.directions.push(shift);
        }
        Some(plan)
    }

    pub fn scratch(&self) -> SizeMatrix {
        SizeMatrix::new(self.levels, self.sites)
    }

    pub fn evaluate(
        &self,
        bins: &[i32],
        morph: &[bool],
        matrix: &mut SizeMatrix,
        out: &mut [f64],
    ) -> Result<()> {
        if bins.iter().any(|&b| b < 0 || b as usize > self.levels) {
            return Err(Error::InvalidInput(
                "bins must be 1..Ng inside sampling_mask and 0 outside".into(),
            ));
        }
        let mut masks = [0u128; 129];
        let (mut present, mut valid, mut morphology) = (0u128, 0u128, 0u128);
        for (i, &b) in bins.iter().enumerate() {
            if b != 0 {
                masks[b as usize] |= 1 << i;
                present |= 1 << (b - 1);
                valid |= 1 << i;
            }
            if morph[i] {
                morphology |= 1 << i;
            }
        }
        let voxels = valid.count_ones();
        if voxels == 0 {
            return Ok(());
        }
        if !self.runs.is_empty() {
            self.run_statistics(&masks, present, voxels, matrix, out);
        }
        if !self.areas.is_empty() || !self.depths.is_empty() {
            self.zone_statistics(&masks, present, valid, morphology, matrix, out)?;
        }
        if !self.dependences.is_empty() {
            for p in indices(valid) {
                let g = bins[p] as usize;
                matrix.add(
                    g,
                    1 + (self.neighbors[p] & masks[g]).count_ones() as usize,
                    1,
                );
            }
            matrix.write(voxels, &self.entropy, &self.dependences, out);
        }
        if !self.tones.is_empty() {
            self.tone_statistics(bins, valid, out);
        }
        Ok(())
    }

    fn run_statistics(
        &self,
        masks: &[u128],
        present: u128,
        voxels: u32,
        matrix: &mut SizeMatrix,
        out: &mut [f64],
    ) {
        let mut sums = [0.; 17];
        let mut errors = [0.; 17];
        for direction in &self.directions {
            for g in indices(present).map(|v| v + 1) {
                let level = masks[g];
                let mut n = level.count_ones();
                let mut tails = direction.forward(level) & level;
                let mut next = tails.count_ones();
                // If A(k) counts length-k strings of equal sites, the number
                // of maximal length-k runs is A(k)-2*A(k+1)+A(k+2).
                let mut length = 1;
                while n != 0 {
                    tails = direction.forward(tails) & level;
                    let after = tails.count_ones();
                    let exact = n + after - 2 * next;
                    if exact != 0 {
                        matrix.add(g, length, exact as u16);
                    }
                    n = next;
                    next = after;
                    length += 1;
                }
            }
            let values = matrix.statistics(voxels, &self.entropy);
            for &(_, key) in &self.runs {
                let i = key as usize;
                let v = values[i];
                let next = sums[i] + v;
                let back = next - sums[i];
                errors[i] += (sums[i] - (next - back)) + (v - back);
                sums[i] = next;
            }
        }
        for &(slot, key) in &self.runs {
            let i = key as usize;
            out[slot] = (sums[i] + errors[i]) / self.directions.len() as f64;
        }
    }

    fn zone_statistics(
        &self,
        masks: &[u128],
        present: u128,
        valid: u128,
        morphology: u128,
        matrix: &mut SizeMatrix,
        out: &mut [f64],
    ) -> Result<()> {
        let voxels = valid.count_ones();
        if !self.depths.is_empty() && valid & !morphology != 0 {
            return Err(Error::InvalidInput(
                "GLDZM intensity mask must lie inside its morphological mask".into(),
            ));
        }
        let mut interiors = [0u128; 8]; // Max inradius is 5 for <=125 sites and square or cubic windows.
        if !self.depths.is_empty() {
            let mut core = morphology;
            for interior in &mut interiors {
                let old = core;
                for &i in &self.faces {
                    let shift = &self.directions[i];
                    core &= shift.forward(old) & ((old >> shift.offset) & shift.src);
                }
                *interior = core;
                if core == 0 {
                    break;
                }
            }
        }
        let mut zones = [(0u8, 0u8, 0u8); 125];
        let mut count = 0;
        for g in indices(present).map(|v| v + 1) {
            let mut rest = masks[g];
            while rest != 0 {
                let mut frontier = rest & rest.wrapping_neg();
                let mut zone = frontier;
                rest ^= frontier;
                while frontier != 0 && rest != 0 {
                    let neighbors = indices(frontier).fold(0, |a, p| a | self.neighbors[p]);
                    frontier = neighbors & rest;
                    zone |= frontier;
                    rest ^= frontier;
                }
                let depth = interiors
                    .iter()
                    .take_while(|&&core| zone & !core == 0)
                    .count()
                    + 1;
                zones[count] = (g as u8, zone.count_ones() as u8, depth as u8);
                count += 1;
            }
        }
        for (keys, distance) in [(&self.areas, false), (&self.depths, true)] {
            if keys.is_empty() {
                continue;
            }
            for &(g, size, depth) in &zones[..count] {
                matrix.add(g as usize, if distance { depth } else { size } as usize, 1);
            }
            matrix.write(voxels, &self.entropy, keys, out);
        }
        Ok(())
    }

    fn tone_statistics(&self, bins: &[i32], valid: u128, out: &mut [f64]) {
        let mut hits = [0u16; 129];
        let mut sums = [0.; 129];
        let mut occupied = 0u128;
        let mut centers = 0u32;
        for p in indices(valid) {
            let n = (self.neighbors[p] & valid).count_ones() as i32;
            if n == 0 {
                continue;
            }
            let g = bins[p];
            let total: i32 = self.nearest[p].iter().map(|&q| bins[q as usize]).sum();
            hits[g as usize] += 1;
            sums[g as usize] += f64::from((n * g - total).abs()) / f64::from(n);
            occupied |= 1 << (g - 1);
            centers += 1;
        }
        if centers != 0 {
            let t = f64::from(centers);
            let (mut weighted, mut sum_s, mut contrast, mut busy, mut complexity, mut strength) =
                (0., 0., 0., 0., 0., 0.);
            let mut remaining = occupied;
            for a in indices(occupied).map(|i| i + 1) {
                let ca = f64::from(hits[a]);
                weighted += ca * sums[a];
                sum_s += sums[a];
                remaining &= remaining - 1;
                for b in indices(remaining).map(|i| i + 1) {
                    let cb = f64::from(hits[b]);
                    let d = (b - a) as f64;
                    contrast += (ca / t) * (cb / t) * d.powi(2);
                    busy += (a as f64 * ca - b as f64 * cb).abs();
                    complexity += d * (ca * sums[a] + cb * sums[b]) / (ca + cb);
                    strength += (ca + cb) / t * d.powi(2);
                }
            }
            let ng = f64::from(occupied.count_ones());
            let coarseness = t / weighted;
            let values = [
                if self.caps {
                    coarseness.min(1e6)
                } else {
                    coarseness
                },
                if ng == 1. {
                    0.
                } else {
                    2. * contrast * sum_s / (t * ng * (ng - 1.))
                },
                if self.caps && ng == 1. {
                    0.
                } else {
                    weighted / (2. * busy)
                },
                2. * complexity / t,
                if self.caps && sum_s == 0. {
                    0.
                } else {
                    2. * strength / sum_s
                },
            ];
            for &(slot, i) in &self.tones {
                out[slot] = values[i];
            }
        } else {
            for &(slot, _) in &self.tones {
                out[slot] = f64::NAN;
            }
        }
    }
}

pub(super) struct SizeMatrix {
    shift: u32,
    mask: usize,
    cells: Vec<u16>,
    occupied: Vec<usize>,
    total: u32,
}
impl SizeMatrix {
    fn new(levels: usize, sites: usize) -> Self {
        let stride = (sites + 1).next_power_of_two();
        Self {
            shift: stride.trailing_zeros(),
            mask: stride - 1,
            cells: vec![0; (levels + 1) * stride],
            occupied: Vec::new(),
            total: 0,
        }
    }
    fn add(&mut self, g: usize, s: usize, count: u16) {
        let i = (g << self.shift) + s;
        if self.cells[i] == 0 {
            self.occupied.push(i);
        }
        self.cells[i] += count;
        self.total += u32::from(count);
    }
    fn statistics(&mut self, voxels: u32, entropy: &[Vec<f64>]) -> [f64; 17] {
        let mut out = [0.; 17];
        let mut gray = [0u16; 129];
        let mut size = [0u16; 126];
        let (mut sg, mut ss, mut gg, mut zz, mut glnu, mut sznu) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let n = f64::from(self.total);
        for i in self.occupied.drain(..) {
            let g = i >> self.shift;
            let s = i & self.mask;
            let c = std::mem::take(&mut self.cells[i]);
            let w = u32::from(c);
            let f = f64::from(c);
            let g2 = (g * g) as f64;
            let s2 = (s * s) as f64;
            sg += w * g as u32;
            ss += w * s as u32;
            gg += w * (g * g) as u32;
            zz += w * (s * s) as u32;
            glnu += w * (2 * u32::from(gray[g]) + w);
            sznu += w * (2 * u32::from(size[s]) + w);
            gray[g] += c;
            size[s] += c;
            out[SizeStatistic::Short as usize] += f / s2;
            out[SizeStatistic::Low as usize] += f / g2;
            out[SizeStatistic::ShortLow as usize] += f / (s2 * g2);
            out[SizeStatistic::ShortHigh as usize] += f * g2 / s2;
            out[SizeStatistic::LongLow as usize] += f * s2 / g2;
            out[SizeStatistic::LongHigh as usize] += f * s2 * g2;
            out[SizeStatistic::Entropy as usize] += entropy[self.total as usize][c as usize];
            out[SizeStatistic::Energy as usize] += f * f;
        }
        for key in [
            SizeStatistic::Short,
            SizeStatistic::Low,
            SizeStatistic::ShortLow,
            SizeStatistic::ShortHigh,
            SizeStatistic::LongLow,
            SizeStatistic::LongHigh,
        ] {
            out[key as usize] /= n;
        }
        out[SizeStatistic::Long as usize] = f64::from(zz) / n;
        out[SizeStatistic::High as usize] = f64::from(gg) / n;
        out[SizeStatistic::GrayNonuniformity as usize] = f64::from(glnu) / n;
        out[SizeStatistic::GrayNormalized as usize] = f64::from(glnu) / (n * n);
        out[SizeStatistic::SizeNonuniformity as usize] = f64::from(sznu) / n;
        out[SizeStatistic::SizeNormalized as usize] = f64::from(sznu) / (n * n);
        out[SizeStatistic::GrayVariance as usize] = f64::from(gg * self.total - sg * sg) / (n * n);
        out[SizeStatistic::SizeVariance as usize] = f64::from(zz * self.total - ss * ss) / (n * n);
        out[SizeStatistic::Percentage as usize] = n / f64::from(voxels);
        out[SizeStatistic::Energy as usize] /= n * n;
        self.total = 0;
        out
    }
    fn write(
        &mut self,
        voxels: u32,
        entropy: &[Vec<f64>],
        keys: &[(usize, SizeStatistic)],
        out: &mut [f64],
    ) {
        let values = self.statistics(voxels, entropy);
        for &(slot, key) in keys {
            out[slot] = values[key as usize];
        }
    }
}
