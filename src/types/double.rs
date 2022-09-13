use crate::{error::ERR_CAST, Result};

/// 用于浮点型计算处理的工具类
pub trait DoubleCastExt {
    fn cast_to_f32(self) -> Result<f32>;
    fn cast_to_f64(self) -> Result<f64>;
}

macro_rules! double_cast_fn_impl {
    ($name:ident,$from:ident,$to:ident) => {
        fn $name(self) -> Result<$to> {
            if self <= $to::MAX as $from && self >= $to::MIN as $from {
                let res = self as $to;
                if res.is_finite() {
                    Ok(res)
                } else {
                    Err(ERR_CAST.msg_detail(format!("$from数据[{}]不能转换为$to类型", self)))
                }
            } else {
                Err(ERR_CAST.msg_detail(format!("$from数据[{}]不能转换为$to类型", self)))
            }
        }
    };
}
macro_rules! double_cast_impl {
    ($from:ident) => {
        impl DoubleCastExt for $from {
            double_cast_fn_impl!(cast_to_f32, $from, f32);
            double_cast_fn_impl!(cast_to_f64, $from, f64);
        }
    };
}
double_cast_impl!(f32);
double_cast_impl!(f64);
