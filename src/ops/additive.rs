//! Add, Neg が満たす条件を表現するためのトレイトたち。
use std::ops::{Add, Neg};

pub trait ClosedAdd: Sized + Add<Output = Self> {}
impl<T: Add<Output = T>> ClosedAdd for T {}

pub trait ClosedNeg: Sized + Neg<Output = Self> {}
impl<T: Neg<Output = T>> ClosedNeg for T {}
