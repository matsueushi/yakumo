//! 加法に関するモジュール。

use crate::algebraic_structure::{Identity, Magma};
use std::marker::PhantomData;
use std::ops::Add;

pub trait ClosedAdd: Sized + Add<Output = Self> {}
impl<T: Add<Output = T>> ClosedAdd for T {}

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

macro_rules! identity_impl {
    ($($t:ty)*) => ($(
        impl Identity for OpAdd<$t> {
            fn id() -> Self::Set {
                0
            }
        }
    )*)
}

identity_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

pub trait Zero {}
impl<T> Zero for OpAdd<T> {}

#[cfg(test)]
mod tests {
    use crate::op_add::*;

    #[test]
    fn test_magma() {
        let op_add = OpAdd::default();
        assert_eq!(op_add.op(1, 1), 2);
    }
}
