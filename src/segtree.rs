//! セグメント木
use crate::algebraic_structure::Monoid;
use crate::utils::ceil_pow2;

pub struct SegTree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    len: usize,
    depth: usize,
    data: Vec<M::Set>,
}

impl<M> SegTree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    pub fn new(n: usize) -> Self {
        let depth = ceil_pow2(n);
        let data = vec![M::id(); 2 * (1 << depth)];
        Self {
            len: n,
            depth,
            data,
        }
    }
    /// データを初期化する。
    pub fn initialize(&mut self, arr: &Vec<M::Set>) {
        for (i, a) in arr.into_iter().enumerate() {
            self.data[1 << self.depth + i] = (*a).clone();
        }
        for i in (1..1 << self.depth).rev() {
            // self.update(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::segtree::*;

    #[test]
    fn test_segtree() {}
}
