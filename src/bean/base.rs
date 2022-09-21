use crate::{Result, Value};

/// 支持对象转换为内置Value格式
pub trait AsValueTrait {
    fn as_value(&self) -> Result<Value>;
}

/// 支持内置Value转换为指定对象
pub trait FromValueTrait {
    fn from_value(value: &Value) -> Result<Self>
    where
        Self: Sized;
}

/// 支持两个相同的Object对象进行合并
pub trait MergeTrait {
    fn merge_self(&mut self, target: &Self) -> Result<Self>
    where
        Self: Sized;
}

/// 支持某个对象合并Value内置对象
pub trait MergeValueTrait {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self>
    where
        Self: Sized;
}

/// 可遍历特征
///
/// 支持以/a/b/c/2的方式获取指定层级上的对象
/// 特殊字行可转义采用~1代替/，采用~0代替~
/// 更多信息可参考：[RFC6901](https://tools.ietf.org/html/rfc6901)
pub trait PointerTrait {
    type Context;
    fn p(&self, pointer: &str) -> Option<&Self::Context>;
}

pub(super) fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}
