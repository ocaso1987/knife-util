//! 对常见数据结构及基本类型进行操作的工具类
mod double;
mod integer;
mod result;
mod string;

pub use double::DoubleExt;
pub use integer::IntegerExt;
pub use result::{Result, ResultExt, OK};
pub use string::StringExt;
