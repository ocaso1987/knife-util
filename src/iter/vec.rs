use crate::Result;

use super::collect_result::CollectResultTrait;

/// 用于列表类型数据操作工具类
pub trait VecExt<T> {
    /// 将vec进行map转换
    fn map_collect<F, R>(&self, fun: F) -> Vec<R>
    where
        F: Fn(&T) -> R;

    /// 将vec进行map转换，当转换过程中出现异常时，抛出异常
    fn map_collect_into_vec<F, R>(&self, fun: F) -> Result<Vec<R>>
    where
        F: Fn(&T) -> Result<R>;
}

impl<T> VecExt<T> for Vec<T> {
    fn map_collect<F, R>(&self, fun: F) -> Vec<R>
    where
        F: Fn(&T) -> R,
    {
        self.iter().map(fun).collect()
    }

    fn map_collect_into_vec<F, R>(&self, fun: F) -> Result<Vec<R>>
    where
        F: Fn(&T) -> Result<R>,
    {
        self.iter().map(fun).collect_into_vec()
    }
}
