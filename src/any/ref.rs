use std::any::type_name;

/// 用于代替&指针的工具
///
/// 与&指针不同的是，可以在一定程度简化对生命周期的定义。
/// 用于代替&指针，支持以指定类型存入数据，又以指定类型进行取出
/// 存入与取出数据时类型需保持一致
/// 需要注意的是，AnyRef被简化为了Send+Sync类型的，但存入的数据并不做检查
/// 在多线程环境下的使用，其数据安全性由开发者自身确认
#[derive(Clone, Copy)]
pub struct AnyRef {
    /// 用于存放实际对象
    pointer: Option<*mut u8>,

    /// 数据类型，用于取出时进行检查
    type_name: &'static str,
}

impl std::fmt::Debug for AnyRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnyRef").field(&self.type_name).finish()
    }
}

unsafe impl Send for AnyRef {}
unsafe impl Sync for AnyRef {}

impl AnyRef {
    /// 初始化一个不包含对象指针的空对象，用于点位
    pub fn new_zero() -> Self {
        AnyRef {
            pointer: None,
            type_name: "",
        }
    }

    /// 存入数据，需指定数据类型V
    pub fn new<V>(v: &V) -> Self {
        let mut res = Self::new_zero();
        res.replace(v);
        res
    }

    /// 检查指针是否绑定数据
    pub fn is_empty(&self) -> bool {
        self.pointer.is_none()
    }

    /// 存入数据，需指定数据类型V
    pub fn replace<V>(&mut self, v: &V) {
        self.check_type::<V>();
        self.type_name = type_name::<V>();
        let pointer = v as *const V as *mut V as *mut u8;
        self.pointer.replace(pointer);
    }

    /// 取出可变数据引用
    #[allow(clippy::mut_from_ref)]
    pub fn to_mut<V>(&self) -> &mut V {
        self.check_type::<V>();
        self.check_none();
        unsafe { &mut *(self.pointer.unwrap() as *const V as *mut V) }
    }

    /// 取出数据引用
    pub fn to_ref<V>(&self) -> &V {
        self.check_type::<V>();
        self.check_none();
        unsafe { &*(self.pointer.unwrap() as *const V as *mut V) }
    }

    /// 取出字符数据，如果存入数据不是字符类型将抛出异常
    pub fn get_string(&self) -> String {
        self.to_ref::<String>().to_string()
    }

    fn check_type<V>(&self) {
        if self.type_name.is_empty() {
            return;
        }
        let type_name = type_name::<V>();
        if type_name != self.type_name {
            panic!("类型不一致，期望:{},实际:{}", type_name, self.type_name);
        }
    }

    fn check_none(&self) {
        if self.pointer.is_none() {
            panic!("数据为空:{}", self.type_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::any::r#ref::AnyRef;

    #[test]
    fn test() {
        assert_eq!(*AnyRef::new(&1).to_ref::<i32>(), 1);
        assert_eq!(*AnyRef::new(&"2").to_mut::<&str>(), "2");
        assert_ne!(*AnyRef::new(&1).to_ref::<i32>(), 2);
        assert_ne!(*AnyRef::new(&"1").to_mut::<&str>(), "2");
    }
}
