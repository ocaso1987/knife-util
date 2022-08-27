//! 异常处理类

use crate::AnyValue;
use lazy_static::lazy_static;
use serde_json::json;
use std::fmt::Display;

/// 异常信息
///
/// 统一全司错误码规范，所有错误信息均需包含此结构体中的内容
#[derive(Debug, Clone)]
pub struct AnyError {
    /// 错误名称，如:ERR_INTERNAL
    pub name: String,

    /// 错误码，6位及以下为该框架提供的通用错误规范，7-8位由项目方统一指定错误码，9位及以上为各应用自行设定
    pub code: String,

    /// 错误信息，通用信息，仅描述错误类型
    pub msg: String,

    /// 错误信息，详情，可结合自定义参数设置输出格式
    pub msg_detail: Option<String>,

    /// 错误原因，通常是被包含的内部错误
    pub cause: Option<AnyValue>,
}

impl Display for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyError")
            .field("name", &self.name)
            .field("code", &self.code)
            .field("msg", &self.msg)
            .finish()
    }
}

unsafe impl Sync for AnyError {}

unsafe impl Send for AnyError {}

impl AnyError {
    fn new(name: &str, code: &str, msg: &str) -> Self {
        AnyError {
            name: name.to_string(),
            code: code.to_string(),
            msg: msg.to_string(),
            msg_detail: None,
            cause: None,
        }
    }

    /// 设置错误信息，并复制一个全新的错误对象
    pub fn msg_detail(&self, value: String) -> Self {
        let mut target = self.clone();
        target.msg_detail.replace(value);
        target
    }

    /// 设置内部来源错误，并复制一个全新的包含AppError原因的错误对象
    pub fn cause(&self, value: anyhow::Error) -> Self {
        let mut target = self.clone();
        target.msg_detail.replace(value.to_string());
        target.cause.replace(AnyValue::new(value));
        target
    }

    /// 定义输出到前端的格式
    pub fn to_json_string(&self) -> String {
        let cause = self
            .cause
            .as_ref()
            .map(|x| x.as_ref::<anyhow::Error>().to_string());
        let v = json!( {
            "name": self.name,
            "code": self.code,
            "msg": self.msg,
            "msg_detail": self.msg_detail,
            "cause": cause
        });
        v.to_string()
    }
}

impl std::error::Error for AnyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause
            .as_ref()
            .map(|x| x.as_ref::<anyhow::Error>().source())
            .flatten()
    }

    fn description(&self) -> &str {
        &self.msg
    }
}
/// 可代替std::result::Result<T, AnyError>操作的工具
pub type Result<T> = std::result::Result<T, AnyError>;

impl From<hyper::Error> for AnyError {
    fn from(err: hyper::Error) -> Self {
        ERR_WEB.cause(anyhow::Error::new(err))
    }
}

impl From<serde_json::Error> for AnyError {
    fn from(err: serde_json::Error) -> Self {
        ERR_CONVERT.cause(anyhow::Error::new(err))
    }
}

impl From<serde_yaml::Error> for AnyError {
    fn from(err: serde_yaml::Error) -> Self {
        ERR_CONVERT.cause(anyhow::Error::new(err))
    }
}

impl From<toml::de::Error> for AnyError {
    fn from(err: toml::de::Error) -> Self {
        ERR_CONVERT.cause(anyhow::Error::new(err))
    }
}

impl From<sqlx::Error> for AnyError {
    fn from(err: sqlx::Error) -> Self {
        ERR_DB.cause(anyhow::Error::new(err))
    }
}

lazy_static! {

    /// 全局数据格式转换异常
    pub static ref ERR_CONVERT: AnyError = AnyError::new("ERR_CONVERT", "100001", "数据转换出现异常");

    /// 全局数据内部格式转换异常
    pub static ref ERR_CAST: AnyError = AnyError::new("ERR_CAST", "100001", "数据内部转换出现异常");

    /// 全局数据解析异常
    pub static ref ERR_PARSE: AnyError = AnyError::new("ERR_PARSE", "100002", "数据解析出现异常");

    /// 全局数据解析异常
    pub static ref ERR_MERGE: AnyError = AnyError::new("ERR_MERGE", "100003", "数据进行合并处理出现异常");

    /// 全局数据格式化异常
    pub static ref ERR_FORMAT: AnyError = AnyError::new("ERR_FORMAT", "100004", "数据格式化出现异常");

    /// 请求参数错误
    pub static ref ERR_ARGUMENT: AnyError = AnyError::new("ERR_ARGUMENT", "100005", "请求参数错误");

    /// Web处理错误
    pub static ref ERR_WEB: AnyError = AnyError::new("ERR_WEB", "100006", "Web处理错误");

    /// 数据库操作异常
    pub static ref ERR_DB: AnyError = AnyError::new("ERR_DB", "100007", "数据库操作异常");

    /// 全局内部异常
    pub static ref ERR_INTERNAL: AnyError = AnyError::new("ERR_INTERNAL", "999999", "内部异常");

}
