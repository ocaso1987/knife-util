//! 集合工具类

use std::collections::HashMap;

use serde_json::Value;

use crate::AnyValue;

/// 集合工具类
pub trait MapUtil {
    /// 集合中插入字符类型
    fn insert_string(&mut self, key: &str, value: String);

    /// 集合类中取出字符类型
    fn get_string(&self, key: &str) -> String;

    /// 集合中插入布尔类型
    fn insert_bool(&mut self, key: &str, value: bool);

    /// 集合类中取出布尔类型
    fn get_bool(&self, key: &str) -> bool;

    /// 集合类中取出布尔类型
    fn get_bool_or(&self, key: &str, default: bool) -> bool;

    /// 集合中插入数值类型
    fn insert_value(&mut self, key: &str, value: Value);
}

impl MapUtil for HashMap<String, AnyValue> {
    fn get_bool(&self, key: &str) -> bool {
        self.get(key)
            .expect(format!("Map对象中{}不存在", key).as_str())
            .as_ref::<bool>()
            .clone()
    }

    fn insert_string(&mut self, key: &str, value: String) {
        self.insert(key.to_string(), AnyValue::new(value));
    }

    fn get_string(&self, key: &str) -> String {
        self.get(key)
            .expect(format!("Map对象中{}不存在", key).as_str())
            .as_ref::<String>()
            .clone()
    }

    fn insert_bool(&mut self, key: &str, value: bool) {
        self.insert(key.to_string(), AnyValue::new(value));
    }

    fn get_bool_or(&self, key: &str, default: bool) -> bool {
        self.get(key)
            .unwrap_or(&AnyValue::new(default))
            .as_ref::<bool>()
            .clone()
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), AnyValue::new(value));
    }
}

impl MapUtil for HashMap<String, Value> {
    fn get_bool(&self, key: &str) -> bool {
        self.get(key)
            .expect(format!("Map对象中{}不存在", key).as_str())
            .as_bool()
            .unwrap()
    }

    fn insert_string(&mut self, key: &str, value: String) {
        self.insert(key.to_string(), Value::String(value));
    }

    fn get_string(&self, key: &str) -> String {
        self.get(key)
            .expect(format!("Map对象中{}不存在", key).as_str())
            .as_str()
            .unwrap()
            .to_string()
    }

    fn insert_bool(&mut self, key: &str, value: bool) {
        self.insert(key.to_string(), Value::Bool(value));
    }

    fn get_bool_or(&self, key: &str, default: bool) -> bool {
        self.get(key)
            .unwrap_or(&Value::Bool(default))
            .as_bool()
            .unwrap()
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), value);
    }
}
