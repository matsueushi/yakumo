//! Floor sumを計算する
use cargo_snippet::snippet;

#[snippet("math/floor_sum")]
pub fn floor_sum(n: usize, m: usize, a: usize, b: usize) -> usize {
    let (x, a0) = (a / m, a % m);
    let (y, b0) = (b / m, b % m);
    let s = if n <= 1 { 0 } else { n * (n - 1) / 2 * x } + n * y;
    if a0 == 0 {
        return s;
    }
    let u = a0 * n + b0;
    let (c, d) = (u / m, u % m);
    s + floor_sum(c, a0, m, d)
}

#[cfg(test)]
mod tests {
    use super::floor_sum;

    fn floor_sum_naive(n: usize, m: usize, a: usize, b: usize) -> usize {
        let mut s = 0;
        for i in 0..n {
            s += (a * i + b) / m;
        }
        s
    }

    #[test]
    fn test_floor_sum() {
        for n in 0..=20 {
            for m in 1..=20 {
                for a in 0..=20 {
                    for b in 0..=20 {
                        assert_eq!(floor_sum(n, m, a, b), floor_sum_naive(n, m, a, b))
                    }
                }
            }
        }
    }
}
