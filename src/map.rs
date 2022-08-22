//! 集合工具类
use std::collections::HashMap;

use crate::{AnyValue, Value};

/// Map工具类
pub trait MapExt<K, V> {}

/// 键为字符类型的Map工具类
pub trait ContextExt {
    /// 集合中获取JSON类型
    fn get_value(&mut self, key: &str) -> Option<&Value>;
    /// 集合中插入JSON类型
    fn insert_value(&mut self, key: &str, value: Value);

    /// 集合类中取出字符类型
    fn get_string(&mut self, key: &str) -> String {
        self.get_value(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_string()
            .unwrap()
            .to_string()
    }
    /// 集合中插入字符类型
    fn insert_string(&mut self, key: &str, value: String) {
        self.insert_value(key, Value::String(value));
    }
    /// 集合类中取出字符类型
    fn get_opt_string(&mut self, key: &str) -> Option<String> {
        self.get_value(key)
            .map(|x| x.as_string().unwrap().to_string())
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

    /// 集合类中取出i64类型
    fn get_i64(&mut self, key: &str) -> i64 {
        self.get_value(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_i64()
            .unwrap()
    }
    /// 集合中插入i64类型
    fn insert_i64(&mut self, key: &str, value: i64) {
        self.insert_value(key, Value::I64(value))
    }
    /// 集合类中取出i64类型
    fn get_opt_i64(&mut self, key: &str) -> Option<i64> {
        self.get_value(key).map(|x| x.as_i64().unwrap())
    }
    /// 集合中插入i64类型
    fn insert_opt_i64(&mut self, key: &str, value: Option<i64>) {
        if let Some(v) = value {
            self.insert_value(key, Value::I64(v))
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
        self.insert_value(key, Value::F64(value))
    }
    /// 集合类中取出f64类型
    fn get_opt_f64(&mut self, key: &str) -> Option<f64> {
        self.get_value(key).map(|x| x.as_f64().unwrap())
    }
    /// 集合中插入f64类型
    fn insert_opt_f64(&mut self, key: &str, value: Option<f64>) {
        if let Some(v) = value {
            self.insert_value(key, Value::F64(v))
        }
    }
}
impl ContextExt for HashMap<String, Value> {
    fn get_value(&mut self, key: &str) -> Option<&Value> {
        self.get(key)
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), value);
    }
}

/// 键为字符类型且值为AnyValue的Map工具操作类
pub trait AnyContextExt {
    /// 集合中插入任意类型数据
    fn insert_any<T>(&mut self, key: &str, value: T);
    /// 获取插入的任意类型数据的引用指针
    fn get_ref<T>(&mut self, key: &str) -> &T;
    /// 获取插入的任意类型数据的可变引用指针
    fn get_mut<T>(&mut self, key: &str) -> &mut T;
    /// 获取插入的任意类型数据的引用指针，如果未找到则返回空
    fn get_opt_ref<T>(&mut self, key: &str) -> Option<&T>;
    /// 获取插入的任意类型数据的可变引用指针，如果未找到则返回空
    fn get_opt_mut<T>(&mut self, key: &str) -> Option<&mut T>;
}
impl ContextExt for HashMap<String, AnyValue> {
    fn get_value(&mut self, key: &str) -> Option<&Value> {
        self.get(key).map(|x| x.as_ref::<Value>())
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        self.insert(key.to_string(), AnyValue::new(value));
    }
}
impl AnyContextExt for HashMap<String, AnyValue> {
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
/// 可代替HashMap<String, AnyValue>操作的工具
pub type AnyContext = HashMap<String, AnyValue>;
