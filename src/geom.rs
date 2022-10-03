//! 幾何に関係するもの
use num::Integer; // 将来的には外したい

/// 座標上の点を表現するための構造体。
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

macro_rules! point_impl {
    ($($t:ty)*) => ($(
        impl Point<$t> {
            /// 二点を通る直線を構成する。
            pub fn line(&self, other: &Self) -> Line<$t> {
                let (x0, y0) = (self.x, self.y);
                let (x1, y1) = (other.x, other.y);
                let a = x1 - x0;
                let b = -y1 + y0;
                let c = x0 * y1 - x1 * y0;
                Line { a, b, c }
            }
        }
    )*)
}

point_impl! { isize i8 i16 i32 i64 i128 }

/// 直線を表す構造体。
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

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
                let g = a.gcd(&b);
                Self {a:a/g,b:b/g,c:c/g}
            }
        }
    )*)
}

line_impl! { isize i8 i16 i32 i64 i128 }