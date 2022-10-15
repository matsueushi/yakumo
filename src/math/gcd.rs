//! 最小公倍数に関するモジュール。

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

/// 拡張ユークリッドの互除法
///
/// ax + by = gcd(a, b) を満たす (gcd(a,b), x, y) を返す。
/// https://qiita.com/drken/items/b97ff231e43bce50199a
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, s, t) = ext_gcd(b, a % b);
    (d, t, s - a / b * t)
}

#[cfg(test)]
mod tests {
    use super::{ext_gcd, Gcd};

    #[test]
    fn test_gcd() {
        assert_eq!(0.gcd(0), 0);
        assert_eq!((-2).gcd(3), 1);
        assert_eq!(4.gcd(6), 2);
    }

    #[test]
    fn test_ext_gcd() {
        assert_eq!(ext_gcd(111, 30), (3, 3, -11));
    }
}
