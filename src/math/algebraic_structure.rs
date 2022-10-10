//! 代数的構造に関するモジュール。

/// マグマ
pub trait Magma {
    type Set: Eq;

    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set;
}

/// 結合法則
pub trait Associative: Magma {}

/// 可換法則
pub trait Commutative: Magma {}

/// 半群
pub trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// 単位元
pub trait Identity: Magma {
    fn id() -> Self::Set;
}

/// モノイド
pub trait Monoid: SemiGroup + Identity {}
impl<T: SemiGroup + Identity> Monoid for T {}

/// 環(未実装)
pub trait Ring {}

/// 体(未実装)
pub trait Field {}
