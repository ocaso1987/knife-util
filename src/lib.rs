//! 通用工具类
pub mod any;
pub mod context;
pub mod error;
pub mod page;
pub mod template;
pub mod types;
pub mod value;

/// Reexport
pub mod crates {
    pub use async_trait;
    pub use bson;
    pub use chrono;
    pub use ctor;
    pub use futures;
    pub use hyper;
    pub use lazy_static;
    pub use opentelemetry_jaeger;
    pub use rbatis;
    pub use rbdc_pg;
    pub use rbs;
    pub use serde_json;
    pub use serde_yaml;
    pub use tokio;
    pub use toml;
    pub use tracing_opentelemetry;
    pub use tracing_subscriber;
}

/// 可代替std::result::Result<T, AnyError>操作的工具
pub type Result<T> = std::result::Result<T, error::AppError>;

/// 默认返回成功
#[allow(non_snake_case)]
pub fn Ok<T>(t: T) -> Result<T> {
    std::result::Result::Ok(t)
}
