//! 用于处理程序内置对象的工具
//!
//! 并负责对Json、Yaml、Toml等格式的对象进行处理及转换

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{DoubleCastTrait, IntegerCastTrait, Result, ERR_CAST};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl Value {
    pub fn as_f64(&self) -> Result<f64> {
        match *self {
            Value::F32(v) => Ok(v.cast_to_f64().unwrap()),
            Value::F64(v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为f64类型", self))),
        }
    }

    pub fn as_str(&self) -> Result<&str> {
        match *self {
            Value::String(ref v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为&str类型", self))),
        }
    }

    pub fn as_str_mut(&mut self) -> Result<&mut str> {
        match *self {
            Value::String(ref mut v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为&mut str类型", self))),
        }
    }

    pub fn as_array(&self) -> Result<&Vec<Value>> {
        match *self {
            Value::Array(ref v) => Ok(v),
            _ => {
                Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为&Vec<Value>类型", self)))
            }
        }
    }

    pub fn as_array_mut(&mut self) -> Result<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!(
                "Value数据[{:?}]不能转换为&mut Vec<Value>类型",
                self
            ))),
        }
    }

    pub fn as_object(&self) -> Result<&BTreeMap<String, Value>> {
        match *self {
            Value::Object(ref v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!(
                "Value数据[{:?}]不能转换为&BTreeMap<String, Value>类型",
                self
            ))),
        }
    }

    pub fn as_object_mut(&mut self) -> Result<&mut BTreeMap<String, Value>> {
        match *self {
            Value::Object(ref mut v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!(
                "Value数据[{:?}]不能转换为&mut BTreeMap<String, Value>类型",
                self
            ))),
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        match *self {
            Value::Bool(v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为bool类型", self))),
        }
    }

    pub fn as_i32(&self) -> Result<i32> {
        match *self {
            Value::I32(v) => Ok(v),
            Value::I64(v) => Ok(v.cast_to_i32().unwrap()),
            Value::U32(v) => Ok(v.cast_to_i32().unwrap()),
            Value::U64(v) => Ok(v.cast_to_i32().unwrap()),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为i32类型", self))),
        }
    }

    pub fn as_i64(&self) -> Result<i64> {
        match *self {
            Value::I32(v) => Ok(v.cast_to_i64().unwrap()),
            Value::I64(v) => Ok(v),
            Value::U32(v) => Ok(v.cast_to_i64().unwrap()),
            Value::U64(v) => Ok(v.cast_to_i64().unwrap()),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为i64类型", self))),
        }
    }

    pub fn as_null(&self) -> Result<()> {
        match *self {
            Value::Null => Ok(()),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为null类型", self))),
        }
    }
}
