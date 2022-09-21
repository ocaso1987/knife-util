use crate::{bean::AsValueTrait, iter::VecExt, Ok, Result, Value};

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

impl<T> AsValueTrait for Vec<T>
where
    T: AsValueTrait,
{
    fn as_value(&self) -> Result<Value> {
        self.map_fold_result(|x| x.as_value()).map(Value::Array)
    }
}

impl<T> AsValueTrait for dyn AsRef<T>
where
    T: AsValueTrait,
{
    fn as_value(&self) -> Result<Value> {
        self.as_ref().as_value()
    }
}

impl<T> AsValueTrait for &T
where
    T: AsValueTrait,
{
    fn as_value(&self) -> Result<Value> {
        <&T>::clone(self).as_value()
    }
}

impl AsValueTrait for &str {
    fn as_value(&self) -> Result<Value> {
        Ok(Value::String(self.to_string()))
    }
}

impl AsValueTrait for String {
    fn as_value(&self) -> Result<Value> {
        Ok(Value::String(self.clone()))
    }
}

impl AsValueTrait for usize {
    fn as_value(&self) -> Result<Value> {
        Ok(Value::I64(*self as i64))
    }
}
