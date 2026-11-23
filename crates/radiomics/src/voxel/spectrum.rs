//! Largest absolute eigenvalue of a symmetric MCC residual. Householder
//! reflections reduce the matrix to tridiagonal form. A bracketed Laguerre
//! iteration finds its outermost root from the characteristic polynomials of
//! the residual and its negative.

pub(super) fn spectral_radius(a: &mut [f64], n: usize) -> f64 {
    let mut off = [0.; 128];
    let mut v = [0.; 128];
    let mut w = [0.; 128];
    for k in 0..n - 2 {
        let start = k + 1;
        v[start..n].copy_from_slice(&a[k * n + start..(k + 1) * n]);
        let norm = v[start..n].iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm == 0. {
            continue;
        }
        let alpha = -norm.copysign(v[start]);
        off[k] = alpha;
        v[start] -= alpha;
        let beta = 2. / v[start..n].iter().map(|x| x * x).sum::<f64>();
        for i in start..n {
            w[i] = beta
                * a[i * n + start..(i + 1) * n]
                    .iter()
                    .zip(&v[start..n])
                    .map(|(x, y)| x * y)
                    .sum::<f64>();
        }
        let correction = 0.5
            * beta
            * v[start..n]
                .iter()
                .zip(&w[start..n])
                .map(|(x, y)| x * y)
                .sum::<f64>();
        for i in start..n {
            w[i] -= correction * v[i];
        }
        for i in start..n {
            for j in i..n {
                let entry = a[i * n + j] - (v[i] * w[j] + w[i] * v[j]);
                a[i * n + j] = entry;
                a[j * n + i] = entry;
            }
        }
    }
    off[n - 2] = a[(n - 2) * n + n - 1];
    let mut diagonal = [0.; 128];
    let mut square = [0.; 128];
    let (mut low, mut high) = (0_f64, 0_f64);
    for i in 0..n {
        diagonal[i] = a[i * n + i];
        square[i] = off[i] * off[i];
        low = low.max(diagonal[i].abs());
        let left = if i == 0 { 0. } else { off[i - 1].abs() };
        high = high.max(diagonal[i].abs() + left + off[i].abs());
    }
    let mut trial = (low + high) * 0.5;
    while high - low > 4. * f64::EPSILON * high {
        if let Some((step, gap_bound)) = outer_root_step(&diagonal[..n], &square[..n], trial) {
            high = trial;
            // The trace of the resolvent bounds the distance to the nearest
            // root. Pad for rounding in the characteristic polynomial.
            low = low.max(trial - gap_bound - 8. * f64::EPSILON * high);
            let proposed = trial - step;
            trial = if proposed > low && proposed < high {
                proposed
            } else {
                0.5 * (low + high)
            };
        } else {
            low = trial;
            trial = 0.5 * (low + high);
        }
    }
    0.5 * (low + high)
}

// At a trial above every eigenvalue the leading tridiagonal determinants are
// positive. Their three-term recurrence also gives the first two derivatives
// of the characteristic polynomial. Laguerre's bound for a real-rooted
// polynomial gives an upper estimate of its largest root. Evaluate for both
// signs to obtain the largest absolute eigenvalue, with bisection as a safeguard.
fn outer_root_step(diagonal: &[f64], square: &[f64], x: f64) -> Option<(f64, f64)> {
    let n = diagonal.len() as f64;
    let mut step = f64::INFINITY;
    let mut bound = f64::INFINITY;
    for sign in [-1., 1.] {
        let (mut previous, mut value) = (0., 1.);
        let (mut previous_first, mut first) = (0., 0.);
        let (mut previous_second, mut second) = (0., 0.);
        let mut b = 0.;
        for (&d, &next_b) in diagonal.iter().zip(square) {
            let a = x - sign * d;
            let next = a * value - b * previous;
            if next <= 0. {
                return None;
            }
            let next_first = value + a * first - b * previous_first;
            let next_second = 2. * first + a * second - b * previous_second;
            let scale = if next < 1e-60 {
                1e60
            } else if next > 1e60 {
                1e-60
            } else {
                1.
            };
            previous = value * scale;
            value = next * scale;
            previous_first = first * scale;
            first = next_first * scale;
            previous_second = second * scale;
            second = next_second * scale;
            b = next_b;
        }
        let distance = value / first;
        let ratio = distance * (second / first);
        let denominator = 1. + ((n - 1.) * (n - 1. - n * ratio).max(0.)).sqrt();
        step = step.min(n * distance / denominator);
        bound = bound.min(n * distance);
    }
    Some((step, bound))
}
