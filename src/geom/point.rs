use cargo_snippet::snippet;

use super::line::Line;

#[snippet("geom/point")]
/// 座標上の点を表現するための構造体。
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[snippet("geom/point")]
macro_rules! point_impl {
    ($($t:ty)*) => ($(
        impl Point<$t> {
            pub fn new(x: $t, y:$t) -> Self {
                Self { x, y }
            }

            /// 二点を通る直線を構成する。
            pub fn line(&self, other: &Self) -> Line<$t> {
                let (x0, y0) = (self.x, self.y);
                let (x1, y1) = (other.x, other.y);
                let a = y1 - y0;
                let b = -x1 + x0;
                let c = x0 * y1 - x1 * y0;
                Line { a, b, c }
            }
        }
    )*)
}

#[snippet("geom/point")]
point_impl! { isize i8 i16 i32 i64 i128 }

#[cfg(test)]
mod tests {
    use super::{Line, Point};

    #[test]
    fn test_point() {
        let p0 = Point::<i64>::new(0, 0);
        let p1 = Point::<i64>::new(1, 2);
        assert_eq!(Line { a: 2, b: -1, c: 0 }, p0.line(&p1));
    }
}
