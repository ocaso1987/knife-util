//! 集合工具类

use std::collections::HashMap;

use serde_json::{Number, Value};

use crate::{cast_u64_to_u16, AnyValue};

/// 集合工具类
pub trait MapUtil {
    /// 集合中获取JSON类型
    fn get_value(&mut self, key: &str) -> Option<&Value>;
    /// 集合中插入JSON类型
    fn insert_value(&mut self, key: &str, value: Value);

    /// 集合类中取出字符类型
    fn get_string(&mut self, key: &str) -> String {
        self.get_value(key)
            .expect(format!("{}不能为空", key).as_str())
            .to_string()
    }
    /// 集合中插入字符类型
    fn insert_string(&mut self, key: &str, value: String) {
        self.insert_value(key, Value::String(value));
    }
    /// 集合类中取出字符类型
    fn get_opt_string(&mut self, key: &str) -> Option<String> {
        self.get_value(key).map(|x| x.as_str().unwrap().to_string())
    }
    /// 集合中插入字符类型
    fn insert_opt_string(&mut self, key: &str, value: Option<String>) {
        if let Some(v) = value {
            self.insert_value(key, Value::String(v))
        }
    }

    /// 集合类中取出布尔类型
    fn get_bool(&mut self, key: &str) -> bool {
        self.get_value(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_bool()
            .unwrap()
    }
    /// 集合类中取出布尔类型
    fn get_bool_or(&mut self, key: &str, default: bool) -> bool {
        self.get_value(key)
            .map(|x| x.as_bool().unwrap())
            .unwrap_or(default)
    }
    /// 集合中插入布尔类型
    fn insert_bool(&mut self, key: &str, value: bool) {
        self.insert_value(key, Value::Bool(value))
    }

    /// 集合类中取出u16类型
    fn get_u16(&mut self, key: &str) -> u16 {
        let value = self
            .get_value(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_u64()
            .unwrap();
        cast_u64_to_u16(value).unwrap()
    }
    /// 集合中插入u16类型
    fn insert_u16(&mut self, key: &str, value: u16) {
        self.insert_value(key, Value::Number(value.into()))
    }

    /// 集合类中取出u64类型
    fn get_u64(&mut self, key: &str) -> u64 {
        let value = self
            .get_value(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_u64()
            .unwrap();
        value
    }
    /// 集合中插入u64类型
    fn insert_u64(&mut self, key: &str, value: u64) {
        self.insert_value(key, Value::Number(value.into()))
    }
    /// 集合类中取出u64类型
    fn get_opt_u64(&mut self, key: &str) -> Option<u64> {
        self.get_value(key).map(|x| x.as_u64().unwrap())
    }
    /// 集合中插入u64类型
    fn insert_opt_u64(&mut self, key: &str, value: Option<u64>) {
        if let Some(v) = value {
            self.insert_value(key, Value::Number(Number::from(v)))
        }
    }

    /// 集合类中取出f64类型
    fn get_f64(&mut self, key: &str) -> f64 {
        let value = self
            .get_value(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_f64()
            .unwrap();
        value
    }
    /// 集合中插入f64类型
    fn insert_f64(&mut self, key: &str, value: f64) {
        self.insert_value(key, Value::Number(Number::from_f64(value).unwrap()))
    }
    /// 集合类中取出f64类型
    fn get_opt_f64(&mut self, key: &str) -> Option<f64> {
        self.get_value(key).map(|x| x.as_f64().unwrap())
    }
    /// 集合中插入f64类型
    fn insert_opt_f64(&mut self, key: &str, value: Option<f64>) {
        if let Some(v) = value {
            self.insert_value(key, Value::Number(Number::from_f64(v).unwrap()))
        }
    }
}

pub trait AnyContextUtil {
    fn insert_any<T>(&mut self, key: &str, value: T);
    fn get_ref<T>(&mut self, key: &str) -> &T;
    fn get_mut<T>(&mut self, key: &str) -> &mut T;
    fn get_opt_ref<T>(&mut self, key: &str) -> Option<&T>;
    fn get_opt_mut<T>(&mut self, key: &str) -> Option<&mut T>;
}
impl MapUtil for HashMap<String, AnyValue> {
    fn get_value(&mut self, key: &str) -> Option<&Value> {
        self.get(key).map(|x| x.as_ref::<Value>())
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), AnyValue::new(value));
    }
}
impl AnyContextUtil for HashMap<String, AnyValue> {
    fn insert_any<T>(&mut self, key: &str, value: T) {
        self.insert(key.to_string(), AnyValue::new(value));
    }
    fn get_ref<T>(&mut self, key: &str) -> &T {
        self.get(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_ref::<T>()
    }
    fn get_mut<T>(&mut self, key: &str) -> &mut T {
        self.get(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_mut::<T>()
    }
    fn get_opt_ref<T>(&mut self, key: &str) -> Option<&T> {
        self.get(key).map(|x| x.as_ref::<T>())
    }
    fn get_opt_mut<T>(&mut self, key: &str) -> Option<&mut T> {
        self.get(key).map(|x| x.as_mut::<T>())
    }
}
pub type AnyContext = HashMap<String, AnyValue>;

impl MapUtil for HashMap<String, Value> {
    fn get_value(&mut self, key: &str) -> Option<&Value> {
        self.get(key)
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), value);
    }
}
