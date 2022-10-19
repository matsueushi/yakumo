//! セグメント木

use super::super::algebra::structure::Monoid;
use super::super::utils::ceil_pow2;
use super::traits::{Fold, MaxRight, MinLeft, SetValue};
use std::ops::{Index, Range};

#[allow(dead_code)]
struct SegTree<M: Monoid> {
    len: usize,
    log_size: usize,
    size: usize,
    data: Vec<M::Set>,
    monoid: M,
}

#[allow(dead_code)]
impl<M> SegTree<M>
where
    M: Monoid,
    M::Set: Clone + Copy,
{
    pub fn new(n: usize) -> Self {
        let m = M::default();
        let log_size = ceil_pow2(n);
        let size = 1 << log_size;
        let data = vec![m.id(); 2 * size];
        Self {
            len: n,
            log_size,
            size,
            data,
            monoid: m,
        }
    }

    fn update(&mut self, i: usize) {
        self.data[i] = self.monoid.op(self.data[i << 1], self.data[(i << 1) + 1]);
    }
}

impl<M> From<Vec<M::Set>> for SegTree<M>
where
    M: Monoid,
    M::Set: Clone + Copy,
{
    fn from(v: Vec<M::Set>) -> Self {
        let mut seg = Self::new(v.len());
        for (i, val) in v.into_iter().enumerate() {
            seg.data[seg.size + i] = val.clone();
        }
        for i in (1..seg.size).rev() {
            seg.update(i);
        }
        seg
    }
}

impl<M> SetValue<M::Set> for SegTree<M>
where
    M: Monoid,
    M::Set: Clone + Copy,
{
    fn set(&mut self, i: usize, val: M::Set) {
        let i = self.size + i;
        self.data[i] = val;
        for j in 1..=self.log_size {
            self.update(i >> j);
        }
    }
}

impl<M> Index<usize> for SegTree<M>
where
    M: Monoid,
{
    type Output = M::Set;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.size + i]
    }
}

impl<M> Fold for SegTree<M>
where
    M: Monoid,
    M::Set: Copy,
{
    type Output = M::Set;

    fn fold(&self, r: Range<usize>) -> Self::Output {
        let mut lpos = self.size + r.start;
        let mut rpos = self.size + r.end;
        let mut lv = self.monoid.id();
        let mut rv = self.monoid.id();
        while lpos < rpos {
            if lpos & 1 == 1 {
                lv = self.monoid.op(lv, self.data[lpos]);
                lpos += 1;
            }
            if rpos & 1 == 1 {
                rpos -= 1;
                rv = self.monoid.op(self.data[rpos], rv);
            }
        }
        self.monoid.op(lv, rv)
    }
}

impl<M> MaxRight<M> for SegTree<M>
where
    M: Monoid,
{
    #[allow(dead_code)]
    fn max_right<F>(&self, _l: usize, _f: F) -> usize
    where
        F: Fn(&M) -> bool,
    {
        todo!()
    }
}

impl<M> MinLeft<M> for SegTree<M>
where
    M: Monoid,
{
    #[allow(dead_code)]
    fn min_left<F>(&self, _r: usize, _f: F) -> usize
    where
        F: Fn(&M) -> bool,
    {
        todo!()
    }
}

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
}

impl<M: Monoid> From<Vec<M::Set>> for NaiveSegTree<M> {
    fn from(v: Vec<M::Set>) -> Self {
        Self {
            data: v,
            monoid: M::default(),
        }
    }
}

impl<M: Monoid> SetValue<M::Set> for NaiveSegTree<M> {
    fn set(&mut self, i: usize, val: M::Set) {
        self.data[i] = val;
    }
}

impl<M: Monoid> Index<usize> for NaiveSegTree<M> {
    type Output = M::Set;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<M> Fold for NaiveSegTree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    type Output = M::Set;

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
