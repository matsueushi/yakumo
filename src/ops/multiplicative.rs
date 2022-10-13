//! Mul が満たす条件を表現するためのトレイトたち。

use std::ops::Mul;

pub trait ClosedMul: Sized + Mul<Output = Self> {}
impl<T: Mul<Output = T>> ClosedMul for T {}
