//! Mul が満たす条件を表現するためのトレイトたち。

use std::ops::Mul;

pub trait ClosedMul: Mul<Output = Self> + Sized {}
impl<T: Mul<Output = T>> ClosedMul for T {}

/// Mul が結合則を満たす。
pub trait MulAssoc: ClosedMul {}

/// Mul が可換である。
pub trait MulComm: ClosedMul {}

/// Mul に関して単位元を持つ。
pub trait One: ClosedMul {
    fn one() -> Self;
}

/// Mul が部分的に逆元を持つ。
pub trait MulPartialRecip: ClosedMul {
    fn mul_partial_recip(self) -> Option<Self>;
}

/// Mul が逆元を持つ。
pub trait MulRecip: ClosedMul {
    fn mul_recip(self) -> Self;
}

impl<T: MulRecip> MulPartialRecip for T {
    fn mul_partial_recip(self) -> Option<Self> {
        Some(self.mul_recip())
    }
}

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

multiplicative_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
