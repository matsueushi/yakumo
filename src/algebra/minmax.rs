//! 最小値、最大値
use cargo_snippet::snippet;

#[snippet("algebra/minmax")]
pub trait Min {
    fn min() -> Self;
}

#[snippet("algebra/minmax")]
pub trait Max {
    fn max() -> Self;
}

#[snippet("algebra/minmax")]
macro_rules! min_max_int_impl {
    ($($t:ident)*) => ($(
        impl Min for $t {
            fn min() -> Self {
                // <$t>::MIN に置き換えたい
                std::$t::MIN
            }
        }

        impl Max for $t {
            fn max() -> Self {
                // <$t>::MAX に置き換えたい
                std::$t::MAX
            }
        }

    )*)
}

#[snippet("algebra/minmax")]
min_max_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
