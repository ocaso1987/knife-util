//! 跟分页处理相关的工具类及模型
mod main;
mod model;

pub use main::get_offset;
pub use model::{PageRequest, PageResult};
