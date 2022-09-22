use std::collections::{BTreeMap, HashMap};

use crate::{any::AnyValue, Result, Value, OK};

use super::{AnyContextTrait, ContextTrait};

impl ContextTrait for HashMap<String, Value> {
    type Context = Value;
    fn get_value(&self, key: &str) -> Result<Option<&Value>> {
        OK(self.get(key))
    }
    fn insert_value(&mut self, key: &str, value: Value) -> Result<()> {
        self.insert(key.to_string(), value);
        OK(())
    }
}

impl ContextTrait for HashMap<String, AnyValue> {
    type Context = AnyValue;
    fn get_value(&self, key: &str) -> Result<Option<&Value>> {
        OK(self.get(key).map(|x| x.to_ref::<Value>()))
    }
    fn insert_value(&mut self, key: &str, value: Value) -> Result<()> {
        self.insert(key.to_string(), AnyValue::new(value));
        OK(())
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
        OK(self.get(key))
    }
    fn insert_value(&mut self, key: &str, value: Value) -> Result<()> {
        self.insert(key.to_string(), value);
        OK(())
    }
}
