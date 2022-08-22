//! 通用工具类
pub(crate) mod any;
pub(crate) mod error;
pub(crate) mod future;
pub(crate) mod map;
pub(crate) mod number;
pub(crate) mod string;
pub(crate) mod template;
pub(crate) mod value;
pub(crate) mod vec;

/// Reexport
pub mod async_trait {
    pub use async_trait::*;
}

/// Reexport
pub mod ctor {
    pub use ctor::*;
}

/// Reexport
pub mod hyper {
    pub use hyper::*;
}

/// Reexport
pub mod tokio {
    pub use tokio::*;
}

pub use any::{AnyRef, AnyValue};
pub use error::{
    AnyError, Result, ERR_ARGUMENT, ERR_CAST, ERR_CONVERT, ERR_DB, ERR_FORMAT, ERR_INTERNAL,
    ERR_PARSE, ERR_WEB,
};
pub use future::{FutureHandler, FutureObj};
pub use map::{AnyContext, AnyContextExt, ContextExt, MapExt};
pub use number::{
    cast_i64_to_i16, cast_i64_to_i32, cast_u64_to_u16, cast_u64_to_u32, cast_u64_to_usize,
};
pub use string::StringExt;
pub use template::{
    render_template, render_template_recursion, ContextType, TemplateContext, TemplateContextExt,
};
pub use vec::VecExt;
pub use value::Value;