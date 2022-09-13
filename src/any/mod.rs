//! 一些可以直接操作内存或指针的工具
//!
//! 通常会进行一些unsafe操作，请在明确知道操作对象的安全情况下使用
pub(crate) mod future;
pub(crate) mod handler;
pub(crate) mod r#ref;
pub(crate) mod value;

pub use future::AnyFuture;
pub use handler::AnyHandler;
pub use r#ref::AnyRef;
pub use value::AnyValue;
