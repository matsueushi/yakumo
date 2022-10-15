//! フェニック木。
use std::ops::{AddAssign, Range, Sub};

/// フェニック木。
/// * 一点加算
/// * 区間取得
/// の二つのクエリを `O(log N)` で処理できるデータ構造
pub struct FenwickTree<T> {
    len: usize,
    data: Vec<T>,
    e: T,
}

impl<T> FenwickTree<T>
where
    T: Copy + Clone + AddAssign<T> + Sub<Output = T>,
{
    /// 長さ n のフェニック木を作成する。
    pub fn new(n: usize, e: T) -> Self {
        Self {
            len: n,
            data: vec![e; n],
            e,
        }
    }

    /// a[i] += val という更新を行う。
    pub fn add(&mut self, i: usize, val: T) {
        let mut i = i + 1;
        while i <= self.len {
            self.data[i - 1] += val;
            i += i & (!i + 1);
        }
    }

    pub fn prefix_sum(&self, r: usize) -> T {
        let mut s = self.e;
        let mut idx = r;
        while idx > 0 {
            s += self.data[idx - 1];
            idx -= idx & (!idx + 1);
        }
        s
    }

    /// 半開区間上の和を計算する。
    pub fn sum(&self, r: Range<usize>) -> T {
        self.prefix_sum(r.end) - self.prefix_sum(r.start)
    }
}

#[cfg(test)]
mod tests {
    use crate::fenwick::*;

    #[test]
    fn test_fenwick_basic() {
        let mut fenwick = FenwickTree::new(5, 0);
        for i in 0..5 {
            fenwick.add(i, i);
        }
        assert_eq!(fenwick.sum(0..0), 0);
        assert_eq!(fenwick.sum(1..3), 3);
        assert_eq!(fenwick.sum(0..5), 10);
    }

    #[test]
    fn test_fenwick_square() {
        for n in 0..=50 {
            let mut fenwick = FenwickTree::new(n, 0);
            for i in 0..n {
                fenwick.add(i, i * i);
            }
            for l in 0..=n {
                for r in l..=n {
                    let mut s = 0;
                    for i in l..r {
                        s += i * i;
                    }
                    assert_eq!(fenwick.sum(l..r), s);
                }
            }
        }
    }
}
