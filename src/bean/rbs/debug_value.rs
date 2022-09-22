use crate::bean::DebugTrait;

impl DebugTrait for rbs::Value {
    fn debug_self(&self) -> String {
        match self {
            rbs::Value::Null => format!("Null"),
            rbs::Value::Bool(v) => format!("Bool({})", v),
            rbs::Value::I32(v) => format!("I32({})", v),
            rbs::Value::I64(v) => format!("I64({})", v),
            rbs::Value::U32(v) => format!("U32({})", v),
            rbs::Value::U64(v) => format!("U64({})", v),
            rbs::Value::F32(v) => format!("F32({})", v),
            rbs::Value::F64(v) => format!("F64({})", v),
            rbs::Value::String(v) => format!("String({})", v),
            rbs::Value::Binary(_) => format!("Binary"),
            rbs::Value::Array(v) => format!("Array({:?})", v),
            rbs::Value::Map(v) => format!("Map({:?})", v),
            rbs::Value::Ext(v1, v2) => format!("Ext({},{:?})", v1, v2),
        }
    }
}
