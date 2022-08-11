//! 异常处理类

use std::{error::Error, fmt::Display};

use lazy_static::lazy_static;

/// 异常信息
///
/// 统一全司错误码规范，所有错误信息均需包含此结构体中的内容
#[derive(Clone, Debug)]
pub struct AppError {
    /// 错误名称，如:ERR_INTERNAL
    pub name: String,

    /// 错误码，6位及以下为该框架提供的通用错误规范，7-8位由项目方统一指定错误码，9位及以上为各应用自行设定
    pub code: String,

    /// 错误信息，通用信息，仅描述错误类型
    pub msg: String,

    /// 错误信息，详情，可结合自定义参数设置输出格式
    pub msg_detail: Option<String>,

    /// 错误信息，详情，可用于描述堆栈信息及调试输出等内容
    pub trace: Option<String>,

    /// 错误原因，通常是被包含的内部错误
    pub cause: Option<&'static dyn Error>,
}

unsafe impl Sync for AppError {}

unsafe impl Send for AppError {}

impl AppError {
    fn new(name: &str, code: &str, msg: &str) -> Self {
        AppError {
            name: name.to_string(),
            code: code.to_string(),
            msg: msg.to_string(),
            msg_detail: None,
            trace: None,
            cause: None,
        }
    }

    /// 设置错误信息，并复制一个全新的错误对象
    pub fn msg_detail(&self, value: String) -> Self {
        let mut target = self.clone();
        target.msg_detail.replace(value);
        target
    }

    /// 设置堆栈信息及调试输出等内容，并复制一个全新的错误对象
    pub fn trace(&self, value: String) -> Self {
        let mut target = self.clone();
        target.trace.replace(value);
        target
    }

    /// 设置内部来源错误，并复制一个全新的包含AppError原因的错误对象
    pub fn cause(&self, value: AppError) -> Self {
        let mut target = self.clone();
        if value.cause.is_some() {
            target.cause.replace(value.cause.unwrap());
        }
        target
    }

    /// 设置内部来源错误，并复制一个全新的包含普通Error原因的错误对象
    pub fn cause_normal(&self, value: &'static dyn Error) -> Self {
        let mut target = self.clone();
        target.cause.replace(value);
        target
    }

    /// 根据错误名称产生中断
    pub fn panic(&self) {
        panic!("{}", self.name);
    }

    /// 比较中断信息与错误信息是否符合，通常用于#[test]代码中
    pub fn should_panic(&self, name: &str) {
        assert_eq!(self.name, name);
    }
}

impl Display for AppError {
    /// 异常输出
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppError")
            .field("name", &self.name)
            .field("code", &self.code)
            .field("msg", &self.msg)
            .finish()
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause
    }

    fn description(&self) -> &str {
        &self.msg
    }
}
pub type Result<T> = ::std::result::Result<T, AppError>;

lazy_static! {

    /// 全局数据格式转换异常
    pub static ref ERR_CONVERT: AppError = AppError::new("ERR_CONVERT", "100001", "数据转换出现异常");

    /// 全局数据内部格式转换异常
    pub static ref ERR_CAST: AppError = AppError::new("ERR_CAST", "100001", "数据内部转换出现异常");

    /// 全局数据解析异常
    pub static ref ERR_PARSE: AppError = AppError::new("ERR_PARSE", "100002", "数据解析出现异常");

    /// 全局数据格式化异常
    pub static ref ERR_FORMAT: AppError = AppError::new("ERR_FORMAT", "100003", "数据格式化出现异常");

    /// 请求参数错误
    pub static ref ERR_ARGUMENT: AppError = AppError::new("ERR_ARGUMENT", "100004", "请求参数错误");

    /// 全局内部异常
    pub static ref ERR_INTERNAL: AppError = AppError::new("ERR_INTERNAL", "999999", "内部异常");

}
