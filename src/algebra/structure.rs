//! 代数的構造に関するモジュール。
use cargo_snippet::snippet;

/// マグマ
#[snippet("algebra-structure")]
pub trait Magma: Default {
    type Set: Eq;

    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set;
}

/// 全ての元がマグマの演算に関して逆元を持つことを示すトレイト。
#[snippet("algebra-structure")]
pub trait Recip: Magma {
    fn recip(&self, x: Self::Set) -> Self::Set;
}

/// 一部の元がマグマの演算に関して逆元を持つことを示すトレイト。
#[snippet("algebra-structure")]
pub trait PartialRecip: Magma {
    fn partial_recip(&self, x: Self::Set) -> Option<Self::Set>;
}

/// 結合法則を満たすことを示すトレイト。
#[snippet("algebra-structure")]
pub trait Associative: Magma {}

/// 可換法則を満たすことを示すトレイト。
#[snippet("algebra-structure")]
pub trait Commutative: Magma {}

/// 分配法則を満たすことを示すトレイト。
#[snippet("algebra-structure")]
pub trait Distributive<A: Magma> {}

/// 半群
#[snippet("algebra-structure")]
pub trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// 単位元
#[snippet("algebra-structure")]
pub trait Identity: Magma {
    fn id(&self) -> Self::Set;
}

/// モノイド
#[snippet("algebra-structure")]
pub trait Monoid: SemiGroup + Identity {}
impl<T: SemiGroup + Identity> Monoid for T {}

/// 群
#[snippet("algebra-structure")]
pub trait Group: Monoid + Recip {}
impl<T: Monoid + Recip> Group for T {}

/// 可換群
#[snippet("algebra-structure")]
pub trait CommutativeGroup: Group + Commutative {}
impl<T: Group + Commutative> CommutativeGroup for T {}

/// 環
#[snippet("algebra-structure")]
pub trait Ring {}

/// 体(未実装)
#[snippet("algebra-structure")]
pub trait Field {}
