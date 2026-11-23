//! Occupied integer coordinates, mirroring `mlxradiomics/numerics/sparse_counts.py`.
//! Status: ported. Single-threaded CPU numerical work on 1-based matrix coordinates.
//! Marginals are sorted coordinate and count vectors, built by dense accumulation when the
//! coordinates are small enough and otherwise through a sparse map.

use std::collections::BTreeMap;

/// Keys retain their original coordinates, independent of the lattice or slice width.
pub(crate) type Counts = BTreeMap<(usize, usize), u64>;

/// Counts per coordinate, in increasing coordinate order.
#[derive(Default, Clone)]
pub(crate) struct Marginal {
    keys: Vec<usize>,
    counts: Vec<u64>,
}

impl Marginal {
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, u64)> + '_ {
        self.keys.iter().copied().zip(self.counts.iter().copied())
    }

    pub fn keys(&self) -> &[usize] {
        &self.keys
    }

    pub fn values(&self) -> std::slice::Iter<'_, u64> {
        self.counts.iter()
    }

    pub fn get(&self, key: usize) -> Option<u64> {
        self.keys
            .binary_search(&key)
            .ok()
            .map(|index| self.counts[index])
    }

    /// Sum the counts of equal keys. Keys up to `max_key` accumulate in a dense array when
    /// that array is not much larger than the number of cells.
    pub fn accumulate(
        cells: usize,
        max_key: usize,
        pairs: impl Iterator<Item = (usize, u64)>,
    ) -> Self {
        let mut marginal = Self::default();
        if max_key <= 64 * cells + 4096 {
            let mut dense = vec![0u64; max_key + 1];
            for (key, count) in pairs {
                dense[key] += count;
            }
            for (key, &count) in dense.iter().enumerate() {
                if count != 0 {
                    marginal.keys.push(key);
                    marginal.counts.push(count);
                }
            }
        } else {
            let mut sparse = BTreeMap::<usize, u64>::new();
            for (key, count) in pairs {
                *sparse.entry(key).or_default() += count;
            }
            for (key, count) in sparse {
                marginal.keys.push(key);
                marginal.counts.push(count);
            }
        }
        marginal
    }
}

pub(crate) fn marginals(counts: &Counts) -> (Marginal, Marginal) {
    let cells = counts.len();
    // Cells are sorted by row, so the row marginal is built from runs of equal rows.
    let mut rows = Marginal::default();
    let mut max_column = 0;
    for (&(i, j), &c) in counts {
        max_column = max_column.max(j);
        if rows.keys.last() == Some(&i) {
            *rows.counts.last_mut().unwrap() += c;
        } else {
            rows.keys.push(i);
            rows.counts.push(c);
        }
    }
    let columns =
        Marginal::accumulate(cells, max_column, counts.iter().map(|(&(_, j), &c)| (j, c)));
    (rows, columns)
}

pub(crate) fn weighted_sum(counts: &Marginal) -> i128 {
    counts.iter().map(|(i, c)| i as i128 * c as i128).sum()
}

/// `(i*n - sum) / n` with the numerator exact; it fits 64 bits for every practical lattice.
pub(crate) fn centered(i: usize, sum: i128, n: u64) -> f64 {
    let numerator = i as i128 * n as i128 - sum;
    match i64::try_from(numerator) {
        Ok(small) => small as f64 / n as f64,
        Err(_) => numerator as f64 / n as f64,
    }
}

pub(crate) fn variance(counts: &Marginal, n: u64) -> f64 {
    let sum = weighted_sum(counts);
    counts
        .iter()
        .map(|(i, c)| c as f64 * centered(i, sum, n).powi(2))
        .sum::<f64>()
        / n as f64
}

pub(crate) fn entropy<'a>(counts: impl Iterator<Item = &'a u64>, n: u64) -> f64 {
    counts
        .map(|&c| c as f64 * (((n - c) as f64) / c as f64).ln_1p())
        .sum::<f64>()
        / (n as f64 * std::f64::consts::LN_2)
}
