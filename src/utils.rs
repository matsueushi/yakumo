pub fn ceil_pow2(n: usize) -> usize {
    let mut x: usize = 0;
    while (1 << x) < n {
        x += 1;
    }
    x
}

#[cfg(test)]
mod tests {
    use crate::utils::ceil_pow2;

    #[test]
    fn test_ceil_pow2() {
        assert_eq!(0, ceil_pow2(0));
        assert_eq!(0, ceil_pow2(1));
        assert_eq!(1, ceil_pow2(2));
        assert_eq!(2, ceil_pow2(3));
        assert_eq!(2, ceil_pow2(4));
        assert_eq!(3, ceil_pow2(5));
    }
}
