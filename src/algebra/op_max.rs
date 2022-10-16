//! 最大値取得を型として表現するためのモジュール。
use cargo_snippet::snippet;

#[snippet("algebra/op_max")]
use std::marker::PhantomData;

use super::minmax::Min;
use super::structure::{Associative, Commutative, Identity, Magma};

/// 最大値取得を表すための構造体
#[snippet("algebra/op_max")]
pub struct OpMax<T> {
    phantom: PhantomData<T>,
}

#[snippet("algebra/op_max")]
impl<T> Default for OpMax<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[snippet("algebra/op_max")]
impl<T: Ord> Magma for OpMax<T> {
    type Set = T;

    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set {
        x.max(y)
    }
}

#[snippet("algebra/op_max")]
impl<T: Ord> Associative for OpMax<T> {}

#[snippet("algebra/op_max")]
impl<T: Ord> Commutative for OpMax<T> {}

#[snippet("algebra/op_max")]
impl<T> Identity for OpMax<T>
where
    T: Ord + Min,
{
    fn id(&self) -> Self::Set {
        <T as Min>::min()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_max_magm() {
        let op_max = OpMax::<i32>::default();
        assert_eq!(op_max.op(1, 2), 2);
        assert_eq!(op_max.id(), std::i32::MIN);
    }
}
