use crate::{error::ERR_CAST, Result};

/// 用于整型型计算处理的工具类
pub trait IntegerCastExt {
    fn cast_to_i8(self) -> Result<i8>;
    fn cast_to_i16(self) -> Result<i16>;
    fn cast_to_i32(self) -> Result<i32>;
    fn cast_to_i64(self) -> Result<i64>;
    fn cast_to_isize(self) -> Result<isize>;
    fn cast_to_u8(self) -> Result<u8>;
    fn cast_to_u16(self) -> Result<u16>;
    fn cast_to_u32(self) -> Result<u32>;
    fn cast_to_u64(self) -> Result<u64>;
    fn cast_to_usize(self) -> Result<usize>;
}

macro_rules! integer_cast_fn_impl {
    ($name:ident,$from:ident,$to:ident) => {
        fn $name(self) -> Result<$to> {
            if self <= $to::MAX as $from && self >= $to::MIN as $from {
                Ok(self as $to)
            } else {
                Err(ERR_CAST.msg_detail(format!("$from数据[{}]不能转换为$to类型", self)))
            }
        }
    };
}
macro_rules! integer_cast_impl {
    ($from:ident) => {
        impl IntegerCastExt for $from {
            integer_cast_fn_impl!(cast_to_i8, $from, i8);
            integer_cast_fn_impl!(cast_to_i16, $from, i16);
            integer_cast_fn_impl!(cast_to_i32, $from, i32);
            integer_cast_fn_impl!(cast_to_i64, $from, i64);
            integer_cast_fn_impl!(cast_to_isize, $from, isize);
            integer_cast_fn_impl!(cast_to_u8, $from, u8);
            integer_cast_fn_impl!(cast_to_u16, $from, u16);
            integer_cast_fn_impl!(cast_to_u32, $from, u32);
            integer_cast_fn_impl!(cast_to_u64, $from, u64);
            integer_cast_fn_impl!(cast_to_usize, $from, usize);
        }
    };
}
integer_cast_impl!(i8);
integer_cast_impl!(i16);
integer_cast_impl!(i32);
integer_cast_impl!(i64);
integer_cast_impl!(isize);
integer_cast_impl!(u8);
integer_cast_impl!(u16);
integer_cast_impl!(u32);
integer_cast_impl!(u64);
integer_cast_impl!(usize);
