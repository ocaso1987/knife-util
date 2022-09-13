use crate::value::Value;

/// 键为字符类型的上下文工具类
pub trait ContextExt {
    type Context;
    /// 如果Map工具类的实现没有指定的get_xx方法时，默认将数据转换为内置Value后进行处理
    fn get_value(&mut self, key: &str) -> Option<&Value>;
    /// 如果Map工具类的实现没有指定的insert_xx方法时，默认将数据转换为内置Value后进行处理
    fn insert_value(&mut self, key: &str, value: Value);

    /// 集合类中取出字符类型
    fn get_string(&mut self, key: &str) -> String {
        self.get_value(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_str()
            .unwrap()
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

    /// 集合类中取出u64类型
    fn get_u64(&mut self, key: &str) -> u64 {
        self.get_value(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_u64()
            .unwrap()
    }
    /// 集合中插入u64类型
    fn insert_u64(&mut self, key: &str, value: u64) {
        self.insert_value(key, Value::U64(value))
    }
    /// 集合类中取出i64类型
    fn get_opt_u64(&mut self, key: &str) -> Option<u64> {
        self.get_value(key).map(|x| x.as_u64().unwrap())
    }
    /// 集合中插入u64类型
    fn insert_opt_u64(&mut self, key: &str, value: Option<u64>) {
        if let Some(v) = value {
            self.insert_value(key, Value::U64(v))
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
