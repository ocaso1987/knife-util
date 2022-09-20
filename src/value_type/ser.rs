use serde::Serialize;

use super::main::Value;

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::I32(v) => serializer.serialize_i32(*v),
            Value::I64(v) => serializer.serialize_i64(*v),
            Value::U32(v) => serializer.serialize_u32(*v),
            Value::U64(v) => serializer.serialize_u64(*v),
            Value::F32(v) => serializer.serialize_f32(*v),
            Value::F64(v) => serializer.serialize_f64(*v),
            Value::Date(v) => serializer.serialize_str(v.to_string().as_str()),
            Value::Time(v) => serializer.serialize_str(v.to_string().as_str()),
            Value::DateTime(v) => serializer.serialize_str(v.to_string().as_str()),
            Value::YearMonth(v) => serializer.serialize_str(v.to_string().as_str()),
            Value::String(v) => serializer.serialize_str(v),
            Value::Binary(v) => serializer.serialize_bytes(v),
            Value::Array(v) => serializer.collect_seq(v),
            Value::Object(v) => serializer.collect_map(v),
        }
    }
}
