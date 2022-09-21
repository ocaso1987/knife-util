use std::collections::HashMap;

use backtrace::Backtrace;
use serde_json::json;

use crate::{any::AnyValue, context::ContextTrait, Value, bean::AsValueTrait};

use super::backtrace::enable_backtrace;

/// 异常信息
///
/// 统一全司错误码规范，所有错误信息均需包含此结构体中的内容
#[derive(Clone, Copy)]
pub struct AppError {
    /// 错误名称，如:ERR_INTERNAL
    name: &'static str,

    /// 错误码，6位及以下为该框架提供的通用错误规范，7-8位由项目方统一指定错误码，9位及以上为各应用自行设定
    code: &'static str,

    /// 错误信息，通用信息，仅描述错误类型
    msg: &'static str,

    /// 错误信息，详情，可结合自定义参数设置输出格式
    msg_detail: Option<&'static str>,

    /// 错误原因，通常是被包含的内部错误
    cause: AnyValue,

    /// 异常出错堆栈信息
    backtrace: AnyValue,

    /// 上下文变量
    context_map: AnyValue,
}

unsafe impl Sync for AppError {}
unsafe impl Send for AppError {}

impl AppError {
    pub fn new(name: &str, code: &str, msg: &str) -> Self {
        unsafe {
            AppError {
                name: &*(name as *const str),
                code: &*(code as *const str),
                msg: &*(msg as *const str),
                msg_detail: None,
                cause: AnyValue::new_zero(),
                backtrace: AnyValue::new_zero(),
                context_map: AnyValue::new_zero(),
            }
        }
    }

    /// 设置错误信息，并复制一个全新的错误对象
    pub fn msg_detail(&self, value: &str) -> Self {
        unsafe {
            let mut target = *self;
            target.msg_detail.replace(&*(value as *const str));
            if enable_backtrace() && target.backtrace.is_empty() {
                target.backtrace.replace(Backtrace::new());
            }
            target
        }
    }

    /// 设置内部来源错误，并复制一个全新的包含AppError原因的错误对象
    pub fn cause<E>(&self, value: E) -> Self
    where
        E: std::error::Error + 'static,
    {
        let target = *self;
        target.cause.replace(unsafe {
            Box::<dyn std::error::Error + 'static>::from_raw(
                Box::<dyn std::error::Error + 'static>::into_raw(Box::new(value)),
            )
        });
        if enable_backtrace() && target.backtrace.is_empty() {
            target.backtrace.replace(Backtrace::new());
        }
        target
    }

    /// 在异常中存储可序列化的键值对
    pub fn context_value(&self, key: String, value: Value) -> Self {
        if self.context_map.is_empty() {
            let target = *self;
            target.context_map.replace(HashMap::from([(key, value)]));
            if enable_backtrace() && target.backtrace.is_empty() {
                target.backtrace.replace(Backtrace::new());
            }
            target
        } else {
            let _ = self
                .context_map
                .to_mut::<HashMap<String, Value>>()
                .insert(key, value);
            *self
        }
    }

    /// 设置堆栈诊断信息
    pub fn backtrace(&self) -> Self {
        let target = *self;
        if target.backtrace.is_empty() {
            target.backtrace.replace(Backtrace::new());
        }
        target
    }

    /// 定义输出到前端的格式
    pub fn to_json_string(&self) -> String {
        let mut ctx = json!( {
            "name": self.name,
            "code": self.code,
            "msg": self.msg,
            "msg_detail": self.msg_detail,
        }).as_value().unwrap();
        if !self.cause.is_empty() {
            let cause = self
                .cause
                .to_ref::<Box<dyn std::error::Error + Send + Sync + 'static>>();
            let cause_str = format!("{:?}", cause);
            ctx.insert_value("cause", Value::String(cause_str)).unwrap();
        }
        if !self.context_map.is_empty()
            && !self
                .context_map
                .to_ref::<HashMap<String, Value>>()
                .is_empty()
        {
            for (k, v) in self.context_map.to_ref::<HashMap<String, Value>>() {
                ctx.insert_value(k, v.clone()).unwrap();
            }
        }
        ctx.to_string()
    }

    pub fn name_ref(&self) -> &str {
        self.name
    }

    pub fn code_ref(&self) -> &str {
        self.code
    }

    pub fn msg_ref(&self) -> &str {
        self.msg
    }

    pub fn msg_detail_ref(&self) -> Option<&str> {
        self.msg_detail
    }

    pub fn context_map_ref(&self) -> Option<&HashMap<String, Value>> {
        if !self.context_map.is_empty() {
            Some(self.context_map.to_ref::<HashMap<String, Value>>())
        } else {
            None
        }
    }

    pub fn cause_ref(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if !self.cause.is_empty() {
            Some(
                self.cause
                    .to_ref::<Box<dyn std::error::Error + 'static>>()
                    .as_ref(),
            )
        } else {
            None
        }
    }
    pub fn backtrace_ref(&self) -> Option<&Backtrace> {
        if !self.backtrace.is_empty() {
            Some(self.backtrace.to_ref::<Backtrace>())
        } else {
            None
        }
    }
}
