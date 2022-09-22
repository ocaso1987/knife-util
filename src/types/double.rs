use crate::{error::ERR_CAST, Result, OK};

/// 用于浮点型计算处理的工具类
pub trait DoubleExt {
    fn cast_to_f32(self) -> Result<f32>;
    fn cast_to_f64(self) -> Result<f64>;
}

pub(crate) trait DoubleFrom<Src> {
    type Output;
    fn cast(src: Src) -> Self::Output;
}

macro_rules! promotion {
    ($($src:ident => $($dst: ident),+);+;) => {
        $(
            $(
                impl DoubleFrom<$src> for $dst {
                    type Output=Result<$dst>;
                    fn cast(src: $src) -> Self::Output{
                        OK(src as $dst)
                    }
                }
            )+
        )+
    }
}

macro_rules! from_high_float {
    ($($src:ident => $($dst: ident),+);+;) => {
        $(
            $(
                impl DoubleFrom<$src> for $dst {
                    type Output=Result<$dst>;
                    fn cast(src: $src) -> Self::Output{
                        if src != src {
                            Err(ERR_CAST.msg_detail(
                                format!(
                                    "数据类型{}不能转换为{}类型,值为{},空值异常",
                                    std::any::type_name::<$src>(),
                                    std::any::type_name::<$dst>(),
                                    src
                                )
                                .as_str(),
                            ))
                        } else if src == $src::INFINITY || src == $src::NEG_INFINITY {
                            Err(ERR_CAST.msg_detail(
                                format!(
                                    "数据类型{}不能转换为{}类型,值为{},无穷值异常",
                                    std::any::type_name::<$src>(),
                                    std::any::type_name::<$dst>(),
                                    src
                                )
                                .as_str(),
                            ))
                        } else if src < $dst::MIN as $src {
                            Err(ERR_CAST.msg_detail(
                                format!(
                                    "数据类型{}不能转换为{}类型,值为{},下标越界",
                                    std::any::type_name::<$src>(),
                                    std::any::type_name::<$dst>(),
                                    src
                                )
                                .as_str(),
                            ))
                        } else  {
                            return OK(src as $dst);
                        }
                    }
                }
            )+
        )+
    }
}

promotion! {
    f32   => f32, f64;
    f64   =>      f64;
}

from_high_float! {
    f64   => f32     ;
}

macro_rules! double_cast_fn_impl {
    ($name:ident,$src:ident,$dst:ident) => {
        fn $name(self) -> Result<$dst> {
            <$dst as DoubleFrom<$src>>::cast(self)
        }
    };
}

macro_rules! double_cast_impl {
    ($src:ident) => {
        impl DoubleExt for $src {
            double_cast_fn_impl!(cast_to_f32, $src, f32);
            double_cast_fn_impl!(cast_to_f64, $src, f64);
        }
    };
}
double_cast_impl!(f32);
double_cast_impl!(f64);

#[cfg(test)]
mod tests {
    use crate::types::DoubleExt;

    #[test]
    fn test_cast() {
        let r1 = 1.0.cast_to_f32();
        assert_eq!(r1.unwrap(), 1.00 as f32);
    }
}
