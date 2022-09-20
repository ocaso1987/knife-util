//! 数据处理与转换工具类
//!
//! 通过内置对象实现对任意格式数据间的处理与转换
pub(crate) mod base;
pub(crate) mod json;
pub(crate) mod rbs;
pub(crate) mod types;
pub(crate) mod value;
pub(crate) mod yaml;

pub use base::{AsValueTrait, FromValueTrait, MergeTrait, MergeValueTrait, PointerTrait};
