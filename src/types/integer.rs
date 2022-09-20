use crate::{error::ERR_CAST, Result};

/// 用于整型型计算处理的工具类
pub trait IntegerExt {
    fn cast_to_i8(self) -> Result<i8>;
    fn cast_to_i16(self) -> Result<i16>;
    fn cast_to_i32(self) -> Result<i32>;
    fn cast_to_i64(self) -> Result<i64>;
    fn cast_to_u8(self) -> Result<u8>;
    fn cast_to_u16(self) -> Result<u16>;
    fn cast_to_u32(self) -> Result<u32>;
    fn cast_to_u64(self) -> Result<u64>;
}

pub(crate) trait IntegerFrom<Src> {
    type Output;
    fn cast(src: Src) -> Self::Output;
}

macro_rules! promotion {
    ($($src:ident => $($dst: ident),+);+;) => {
        $(
            $(
                impl IntegerFrom<$src> for $dst {
                    type Output=Result<$dst>;
                    fn cast(src: $src) -> Self::Output{
                        Ok(src as $dst)
                    }
                }
            )+
        )+
    }
}

macro_rules! half_promotion {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl IntegerFrom<$src> for $dst {
                    type Output=Result<$dst>;
                    fn cast(src: $src) -> Self::Output{
                        if src < 0 {
                            Err(ERR_CAST.msg_detail(
                                format!(
                                    "数据类型{}不能转换为{}类型,值为{}，下标小于0",
                                    std::any::type_name::<$src>(),
                                    std::any::type_name::<$dst>(),
                                    src
                                )
                                .as_str(),
                            ))
                        } else {
                            Ok(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

macro_rules! from_unsigned {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl IntegerFrom<$src> for $dst {
                    type Output=Result<$dst>;
                    fn cast(src: $src) -> Self::Output{
                        if src > $dst::MAX as $src {
                            Err(ERR_CAST.msg_detail(
                                format!(
                                    "数据类型{}不能转换为{}类型,值为{}，上标超过最大值",
                                    std::any::type_name::<$src>(),
                                    std::any::type_name::<$dst>(),
                                    src
                                )
                                .as_str(),
                            ))
                        } else {
                            Ok(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

macro_rules! from_signed {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl IntegerFrom<$src> for $dst {
                    type Output=Result<$dst>;
                    fn cast(src: $src) -> Self::Output{
                        if src < $dst::MIN as $src {
                            Err(ERR_CAST.msg_detail(
                                format!(
                                    "数据类型{}不能转换为{}类型,值为{}，下标超过最小值",
                                    std::any::type_name::<$src>(),
                                    std::any::type_name::<$dst>(),
                                    src
                                )
                                .as_str(),
                            ))
                        } else if src > $dst::MAX as $src {
                            Err(ERR_CAST.msg_detail(
                                format!(
                                    "数据类型{}不能转换为{}类型,值为{}，上标超过最大值",
                                    std::any::type_name::<$src>(),
                                    std::any::type_name::<$dst>(),
                                    src
                                )
                                .as_str(),
                            ))
                        } else {
                            Ok(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

promotion! {
    i8    => i8, i16, i32, i64                   ;
    i16   =>     i16, i32, i64                   ;
    i32   =>          i32, i64                   ;
    i64   =>               i64                   ;
    u8    =>     i16, i32, i64, u8, u16, u32, u64;
    u16   =>          i32, i64,     u16, u32, u64;
    u32   =>               i64,          u32, u64;
    u64   =>                                  u64;
}

half_promotion! {
    i8    =>                    u8, u16, u32, u64;
    i16   =>                        u16, u32, u64;
    i32   =>                             u32, u64;
    i64   =>                                  u64;
}

from_unsigned! {
    u8    => i8;
    u16   => i8, i16,           u8;
    u32   => i8, i16, i32,      u8, u16;
    u64   => i8, i16, i32, i64, u8, u16, u32     ;
}

from_signed! {
    i16   => i8,                u8;
    i32   => i8, i16,           u8, u16;
    i64   => i8, i16, i32,      u8, u16, u32     ;
}

macro_rules! integer_cast_fn_impl {
    ($name:ident,$src:ident,$dst:ident) => {
        fn $name(self) -> Result<$dst> {
            <$dst as IntegerFrom<$src>>::cast(self)
        }
    };
}

macro_rules! integer_cast_impl {
    ($src:ident) => {
        impl IntegerExt for $src {
            integer_cast_fn_impl!(cast_to_i8, $src, i8);
            integer_cast_fn_impl!(cast_to_i16, $src, i16);
            integer_cast_fn_impl!(cast_to_i32, $src, i32);
            integer_cast_fn_impl!(cast_to_i64, $src, i64);
            integer_cast_fn_impl!(cast_to_u8, $src, u8);
            integer_cast_fn_impl!(cast_to_u16, $src, u16);
            integer_cast_fn_impl!(cast_to_u32, $src, u32);
            integer_cast_fn_impl!(cast_to_u64, $src, u64);
        }
    };
}
integer_cast_impl!(i8);
integer_cast_impl!(i16);
integer_cast_impl!(i32);
integer_cast_impl!(i64);
integer_cast_impl!(u8);
integer_cast_impl!(u16);
integer_cast_impl!(u32);
integer_cast_impl!(u64);

#[cfg(test)]
mod tests {
    use super::IntegerExt;

    #[test]
    fn test_cast() {
        let r1 = 1.cast_to_i64();
        assert_eq!(r1.unwrap(), 1 as i64);
    }
}
