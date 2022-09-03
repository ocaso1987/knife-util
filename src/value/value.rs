//! 用于处理程序内置对象的工具
//!
//! 并负责对Json、Yaml、Toml等格式的对象进行处理及转换
//! 注意的是内置对象没有固定格式，不支持反序列化操作
//! 如有该操作需求，需通过配合文件方式进行序列化/反序列化操作后进行数据合并

use std::collections::BTreeMap;

use serde::Serialize;

use crate::{DoubleCastTrait, IntegerCastTrait, Result, ERR_CAST};

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Binary(Vec<u8>),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(v) => v.serialize(serializer),
            Value::I32(v) => v.serialize(serializer),
            Value::I64(v) => v.serialize(serializer),
            Value::U32(v) => v.serialize(serializer),
            Value::U64(v) => v.serialize(serializer),
            Value::F32(v) => v.serialize(serializer),
            Value::F64(v) => v.serialize(serializer),
            Value::String(v) => v.serialize(serializer),
            Value::Binary(v) => v.serialize(serializer),
            Value::Array(v) => v.serialize(serializer),
            Value::Object(v) => v.serialize(serializer),
        }
    }
}

impl Value {
    pub fn as_null(&self) -> Result<()> {
        match *self {
            Value::Null => Ok(()),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为null类型", self))),
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

    pub fn as_u64(&self) -> Result<u64> {
        match *self {
            Value::I32(v) => Ok(v.cast_to_u64().unwrap()),
            Value::I64(v) => Ok(v.cast_to_u64().unwrap()),
            Value::U32(v) => Ok(v.cast_to_u64().unwrap()),
            Value::U64(v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为u64类型", self))),
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        match *self {
            Value::F32(v) => Ok(v.cast_to_f64().unwrap()),
            Value::F64(v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为f64类型", self))),
        }
    }

    pub fn as_binary(&self) -> Result<&Vec<u8>> {
        match *self {
            Value::Binary(ref v) => Ok(v),
            _ => Err(ERR_CAST.msg_detail(format!("Value数据[{:?}]不能转换为Binary类型", self))),
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

    pub fn is_object(&self) -> bool {
        match *self {
            Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn with_object<F>(&mut self, f: F)
    where
        F: FnOnce(&mut BTreeMap<String, Value>),
    {
        let map = self.as_object_mut().unwrap();
        f(map);
    }
}
