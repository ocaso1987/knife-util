/// 用于列表类型数据操作工具类
pub trait VecExt<T> {
    fn map<F, R>(&self, fun: F) -> Vec<R>
    where
        F: Fn(&T) -> R;
}

impl<T> VecExt<T> for Vec<T> {
    fn map<F, R>(&self, fun: F) -> Vec<R>
    where
        F: Fn(&T) -> R,
    {
        self.iter().map(fun).collect()
    }
}
