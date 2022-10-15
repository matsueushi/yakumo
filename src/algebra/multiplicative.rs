//! Mul が満たす条件を表現するためのモジュール。
use cargo_snippet::snippet;

#[snippet("algebra/multiplicative")]
use std::ops::Mul;

#[snippet("algebra/multiplicative")]
pub trait ClosedMul: Mul<Output = Self> + Sized {}
impl<T: Mul<Output = T>> ClosedMul for T {}

/// Mul が部分的に逆元を持つ。
#[snippet("algebra/multiplicative")]
pub trait PartialMulRecip: ClosedMul {
    fn partial_mul_recip(self) -> Option<Self>;
}

/// Mul が逆元を持つ。
#[snippet("algebra/multiplicative")]
pub trait MulRecip: ClosedMul {
    fn mul_recip(self) -> Self;
}

#[snippet("algebra/multiplicative")]
impl<T: MulRecip> PartialMulRecip for T {
    fn partial_mul_recip(self) -> Option<Self> {
        Some(self.mul_recip())
    }
}
