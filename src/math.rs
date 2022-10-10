//! 数学に関するモジュール。

/// 最小公倍数
pub trait Gcd {
    // 最小公倍数を計算する。
    fn gcd(self, rhs: Self) -> Self;
}

macro_rules! ugcd_impl {
    ($($t:ty)*) => ($(
        impl Gcd for $t {
            fn gcd(self, rhs: Self) -> Self {
                if rhs == 0 {
                    self
                } else {
                    rhs.gcd(self % rhs)
                }
            }
        }
    )*)
}

macro_rules! igcd_impl {
    ($($t:ty)*) => ($(
        impl Gcd for $t {
            fn gcd(self, rhs: Self) -> Self {
                println!("{} {}",self,rhs);
                if rhs == 0 {
                    self.abs()
                } else {
                    rhs.gcd(self % rhs)
                }
            }
        }
    )*)
}

ugcd_impl! { usize u8 u16 u32 u64 u128 }
igcd_impl! { isize i8 i16 i32 i64 i128 }

/// Floor sumを計算する
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
    use crate::math::{floor_sum, Gcd};

    #[test]
    fn test_gcd() {
        assert_eq!(0.gcd(0), 0);
        assert_eq!((-2).gcd(3), 1);
        assert_eq!(4.gcd(6), 2);
    }

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
