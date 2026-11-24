//! ITK physical-to-index inversion for two and three dimensions. Nearest-neighbour
//! ties depend on the rounding of the singular-vector decomposition used by ITK.
//! The column and row Householder reflections, bidiagonal QR iteration and inverse
//! product follow VNL's LINPACK DSVDC path. Explicit fused operations reproduce
//! the SimpleITK 2.5.6 arm64 reference across Rust build targets.

type Vector = [f64; 3];
// Columns are contiguous, as in the LINPACK factorization used by VNL.
type Columns = [Vector; 3];

fn norm(x: &[f64]) -> f64 {
    let mut scale = 0.0;
    let mut sum: f64 = 1.0;
    for value in x.iter().map(|v| v.abs()).filter(|&v| v != 0.0) {
        if scale < value {
            sum = sum.mul_add((scale / value).powi(2), 1.0);
            scale = value;
        } else {
            let ratio = value / scale;
            sum = ratio.mul_add(ratio, sum);
        }
    }
    scale * sum.sqrt()
}

fn reflector(column: &mut [f64]) -> f64 {
    let mut length = norm(column);
    if length != 0.0 {
        if column[0] != 0.0 {
            length = length.copysign(column[0]);
        }
        let inverse = length.recip();
        for value in &mut *column {
            *value *= inverse;
        }
        column[0] += 1.0;
    }
    -length
}

fn apply_reflector(column: &[f64], target: &mut [f64]) {
    let dot = column
        .iter()
        .zip(target.iter())
        .fold(0.0, |sum, (&a, &b)| a.mul_add(b, sum));
    let coefficient = -dot / column[0];
    for (value, &entry) in target.iter_mut().zip(column) {
        *value = coefficient.mul_add(entry, *value);
    }
}

fn identity(n: usize) -> Columns {
    let mut matrix = [[0.0; 3]; 3];
    for (index, column) in matrix[..n].iter_mut().enumerate() {
        column[index] = 1.0;
    }
    matrix
}

struct Bidiagonal {
    diagonal: Vector,
    upper: Vector,
    left: Columns,
    right: Columns,
}

impl Bidiagonal {
    fn decompose(mut input: Columns, n: usize) -> Self {
        let mut diagonal = [0.0; 3];
        let mut upper = [0.0; 3];
        let mut left = identity(n);
        let mut right = identity(n);
        for k in 0..n - 1 {
            diagonal[k] = reflector(&mut input[k][k..n]);
            let column = input[k];
            left[k][k..n].copy_from_slice(&column[k..n]);
            for j in k + 1..n {
                if diagonal[k] != 0.0 {
                    apply_reflector(&column[k..n], &mut input[j][k..n]);
                }
                upper[j] = input[j][k];
            }
            // A three-dimensional image needs one right Householder reflection.
            if k + 2 < n {
                upper[k] = reflector(&mut upper[k + 1..n]);
                if upper[k] != 0.0 {
                    let mut work = [0.0; 3];
                    for j in k + 1..n {
                        for (i, value) in work.iter_mut().enumerate().take(n).skip(k + 1) {
                            *value = upper[j].mul_add(input[j][i], *value);
                        }
                    }
                    for j in k + 1..n {
                        let coefficient = -upper[j] / upper[k + 1];
                        for i in k + 1..n {
                            input[j][i] = coefficient.mul_add(work[i], input[j][i]);
                        }
                        apply_reflector(&upper[k + 1..n], &mut right[j][k + 1..n]);
                    }
                }
            }
        }
        diagonal[n - 1] = input[n - 1][n - 1];
        upper[n - 2] = input[n - 1][n - 2];
        upper[n - 1] = 0.0;
        for k in (0..n - 1).rev() {
            if diagonal[k] == 0.0 {
                left[k][k] = 1.0;
            } else {
                let column = left[k];
                for target in &mut left[k + 1..n] {
                    apply_reflector(&column[k..n], &mut target[k..n]);
                }
                for value in &mut left[k][k..n] {
                    *value = -*value;
                }
                left[k][k] += 1.0;
            }
        }
        Self {
            diagonal,
            upper,
            left,
            right,
        }
    }

    fn diagonalize(&mut self, n: usize) -> bool {
        let mut active = n;
        let mut iterations = 0;
        while active > 0 && iterations < 1000 {
            iterations += 1;
            let (s, e) = (&mut self.diagonal, &mut self.upper);
            let end = active - 1;
            let start = (0..end)
                .rev()
                .find(|&k| {
                    let adjacent = s[k].abs() + s[k + 1].abs();
                    adjacent + e[k].abs() == adjacent
                })
                .map_or(0, |k| {
                    e[k] = 0.0;
                    k + 1
                });
            if start == end {
                self.finish_value(end, n);
                active -= 1;
                iterations = 0;
                continue;
            }
            let split = (start..=end).rev().find(|&k| {
                let adjacent = if k != end { e[k].abs() } else { 0.0 }
                    + if k != start { e[k - 1].abs() } else { 0.0 };
                adjacent + s[k].abs() == adjacent
            });
            if let Some(k) = split {
                self.split_zero(k, start, end, n);
                continue;
            }
            self.qr_step(start, end, n);
        }
        active == 0
    }

