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

#[cfg(test)]
mod tests {
    use crate::math::gcd::Gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(0.gcd(0), 0);
        assert_eq!((-2).gcd(3), 1);
        assert_eq!(4.gcd(6), 2);
    }
}
