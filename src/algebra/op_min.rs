//! 最大値取得を型として表現するためのモジュール。
use cargo_snippet::snippet;

#[snippet("algebra/op_min")]
#[snippet("algebra/op_min")]
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
macro_rules! op_min_int_impl {
    ($($t:ty)*) => ($(
        impl Magma for OpMin<$t> {
            type Set = $t;

            fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set {
                x.min(y)
            }
        }

        impl Associative for OpMin<$t> {}
        impl Commutative for OpMin<$t> {}
    )*)
}
#[snippet("algebra/op_min")]
macro_rules! op_min_id_int_impl {
    ($($t:ident)*) => ($(
        impl Identity for OpMin<$t> {
            fn id(&self) -> Self::Set {
                // <$t>::MAX に置き換えて上と統合したい
                std::$t::MAX
            }
        }

    )*)
}

#[snippet("algebra/op_min")]
op_min_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
#[snippet("algebra/op_min")]
op_min_id_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_min_magm() {
        let op_min = OpMin::<i32>::default();
        assert_eq!(op_min.op(1, 2), 1);
        assert_eq!(op_min.id(), i32::MAX);
    }
}