    // Keep converged values positive and in descending order, along with their vectors.
    fn finish_value(&mut self, end: usize, n: usize) {
        if self.diagonal[end] < 0.0 {
            self.diagonal[end] = -self.diagonal[end];
            for x in &mut self.right[end] {
                *x = -*x;
            }
        }
        let mut k = end;
        while k + 1 < n && self.diagonal[k] < self.diagonal[k + 1] {
            self.diagonal.swap(k, k + 1);
            self.left.swap(k, k + 1);
            self.right.swap(k, k + 1);
            k += 1;
        }
    }

    // A negligible diagonal splits a bidiagonal block. Chase the remaining upper
    // entry into the right vectors at its end, or the left vectors inside the block.
    fn split_zero(&mut self, k: usize, start: usize, end: usize, n: usize) {
        let Self {
            diagonal: s,
            upper: e,
            left,
            right,
        } = self;
        s[k] = 0.0;
        if k == end {
            let mut f = std::mem::take(&mut e[end - 1]);
            for j in (start..end).rev() {
                let (radius, cosine, sine) = givens(s[j], f);
                s[j] = radius;
                if j > start {
                    f = -sine * e[j - 1];
                    e[j - 1] *= cosine;
                }
                rotate_columns(right, j, end, cosine, sine, n);
            }
        } else {
            let mut f = std::mem::take(&mut e[k]);
            for j in k + 1..=end {
                let (radius, cosine, sine) = givens(s[j], f);
                s[j] = radius;
                f = -sine * e[j];
                e[j] *= cosine;
                rotate_columns(left, j, k, cosine, sine, n);
            }
        }
    }

    // Use the trailing two singular values for the shift, then chase the bulge
    // along the active block with alternating right and left Givens rotations.
    fn qr_step(&mut self, start: usize, end: usize, n: usize) {
        let Self {
            diagonal: s,
            upper: e,
            left,
            right,
        } = self;
        let scale = [s[start], e[start], s[end], s[end - 1], e[end - 1]]
            .into_iter()
            .map(f64::abs)
            .fold(0.0, f64::max);
        let (last, previous, edge) = (s[end] / scale, s[end - 1] / scale, e[end - 1] / scale);
        let b = (previous + last).mul_add(previous - last, edge * edge) * 0.5;
        let c = (last * edge).powi(2);
        let shift = if b == 0.0 && c == 0.0 {
            0.0
        } else {
            let root = b.mul_add(b, c).sqrt();
            c / (b + if b < 0.0 { -root } else { root })
        };
        let first = s[start] / scale;
        let mut f = (first + last).mul_add(first - last, shift);
        let mut g = first * (e[start] / scale);
        for k in start..end {
            let (radius, cosine, sine) = givens(f, g);
            if k != start {
                e[k - 1] = radius;
            }
            f = cosine.mul_add(s[k], sine * e[k]);
            e[k] = cosine.mul_add(e[k], -(sine * s[k]));
            g = sine * s[k + 1];
            s[k + 1] *= cosine;
            rotate_columns(right, k, k + 1, cosine, sine, n);
            let (radius, cosine, sine) = givens(f, g);
            s[k] = radius;
            f = cosine.mul_add(e[k], sine * s[k + 1]);
            s[k + 1] = (-sine).mul_add(e[k], cosine * s[k + 1]);
            g = sine * e[k + 1];
            e[k + 1] *= cosine;
            rotate_columns(left, k, k + 1, cosine, sine, n);
        }
        e[end - 1] = f;
    }
}

fn rotate_columns(columns: &mut Columns, a: usize, b: usize, cosine: f64, sine: f64, n: usize) {
    for i in 0..n {
        let (x, y) = (columns[a][i], columns[b][i]);
        columns[a][i] = cosine.mul_add(x, sine * y);
        columns[b][i] = cosine.mul_add(y, -(sine * x));
    }
}

fn givens(a: f64, b: f64) -> (f64, f64, f64) {
    let scale = a.abs() + b.abs();
    if scale == 0.0 {
        return (0.0, 1.0, 0.0);
    }
    let sign = if a.abs() > b.abs() { a } else { b };
    let radius =
        (scale * (a / scale).mul_add(a / scale, (b / scale).powi(2)).sqrt()).copysign(sign);
    (radius, a / radius, b / radius)
}

pub(crate) fn physical_to_index(direction: &[f64], spacing: &[f64]) -> [[f64; 3]; 3] {
    let n = spacing.len();
    let mut columns = [[0.0; 3]; 3];
    for (column, values) in columns[..n].iter_mut().enumerate() {
        for (row, value) in values[..n].iter_mut().enumerate() {
            *value = direction[row * n + column] * spacing[column];
        }
    }
    let mut svd = Bidiagonal::decompose(columns, n);
    if !svd.diagonalize(n)
        || svd.diagonal[..n]
            .iter()
            .any(|s| !s.is_finite() || *s == 0.0)
    {
        // Extreme finite spacing can underflow a scaled column. Volume validation
        // guarantees orthonormal direction cosines, so use the unscaled axes here.
        let mut inverse = [[0.0; 3]; 3];
        for (i, row) in inverse[..n].iter_mut().enumerate() {
            for (j, entry) in row[..n].iter_mut().enumerate() {
                *entry = direction[j * n + i] / spacing[i];
            }
        }
        return inverse;
    }
    let mut inverse = [[0.0; 3]; 3];
    for (i, row) in inverse[..n].iter_mut().enumerate() {
        for (j, value) in row[..n].iter_mut().enumerate() {
            for k in 0..n {
                *value =
                    (svd.right[k][i] * svd.diagonal[k].recip()).mul_add(svd.left[k][j], *value);
            }
        }
    }
    inverse
}
