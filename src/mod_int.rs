//! 剰余演算を行うモジュール。

use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Modulo {
    fn modulo() -> i64;
}

/// 有限体。
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FiniteField<M>(i64, PhantomData<M>);

impl<M: Modulo> Display for FiniteField<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mod {}", self.0, M::modulo())
    }
}

impl<M: Modulo> Debug for FiniteField<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mod {}", self.0, M::modulo())
    }
}

impl<M: Modulo> FiniteField<M> {
    /// 整数を `0 <= x < m` に正規化してインスタンスを作成する。
    pub fn new(x: i64) -> Self {
        let v = x.rem_euclid(M::modulo());
        Self(v, PhantomData)
    }

    /// `0 <= x < m` となる代表元を返す。
    pub fn val(self) -> i64 {
        self.0
    }

    /// 二分累乗法で `x^exp % m` を計算する。
    pub fn pow(self, exp: u32) -> Self {
        if exp == 0 {
            return Self::new(1);
        }
        let mut x = self.0;
        let mut res = 1;
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                res *= x;
                res %= M::modulo()
            }
            x *= x;
            x %= M::modulo();
            exp >>= 1;
        }
        Self::new(res)
    }
}

impl<M: Modulo> Add for FiniteField<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.val() + rhs.val())
    }
}

impl<M: Modulo> AddAssign for FiniteField<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.0 = self.0.rem_euclid(M::modulo())
    }
}

impl<M: Modulo> Sub for FiniteField<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.val() - rhs.val())
    }
}

impl<M: Modulo> SubAssign for FiniteField<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.0 = self.0.rem_euclid(M::modulo())
    }
}

impl<M: Modulo> Mul for FiniteField<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.val() * rhs.val())
    }
}

impl<M: Modulo> MulAssign for FiniteField<M> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.0 = self.0.rem_euclid(M::modulo())
    }
}

/// mod を定義するためのマクロ。
#[macro_export]
macro_rules! modulo_impl {
    ($i: ident, $m: expr) => {
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct $i;
        impl Modulo for $i {
            fn modulo() -> i64 {
                $m
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::mod_int::*;

    const P: i64 = 1000000007;
    modulo_impl!(Mod1000000007, P);
    type F = FiniteField<Mod1000000007>;

    #[test]
    fn test_finite_field_eq() {
        assert_eq!(F::new(0), F::new(P));
    }

    #[test]
    fn test_finite_field_format() {
        let x = F::new(0);
        assert_eq!(format!("{:?}", x), "0 mod 1000000007");
        assert_eq!(format!("{:#?}", x), "0 mod 1000000007");
    }

    #[test]
    fn test_finite_field_pow() {
        use std::iter::repeat;

        let x = F::new(-1);
        assert_eq!(x.val(), P - 1);

        let exps = [0, 1, 10, 100, 1000, 10000];
        for exp in &exps {
            let y = repeat(3).take(*exp as usize).fold(1, |acc, x| acc * x % P);
            assert_eq!(F::new(3).pow(*exp).val(), y);
        }
    }

    #[test]
    fn test_finite_field_ops() {
        assert_eq!(F::new(1) + F::new(2), F::new(3));

        let mut x = F::new(1);
        x += F::new(2);
        assert_eq!(x, F::new(3));

        assert_eq!(F::new(1) - F::new(2), F::new(P - 1));

        let mut x = F::new(1);
        x -= F::new(2);
        assert_eq!(x, F::new(P - 1));

        assert_eq!(F::new(2) * F::new(3), F::new(6));

        let mut x = F::new(2);
        x *= F::new(3);
        assert_eq!(x, F::new(6));
    }
}
