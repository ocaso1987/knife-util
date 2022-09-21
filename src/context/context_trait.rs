use crate::{bean::AsValueTrait, date::YearMonth, error::ERR_DATA, Ok, Result, Value};

/// 键为字符类型的上下文工具类
pub trait ContextTrait {
    type Context;
    /// 如果Map工具类的实现没有指定的get_xx方法时，默认将数据转换为内置Value后进行处理
    fn get_value(&self, key: &str) -> Result<Option<&Value>>;
    /// 如果Map工具类的实现没有指定的insert_xx方法时，默认将数据转换为内置Value后进行处理
    fn insert_value(&mut self, key: &str, value: Value) -> Result<()>;

    /// 集合类中取出字符类型
    fn get_string(&self, key: &str) -> Result<String> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_str().map(|x| x.to_string()),
                None => Err(ERR_DATA.msg_detail(format!("{}不能为空", key).as_str())),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入字符类型
    fn insert_string(&mut self, key: &str, value: String) -> Result<()> {
        self.insert_value(key, Value::String(value))
    }
    /// 集合类中取出字符类型
    fn get_opt_string(&self, key: &str) -> Result<Option<String>> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_str().map(|x| Some(x.to_string())),
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入字符类型
    fn insert_opt_string(&mut self, key: &str, value: Option<String>) -> Result<()> {
        if let Some(v) = value {
            self.insert_value(key, Value::String(v))
        } else {
            Ok(())
        }
    }

    /// 集合类中取出布尔类型
    fn get_bool(&self, key: &str) -> Result<bool> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_bool(),
                None => Err(ERR_DATA.msg_detail(format!("{}不能为空", key).as_str())),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合类中取出布尔类型
    fn get_bool_or(&self, key: &str, default: bool) -> Result<bool> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_bool(),
                None => Ok(default),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入布尔类型
    fn insert_bool(&mut self, key: &str, value: bool) -> Result<()> {
        self.insert_value(key, Value::Bool(value))
    }

    /// 集合类中取出i64类型
    fn get_i64(&self, key: &str) -> Result<i64> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_i64(),
                None => Err(ERR_DATA.msg_detail(format!("{}不能为空", key).as_str())),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入i64类型
    fn insert_i64(&mut self, key: &str, value: i64) -> Result<()> {
        self.insert_value(key, Value::I64(value))
    }
    /// 集合类中取出i64类型
    fn get_opt_i64(&self, key: &str) -> Result<Option<i64>> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_i64().map(Some),
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入i64类型
    fn insert_opt_i64(&mut self, key: &str, value: Option<i64>) -> Result<()> {
        if let Some(v) = value {
            self.insert_value(key, Value::I64(v))
        } else {
            Ok(())
        }
    }

    /// 集合类中取出u64类型
    fn get_u64(&self, key: &str) -> Result<u64> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_u64(),
                None => Err(ERR_DATA.msg_detail(format!("{}不能为空", key).as_str())),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入u64类型
    fn insert_u64(&mut self, key: &str, value: u64) -> Result<()> {
        self.insert_value(key, Value::U64(value))
    }
    /// 集合类中取出i64类型
    fn get_opt_u64(&self, key: &str) -> Result<Option<u64>> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_u64().map(Some),
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入u64类型
    fn insert_opt_u64(&mut self, key: &str, value: Option<u64>) -> Result<()> {
        if let Some(v) = value {
            self.insert_value(key, Value::U64(v))
        } else {
            Ok(())
        }
    }

    /// 集合类中取出f64类型
    fn get_f64(&self, key: &str) -> Result<f64> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_f64(),
                None => Err(ERR_DATA.msg_detail(format!("{}不能为空", key).as_str())),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入f64类型
    fn insert_f64(&mut self, key: &str, value: f64) -> Result<()> {
        self.insert_value(key, Value::F64(value))
    }
    /// 集合类中取出f64类型
    fn get_opt_f64(&self, key: &str) -> Result<Option<f64>> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_f64().map(Some),
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入f64类型
    fn insert_opt_f64(&mut self, key: &str, value: Option<f64>) -> Result<()> {
        if let Some(v) = value {
            self.insert_value(key, Value::F64(v))
        } else {
            Ok(())
        }
    }

    /// 集合类中取出年月类型
    fn get_yearmonth(&self, key: &str) -> Result<YearMonth> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_year_month(),
                None => Err(ERR_DATA.msg_detail(format!("{}不能为空", key).as_str())),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入年月类型
    fn insert_yearmonth(&mut self, key: &str, value: YearMonth) -> Result<()> {
        self.insert_value(key, Value::YearMonth(value))
    }
    /// 集合类中取出年月类型
    fn get_opt_yearmonth(&self, key: &str) -> Result<Option<YearMonth>> {
        match self.get_value(key) {
            std::result::Result::Ok(v) => match v {
                Some(v2) => v2.as_year_month().map(Some),
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }
    /// 集合中插入年月类型
    fn insert_opt_yearmonth(&mut self, key: &str, value: Option<YearMonth>) -> Result<()> {
        if let Some(v) = value {
            self.insert_value(key, Value::YearMonth(v))
        } else {
            Ok(())
        }
    }

    /// 集合中插入JSON类型
    fn insert_json(&mut self, key: &str, value: &serde_json::Value) -> Result<()> {
        self.insert_value(key, value.as_value()?)
    }
}
