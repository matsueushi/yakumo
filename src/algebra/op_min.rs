//! 最大値取得を型として表現するためのモジュール。
use cargo_snippet::snippet;

use super::minmax::Max;
use super::structure::{Associative, Commutative, Identity, Magma};

#[snippet("algebra/op_min")]
use std::marker::PhantomData;

/// 最大値取得を表すための構造体
#[snippet("algebra/op_min")]
pub struct OpMin<T> {
    phantom: PhantomData<T>,
}

#[snippet("algebra/op_min")]
impl<T> Default for OpMin<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[snippet("algebra/op_min")]
impl<T: Ord> Magma for OpMin<T> {
    type Set = T;

    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set {
        x.min(y)
    }
}

#[snippet("algebra/op_min")]
impl<T: Ord> Associative for OpMin<T> {}

#[snippet("algebra/op_min")]
impl<T: Ord> Commutative for OpMin<T> {}

#[snippet("algebra/op_min")]
impl<T> Identity for OpMin<T>
where
    T: Ord + Max,
{
    fn id(&self) -> Self::Set {
        <T as Max>::max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_min_magm() {
        let op_min = OpMin::<i32>::default();
        assert_eq!(op_min.op(1, 2), 1);
        assert_eq!(op_min.id(), std::i32::MAX);
    }
}
