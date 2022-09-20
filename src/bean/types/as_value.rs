use crate::{bean::AsValueTrait, Ok, Result, Value};

impl<T> AsValueTrait for Option<T>
where
    T: AsValueTrait,
{
    fn as_value(&self) -> Result<Value> {
        match self {
            Some(v) => v.as_value(),
            None => Ok(Value::Null),
        }
    }
}
impl AsValueTrait for &str {
    fn as_value(&self) -> Result<Value> {
        Ok(Value::String(self.to_string()))
    }
}
