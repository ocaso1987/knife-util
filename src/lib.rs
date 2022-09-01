//! 通用工具类
pub(crate) mod any;
pub(crate) mod bean;
pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod future;
pub(crate) mod number;
pub(crate) mod string;
pub(crate) mod template;
pub(crate) mod vec;

/// Reexport
pub mod crates {
    pub use anyhow;
    pub use async_trait;
    pub use bson;
    pub use ctor;
    pub use hyper;
    pub use lazy_static;
    pub use opentelemetry_jaeger;
    pub use serde_json;
    pub use serde_yaml;
    pub use sqlx;
    pub use tokio;
    pub use toml;
    pub use tracing_opentelemetry;
    pub use tracing_subscriber;
}

pub use any::{AnyRef, AnyValue};
pub use bean::{BsonConvertExt, MergeExt, PointerExt};
pub use context::{AnyContext, AnyContextExt, ContextExt};
pub use error::{
    AnyError, Result, ERR_ARGUMENT, ERR_CAST, ERR_CONVERT, ERR_DB, ERR_FORMAT, ERR_INTERNAL,
    ERR_MERGE, ERR_PARSE, ERR_WEB,
};
pub use future::{FutureHandler, FutureObj};
pub use number::{
    cast_i32_to_i64, cast_i64_to_i16, cast_i64_to_i32, cast_u64_to_i64, cast_u64_to_u16,
    cast_u64_to_u32, cast_u64_to_usize,
};
pub use string::StringExt;
pub use template::{
    render_simple_template, render_sql_template, render_template, render_template_recursion,
    ContextType, TemplateContext, TemplateContextExt,
};
pub use vec::VecExt;
