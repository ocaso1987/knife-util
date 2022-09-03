//! 异常处理类

use crate::AnyValue;
use core::fmt;
use lazy_static::lazy_static;
use serde_json::json;
use std::{
    fmt::{Display, Write},
    option::Option,
};

/// 异常信息
///
/// 统一全司错误码规范，所有错误信息均需包含此结构体中的内容
#[derive(Clone)]
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
    pub cause: AnyValue,
}

impl std::fmt::Debug for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyError")
            .field("name", &self.name)
            .field("code", &self.code)
            .field("msg", &self.msg)
            .field("msg_detail", &self.msg_detail)
            .finish()
            .unwrap();

        let cause = self.cause.as_ref::<anyhow::Error>();
        let source = Some(cause.as_ref() as &dyn std::error::Error);
        if let Some(cause) = source {
            write!(f, "\nCaused by:")?;
            let multiple = cause.source().is_some();
            for (n, error) in anyhow::Chain::new(cause).enumerate() {
                writeln!(f)?;
                let mut indented = Indented {
                    inner: f,
                    number: if multiple { Some(n) } else { None },
                    started: false,
                };
                write!(indented, "{}", error)?;
            }
        }

        std::result::Result::Ok(())
    }
}

impl Display for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyError")
            .field("name", &self.name)
            .field("code", &self.code)
            .field("msg", &self.msg)
            .field("msg_detail", &self.msg_detail)
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
            cause: AnyValue::new_zero(),
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
        let target = self.clone();
        target.cause.replace(value);
        target
    }

    /// 定义输出到前端的格式
    pub fn to_json_string(&self) -> String {
        let cause = self.cause.as_ref::<anyhow::Error>().to_string();
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
        Some(self.cause.as_ref::<anyhow::Error>().as_ref() as &dyn std::error::Error)
    }

    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self.cause.as_ref::<anyhow::Error>().as_ref() as &dyn std::error::Error)
    }
}

impl serde::ser::Error for AnyError {
    fn custom<T: Display>(msg: T) -> Self {
        ERR_SERIALIZE.msg_detail(msg.to_string())
    }
}

impl serde::de::Error for AnyError {
    fn custom<T: Display>(msg: T) -> Self {
        ERR_DESERIALIZE.msg_detail(msg.to_string())
    }
}

/// 可代替std::result::Result<T, AnyError>操作的工具
pub type Result<T> = std::result::Result<T, AnyError>;

/// 默认返回成功
#[allow(non_snake_case)]
pub fn Ok<T>(t: T) -> Result<T> {
    Result::Ok(t)
}

impl From<std::io::Error> for AnyError {
    fn from(err: std::io::Error) -> Self {
        ERR_IO.cause(anyhow::Error::new(err))
    }
}

impl From<std::env::VarError> for AnyError {
    fn from(err: std::env::VarError) -> Self {
        ERR_ENV_VAR.cause(anyhow::Error::new(err))
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

impl From<rbatis::Error> for AnyError {
    fn from(err: rbatis::Error) -> Self {
        ERR_DB.cause(anyhow::Error::new(err))
    }
}

impl From<hyper::Error> for AnyError {
    fn from(err: hyper::Error) -> Self {
        ERR_WEB.cause(anyhow::Error::new(err))
    }
}

struct Indented<'a, D> {
    inner: &'a mut D,
    number: Option<usize>,
    started: bool,
}

impl<T> Write for Indented<'_, T>
where
    T: Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (i, line) in s.split('\n').enumerate() {
            if !self.started {
                self.started = true;
                match self.number {
                    Some(number) => write!(self.inner, "{: >5}: ", number)?,
                    None => self.inner.write_str("    ")?,
                }
            } else if i > 0 {
                self.inner.write_char('\n')?;
                if self.number.is_some() {
                    self.inner.write_str("       ")?;
                } else {
                    self.inner.write_str("    ")?;
                }
            }

            self.inner.write_str(line)?;
        }

        std::result::Result::Ok(())
    }
}

lazy_static! {

    /// 读取环境变量出现异常
    pub static ref ERR_ENV_VAR: AnyError = AnyError::new("ERR_ENV_VAR", "100001", "读取环境变量出现异常");

    /// 读取IO出现异常
    pub static ref ERR_IO: AnyError = AnyError::new("ERR_IO", "100002", "读取IO操作出现异常");

    /// 全局数据格式转换异常
    pub static ref ERR_CONVERT: AnyError = AnyError::new("ERR_CONVERT", "100003", "数据转换出现异常");

    /// 全局数据内部格式转换异常
    pub static ref ERR_CAST: AnyError = AnyError::new("ERR_CAST", "100004", "数据内部转换出现异常");

    /// 序列化数据异常
    pub static ref ERR_SERIALIZE: AnyError = AnyError::new("ERR_SERIALIZE", "100005", "序列化数据出现异常");

    /// 反序列化数据异常
    pub static ref ERR_DESERIALIZE: AnyError = AnyError::new("ERR_DESERIALIZE", "100006", "反序列化数据出现异常");

    /// 全局数据解析异常
    pub static ref ERR_PARSE: AnyError = AnyError::new("ERR_PARSE", "100007", "数据解析出现异常");

    /// 全局数据解析异常
    pub static ref ERR_MERGE: AnyError = AnyError::new("ERR_MERGE", "100008", "数据进行合并处理出现异常");

    /// 全局数据格式化异常
    pub static ref ERR_FORMAT: AnyError = AnyError::new("ERR_FORMAT", "100009", "数据格式化出现异常");

    /// 请求参数错误
    pub static ref ERR_ARGUMENT: AnyError = AnyError::new("ERR_ARGUMENT", "100010", "请求参数错误");

    /// Web处理错误
    pub static ref ERR_WEB: AnyError = AnyError::new("ERR_WEB", "100011", "Web处理错误");

    /// 数据库操作异常
    pub static ref ERR_DB: AnyError = AnyError::new("ERR_DB", "100012", "数据库操作异常");

    /// 全局内部异常
    pub static ref ERR_INTERNAL: AnyError = AnyError::new("ERR_INTERNAL", "999999", "内部异常");

}
