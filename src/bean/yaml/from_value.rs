use indexmap::IndexMap;

use crate::{
    bean::FromValueTrait, error::ERR_INTERNAL, iter::CollectResultTrait, types::DoubleExt, Result,
    Value, OK,
};

impl FromValueTrait for serde_yaml::Value {
    fn from_value(value: &Value) -> Result<Self> {
        match value {
            Value::Null => OK(serde_yaml::Value::Null),
            Value::Bool(v) => OK(serde_yaml::Value::Bool(*v)),
            Value::I32(v) => OK(serde_yaml::Value::Number(serde_yaml::Number::from(*v))),
            Value::I64(v) => OK(serde_yaml::Value::Number(serde_yaml::Number::from(*v))),
            Value::U32(v) => OK(serde_yaml::Value::Number(serde_yaml::Number::from(*v))),
            Value::U64(v) => OK(serde_yaml::Value::Number(serde_yaml::Number::from(*v))),
            Value::F32(v) => OK(serde_yaml::Value::Number(serde_yaml::Number::from(
                (*v).cast_to_f64().unwrap(),
            ))),
            Value::F64(v) => OK(serde_yaml::Value::Number(serde_yaml::Number::from(*v))),
            Value::Date(v) => OK(serde_yaml::Value::String(v.to_string())),
            Value::Time(v) => OK(serde_yaml::Value::String(v.to_string())),
            Value::DateTime(v) => OK(serde_yaml::Value::String(v.to_string())),
            Value::YearMonth(v) => OK(serde_yaml::Value::String(v.to_string())),
            Value::String(v) => OK(serde_yaml::Value::String(v.clone())),
            Value::Binary(_v) => {
                Err(ERR_INTERNAL.msg_detail("不支持从内置Value对象转换Binary类型到Yaml对象"))
            }
            Value::Array(arr) => arr
                .iter()
                .map(|x| Self::from_value(x))
                .collect_into_vec()
                .map(serde_yaml::Value::Sequence),
            Value::Object(obj) => {
                let mut map = IndexMap::new();
                for (k, v) in obj {
                    map.insert(
                        serde_yaml::Value::String(k.to_string()),
                        serde_yaml::Value::from_value(v).unwrap(),
                    );
                }
                OK(serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(
                    map,
                )))
            }
        }
    }
}
