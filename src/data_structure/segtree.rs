//! セグメント木。
use cargo_snippet::snippet;

use super::super::algebra::structure::Monoid;
use super::traits::{BisectFold, BisectFoldRev, Fold, SetValue};
use crate::utils::integer::ceil_pow2;

#[snippet("data_structure/segtree")]
use std::ops::{Index, Range};

/// セグメント木。
///
/// モノイドに対して、
/// - 要素の一点更新
/// - 区間の要素の積の取得
///
/// を `O(log N)` で行うことができる。
#[snippet("data_structure/segtree")]
pub struct SegTree<M: Monoid> {
    len: usize,
    log_size: usize,
    size: usize,
    data: Vec<M::Set>,
    monoid: M,
}

#[snippet("data_structure/segtree")]
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

#[snippet("data_structure/segtree")]
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

#[snippet("data_structure/segtree")]
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

#[snippet("data_structure/segtree")]
impl<M> Index<usize> for SegTree<M>
where
    M: Monoid,
{
    type Output = M::Set;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.size + i]
    }
}

#[snippet("data_structure/segtree")]
impl<M> Fold for SegTree<M>
where
    M: Monoid,
    M::Set: Copy,
{
    type Output = M::Set;

    fn fold(&self, r: Range<usize>) -> Self::Output {
        // r.is_empty() にしたい
        if r.start == r.end {
            return self.monoid.id();
        }
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
            lpos >>= 1;
            rpos >>= 1;
        }
        self.monoid.op(lv, rv)
    }
}

