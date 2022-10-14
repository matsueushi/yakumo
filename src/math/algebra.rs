//! 代数的構造に関するモジュール。

/// マグマ
pub trait Magma {
    type Set: Eq;

    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set;
}

/// 全ての元がマグマの演算に関して逆元を持つことを示すトレイト。
pub trait Recip: Magma {
    fn recip(&self, x: Self::Set) -> Self::Set;
}

/// 一部の元がマグマの演算に関して逆元を持つことを示すトレイト。
pub trait PartialRecip: Magma {
    fn partial_recip(&self, x: Self::Set) -> Option<Self::Set>;
}

/// 結合法則
pub trait Associative: Magma {}

/// 可換法則
pub trait Commutative: Magma {}

/// 分配法則
pub trait Distributive<A: Magma> {}

/// 半群
pub trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// 単位元
pub trait Identity: Magma {
    fn id(&self) -> Self::Set;
}

/// モノイド
pub trait Monoid: SemiGroup + Identity {}
impl<T: SemiGroup + Identity> Monoid for T {}

/// 群
pub trait Group: Monoid + Recip {}
impl<T: Monoid + Recip> Group for T {}

/// 可換群
pub trait CommutativeGroup: Group + Commutative {}
impl<T: Group + Commutative> CommutativeGroup for T {}

/// 環
pub trait Ring {}

/// 体(未実装)
pub trait Field {}
