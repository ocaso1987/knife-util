//! 上下文操作工具类
//!
//! 可以使对象支持上下文操作，并对其进行基本类型的存取数据
mod any_context_trait;
mod context_trait;
mod map;

pub use any_context_trait::AnyContextTrait;
pub use context_trait::ContextTrait;
