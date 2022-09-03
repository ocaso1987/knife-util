//! 通用工具类
pub(crate) mod any;
pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod future;
pub(crate) mod number;
pub(crate) mod string;
pub(crate) mod template;
pub(crate) mod vec;

pub(crate) mod value {
    pub(crate) mod convert;
    pub(crate) mod merge;
    pub(crate) mod pointer;
    pub(crate) mod value;
}

/// Reexport
pub mod crates {
    pub use anyhow;
    pub use async_trait;
    pub use bson;
    pub use chrono;
    pub use ctor;
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

pub use any::{AnyRef, AnyValue};
pub use context::{AnyContext, AnyContextExt, ContextExt};
pub use error::{
    AnyError, Ok, Result, ERR_ARGUMENT, ERR_CAST, ERR_CONVERT, ERR_DB, ERR_FORMAT, ERR_INTERNAL,
    ERR_MERGE, ERR_PARSE, ERR_WEB,
};
pub use future::{FutureHandler, FutureObj};
pub use number::{DoubleCastTrait, IntegerCastTrait};
pub use string::StringExt;
pub use template::{
    render_simple_template, render_sql_template, render_template, render_template_recursion,
    ContextType, TemplateContext, TemplateContextExt,
};
pub use value::{
    convert::ValueConvertExt, merge::ValueMergeExt, pointer::ValuePointerExt, value::Value,
};
pub use vec::VecExt;