#[snippet("data_structure/segtree")]
impl<M> BisectFold<M::Set> for SegTree<M>
where
    M: Monoid,
    M::Set: Clone + Copy,
{
    fn bisect_fold<F>(&self, l: usize, f: F) -> usize
    where
        F: Fn(&M::Set) -> bool,
    {
        if l == self.len {
            return self.len;
        }

        let mut l = self.size + l;
        let mut v = self.monoid.id();
        loop {
            while l & 1 == 0 {
                l >>= 1;
            }
            if !(f(&self.monoid.op(v, self.data[l]))) {
                while l < self.size {
                    l <<= 1;
                    let val = self.monoid.op(v, self.data[l]);
                    if f(&val) {
                        v = val;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            v = self.monoid.op(v, self.data[l]);
            l += 1;
            if l & ((!l) + 1) == l {
                break;
            }
        }
        self.len
    }
}

#[snippet("data_structure/segtree")]
impl<M> BisectFoldRev<M::Set> for SegTree<M>
where
    M: Monoid,
    M::Set: Clone + Copy,
{
    fn bisect_fold_rev<F>(&self, r: usize, f: F) -> usize
    where
        F: Fn(&M::Set) -> bool,
    {
        if r == 0 {
            return 0;
        }

        let mut r = self.size + r;
        let mut v = self.monoid.id();

        loop {
            r -= 1;
            while r > 1 && r & 1 == 1 {
                r >>= 1;
            }
            if !(f(&self.monoid.op(self.data[r], v))) {
                while r < self.size {
                    r = 2 * r + 1;
                    let val = self.monoid.op(self.data[r], v);
                    if f(&val) {
                        v = val;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            v = self.monoid.op(self.data[r], v);
            if r & ((!r) + 1) == r {
                break;
            }
        }
        0
    }
}

/// デバッグやテスト用の素朴な実装。
#[derive(Debug, Clone)]
struct NaiveSegTree<M: Monoid> {
    len: usize,
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
            len: n,
            data: vec![monoid.id(); n],
            monoid,
        }
    }
}

impl<M: Monoid> From<Vec<M::Set>> for NaiveSegTree<M> {
    fn from(v: Vec<M::Set>) -> Self {
        Self {
            len: v.len(),
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

impl<M> BisectFold<M::Set> for NaiveSegTree<M>
where
    M: Monoid,
    M::Set: Clone + Copy,
{
    fn bisect_fold<F>(&self, l: usize, f: F) -> usize
    where
        F: Fn(&M::Set) -> bool,
    {
        let mut v = self.monoid.id();
        assert!(f(&v));

        let mut l = l;
        while l < self.len {
            v = self.monoid.op(v, self.data[l]);
            if !f(&v) {
                break;
            }
            l += 1;
        }
        l
    }
}

impl<M> BisectFoldRev<M::Set> for NaiveSegTree<M>
where
    M: Monoid,
    M::Set: Clone + Copy,
{
    fn bisect_fold_rev<F>(&self, r: usize, f: F) -> usize
    where
        F: Fn(&M::Set) -> bool,
    {
        let mut v = self.monoid.id();
        assert!(f(&v));

        let mut r = r;
        while r > 0 {
            r -= 1;
            v = self.monoid.op(self.data[r], v);
            if !f(&v) {
                r += 1;
                break;
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::op_add::OpAdd;
    use crate::algebra::op_max::OpMax;
    use crate::algebra::op_min::OpMin;
    use crate::algebra::op_mul::OpMul;

    use super::*;

    // [0, 1, 4, 9, 16]

    #[test]
    fn test_segtree_op_add() {
        let len = 5;

        macro_rules! test_impl {
            ($($t:ident)*) => ($(
                let mut seg = $t::<OpAdd<usize>>::new(len);

                for i in 0..5 {
                    seg.set(i, i * i);
                }

                assert_eq!(seg[0], 0);
                assert_eq!(seg[2], 4);

                assert_eq!(seg.fold(0..0), 0);
                assert_eq!(seg.fold(0..3), 5);
                assert_eq!(seg.fold(1..3), 5);

                assert_eq!(seg.bisect_fold(0, |&x| x <= 5), 3);
                assert_eq!(seg.bisect_fold(0, |&x| x < 5), 2);

                assert_eq!(seg.bisect_fold_rev(5, |&x| x <= 25), 3);
                assert_eq!(seg.bisect_fold_rev(5, |&x| x < 25), 4);
            )*)
        }

        test_impl! { NaiveSegTree SegTree }
    }

    #[test]
    fn test_segtree_op_mul() {
        let len = 5;

        macro_rules! test_impl {
            ($($t:ident)*) => ($(
                let mut seg = $t::<OpMul<usize>>::new(len);

                for i in 0..5 {
                    seg.set(i, i * i);
                }

                assert_eq!(seg[0], 0);
                assert_eq!(seg[2], 4);

                assert_eq!(seg.fold(0..0), 1);
                assert_eq!(seg.fold(0..3), 0);
                assert_eq!(seg.fold(1..3), 4);

            )*)
        }
        test_impl! { NaiveSegTree SegTree }
    }

    #[test]
    fn test_segtree_op_min() {
        let len = 5;

        macro_rules! test_impl {
            ($($t:ident)*) => ($(
                let mut seg = $t::<OpMin<usize>>::new(len);

                for i in 0..5 {
                    seg.set(i, i * i);
                }

                assert_eq!(seg[0], 0);
                assert_eq!(seg[2], 4);

                assert_eq!(seg.fold(0..0), std::usize::MAX);
                assert_eq!(seg.fold(0..3), 0);
                assert_eq!(seg.fold(1..3), 1);

            )*)
        }
        test_impl! { NaiveSegTree SegTree }
    }

    #[test]
    fn test_segtree_op_max() {
        let len = 5;

        macro_rules! test_impl {
            ($($t:ident)*) => ($(
                let mut seg = $t::<OpMax<usize>>::new(len);

                for i in 0..5 {
                    seg.set(i, i * i);
                }

                assert_eq!(seg[0], 0);
                assert_eq!(seg[2], 4);

                assert_eq!(seg.fold(0..0), 0);
                assert_eq!(seg.fold(0..3), 4);
                assert_eq!(seg.fold(1..3), 4);

            )*)
        }
        test_impl! { NaiveSegTree SegTree }
    }
}
