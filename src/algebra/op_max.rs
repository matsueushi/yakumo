//! 最大値取得を型として表現するためのモジュール。
use cargo_snippet::snippet;

#[snippet("algebra/op_max")]
#[snippet("algebra/op_max")]
use super::structure::{Associative, Commutative, Identity, Magma};

#[snippet("algebra/op_max")]
use std::marker::PhantomData;

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
macro_rules! op_max_int_impl {
    ($($t:ty)*) => ($(
        impl Magma for OpMax<$t> {
            type Set = $t;

            fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set {
                x.max(y)
            }
        }

        impl Associative for OpMax<$t> {}
        impl Commutative for OpMax<$t> {}
    )*)
}
#[snippet("algebra/op_max")]
macro_rules! op_max_id_int_impl {
    ($($t:ident)*) => ($(
        impl Identity for OpMax<$t> {
            fn id(&self) -> Self::Set {
                // <$t>::MIN に置き換えて上と統合したい
                std::$t::MIN
            }
        }

    )*)
}

#[snippet("algebra/op_max")]
op_max_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
#[snippet("algebra/op_max")]
op_max_id_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

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
