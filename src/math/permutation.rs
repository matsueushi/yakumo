//! 並び替えに関するモジュール。

pub fn next_permutation<T: PartialOrd>(nums: &mut [T]) -> bool {
    let last_asc = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        None => {
            nums.reverse();
            return false;
        }
        Some(i) => i,
    };
    match nums[last_asc + 1..]
        .iter()
        .rposition(|x| x > &nums[last_asc])
    {
        None => return false,
        Some(i) => {
            nums.swap(last_asc, last_asc + 1 + i);
            nums[last_asc + 1..].reverse();
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::math::permutation::*;

    #[test]
    fn test_next_permutation() {
        let mut nums = vec![1, 2, 3];
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![1, 3, 2]);
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![2, 1, 3]);
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![2, 3, 1]);
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![3, 1, 2]);
        assert!(next_permutation(&mut nums));
        assert_eq!(nums, vec![3, 2, 1]);
        assert!(!next_permutation(&mut nums));
        assert_eq!(nums, vec![1, 2, 3]);
    }
}
