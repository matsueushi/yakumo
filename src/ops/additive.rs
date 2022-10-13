//! Add, Neg が満たす条件を表現するためのトレイトたち。
use std::ops::{Add, Neg};

pub trait ClosedAdd: Add<Output = Self> + Sized {}
impl<T: Add<Output = T>> ClosedAdd for T {}

pub trait ClosedNeg: Neg<Output = Self> + Sized {}
impl<T: Neg<Output = T>> ClosedNeg for T {}

/// Add が結合則を満たす。
pub trait AddAssoc: ClosedAdd {}

/// Add が可換である。
pub trait AddComm: ClosedAdd {}

/// Add に関して単位元を持つ。
pub trait Zero: ClosedAdd {
    fn zero() -> Self;
}

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

multiplicative_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
