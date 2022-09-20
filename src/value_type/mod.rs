//! Value内置对象
//!
//! 实现对任意对象间的数据转换
pub(crate) mod display;
pub(crate) mod main;
pub(crate) mod ser;
pub(crate) mod tests;

pub use main::Value;
