//! セグメント木

#[cfg(test)]
mod tests {
    use super::super::fold::*;
    use std::ops::{Index, Range};

    struct NaiveSegTree<T> {
        data: Vec<T>,
    }

    impl NaiveSegTree<usize> {
        pub fn new(n: usize) -> Self {
            Self { data: vec![0; n] }
        }

        pub fn set(&mut self, i: usize, val: usize) {
            self.data[i] = val;
        }
    }

    impl Index<usize> for NaiveSegTree<usize> {
        type Output = usize;
        fn index(&self, i: usize) -> &Self::Output {
            &self.data[i]
        }
    }

    impl Fold<usize> for NaiveSegTree<usize> {
        fn fold(&self, r: Range<usize>) -> usize {
            let mut x = std::usize::MAX;
            for i in r {
                x = x.min(self.data[i]);
            }
            x
        }
    }

    #[test]
    fn test_naive_segtree_interface() {
        let mut naive_seg = NaiveSegTree::<usize>::new(5);

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
