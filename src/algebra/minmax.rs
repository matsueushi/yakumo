pub trait Min {
    fn min() -> Self;
}

pub trait Max {
    fn max() -> Self;
}

macro_rules! min_max_int_impl {
    ($($t:ident)*) => ($(

        impl Min for $t {
            fn min() -> Self {
                std::$t::MIN
            }
        }

        impl Max for $t {
            fn max() -> Self {
                std::$t::MAX
            }
        }

    )*)
}

min_max_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
