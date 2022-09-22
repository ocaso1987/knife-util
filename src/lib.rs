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
pub use types::{Result, ResultExt, OK};

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
