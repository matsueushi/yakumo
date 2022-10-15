//! セグメント木

use super::super::algebra::structure::Monoid;
use super::fold::Fold;
use std::ops::{Index, Range};

/// デバッグやテスト用の素朴な実装。
struct NaiveSegTree<M: Monoid> {
    data: Vec<M::Set>,
    monoid: M,
}

impl<M> NaiveSegTree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let monoid = M::default();
        Self {
            data: vec![monoid.id(); n],
            monoid,
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, i: usize, val: M::Set) {
        self.data[i] = val;
    }
}

impl<M: Monoid> From<Vec<M::Set>> for NaiveSegTree<M> {
    fn from(v: Vec<M::Set>) -> Self {
        Self {
            data: v,
            monoid: M::default(),
        }
    }
}

impl<M: Monoid> Index<usize> for NaiveSegTree<M> {
    type Output = M::Set;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<M> Fold<M::Set> for NaiveSegTree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    fn fold(&self, r: Range<usize>) -> M::Set {
        let mut x = self.monoid.id();
        for i in r {
            x = self.monoid.op(x, self.data[i].clone());
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::op_add::OpAdd;
    use crate::algebra::op_min::OpMin;

    use super::*;

    #[test]
    fn test_naive_segtree_op_add() {
        let mut naive_seg = NaiveSegTree::<OpAdd<usize>>::new(5);

        for i in 0..5 {
            naive_seg.set(i, i * i);
        }

        assert_eq!(naive_seg[0], 0);
        assert_eq!(naive_seg[2], 4);

        assert_eq!(naive_seg.fold(0..0), 0);
        assert_eq!(naive_seg.fold(0..3), 5);
        assert_eq!(naive_seg.fold(1..3), 5);
    }

    #[test]
    fn test_naive_segtree_op_min() {
        let mut naive_seg = NaiveSegTree::<OpMin<usize>>::new(5);

        for i in 0..5 {
            naive_seg.set(i, i * i);
        }

        assert_eq!(naive_seg[0], 0);
        assert_eq!(naive_seg[2], 4);

        assert_eq!(naive_seg.fold(0..0), std::usize::MAX);
        assert_eq!(naive_seg.fold(0..3), 0);
        assert_eq!(naive_seg.fold(1..3), 1);
    }
}
