//! Geometry and entropy lookups shared by compact local texture matrices.

use crate::Settings;
use crate::imaging::preprocessing::directions;

pub(super) fn entropy_table(maximum: usize) -> Vec<Vec<f64>> {
    (0..=maximum)
        .map(|n| {
            (0..=n)
                .map(|c| {
                    if c == 0 {
                        0.
                    } else {
                        c as f64 / n as f64
                            * (((n - c) as f64 / c as f64).ln_1p() / std::f64::consts::LN_2)
                    }
                })
                .collect()
        })
        .collect()
}

pub(super) fn indices(mut set: u128) -> impl Iterator<Item = usize> {
    std::iter::from_fn(move || {
        if set == 0 {
            return None;
        }
        let i = set.trailing_zeros() as usize;
        set &= set - 1;
        Some(i)
    })
}

type LocalPairs = (bool, Vec<(u8, u8)>);

/// Valid local index pairs in extraction order, with axial directions marked.
pub(super) fn directional_pairs(shape: &[usize], settings: &Settings) -> Vec<LocalPairs> {
    let sites = shape.iter().product::<usize>();
    debug_assert!(sites <= 125);
    let strides: Vec<_> = (0..shape.len())
        .map(|a| shape[a + 1..].iter().product::<usize>())
        .collect();
    let mut bounded = settings.clone();
    bounded
        .distances
        .retain(|&d| (d as usize) < *shape.iter().max().unwrap());
    directions(shape, &bounded)
        .into_iter()
        .map(|delta| {
            let pairs = (0..sites)
                .filter_map(|p| {
                    let mut q = 0;
                    for a in 0..shape.len() {
                        let coordinate = (p / strides[a] % shape[a]) as i64 + delta[a];
                        if !(0..shape[a] as i64).contains(&coordinate) {
                            return None;
                        }
                        q += coordinate as usize * strides[a];
                    }
                    Some((p as u8, q as u8))
                })
                .collect();
            (delta.iter().filter(|&&d| d != 0).count() == 1, pairs)
        })
        .collect()
}
