//! 剰余演算を行うモジュール。
use cargo_snippet::snippet;

#[snippet("math/mod_int")]
use std::fmt::{Debug, Display};
#[snippet("math/mod_int")]
use std::marker::PhantomData;
#[snippet("math/mod_int")]
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[snippet("math/mod_int")]
use super::super::algebra::additive::{AddAssoc, AddComm};
#[snippet("math/mod_int")]
use super::super::algebra::multiplicative::{MulAssoc, MulComm, PartialMulRecip};

#[snippet("math/mod_int")]
use super::gcd::ext_gcd;

#[snippet("math/mod_int")]
pub trait Modulo {
    /// 法。
    fn modulo() -> i64;
}

/// 有限体。
#[snippet("math/mod_int")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FiniteField<M>(i64, PhantomData<M>);

#[snippet("math/mod_int")]
impl<M: Modulo> Display for FiniteField<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mod {}", self.0, M::modulo())
    }
}

#[snippet("math/mod_int")]
impl<M: Modulo> Debug for FiniteField<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mod {}", self.0, M::modulo())
    }
}

#[snippet("math/mod_int")]
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

#[snippet("math/mod_int")]
impl<M: Modulo> Add for FiniteField<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.val() + rhs.val())
    }
}

#[snippet("math/mod_int")]
impl<M: Modulo> AddAssign for FiniteField<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.0 = self.0.rem_euclid(M::modulo())
    }
}

#[snippet("math/mod_int")]
impl<M: Modulo> Sub for FiniteField<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.val() - rhs.val())
    }
}

#[snippet("math/mod_int")]
impl<M: Modulo> SubAssign for FiniteField<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.0 = self.0.rem_euclid(M::modulo())
    }
}

#[snippet("math/mod_int")]
impl<M: Modulo> Mul for FiniteField<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.val() * rhs.val())
    }
}

#[snippet("math/mod_int")]
impl<M: Modulo> MulAssign for FiniteField<M> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.0 = self.0.rem_euclid(M::modulo())
    }
}

impl<M: Modulo> Div for FiniteField<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        match rhs.partial_mul_recip() {
            Some(v) => self * v,
            None => panic!("cannot devide by 0"),
        }
    }
}

#[snippet("math/mod_int")]
impl<M: Modulo> DivAssign for FiniteField<M> {
    fn div_assign(&mut self, rhs: Self) {
        match rhs.partial_mul_recip() {
            Some(v) => *self *= v,
            None => panic!("cannot devide by 0"),
        }
    }
}

#[snippet("math/mod_int")]
impl<M: Modulo> PartialMulRecip for FiniteField<M> {
    fn partial_mul_recip(self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            let v = self.val();
            let m = M::modulo();
            let (_, x, _) = ext_gcd(v, m);
            let inv = x.rem_euclid(m);
            Some(Self::new(inv))
        }
    }
}

#[snippet("math/mod_int")]
impl<M: Modulo> AddAssoc for FiniteField<M> {}
#[snippet("math/mod_int")]
impl<M: Modulo> AddComm for FiniteField<M> {}
#[snippet("math/mod_int")]
impl<M: Modulo> MulAssoc for FiniteField<M> {}
#[snippet("math/mod_int")]
impl<M: Modulo> MulComm for FiniteField<M> {}

/// mod を定義するためのマクロ。
#[snippet("math/mod_int")]
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
    use super::*;

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
        // Add
        assert_eq!(F::new(1) + F::new(2), F::new(3));

        // AddAssign
        let mut x = F::new(1);
        x += F::new(2);
        assert_eq!(x, F::new(3));

        // Sub
        assert_eq!(F::new(1) - F::new(2), F::new(P - 1));

        // SubAssign
        let mut x = F::new(1);
        x -= F::new(2);
        assert_eq!(x, F::new(P - 1));

        // Mul
        assert_eq!(F::new(2) * F::new(3), F::new(6));

        // MulAssign
        let mut x = F::new(2);
        x *= F::new(3);
        assert_eq!(x, F::new(6));

        // Div
        assert_eq!(F::new(2) / F::new(3), F::new(666666672));

        // DivAssign
        let mut x = F::new(2);
        x /= F::new(3);
        assert_eq!(x, F::new(666666672));

        // PartialMulRecip
        assert_eq!(F::new(0).partial_mul_recip(), None);
        assert_eq!(F::new(2).partial_mul_recip(), Some(F::new(500000004)));
    }
}
