use std::collections::{BTreeMap, HashMap};

use crate::{any::AnyValue, Ok, Result, Value};

use super::{AnyContextTrait, ContextTrait};

impl ContextTrait for HashMap<String, Value> {
    type Context = Value;
    fn get_value(&self, key: &str) -> Result<Option<&Value>> {
        Ok(self.get(key))
    }
    fn insert_value(&mut self, key: &str, value: Value) -> Result<()> {
        self.insert(key.to_string(), value);
        Ok(())
    }
}

impl ContextTrait for HashMap<String, AnyValue> {
    type Context = AnyValue;
    fn get_value(&self, key: &str) -> Result<Option<&Value>> {
        Ok(self.get(key).map(|x| x.to_ref::<Value>()))
    }
    fn insert_value(&mut self, key: &str, value: Value) -> Result<()> {
        self.insert(key.to_string(), AnyValue::new(value));
        Ok(())
    }
}

impl AnyContextTrait for HashMap<String, AnyValue> {
    fn get_any(&self, key: &str) -> Option<&AnyValue> {
        self.get(key)
    }
    fn insert_any<T>(&mut self, key: &str, value: T) {
        self.insert(key.to_string(), AnyValue::new(value));
    }
}

impl ContextTrait for BTreeMap<String, Value> {
    type Context = Value;
    fn get_value(&self, key: &str) -> Result<Option<&Value>> {
        Ok(self.get(key))
    }
    fn insert_value(&mut self, key: &str, value: Value) -> Result<()> {
        self.insert(key.to_string(), value);
        Ok(())
    }
}
