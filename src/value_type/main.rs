use std::collections::BTreeMap;

use crate::{
    date::{Date, DateTime, Time, YearMonth},
    error::ERR_CAST,
    types::{DoubleExt, IntegerExt},
    Result, OK,
};

/// 用于处理程序内置对象的工具
///
/// 并负责对Json、Yaml、Toml等格式的对象进行处理及转换
/// 注意的是内置对象不支持反序列化操作
/// 如有该操作需求，需通过配置文件方式进行序列化/反序列化操作后进行数据合并
#[derive(Clone)]
pub enum Value {
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Date(Date),
    Time(Time),
    DateTime(DateTime),
    YearMonth(YearMonth),
    Binary(Vec<u8>),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl Value {
    pub fn is_null(&self) -> Result<bool> {
        match *self {
            Value::Null => OK(true),
            _ => OK(false),
        }
    }

    pub fn is_empty(&self) -> Result<bool> {
        match self {
            Value::Null => OK(true),
            Value::String(v) => OK(v.is_empty()),
            Value::Array(v) => OK(v.is_empty()),
            Value::Object(v) => OK(v.is_empty()),
            _ => {
                Err(ERR_CAST
                    .msg_detail(format!("Value数据[{:?}]无法处理is_empty操作", self).as_str()))
            }
        }
    }

    pub fn is_zero(&self) -> Result<bool> {
        match *self {
            Value::Null => OK(true),
            Value::I32(v) => OK(v == 0),
            Value::I64(v) => OK(v == 0),
            Value::U32(v) => OK(v == 0),
            Value::U64(v) => OK(v == 0),
            Value::F32(v) => OK(v == 0.0),
            Value::F64(v) => OK(v == 0.0),
            _ => {
                Err(ERR_CAST
                    .msg_detail(format!("Value数据[{:?}]无法处理is_zero操作", self).as_str()))
            }
        }
    }

    pub fn as_null(&self) -> Result<()> {
        match *self {
            Value::Null => OK(()),
            _ => {
                Err(ERR_CAST
                    .msg_detail(format!("Value数据[{:?}]不能转换为null类型", self).as_str()))
            }
        }
    }

    pub fn as_str(&self) -> Result<&str> {
        match *self {
            Value::String(ref v) => OK(v),
            _ => {
                Err(ERR_CAST
                    .msg_detail(format!("Value数据[{:?}]不能转换为&str类型", self).as_str()))
            }
        }
    }

    pub fn as_str_mut(&mut self) -> Result<&mut str> {
        match *self {
            Value::String(ref mut v) => OK(v),
            _ => Err(ERR_CAST
                .msg_detail(format!("Value数据[{:?}]不能转换为&mut str类型", self).as_str())),
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        match *self {
            Value::Bool(v) => OK(v),
            _ => {
                Err(ERR_CAST
                    .msg_detail(format!("Value数据[{:?}]不能转换为bool类型", self).as_str()))
            }
        }
    }

    pub fn as_i32(&self) -> Result<i32> {
        match *self {
            Value::I32(v) => OK(v),
            Value::I64(v) => OK(v.cast_to_i32()?),
            Value::U32(v) => OK(v.cast_to_i32()?),
            Value::U64(v) => OK(v.cast_to_i32()?),
            _ => {
                Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为i32类型", self).as_str()))
            }
        }
    }

    pub fn as_i64(&self) -> Result<i64> {
        match *self {
            Value::I32(v) => OK(v.cast_to_i64()?),
            Value::I64(v) => OK(v),
            Value::U32(v) => OK(v.cast_to_i64()?),
            Value::U64(v) => OK(v.cast_to_i64()?),
            _ => {
                Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为i64类型", self).as_str()))
            }
        }
    }

    pub fn as_u64(&self) -> Result<u64> {
        match *self {
            Value::I32(v) => OK(v.cast_to_u64()?),
            Value::I64(v) => OK(v.cast_to_u64()?),
            Value::U32(v) => OK(v.cast_to_u64()?),
            Value::U64(v) => OK(v),
            _ => {
                Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为u64类型", self).as_str()))
            }
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        match *self {
            Value::F32(v) => OK(v.cast_to_f64()?),
            Value::F64(v) => OK(v),
            _ => {
                Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为f64类型", self).as_str()))
            }
        }
    }

    pub fn as_binary(&self) -> Result<&Vec<u8>> {
        match *self {
            Value::Binary(ref v) => OK(v),
            _ => {
                Err(ERR_CAST
                    .msg_detail(format!("Value数据[{:?}]不能转换为Binary类型", self).as_str()))
            }
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(*self, Value::Array(_))
    }

    pub fn as_array(&self) -> Result<&Vec<Value>> {
        match *self {
            Value::Array(ref v) => OK(v),
            _ => Err(ERR_CAST
                .msg_detail(format!("Value数据[{:?}]不能转换为&Vec<Value>类型", self).as_str())),
        }
    }

    pub fn as_array_mut(&mut self) -> Result<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut v) => OK(v),
            _ => Err(ERR_CAST.msg_detail(
                format!("Value数据[{:?}]不能转换为&mut Vec<Value>类型", self).as_str(),
            )),
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(*self, Value::Object(_))
    }

    pub fn as_object(&self) -> Result<&BTreeMap<String, Value>> {
        match *self {
            Value::Object(ref v) => OK(v),
            _ => Err(ERR_CAST.msg_detail(
                format!(
                    "Value数据[{:?}]不能转换为&BTreeMap<String, Value>类型",
                    self
                )
                .as_str(),
            )),
        }
    }

    #[allow(clippy::cast_ref_to_mut)]
    pub fn as_object_mut(&self) -> Result<&mut BTreeMap<String, Value>> {
        match *self {
            Value::Object(ref v) => OK(unsafe {
                &mut *(v as *const BTreeMap<String, Value> as *mut BTreeMap<String, Value>)
            }),
            _ => Err(ERR_CAST.msg_detail(
                format!(
                    "Value数据[{:?}]不能转换为&mut BTreeMap<String, Value>类型",
                    self
                )
                .as_str(),
            )),
        }
    }

    pub fn as_date(&self) -> Result<Date> {
        match self {
            Value::String(v) => Date::parse_str(v.as_str()),
            Value::Date(v) => OK(*v),
            _ => {
                Err(ERR_CAST
                    .msg_detail(format!("Value数据[{:?}]不能转换为Date类型", self).as_str()))
            }
        }
    }

    pub fn as_datetime(&self) -> Result<DateTime> {
        match self {
            Value::String(v) => DateTime::parse_str(v.as_str()),
            Value::DateTime(v) => OK(*v),
            _ => Err(ERR_CAST
                .msg_detail(format!("Value数据[{:?}]不能转换为YearMonth类型", self).as_str())),
        }
    }

    pub fn as_time(&self) -> Result<Time> {
        match self {
            Value::String(v) => Time::parse_str(v.as_str()),
            Value::Time(v) => OK(*v),
            _ => {
                Err(ERR_CAST
                    .msg_detail(format!("Value数据[{:?}]不能转换为Time类型", self).as_str()))
            }
        }
    }

    pub fn as_year_month(&self) -> Result<YearMonth> {
        match self {
            Value::String(v) => YearMonth::parse_str(v.as_str()),
            Value::Date(v) => YearMonth::from_chrono_date(&v.to_chrono_date()),
            Value::DateTime(v) => YearMonth::from_chrono_date(&v.to_chrono_date()),
            Value::YearMonth(v) => OK(*v),
            _ => Err(ERR_CAST
                .msg_detail(format!("Value数据[{:?}]不能转换为YearMonth类型", self).as_str())),
        }
    }
}
