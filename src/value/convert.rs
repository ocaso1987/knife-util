use std::collections::BTreeMap;

use indexmap::IndexMap;

use crate::{DoubleCastTrait, IntegerCastTrait, VecExt};

use super::value::Value;

/// 支持对象与内置Value格式互相转换
pub trait ValueConvertExt {
    fn as_value(&self) -> Value;
    fn from_value(value: &Value) -> Self;
}

impl ValueConvertExt for serde_json::Value {
    fn as_value(&self) -> Value {
        match self {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(v) => Value::Bool(v.clone()),
            serde_json::Value::Number(v) => {
                if v.is_f64() {
                    return Value::F64(v.as_f64().unwrap());
                } else if v.is_i64() {
                    return Value::I64(v.as_i64().unwrap());
                } else if v.is_u64() {
                    return Value::U64(v.as_u64().unwrap());
                } else {
                    panic!("无法到达的代码");
                }
            }
            serde_json::Value::String(v) => Value::String(v.clone()),
            serde_json::Value::Array(v) => Value::Array(v.map(|x| x.as_value())),
            serde_json::Value::Object(o) => {
                let mut map = BTreeMap::new();
                for (k, v) in o {
                    map.insert(k.to_string(), v.as_value());
                }
                Value::Object(map)
            }
        }
    }

    fn from_value(value: &Value) -> Self {
        match value {
            Value::Null => serde_json::Value::Null,
            Value::Bool(v) => serde_json::Value::Bool(v.clone()),
            Value::I32(v) => serde_json::Value::Number(serde_json::Number::from(v.clone())),
            Value::I64(v) => serde_json::Value::Number(serde_json::Number::from(v.clone())),
            Value::U32(v) => serde_json::Value::Number(serde_json::Number::from(v.clone())),
            Value::U64(v) => serde_json::Value::Number(serde_json::Number::from(v.clone())),
            Value::F32(v) => serde_json::Value::Number(
                serde_json::Number::from_f64(v.clone().cast_to_f64().unwrap()).unwrap(),
            ),
            Value::F64(v) => {
                serde_json::Value::Number(serde_json::Number::from_f64(v.clone()).unwrap())
            }
            Value::String(v) => serde_json::Value::String(v.clone()),
            Value::Binary(_v) => panic!("不支持从内置Value对象转换Binary类型到Json对象"),
            Value::Array(arr) => serde_json::Value::Array(arr.map(|x| Self::from_value(x))),
            Value::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (k, v) in obj {
                    map.insert(k.to_string(), Self::from_value(v));
                }
                serde_json::Value::Object(map)
            }
        }
    }
}

impl ValueConvertExt for serde_yaml::Value {
    fn as_value(&self) -> Value {
        match self {
            serde_yaml::Value::Null => Value::Null,
            serde_yaml::Value::Bool(v) => Value::Bool(v.clone()),
            serde_yaml::Value::Number(v) => {
                if v.is_f64() {
                    return Value::F64(v.as_f64().unwrap());
                } else if v.is_i64() {
                    return Value::I64(v.as_i64().unwrap());
                } else if v.is_u64() {
                    return Value::U64(v.as_u64().unwrap());
                } else {
                    panic!("无法到达的代码");
                }
            }
            serde_yaml::Value::String(v) => Value::String(v.clone()),
            serde_yaml::Value::Sequence(v) => Value::Array(v.map(|x| x.as_value())),
            serde_yaml::Value::Mapping(o) => {
                let mut map = BTreeMap::new();
                for (k, v) in o {
                    map.insert(k.as_str().unwrap().to_string(), v.as_value());
                }
                Value::Object(map)
            }
            serde_yaml::Value::Tagged(_) => panic!("暂不支持Yaml使用Tag类型"),
        }
    }

    fn from_value(value: &Value) -> Self {
        match value {
            Value::Null => serde_yaml::Value::Null,
            Value::Bool(v) => serde_yaml::Value::Bool(v.clone()),
            Value::I32(v) => serde_yaml::Value::Number(serde_yaml::Number::from(v.clone())),
            Value::I64(v) => serde_yaml::Value::Number(serde_yaml::Number::from(v.clone())),
            Value::U32(v) => serde_yaml::Value::Number(serde_yaml::Number::from(v.clone())),
            Value::U64(v) => serde_yaml::Value::Number(serde_yaml::Number::from(v.clone())),
            Value::F32(v) => serde_yaml::Value::Number(serde_yaml::Number::from(
                v.clone().cast_to_f64().unwrap(),
            )),
            Value::F64(v) => serde_yaml::Value::Number(serde_yaml::Number::from(v.clone())),
            Value::String(v) => serde_yaml::Value::String(v.clone()),
            Value::Binary(_v) => panic!("不支持从内置Value对象转换Binary类型到Yaml对象"),
            Value::Array(arr) => serde_yaml::Value::Sequence(arr.map(|x| Self::from_value(x))),
            Value::Object(obj) => {
                let mut map = IndexMap::new();
                for (k, v) in obj {
                    map.insert(
                        serde_yaml::Value::String(k.to_string()),
                        Self::from_value(v),
                    );
                }
                serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(map))
            }
        }
    }
}

impl ValueConvertExt for toml::Value {
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

impl ValueConvertExt for rbs::Value {
    fn as_value(&self) -> Value {
        match self {
            rbs::Value::Null => Value::Null,
            rbs::Value::Bool(v) => Value::Bool(v.clone()),
            rbs::Value::I32(v) => Value::I32(v.clone()),
            rbs::Value::I64(v) => Value::I64(v.clone()),
            rbs::Value::U32(v) => Value::U32(v.clone()),
            rbs::Value::U64(v) => Value::U64(v.clone()),
            rbs::Value::F32(v) => Value::F32(v.clone()),
            rbs::Value::F64(v) => Value::F64(v.clone()),
            rbs::Value::String(v) => Value::String(v.clone()),
            rbs::Value::Binary(v) => Value::Binary(v.clone()),
            rbs::Value::Array(arr) => Value::Array(arr.map(|x| x.as_value())),
            rbs::Value::Map(vm) => {
                let mut map = BTreeMap::new();
                for (k, v) in vm {
                    map.insert(k.as_str().unwrap().to_string(), v.as_value());
                }
                Value::Object(map)
            }
            rbs::Value::Ext(ty, _buf) => {
                panic!("不支持从rbs::Value转换Ext类型[{}]到内置Value对象", ty)
            }
        }
    }

    fn from_value(value: &Value) -> Self {
        match value {
            Value::Null => rbs::Value::Null,
            Value::Bool(v) => rbs::Value::Bool(v.clone()),
            Value::I32(v) => rbs::Value::I32(v.clone()),
            Value::I64(v) => rbs::Value::I64(v.clone()),
            Value::U32(v) => rbs::Value::U32(v.clone()),
            Value::U64(v) => rbs::Value::U64(v.clone()),
            Value::F32(v) => rbs::Value::F32(v.clone()),
            Value::F64(v) => rbs::Value::F64(v.clone()),
            Value::String(v) => rbs::Value::String(v.clone()),
            Value::Binary(v) => rbs::Value::Binary(v.clone()),
            Value::Array(arr) => rbs::Value::Array(arr.map(|x| Self::from_value(x))),
            Value::Object(obj) => {
                let mut map = rbs::value::map::ValueMap::new();
                for (k, v) in obj {
                    map.insert(rbs::Value::String(k.clone()), Self::from_value(v));
                }
                rbs::Value::Map(map)
            }
        }
    }
}
