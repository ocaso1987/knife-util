use std::collections::BTreeMap;

use crate::{bean::AsValueTrait, error::ERR_INTERNAL, iter::CollectResultTrait, Result, Value, OK};

impl AsValueTrait for rbs::Value {
    fn as_value(&self) -> Result<Value> {
        match self {
            rbs::Value::Null => OK(Value::Null),
            rbs::Value::Bool(v) => OK(Value::Bool(*v)),
            rbs::Value::I32(v) => OK(Value::I32(*v)),
            rbs::Value::I64(v) => OK(Value::I64(*v)),
            rbs::Value::U32(v) => OK(Value::U32(*v)),
            rbs::Value::U64(v) => OK(Value::U64(*v)),
            rbs::Value::F32(v) => OK(Value::F32(*v)),
            rbs::Value::F64(v) => OK(Value::F64(*v)),
            rbs::Value::String(v) => OK(Value::String(v.clone())),
            rbs::Value::Binary(v) => OK(Value::Binary(v.clone())),
            rbs::Value::Array(arr) => arr
                .iter()
                .map(|x| x.as_value())
                .collect_into_vec()
                .map(Value::Array),
            rbs::Value::Map(vm) => {
                let mut map = BTreeMap::new();
                for (k, v) in vm {
                    map.insert(k.as_str().unwrap().to_string(), v.as_value().unwrap());
                }
                OK(Value::Object(map))
            }
            rbs::Value::Ext(ty, _buf) => Err(ERR_INTERNAL.msg_detail(
                format!("不支持从rbs::Value转换Ext类型[{}]到内置Value对象", ty).as_str(),
            )),
        }
    }
}
