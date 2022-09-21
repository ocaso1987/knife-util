use std::collections::BTreeMap;

use crate::{bean::AsValueTrait, error::ERR_INTERNAL, iter::VecExt, Ok, Result, Value};

impl AsValueTrait for serde_yaml::Value {
    fn as_value(&self) -> Result<Value> {
        match self {
            serde_yaml::Value::Null => Ok(Value::Null),
            serde_yaml::Value::Bool(v) => Ok(Value::Bool(*v)),
            serde_yaml::Value::Number(v) => {
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
            serde_yaml::Value::String(v) => Ok(Value::String(v.clone())),
            serde_yaml::Value::Sequence(v) => v.map_fold_result(|x| x.as_value()).map(Value::Array),
            serde_yaml::Value::Mapping(o) => {
                let mut map = BTreeMap::new();
                for (k, v) in o {
                    map.insert(k.as_str().unwrap().to_string(), v.as_value().unwrap());
                }
                Ok(Value::Object(map))
            }
            serde_yaml::Value::Tagged(_) => Err(ERR_INTERNAL.msg_detail("暂不支持Yaml使用Tag类型")),
        }
    }
}
