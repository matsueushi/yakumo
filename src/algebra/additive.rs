//! Add, Neg が満たす条件を表現するためのモジュール。
use cargo_snippet::snippet;

#[snippet("algebra/additive")]
use std::ops::{Add, Neg};

#[snippet("algebra/additive")]
pub trait ClosedAdd: Add<Output = Self> + Sized {}
#[snippet("algebra/additive")]
impl<T: Add<Output = T>> ClosedAdd for T {}

#[snippet("algebra/additive")]
pub trait ClosedNeg: Neg<Output = Self> + Sized {}
#[snippet("algebra/additive")]
impl<T: Neg<Output = T>> ClosedNeg for T {}
