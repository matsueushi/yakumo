//! Fold 演算。

use std::ops::Range;

/// Fold 演算
/// https://en.wikipedia.org/wiki/Fold_%28higher-order_function%29
pub trait Fold<T> {
    /// fold 演算を行う。
    fn fold(&self, r: Range<usize>) -> T;
}
