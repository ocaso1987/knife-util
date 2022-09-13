use std::collections::BTreeMap;

use indexmap::IndexMap;

use crate::{
    types::{DoubleCastExt, VecExt},
    value::{traits::ConvertExt, Value},
};

impl ConvertExt for serde_yaml::Value {
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
                        serde_yaml::Value::from_value(v),
                    );
                }
                serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(map))
            }
        }
    }
}
