use std::collections::BTreeMap;

use crate::{
    bean::{traits::ConvertExt, Value},
    types::{DoubleCastExt, VecExt},
};

impl ConvertExt for serde_json::Value {
    fn as_value(&self) -> Value {
        match self {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(v) => Value::Bool(*v),
            serde_json::Value::Number(v) => {
                if v.is_f64() {
                    Value::F64(v.as_f64().unwrap())
                } else if v.is_i64() {
                    Value::I64(v.as_i64().unwrap())
                } else if v.is_u64() {
                    Value::U64(v.as_u64().unwrap())
                } else {
                    panic!("无法到达的代码")
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
            Value::Bool(v) => serde_json::Value::Bool(*v),
            Value::I32(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            Value::I64(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            Value::U32(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            Value::U64(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            Value::F32(v) => serde_json::Value::Number(
                serde_json::Number::from_f64((*v).cast_to_f64().unwrap()).unwrap(),
            ),
            Value::F64(v) => serde_json::Value::Number(serde_json::Number::from_f64(*v).unwrap()),
            Value::Date(v) => serde_json::Value::String(v.format("%Y-%m-%d").to_string()),
            Value::Time(v) => serde_json::Value::String(v.format("%H:%M:%S").to_string()),
            Value::DateTime(v) => {
                serde_json::Value::String(v.format("%Y-%m-%d %H:%M:%S").to_string())
            }
            Value::YearMonth(v) => serde_json::Value::String(v.format("%Y-%m").to_string()),
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
