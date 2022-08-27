//! 用于数值计算处理的工具类
use crate::{error::ERR_CAST, Result};

/// i32转i64
pub fn cast_i32_to_i64(n: i32) -> Result<i64> {
    Ok(n as i64)
}

/// u64转u16，数据越界则提示错误信息
pub fn cast_u64_to_u16(n: u64) -> Result<u16> {
    if n <= u16::max_value() as u64 {
        Ok(n as u16)
    } else {
        Err(ERR_CAST.msg_detail(format!("数据{}不能转换为u16类型", n)))
    }
}

/// u64转u32，数据越界则提示错误信息
pub fn cast_u64_to_u32(n: u64) -> Result<u32> {
    if n <= u32::max_value() as u64 {
        Ok(n as u32)
    } else {
        Err(ERR_CAST.msg_detail(format!("数据{}不能转换为u32类型", n)))
    }
}

/// u64转i64，数据越界则提示错误信息
pub fn cast_u64_to_i64(n: u64) -> Result<i64> {
    if n <= i64::max_value() as u64 && n >= i64::min_value() as u64 {
        Ok(n as i64)
    } else {
        Err(ERR_CAST.msg_detail(format!("数据{}不能转换为i64类型", n)))
    }
}

/// u64转usize，数据越界则提示错误信息
pub fn cast_u64_to_usize(n: u64) -> Result<usize> {
    if n <= usize::max_value() as u64 {
        Ok(n as usize)
    } else {
        Err(ERR_CAST.msg_detail(format!("数据{}不能转换为usize类型", n)))
    }
}

/// i64转i32，数据越界则提示错误信息
pub fn cast_i64_to_i32(n: i64) -> Result<i32> {
    if n <= i32::max_value() as i64 && n >= i32::min_value() as i64 {
        Ok(n as i32)
    } else {
        Err(ERR_CAST.msg_detail(format!("数据{}不能转换为i32类型", n)))
    }
}

/// i64转i16，数据越界则提示错误信息
pub fn cast_i64_to_i16(n: i64) -> Result<i16> {
    if n <= i16::max_value() as i64 && n >= i16::min_value() as i64 {
        Ok(n as i16)
    } else {
        Err(ERR_CAST.msg_detail(format!("数据{}不能转换为i16类型", n)))
    }
}
