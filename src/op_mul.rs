//! 乗法

use crate::algebraic_structure::{Identity, Magma};
use std::marker::PhantomData;
use std::ops::Mul;

pub trait ClosedMul: Sized + Mul<Output = Self> {}
impl<T: Mul<Output = T>> ClosedMul for T {}

/// 乗算を表すための構造体
pub struct OpMul<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for OpMul<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T: Eq + ClosedMul> Magma for OpMul<T> {
    type Set = T;

    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set {
        x * y
    }
}

macro_rules! identity_impl {
    ($($t:ty)*) => ($(
        impl Identity for OpMul<$t> {
            fn id() -> Self::Set {
                1
            }
        }
    )*)
}

identity_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

#[cfg(test)]
mod tests {
    use crate::op_mul::*;

    #[test]
    fn test_magma() {
        let op_mul = OpMul::default();
        assert_eq!(op_mul.op(2, 3), 6);
    }
}
