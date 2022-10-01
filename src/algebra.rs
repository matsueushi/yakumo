use std::marker::PhantomData;
use std::ops::Add;

pub trait ClosedAdd: Sized + Add<Output = Self> {}
impl<T: Add<Output = T>> ClosedAdd for T {}

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

pub trait Magma<T>
where
    T: Eq,
{
    fn op(&self, x: T, y: T) -> T;
}

impl<T> Magma<T> for OpAdd<T>
where
    T: Eq + ClosedAdd,
{
    fn op(&self, x: T, y: T) -> T {
        x + y
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::*;

    #[test]
    fn test_magma() {
        let op_add = OpAdd::default();
        assert_eq!(24, op_add.op(10, 14));
    }
}
