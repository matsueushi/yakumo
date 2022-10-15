//! Fold 演算。
use cargo_snippet::snippet;

#[snippet("data_strucuture/fold")]
use std::ops::Range;

/// Fold 演算
/// https://en.wikipedia.org/wiki/Fold_%28higher-order_function%29
#[snippet("data_strucuture/fold")]
pub trait Fold<T> {
    /// fold 演算を行う。
    fn fold(&self, r: Range<usize>) -> T;
}
