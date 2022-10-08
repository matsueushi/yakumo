use std::marker::PhantomData;

pub trait Modulo {
    fn modulo() -> i64;
}

pub struct FiniteField<M>(i64, PhantomData<M>);

impl<M: Modulo> FiniteField<M> {
    /// 整数を `0 <= x < m` に正規化してインスタンスを作成する。
    pub fn new(x: i64) -> Self {
        let v = x.rem_euclid(M::modulo());
        Self(v, PhantomData)
    }

    /// `0 <= x < m` となる代表元を返す。
    pub fn val(&self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    // use crate::mod_int::*;

    #[test]
    fn test_finite_field() {}
}
