use std::collections::BTreeMap;

use crate::{
    types::{DoubleCastExt, IntegerCastExt, VecExt},
    value::{traits::ConvertExt, Value},
};

impl ConvertExt for toml::Value {
    fn as_value(&self) -> Value {
        match self {
            toml::Value::String(v) => Value::String(v.clone()),
            toml::Value::Integer(v) => Value::I64(v.clone()),
            toml::Value::Float(v) => Value::F64(v.clone()),
            toml::Value::Boolean(v) => Value::Bool(v.clone()),
            toml::Value::Datetime(v) => Value::String(v.to_string()),
            toml::Value::Array(arr) => Value::Array(arr.map(|x| x.as_value())),
            toml::Value::Table(tab) => {
                let mut map = BTreeMap::new();
                for (k, v) in tab {
                    map.insert(k.to_string(), v.as_value());
                }
                Value::Object(map)
            }
        }
    }

    fn from_value(value: &Value) -> Self {
        match value {
            Value::Null => toml::Value::String("".to_string()),
            Value::Bool(v) => toml::Value::Boolean(v.clone()),
            Value::I32(v) => toml::Value::Integer(v.cast_to_i64().unwrap()),
            Value::I64(v) => toml::Value::Integer(v.clone()),
            Value::U32(v) => toml::Value::Integer(v.cast_to_i64().unwrap()),
            Value::U64(v) => toml::Value::Integer(v.cast_to_i64().unwrap()),
            Value::F32(v) => toml::Value::Float(v.cast_to_f64().unwrap()),
            Value::F64(v) => toml::Value::Float(v.clone()),
            Value::String(v) => toml::Value::String(v.clone()),
            Value::Binary(_v) => panic!("不支持从内置Value对象转换Binary类型到Toml对象"),
            Value::Array(arr) => toml::Value::Array(arr.map(|x| Self::from_value(x))),
            Value::Object(obj) => {
                let mut map = toml::map::Map::new();
                for (k, v) in obj {
                    map.insert(k.to_string(), Self::from_value(v));
                }
                toml::Value::Table(map)
            }
        }
    }
}
