//! 加法演算を型として表現するためのモジュール。
use cargo_snippet::snippet;

#[snippet("algebra/op_add")]
use std::marker::PhantomData;

use super::additive::{ClosedAdd, ClosedNeg};
use super::structure::{Associative, Commutative, Identity, Magma, Recip};

/// 加算を表すための構造体
#[snippet("algebra/op_add")]
pub struct OpAdd<T> {
    phantom: PhantomData<T>,
}

#[snippet("algebra/op_add")]
impl<T> Default for OpAdd<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[snippet("algebra/op_add")]
impl<T: Eq + ClosedAdd> Magma for OpAdd<T> {
    type Set = T;

    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set {
        x + y
    }
}

#[snippet("algebra/op_add")]
impl<T: Eq + ClosedAdd + ClosedNeg> Recip for OpAdd<T> {
    fn recip(&self, x: Self::Set) -> Self::Set {
        -x
    }
}

#[snippet("algebra/op_add")]
macro_rules! op_add_int_impl {
    ($($t:ty)*) => ($(
        impl Associative for OpAdd<$t> {}

        impl Commutative for OpAdd<$t> {}

        impl Identity for OpAdd<$t> {
            fn id(&self) -> Self::Set {
                0
            }
        }
    )*)
}

#[snippet("algebra/op_add")]
op_add_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_add_magma() {
        let op_add = OpAdd::default();
        assert_eq!(op_add.op(1, 1), 2);
        assert_eq!(op_add.id(), 0);
    }
}
