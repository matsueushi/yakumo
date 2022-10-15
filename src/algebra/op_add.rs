//! 加法演算を型として表現するためのモジュール。

use super::additive::{AddAssoc, AddComm, ClosedAdd, ClosedNeg, Zero};
use super::structure::{Associative, Commutative, Identity, Magma, Recip};

use std::marker::PhantomData;

/// 加算を表すための構造体
pub struct OpAdd<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for OpAdd<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T: Eq + ClosedAdd> Magma for OpAdd<T> {
    type Set = T;

    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set {
        x + y
    }
}

impl<T: Eq + ClosedAdd + ClosedNeg> Recip for OpAdd<T> {
    fn recip(&self, x: Self::Set) -> Self::Set {
        -x
    }
}

impl<T: Eq + ClosedAdd + AddAssoc> Associative for OpAdd<T> {}

impl<T: Eq + ClosedAdd + AddComm> Commutative for OpAdd<T> {}

impl<T: Eq + ClosedAdd + Zero> Identity for OpAdd<T> {
    fn id(&self) -> Self::Set {
        T::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_add_magma() {
        let op_add = OpAdd::default();
        assert_eq!(op_add.op(1, 1), 2);
    }
}
