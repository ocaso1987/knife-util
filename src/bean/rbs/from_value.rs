use crate::{
    bean::FromValueTrait, error::ERR_INTERNAL, iter::CollectResultTrait, Result, Value, OK,
};

impl FromValueTrait for rbs::Value {
    fn from_value(value: &Value) -> Result<Self> {
        match value {
            Value::Null => OK(rbs::Value::Null),
            Value::Bool(v) => OK(rbs::Value::Bool(*v)),
            Value::I32(v) => OK(rbs::Value::I32(*v)),
            Value::I64(v) => OK(rbs::Value::I64(*v)),
            Value::U32(v) => OK(rbs::Value::U32(*v)),
            Value::U64(v) => OK(rbs::Value::U64(*v)),
            Value::F32(v) => OK(rbs::Value::F32(*v)),
            Value::F64(v) => OK(rbs::Value::F64(*v)),
            Value::Date(v) => OK(rbs::Value::Ext(
                "Date",
                Box::new(rbs::Value::String(v.to_string())),
            )),
            Value::Time(v) => OK(rbs::Value::Ext(
                "Time",
                Box::new(rbs::Value::String(v.to_string())),
            )),
            Value::DateTime(v) => OK(rbs::Value::Ext(
                "DateTime",
                Box::new(rbs::Value::String(v.to_string())),
            )),
            Value::YearMonth(v) => Err(ERR_INTERNAL.msg_detail(
                format!(
                    "不支持从内置Value对象转换YearMonth类型[{:?}]到rbs::Value",
                    v
                )
                .as_str(),
            )),
            Value::String(v) => OK(rbs::Value::String(v.clone())),
            Value::Binary(v) => OK(rbs::Value::Binary(v.clone())),
            Value::Array(arr) => arr
                .iter()
                .map(|x| Self::from_value(x))
                .collect_into_vec()
                .map(rbs::Value::Array),
            Value::Object(obj) => {
                let mut map = rbs::value::map::ValueMap::new();
                for (k, v) in obj {
                    map.insert(rbs::Value::String(k.clone()), Self::from_value(v).unwrap());
                }
                OK(rbs::Value::Map(map))
            }
        }
    }
}
