//! 通用错误处理工具
pub(crate) mod constant;
pub(crate) mod display;
pub(crate) mod main;
pub(crate) mod cast;
pub(crate) mod from;
pub(crate) mod tests;

pub use constant::*;
pub use main::AppError;
