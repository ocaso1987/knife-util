use crate::{context::ContextTrait, error::ERR_DATA, Ok, Result, Value};

impl ContextTrait for Value {
    type Context = Value;

    fn get_value(&self, key: &str) -> Result<Option<&Value>> {
        if self.is_object() {
            let map = self.as_object_mut()?;
            Ok(map.get(key))
        } else {
            Ok(None)
        }
    }

    fn insert_value(&mut self, key: &str, value: Value) -> Result<()> {
        if self.is_object() {
            let map = self.as_object_mut()?;
            map.insert(key.to_string(), value);
            Ok(())
        } else {
            Err(ERR_DATA.msg_detail("不是合法的Value::Object对象"))
        }
    }
}
