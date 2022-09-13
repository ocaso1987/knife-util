use std::collections::BTreeMap;

use crate::{
    types::VecExt,
    value::{traits::ConvertExt, Value},
};

impl ConvertExt for rbs::Value {
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