//! 跟内置Value对象的工具类
//!
//! 支持对Bson、JSON、Yaml、Toml等格式的数据转换
pub(crate) mod base;
pub(crate) mod ser;
pub(crate) mod traits;
pub(crate) mod value;

pub(crate) mod implement {
    pub(crate) mod merge;
    pub(crate) mod pointer;
}

pub(crate) mod bson {
    pub(crate) mod pointer;
}

pub(crate) mod json {
    pub(crate) mod convert;
    pub(crate) mod pointer;
}

pub(crate) mod rbs {
    pub(crate) mod convert;
}

pub(crate) mod toml {
    pub(crate) mod convert;
    pub(crate) mod pointer;
}

pub(crate) mod yaml {
    pub(crate) mod convert;
    pub(crate) mod pointer;
}

pub use traits::ConvertExt;
pub use traits::MergeExt;
pub use traits::PointerExt;
pub use value::Value;
