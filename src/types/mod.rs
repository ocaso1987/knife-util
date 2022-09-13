//! 对常见数据结构及基本类型进行操作的工具类
pub(crate) mod double;
pub(crate) mod integer;
pub(crate) mod string;
pub(crate) mod vec;

pub use double::DoubleCastExt;
pub use integer::IntegerCastExt;
pub use string::StringExt;
pub use vec::VecExt;
