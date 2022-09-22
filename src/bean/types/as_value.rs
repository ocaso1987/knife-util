use crate::{bean::AsValueTrait, iter::CollectResultTrait, Result, Value, OK};

impl<T> AsValueTrait for Option<T>
where
    T: AsValueTrait,
{
    fn as_value(&self) -> Result<Value> {
        match self {
            Some(v) => v.as_value(),
            None => OK(Value::Null),
        }
    }
}

impl<T> AsValueTrait for Vec<T>
where
    T: AsValueTrait,
{
    fn as_value(&self) -> Result<Value> {
        self.iter()
            .map(|x| x.as_value())
            .collect_into_vec()
            .map(Value::Array)
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

impl AsValueTrait for bool {
    fn as_value(&self) -> Result<Value> {
        OK(Value::Bool(*self))
    }
}

impl AsValueTrait for &str {
    fn as_value(&self) -> Result<Value> {
        OK(Value::String(self.to_string()))
    }
}

impl AsValueTrait for String {
    fn as_value(&self) -> Result<Value> {
        OK(Value::String(self.clone()))
    }
}

impl AsValueTrait for usize {
    fn as_value(&self) -> Result<Value> {
        OK(Value::I64(*self as i64))
    }
}

impl AsValueTrait for i32 {
    fn as_value(&self) -> Result<Value> {
        OK(Value::I32(*self))
    }
}

impl AsValueTrait for i64 {
    fn as_value(&self) -> Result<Value> {
        OK(Value::I64(*self))
    }
}

impl AsValueTrait for u64 {
    fn as_value(&self) -> Result<Value> {
        OK(Value::U64(*self))
    }
}

impl AsValueTrait for f64 {
    fn as_value(&self) -> Result<Value> {
        OK(Value::F64(*self))
    }
}
