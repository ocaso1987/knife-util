use std::collections::BTreeMap;

use crate::{bean::AsValueTrait, error::ERR_INTERNAL, iter::CollectResultTrait, Result, Value, OK};

impl AsValueTrait for serde_json::Value {
    fn as_value(&self) -> Result<Value> {
        match self {
            serde_json::Value::Null => OK(Value::Null),
            serde_json::Value::Bool(v) => OK(Value::Bool(*v)),
            serde_json::Value::Number(v) => {
                if v.is_f64() {
                    OK(Value::F64(v.as_f64().unwrap()))
                } else if v.is_i64() {
                    OK(Value::I64(v.as_i64().unwrap()))
                } else if v.is_u64() {
                    OK(Value::U64(v.as_u64().unwrap()))
                } else {
                    Err(ERR_INTERNAL.msg_detail("无法到达的代码"))
                }
            }
            serde_json::Value::String(v) => OK(Value::String(v.clone())),
            serde_json::Value::Array(v) => v
                .iter()
                .map(|x| x.as_value())
                .collect_into_vec()
                .map(Value::Array),
            serde_json::Value::Object(o) => {
                let mut map = BTreeMap::new();
                for (k, v) in o {
                    map.insert(k.to_string(), v.as_value().unwrap());
                }
                OK(Value::Object(map))
            }
        }
    }
}
