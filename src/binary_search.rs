use std::cmp::Ordering::{Greater, Less};

/// AtCoderで使える Rust のバージョンが上がったら必要なくなるかも
trait BinarySearch<T> {
    fn search_sorted_first(&self, x: &T) -> usize;
    fn search_sorted_last(&self, x: &T) -> usize;
}

impl<T> BinarySearch<T> for [T]
where
    T: PartialOrd<T>,
{
    /// スライスがソートされていると仮定する。`x` 以上の値を持つ最初のインデックスを返す
    fn search_sorted_first(&self, x: &T) -> usize {
        self.binary_search_by(|v| if v < x { Less } else { Greater })
            .unwrap_or_else(|i| i)
    }
    /// スライスがソートされていると仮定する。`x` 以下の値を持つ最後のインデックスを返す
    fn search_sorted_last(&self, x: &T) -> usize {
        self.binary_search_by(|v| if v <= x { Less } else { Greater })
            .unwrap_or_else(|i| i)
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearch;

    #[test]
    fn test_search_sorted_first() {
        let v = [1, 2, 4, 5, 5, 7];
        assert_eq!(v.search_sorted_first(&4), 2);
        assert_eq!(v.search_sorted_first(&5), 3);
        assert_eq!(v.search_sorted_first(&3), 2);
        assert_eq!(v.search_sorted_first(&9), 6);
        assert_eq!(v.search_sorted_first(&0), 0);
    }

    #[test]
    fn test_search_sorted_last() {
        let v = [1, 2, 4, 5, 5, 7];
        assert_eq!(v.search_sorted_last(&4), 3);
        assert_eq!(v.search_sorted_last(&5), 5);
        assert_eq!(v.search_sorted_last(&3), 2);
        assert_eq!(v.search_sorted_last(&9), 6);
        assert_eq!(v.search_sorted_last(&0), 0);
    }
}
