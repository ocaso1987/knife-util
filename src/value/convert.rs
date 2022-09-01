use std::collections::BTreeMap;

use crate::{DoubleCastTrait, VecExt};

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

// impl ConvertExt for serde_yaml::Value {
//     fn as_value(&self) -> Value {
//         match self {
//             serde_yaml::Value::Null => bson::Bson::Null,
//             serde_yaml::Value::Bool(v) => bson::Bson::Boolean(v.clone()),
//             serde_yaml::Value::Number(v) => {
//                 if v.is_f64() {
//                     return bson::Bson::Double(v.as_f64().unwrap());
//                 } else if v.is_i64() {
//                     return bson::Bson::Int64(v.as_i64().unwrap());
//                 } else if v.is_u64() {
//                     return bson::Bson::Int64(cast_u64_to_i64(v.as_u64().unwrap()).unwrap());
//                 } else {
//                     panic!("无法到达的代码");
//                 }
//             }
//             serde_yaml::Value::String(v) => bson::Bson::String(v.clone()),
//             serde_yaml::Value::Sequence(v) => bson::Bson::Array(v.map(|x| x.as_bson())),
//             serde_yaml::Value::Mapping(o) => {
//                 let mut map = IndexMap::new();
//                 for (k, v) in o {
//                     map.insert(k.as_str().unwrap().to_string(), v.as_bson());
//                 }
//                 bson::Bson::Document(bson::Document::from_iter(map.into_iter()))
//             }
//             serde_yaml::Value::Tagged(_) => panic!("暂不支持Yaml使用Tag类型"),
//         }
//     }

//     fn from_value(value: &Value) -> Self {
//         match bson {
//             Bson::Double(v) => serde_yaml::Value::Number(serde_yaml::Number::from(v.clone())),
//             Bson::String(v) => serde_yaml::Value::String(v.clone()),
//             Bson::Array(v) => serde_yaml::Value::Sequence(v.map(|x| Self::from_bson(x))),
//             Bson::Document(o) => {
//                 let mut map = IndexMap::new();
//                 for (k, v) in o {
//                     map.insert(serde_yaml::Value::String(k.to_string()), Self::from_bson(v));
//                 }
//                 serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(map.into_iter()))
//             }
//             Bson::Boolean(v) => serde_yaml::Value::Bool(v.clone()),
//             Bson::Null => serde_yaml::Value::Null,
//             Bson::RegularExpression(_) => panic!("暂不支持Bson以RegularExpression类型进行格式转换"),
//             Bson::JavaScriptCode(_) => panic!("暂不支持Bson以JavaScriptCode类型进行格式转换"),
//             Bson::JavaScriptCodeWithScope(_) => {
//                 panic!("暂不支持Bson以JavaScriptCodeWithScope类型进行格式转换")
//             }
//             Bson::Int32(v) => serde_yaml::Value::Number(serde_yaml::Number::from(
//                 cast_i32_to_i64(v.clone()).unwrap(),
//             )),
//             Bson::Int64(v) => serde_yaml::Value::Number(serde_yaml::Number::from(v.clone())),
//             Bson::Timestamp(v) => serde_yaml::Value::String(v.to_string()),
//             Bson::Binary(_) => {
//                 panic!("暂不支持Bson以Binary类型进行格式转换")
//             }
//             Bson::ObjectId(v) => serde_yaml::Value::String(v.to_string()),
//             Bson::DateTime(v) => serde_yaml::Value::String(v.to_string()),
//             Bson::Symbol(v) => serde_yaml::Value::String(v.to_string()),
//             Bson::Decimal128(_) => panic!("暂不支持Bson以Decimal128类型进行格式转换"),
//             Bson::Undefined => serde_yaml::Value::Null,
//             Bson::MaxKey => {
//                 panic!("暂不支持Bson以MaxKey类型进行格式转换")
//             }
//             Bson::MinKey => {
//                 panic!("暂不支持Bson以MinKey类型进行格式转换")
//             }
//             Bson::DbPointer(_) => {
//                 panic!("暂不支持Bson以DbPointer类型进行格式转换")
//             }
//         }
//     }
// }
