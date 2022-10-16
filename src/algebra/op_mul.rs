//! 乗法演算を型として表現するためのモジュール。
use cargo_snippet::snippet;

#[snippet("algebra/op_mul")]
use std::marker::PhantomData;

use super::multiplicative::{ClosedMul, MulRecip, PartialMulRecip};
use super::structure::{Associative, Commutative, Identity, Magma, PartialRecip, Recip};

/// 乗算を表すための構造体
#[snippet("algebra/op_mul")]
pub struct OpMul<T> {
    phantom: PhantomData<T>,
}

#[snippet("algebra/op_mul")]
impl<T> Default for OpMul<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[snippet("algebra/op_mul")]
impl<T: Eq + ClosedMul> Magma for OpMul<T> {
    type Set = T;

    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set {
        x * y
    }
}

#[snippet("algebra/op_mul")]
impl<T: Eq + ClosedMul + MulRecip> Recip for OpMul<T> {
    fn recip(&self, x: Self::Set) -> Self::Set {
        x.mul_recip()
    }
}

#[snippet("algebra/op_mul")]
impl<T: Eq + ClosedMul + PartialMulRecip> PartialRecip for OpMul<T> {
    fn partial_recip(&self, x: Self::Set) -> Option<Self::Set> {
        x.partial_mul_recip()
    }
}

#[snippet("algebra/op_mul")]
macro_rules! op_mul_int_impl {
    ($($t:ty)*) => ($(
        impl Associative for OpMul<$t> {}

        impl Commutative for OpMul<$t> {}

        impl Identity for OpMul<$t> {
            fn id(&self) -> Self::Set {
                1
            }
        }
    )*)
}

#[snippet("algebra/additive")]
op_mul_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_mul_magma() {
        let op_mul = OpMul::default();
        assert_eq!(op_mul.op(2, 3), 6);
        assert_eq!(op_mul.id(), 1);
    }
}
