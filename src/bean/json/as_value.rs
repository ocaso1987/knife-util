use std::collections::BTreeMap;

use crate::{bean::AsValueTrait, error::ERR_INTERNAL, iter::vec::VecExt, Ok, Result, Value};

impl AsValueTrait for serde_json::Value {
    fn as_value(&self) -> Result<Value> {
        match self {
            serde_json::Value::Null => Ok(Value::Null),
            serde_json::Value::Bool(v) => Ok(Value::Bool(*v)),
            serde_json::Value::Number(v) => {
                if v.is_f64() {
                    Ok(Value::F64(v.as_f64().unwrap()))
                } else if v.is_i64() {
                    Ok(Value::I64(v.as_i64().unwrap()))
                } else if v.is_u64() {
                    Ok(Value::U64(v.as_u64().unwrap()))
                } else {
                    Err(ERR_INTERNAL.msg_detail("无法到达的代码"))
                }
            }
            serde_json::Value::String(v) => Ok(Value::String(v.clone())),
            serde_json::Value::Array(v) => {
                v.map_collect_into_vec(|x| x.as_value()).map(Value::Array)
            }
            serde_json::Value::Object(o) => {
                let mut map = BTreeMap::new();
                for (k, v) in o {
                    map.insert(k.to_string(), v.as_value().unwrap());
                }
                Ok(Value::Object(map))
            }
        }
    }
}
