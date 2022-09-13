use std::collections::{BTreeMap, HashMap};

use crate::{any::AnyValue, value::Value};

use super::{AnyContextExt, ContextExt};

impl ContextExt for HashMap<String, Value> {
    type Context = Value;
    fn get_value(&mut self, key: &str) -> Option<&Value> {
        self.get(key)
    }
    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), value);
    }
}

impl ContextExt for BTreeMap<String, Value> {
    type Context = Value;
    fn get_value(&mut self, key: &str) -> Option<&Value> {
        self.get(key)
    }
    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), value);
    }
}

impl ContextExt for HashMap<String, AnyValue> {
    type Context = AnyValue;
    fn get_value(&mut self, key: &str) -> Option<&Value> {
        self.get(key).map(|x| x.as_ref::<Value>())
    }
    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), AnyValue::new(value));
    }
}

impl AnyContextExt for HashMap<String, AnyValue> {
    fn get_any(&mut self, key: &str) -> Option<&AnyValue> {
        self.get(key)
    }
    fn insert_any<T>(&mut self, key: &str, value: T) {
        self.insert(key.to_string(), AnyValue::new(value));
    }
}
