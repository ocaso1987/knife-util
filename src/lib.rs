//! 通用工具类
pub(crate) mod any;
pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod map;
pub(crate) mod string;
pub(crate) mod template;
pub(crate) mod vec;

pub mod async_trait {
    pub use async_trait::*;
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

pub mod hyper {
    pub use hyper::*;
}

pub mod futures {
    pub use futures::*;
}

pub mod ctor {
    pub use ctor::*;
}

pub use any::{AnyRef, AnyValue};
pub use error::{AppError, ERR_ARGUMENT, ERR_CONVERT, ERR_FORMAT, ERR_INTERNAL, ERR_PARSE};
pub use handler::Handler;
pub use map::MapUtil;
pub use string::StringUtil;
pub use template::{render_template, render_template_recursion, ContextMapUtil, ContextType};
pub use vec::VecUtil;
