//! フェニック木

/// フェニック木。
/// * 一点加算
/// * 区間取得
/// の二つのクエリを `O(log N)` で処理できるデータ構造
pub struct FenwickTree {
    len: usize,
    data: Vec<usize>,
}

impl FenwickTree {
    /// 長さ n のフェニック木を作成する。
    pub fn new(n: usize) -> Self {
        Self {
            len: n,
            data: vec![0; n],
        }
    }

    /// a[i] += val という更新を行う。
    pub fn add(&mut self, i: usize, val: usize) {
        let mut i = i + 1;
        while i <= self.len {
            self.data[i - 1] += val;
            i += i & (!i + 1);
        }
    }

    pub fn prefix_sum(&self, r: usize) -> usize {
        let mut s = 0;
        let mut idx = r;
        while idx > 0 {
            s += self.data[idx - 1];
            idx -= idx & (!idx + 1);
        }
        s
    }

    pub fn sum(&self, l: usize, r: usize) -> usize {
        self.prefix_sum(r) - self.prefix_sum(l)
    }
}
