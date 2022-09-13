use std::collections::HashMap;

use crate::{
    any::AnyValue,
    value::{ConvertExt, Value},
};

/// 异常信息
///
/// 统一全司错误码规范，所有错误信息均需包含此结构体中的内容
#[derive(Clone)]
pub struct AppError {
    /// 错误名称，如:ERR_INTERNAL
    pub name: String,

    /// 错误码，6位及以下为该框架提供的通用错误规范，7-8位由项目方统一指定错误码，9位及以上为各应用自行设定
    pub code: String,

    /// 错误信息，通用信息，仅描述错误类型
    pub msg: String,

    /// 错误信息，详情，可结合自定义参数设置输出格式
    pub msg_detail: Option<String>,

    /// 错误原因，通常是被包含的内部错误
    pub cause: AnyValue,

    /// 上下文变量
    pub context_map: Option<HashMap<String, Value>>,
}

unsafe impl Sync for AppError {}
unsafe impl Send for AppError {}

impl AppError {
    pub(crate) fn new(name: &str, code: &str, msg: &str) -> Self {
        AppError {
            name: name.to_string(),
            code: code.to_string(),
            msg: msg.to_string(),
            msg_detail: None,
            cause: AnyValue::new_zero(),
            context_map: None,
        }
    }

    /// 设置错误信息，并复制一个全新的错误对象
    pub fn msg_detail(&self, value: String) -> Self {
        let mut target = self.clone();
        target.msg_detail.replace(value);
        target
    }

    /// 设置内部来源错误，并复制一个全新的包含AppError原因的错误对象
    pub fn cause<E>(&self, value: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        let target = self.clone();
        target.cause.replace(unsafe {
            Box::<dyn std::error::Error + Send + Sync + 'static>::from_raw(Box::<
                dyn std::error::Error + Send + Sync + 'static,
            >::into_raw(
                Box::new(value)
            ))
        });
        target
    }

    /// 在异常中存储可序列化的键值对
    pub fn context_value(mut self, key: String, value: Value) -> Self {
        if self.context_map.is_some() {
            let _ = self.context_map.as_mut().unwrap().insert(key, value);
            self
        } else {
            let mut err = self.clone();
            err.context_map.replace(HashMap::from([(key, value)]));
            err
        }
    }

    /// 定义输出到前端的格式
    pub fn to_json_string(&self) -> String {
        let mut map = serde_json::json!( {
            "name": self.name,
            "code": self.code,
            "msg": self.msg,
            "msg_detail": self.msg_detail,
        })
        .as_object()
        .unwrap()
        .clone();
        if !self.cause.is_empty() {
            let cause = self
                .cause
                .as_ref::<Box<dyn std::error::Error + Send + Sync + 'static>>();
            let cause_str = format!("{:?}", cause);
            map.insert("cause".to_string(), serde_json::Value::String(cause_str));
        }
        if self.context_map.is_some() && !self.context_map.as_ref().unwrap().is_empty() {
            for (k, v) in self.context_map.as_ref().unwrap() {
                map.insert(k.to_string(), serde_json::Value::from_value(v));
            }
        }
        serde_json::to_string(&map).unwrap()
    }
}
