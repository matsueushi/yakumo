//! 乗法演算を型として表現するためのモジュール。

use super::super::math::algebra::{Associative, Commutative, Identity, Magma};
use super::multiplicative::ClosedMul;

use std::marker::PhantomData;

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

macro_rules! op_mul_int_impl {
    ($($t:ty)*) => ($(
        impl Associative for OpMul<$t> {}

        impl Commutative for OpMul<$t> {}

        impl Identity for OpMul<$t> {
            fn id() -> Self::Set {
                1
            }
        }
    )*)
}

op_mul_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

#[cfg(test)]
mod tests {
    use crate::ops::op_mul::*;

    #[test]
    fn test_magma() {
        let op_mul = OpMul::default();
        assert_eq!(op_mul.op(2, 3), 6);
    }
}
