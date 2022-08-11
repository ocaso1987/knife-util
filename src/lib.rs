//! 通用工具类
pub(crate) mod any;
pub(crate) mod error;
pub(crate) mod future;
pub(crate) mod map;
pub(crate) mod number;
pub(crate) mod string;
pub(crate) mod template;
pub(crate) mod vec;

pub mod async_trait {
    pub use async_trait::*;
}

pub mod ctor {
    pub use ctor::*;
}

pub mod hyper {
    pub use hyper::*;
}

pub mod serde_json {
    pub use serde_json::*;
}

pub mod serde_yaml {
    pub use serde_yaml::*;
}

pub mod toml {
    pub use toml::*;
}

pub mod tokio {
    pub use tokio::*;
}

pub use any::{AnyRef, AnyValue};
pub use error::{
    AppError, Result, ERR_ARGUMENT, ERR_CAST, ERR_CONVERT, ERR_FORMAT, ERR_INTERNAL, ERR_PARSE,
};
pub use future::{FutureHandler, FutureObj};
pub use map::{AnyContext, AnyContextUtil, MapUtil};
pub use number::{
    cast_i64_to_i16, cast_i64_to_i32, cast_u64_to_u16, cast_u64_to_u32, cast_u64_to_usize,
};
pub use string::StringUtil;
pub use template::{
    render_template, render_template_recursion, ContextType, TemplateContext, TemplateContextUtil,
};
pub use vec::VecUtil;
