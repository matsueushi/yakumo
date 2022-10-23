//! データ構造が利用するトレイト。
use cargo_snippet::snippet;

#[snippet("data_structure/traits")]
use std::ops::Range;

/// インデックスを指定して値がセットできる
#[snippet("data_structure/traits")]
pub trait SetValue<T> {
    /// 値をセットする。
    fn set(&mut self, index: usize, val: T);
}

/// Fold 演算
///
/// <https://en.wikipedia.org/wiki/Fold_%28higher-order_function%29>
#[snippet("data_structure/traits")]
pub trait Fold {
    type Output;

    /// fold 演算を行う。
    fn fold(&self, r: Range<usize>) -> Self::Output;
}

#[snippet("data_structure/traits")]
pub trait BisectFold<S> {
    fn bisect_fold<F>(&self, l: usize, pred: F) -> usize
    where
        F: Fn(&S) -> bool;
}

#[snippet("data_structure/traits")]
pub trait BisectFoldRev<S> {
    fn bisect_fold_rev<F>(&self, r: usize, pred: F) -> usize
    where
        F: Fn(&S) -> bool;
}
