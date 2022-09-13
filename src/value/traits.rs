use crate::Result;

use super::Value;

/// 支持对象与内置Value格式互相转换
pub trait ConvertExt {
    fn as_value(&self) -> Value;
    fn from_value(value: &Value) -> Self;
}

/// 支持两个相同的Object对象进行合并
pub trait MergeExt {
    type Context;
    fn merge<'a>(&'a mut self, target: &'a Self) -> Result<Self::Context>;
}

/// 可遍历特征
///
/// 支持以/a/b/c/2的方式获取指定层级上的对象
/// 特殊字行可转义采用~1代替/，采用~0代替~
/// 更多信息可参考：[RFC6901](https://tools.ietf.org/html/rfc6901)
pub trait PointerExt {
    type Context;
    fn p(&self, pointer: &str) -> Option<&Self::Context>;
}
