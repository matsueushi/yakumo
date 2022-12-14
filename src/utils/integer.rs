use cargo_snippet::snippet;

#[snippet("utils/ceil_pow2")]
pub fn ceil_pow2(n: usize) -> usize {
    let mut x: usize = 0;
    while (1 << x) < n {
        x += 1;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::ceil_pow2;

    #[test]
    fn test_ceil_pow2() {
        assert_eq!(ceil_pow2(0), 0);
        assert_eq!(ceil_pow2(1), 0);
        assert_eq!(ceil_pow2(2), 1);
        assert_eq!(ceil_pow2(3), 2);
        assert_eq!(ceil_pow2(4), 2);
        assert_eq!(ceil_pow2(5), 3);
    }
}
