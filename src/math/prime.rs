//! 素数に関するモジュール。

/// エラトステネスの篩
///
/// `n` 以下の素数を返す。
///
/// # Examples
/// ```
/// use yakumo::math::prime::sieve;
///
/// assert_eq!(sieve(0), vec![]);
/// assert_eq!(sieve(1), vec![]);
/// assert_eq!(sieve(2), vec![2]);
/// assert_eq!(sieve(10), vec![2, 3, 5, 7]);
/// assert_eq!(sieve(11), vec![2, 3, 5, 7, 11]);
/// ```
pub fn sieve(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    if n < 2 {
        return primes;
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = true;
    is_prime[1] = true;
    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }
        primes.push(i);
        for j in (2 * i..=n).step_by(i) {
            is_prime[j] = false;
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::sieve;

    #[test]
    fn test_primes() {
        assert_eq!(sieve(0), vec![]);
        assert_eq!(sieve(1), vec![]);
        assert_eq!(sieve(2), vec![2]);
        assert_eq!(sieve(10), vec![2, 3, 5, 7]);
        assert_eq!(sieve(11), vec![2, 3, 5, 7, 11]);
    }
}
