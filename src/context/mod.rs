//! 上下文操作工具类
//!
//! 可以使对象支持上下文操作，并对其进行基本类型的存取数据
pub(crate) mod any_context;
pub(crate) mod context;
pub(crate) mod implement;

pub use any_context::AnyContextExt;
pub use context::ContextExt;
