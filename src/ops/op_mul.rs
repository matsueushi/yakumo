//! 乗法演算を型として表現するためのモジュール。

use super::super::math::algebra::{Associative, Commutative, Identity, Magma};
use super::multiplicative::{ClosedMul, MulAssoc, MulComm, One};

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

impl<T: Eq + ClosedMul + MulAssoc> Associative for OpMul<T> {}

impl<T: Eq + ClosedMul + MulComm> Commutative for OpMul<T> {}

impl<T: Eq + ClosedMul + One> Identity for OpMul<T> {
    fn id(&self) -> Self::Set {
        T::one()
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::op_mul::*;

    #[test]
    fn test_magma() {
        let op_mul = OpMul::default();
        assert_eq!(op_mul.op(2, 3), 6);
    }
}
