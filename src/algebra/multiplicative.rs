//! Mul が満たす条件を表現するためのモジュール。
use cargo_snippet::snippet;

#[snippet("algebra/multiplicative")]
use std::ops::Mul;

#[snippet("algebra/multiplicative")]
pub trait ClosedMul: Mul<Output = Self> + Sized {}
impl<T: Mul<Output = T>> ClosedMul for T {}

/// Mul が結合則を満たす。
#[snippet("algebra/multiplicative")]
pub trait MulAssoc: ClosedMul {}

/// Mul が可換である。
#[snippet("algebra/multiplicative")]
pub trait MulComm: ClosedMul {}

/// Mul に関して単位元を持つ。
#[snippet("algebra/multiplicative")]
pub trait One: ClosedMul {
    fn one() -> Self;
}

/// Mul が部分的に逆元を持つ。
#[snippet("algebra/multiplicative")]
pub trait PartialMulRecip: ClosedMul {
    fn partial_mul_recip(self) -> Option<Self>;
}

/// Mul が逆元を持つ。
#[snippet("algebra/multiplicative")]
pub trait MulRecip: ClosedMul {
    fn mul_recip(self) -> Self;
}

#[snippet("algebra/multiplicative")]
impl<T: MulRecip> PartialMulRecip for T {
    fn partial_mul_recip(self) -> Option<Self> {
        Some(self.mul_recip())
    }
}

#[snippet("algebra/multiplicative")]
macro_rules! multiplicative_int_impl {
    ($($t:ty)*) => ($(
        impl MulAssoc for $t {}
        impl MulComm for $t {}
        impl One for $t {
            fn one() -> Self {
                1
            }
        }
    )*)
}

#[snippet("algebra/multiplicative")]
multiplicative_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
