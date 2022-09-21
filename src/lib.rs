//! 通用工具类
pub mod any;
pub mod bean;
pub mod context;
pub mod date;
pub mod error;
pub mod future;
pub mod iter;
pub mod page;
pub mod template;
pub mod types;

mod value_type;
pub use value_type::Value;

/// Reexport
pub mod crates {
    pub use async_trait;
    pub use ctor;
    pub use futures;
    pub use hyper;
    pub use lazy_static;
    pub use opentelemetry;
    pub use rbatis;
    pub use rbdc_pg;
    pub use rbs;
    pub use tokio;
    pub use tracing_core;
    pub use tracing_opentelemetry;
    pub use tracing_subscriber;
}

/// Reexport，但不会在knife_framework中导出
pub mod crates_builtin {
    pub use chrono;
    pub use serde_json;
    pub use serde_yaml;
}

/// 可代替std::result::Result<T, AnyError>操作的工具
pub type Result<T> = std::result::Result<T, error::AppError>;

/// 默认返回成功
#[allow(non_snake_case)]
pub fn Ok<T>(t: T) -> Result<T> {
    std::result::Result::Ok(t)
}
