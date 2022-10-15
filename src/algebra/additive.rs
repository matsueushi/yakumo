//! Add, Neg が満たす条件を表現するためのモジュール。
use cargo_snippet::snippet;

#[snippet("algebra/additive")]
use std::ops::{Add, Neg};

#[snippet("algebra/additive")]
pub trait ClosedAdd: Add<Output = Self> + Sized {}
#[snippet("algebra/additive")]
impl<T: Add<Output = T>> ClosedAdd for T {}

#[snippet("algebra/additive")]
pub trait ClosedNeg: Neg<Output = Self> + Sized {}
#[snippet("algebra/additive")]
impl<T: Neg<Output = T>> ClosedNeg for T {}

/// Add が結合則を満たす。
#[snippet("algebra/additive")]
pub trait AddAssoc: ClosedAdd {}

/// Add が可換である。
#[snippet("algebra/additive")]
pub trait AddComm: ClosedAdd {}

/// Add に関して単位元を持つ。
#[snippet("algebra/additive")]
pub trait Zero: ClosedAdd {
    fn zero() -> Self;
}

#[snippet("algebra/additive")]
macro_rules! multiplicative_int_impl {
    ($($t:ty)*) => ($(
        impl AddAssoc for $t {}
        impl AddComm for $t {}
        impl Zero for $t {
            fn zero() -> Self {
                0
            }
        }
    )*)
}

#[snippet("algebra/additive")]
multiplicative_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
