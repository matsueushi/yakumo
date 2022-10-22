use cargo_snippet::snippet;

use super::super::math::gcd::Gcd;

/// 直線を表す構造体。
#[snippet("geom/line")]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

#[snippet("geom/line")]
macro_rules! line_impl {
    ($($t:ty)*) => ($(
        impl Line<$t> {
            pub fn normalize(&self) -> Self {
                let mut a = self.a;
                let mut b = self.b;
                let mut c = self.c;
                if a < 0 || (a==0 && b<0) {
                    a = -a;
                    b = -b;
                    c = -c;
                }
                let g = a.gcd(b);
                Self { a: a / g, b: b / g, c: c / g}
            }
        }
    )*)
}

#[snippet("geom/line")]
line_impl! { isize i8 i16 i32 i64 i128 }
