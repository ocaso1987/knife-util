use crate::{
    bean::FromValueTrait, error::ERR_INTERNAL, iter::CollectResultTrait, types::DoubleExt, Result,
    Value, OK,
};

impl FromValueTrait for serde_json::Value {
    fn from_value(value: &Value) -> Result<Self> {
        match value {
            Value::Null => OK(serde_json::Value::Null),
            Value::Bool(v) => OK(serde_json::Value::Bool(*v)),
            Value::I32(v) => OK(serde_json::Value::Number(serde_json::Number::from(*v))),
            Value::I64(v) => OK(serde_json::Value::Number(serde_json::Number::from(*v))),
            Value::U32(v) => OK(serde_json::Value::Number(serde_json::Number::from(*v))),
            Value::U64(v) => OK(serde_json::Value::Number(serde_json::Number::from(*v))),
            Value::F32(v) => OK(serde_json::Value::Number(
                serde_json::Number::from_f64((*v).cast_to_f64().unwrap()).unwrap(),
            )),
            Value::F64(v) => OK(serde_json::Value::Number(
                serde_json::Number::from_f64(*v).unwrap(),
            )),
            Value::Date(v) => OK(serde_json::Value::String(v.to_string())),
            Value::Time(v) => OK(serde_json::Value::String(v.to_string())),
            Value::DateTime(v) => OK(serde_json::Value::String(v.to_string())),
            Value::YearMonth(v) => OK(serde_json::Value::String(v.to_string())),
            Value::String(v) => OK(serde_json::Value::String(v.clone())),
            Value::Binary(_v) => {
                Err(ERR_INTERNAL.msg_detail("不支持从内置Value对象转换Binary类型到Json对象"))
            }
            Value::Array(arr) => arr
                .iter()
                .map(|x| Self::from_value(x))
                .collect_into_vec()
                .map(serde_json::Value::Array),
            Value::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (k, v) in obj {
                    map.insert(k.to_string(), Self::from_value(v).unwrap());
                }
                OK(serde_json::Value::Object(map))
            }
        }
    }
}
