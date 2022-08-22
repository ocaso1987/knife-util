use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Result, ERR_CAST};

/// 数据对象类型，统一封装可序列化的基本类型与复杂类型
#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    /// 空值
    Null,
    /// 布尔类型
    Bool(bool),
    /// 整型
    I64(i64),
    /// 浮点型
    F64(f64),
    /// 字符型
    String(String),
    /// 列表类型
    Array(Vec<Value>),
    /// 对象类型
    Object(BTreeMap<String, Value>),

    /// 任意精度整数
    Integer(rug::Integer),
    /// 任意精度有理数
    Rational(rug::Rational),
    /// 任意精度浮点数
    Float(rug::Float),
    /// 任意精度复数
    Complex(rug::Complex),

    /// JSON类型
    Json(serde_json::Value),
    /// YAML类型
    Yaml(serde_yaml::Value),
    /// TOML类型
    Toml(toml::Value),
}

impl Value {
    pub(crate) fn as_string(&self) -> Result<String> {
        match self {
            Value::String(t) => Ok(t.clone()),
            Value::Json(t) => t
                .as_str()
                .map(|x| x.to_string())
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            Value::Yaml(t) => t
                .as_str()
                .map(|x| x.to_string())
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            Value::Toml(t) => t
                .as_str()
                .map(|x| x.to_string())
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            _ => Err(ERR_CAST.msg_detail("数据不为String类型".to_string())),
        }
    }

    pub(crate) fn as_bool(&self) -> Result<bool> {
        match self {
            Value::Bool(t) => Ok(t.clone()),
            Value::Json(t) => t
                .as_bool()
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            Value::Yaml(t) => t
                .as_bool()
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            Value::Toml(t) => t
                .as_bool()
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            _ => Err(ERR_CAST.msg_detail("数据不为Bool类型".to_string())),
        }
    }

    pub(crate) fn as_i64(&self) -> Result<i64> {
        match self {
            Value::I64(t) => Ok(t.clone()),
            Value::Json(t) => t
                .as_i64()
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            Value::Yaml(t) => t
                .as_i64()
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            Value::Toml(t) => t
                .as_integer()
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            _ => Err(ERR_CAST.msg_detail("数据不为I64类型".to_string())),
        }
    }

    pub(crate) fn as_f64(&self) -> Result<f64> {
        match self {
            Value::F64(t) => Ok(t.clone()),
            Value::Json(t) => t
                .as_f64()
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            Value::Yaml(t) => t
                .as_f64()
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            Value::Toml(t) => t
                .as_float()
                .ok_or(ERR_CAST.msg_detail("数据为空".to_string())),
            _ => Err(ERR_CAST.msg_detail("数据不为F64类型".to_string())),
        }
    }
}
