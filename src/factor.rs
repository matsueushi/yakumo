//! 因数分解に関するモジュール。

use std::collections::HashMap;

/// 因数分解
///
/// 与えられた数を因数分解し、キーを素数、値を指数とする`HashMap`として返す。
pub fn factorization(n: usize) -> HashMap<usize, usize> {
    let mut factors = HashMap::new();
    let mut i = 1;
    let mut x = n;
    loop {
        i += 1;
        if i * i > n {
            break;
        }
        while x % i == 0 {
            *factors.entry(i).or_insert(0) += 1;
            x /= i;
        }
    }
    // n が素数の場合を考慮する
    if x != 1 {
        factors.insert(x, 1);
    }
    factors
}

#[cfg(test)]
mod tests {
    use crate::factor::factorization;
    use std::collections::HashMap;

    #[test]
    fn test_factorization() {
        assert_eq!(factorization(1), HashMap::new());
        assert_eq!(factorization(7), vec![(7, 1)].into_iter().collect());
        assert_eq!(factorization(8), vec![(2, 3)].into_iter().collect());
    }
}
