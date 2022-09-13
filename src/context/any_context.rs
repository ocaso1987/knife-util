use crate::any::AnyValue;

/// 键为字符类型且值为AnyValue的上下文操作类
///
/// 由于当前特征中存储的是非序列化的任意对象，因此此特征中的操作不能与ContextExt中的操作混用
/// 譬如：
///     map.insert_any("a", "abc".to_string());
///     map.get_string("a");    错误!
/// 反之亦然
pub trait AnyContextExt {
    /// 集合中获取AnyValue上下文类型数据
    fn get_any(&mut self, key: &str) -> Option<&AnyValue>;
    /// 集合中插入AnyValue上下文类型数据
    fn insert_any<T>(&mut self, key: &str, value: T);

    /// 获取插入的任意类型数据的引用指针
    fn get_ref<T>(&mut self, key: &str) -> &T {
        self.get_any(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_ref::<T>()
    }
    /// 获取插入的任意类型数据的可变引用指针
    fn get_mut<T>(&mut self, key: &str) -> &mut T {
        self.get_any(key)
            .expect(format!("{}不能为空", key).as_str())
            .as_mut::<T>()
    }
    /// 获取插入的任意类型数据的引用指针，如果未找到则返回空
    fn get_opt_ref<T>(&mut self, key: &str) -> Option<&T> {
        self.get_any(key).map(|x| x.as_ref::<T>())
    }
    /// 获取插入的任意类型数据的可变引用指针，如果未找到则返回空
    fn get_opt_mut<T>(&mut self, key: &str) -> Option<&mut T> {
        self.get_any(key).map(|x| x.as_mut::<T>())
    }
}
