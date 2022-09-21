use std::collections::BTreeMap;

use crate::{bean::AsValueTrait, error::ERR_INTERNAL, iter::VecExt, Ok, Result, Value};

impl AsValueTrait for rbs::Value {
    fn as_value(&self) -> Result<Value> {
        match self {
            rbs::Value::Null => Ok(Value::Null),
            rbs::Value::Bool(v) => Ok(Value::Bool(*v)),
            rbs::Value::I32(v) => Ok(Value::I32(*v)),
            rbs::Value::I64(v) => Ok(Value::I64(*v)),
            rbs::Value::U32(v) => Ok(Value::U32(*v)),
            rbs::Value::U64(v) => Ok(Value::U64(*v)),
            rbs::Value::F32(v) => Ok(Value::F32(*v)),
            rbs::Value::F64(v) => Ok(Value::F64(*v)),
            rbs::Value::String(v) => Ok(Value::String(v.clone())),
            rbs::Value::Binary(v) => Ok(Value::Binary(v.clone())),
            rbs::Value::Array(arr) => arr.map_fold_result(|x| x.as_value()).map(Value::Array),
            rbs::Value::Map(vm) => {
                let mut map = BTreeMap::new();
                for (k, v) in vm {
                    map.insert(k.as_str().unwrap().to_string(), v.as_value().unwrap());
                }
                Ok(Value::Object(map))
            }
            rbs::Value::Ext(ty, _buf) => Err(ERR_INTERNAL.msg_detail(
                format!("不支持从rbs::Value转换Ext类型[{}]到内置Value对象", ty).as_str(),
            )),
        }
    }
}
