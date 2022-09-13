use std::{
    alloc::{alloc_zeroed, Layout},
    any::type_name,
    fmt::Display,
    mem::MaybeUninit,
};

/// 用于代替Box<dyn Any>的工具
///
/// 与Box不同的是，可以执行一次take操作。
/// 用于代替Box<dyn Any>，支持以指定类型存入数据，又以指定类型进行取出
/// 存入与取出数据时类型需保持一致
/// 需要注意的是，AnyValue被简化为了Send+Sync类型的，但存入的数据并不做检查
/// 在多线程环境下的使用，其数据安全性由开发者自身确认
#[derive(Clone)]
pub struct AnyValue {
    /// 用于存放实际对象
    pointer: Option<*mut u8>,

    /// 数据类型，用于取出时进行检查
    type_name: String,

    /// 数据是否已取出
    is_taken: bool,
}

impl Display for AnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnyValue").field(&self.type_name).finish()
    }
}

impl std::fmt::Debug for AnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnyValue").field(&self.type_name).finish()
    }
}

unsafe impl Send for AnyValue {}
unsafe impl Sync for AnyValue {}

impl AnyValue {
    /// 初始化一个不包含对象指针的空对象，用于点位
    pub fn new_zero() -> Self {
        AnyValue {
            pointer: None,
            type_name: "".to_string(),
            is_taken: false,
        }
    }
    /// 存入数据，需指定数据类型V
    pub fn new<V>(v: V) -> Self {
        let res = Self::new_zero();
        res.replace(v);
        res
    }

    /// 存入数据，需指定数据类型V
    pub fn new_volatile<V>(v: V) -> Self {
        let res = Self::new_zero();
        res.replace_volatile(v);
        res
    }

    /// 检查指针是否绑定数据
    pub fn is_empty(&self) -> bool {
        self.pointer.is_none()
    }

    /// 替换数据，需指定数据类型V
    pub fn replace<V>(&self, v: V) {
        self.check_type::<V>();
        self.replace_with_write(v, |dst, src| unsafe { std::ptr::write(dst, src) })
    }

    /// 替换数据，需指定数据类型V
    pub fn replace_volatile<V>(&self, v: V) {
        self.check_type::<V>();
        self.replace_with_write(v, |dst, src| unsafe { std::ptr::write_volatile(dst, src) })
    }

    /// 替换数据，需指定数据类型V
    pub fn replace_with_write<V, F>(&self, v: V, f: F)
    where
        F: Fn(*mut V, V),
    {
        let layout = Layout::new::<MaybeUninit<V>>();
        let pointer = unsafe { alloc_zeroed(layout) };
        f(pointer.cast::<V>(), v);
        unsafe {
            let ptr = &mut *(self as *const Self as *mut Self);
            ptr.type_name = type_name::<V>().to_string();
            ptr.pointer = Some(pointer);
            ptr.is_taken = false;
        }
    }

    /// 取出可变数据引用，可采用继承类型的特征，不需要与原始类型完全一致
    pub fn as_mut<V>(&self) -> &mut V {
        self.check_taken();
        self.check_type::<V>();
        self.check_none();
        unsafe { &mut *(self.pointer.unwrap() as *const V as *mut V) }
    }

    /// 取出数据引用，可采用继承类型的特征，不需要与原始类型完全一致
    pub fn as_ref<V>(&self) -> &V {
        self.check_taken();
        self.check_type::<V>();
        self.check_none();
        unsafe { &*(self.pointer.unwrap() as *const V as *mut V) }
    }

    /// 取出数据，只能执行一次
    pub fn take<V>(&self) -> V {
        self.check_taken();
        self.check_type::<V>();
        self.check_none();
        self.take_with_read(|src| unsafe { std::ptr::read(src) })
    }

    /// 取出数据，只能执行一次
    pub fn take_volatile<V>(&self) -> V {
        self.check_taken();
        self.check_type::<V>();
        self.check_none();
        self.take_with_read(|src| unsafe { std::ptr::read_volatile(src) })
    }

    /// 取出数据，只能执行一次
    pub fn take_with_read<V, F>(&self, f: F) -> V
    where
        F: Fn(*const V) -> V,
    {
        let ptr = self.pointer.unwrap() as *const V;
        let v = f(ptr);
        unsafe { &mut *(self as *const AnyValue as *mut AnyValue) }.is_taken = true;
        v
    }

    /// 取出字符数据，如果存入数据不是字符类型将抛出异常
    pub fn get_string(&self) -> String {
        self.as_ref::<String>().to_string()
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

    fn check_taken(&self) {
        if self.is_taken {
            panic!("该值已取出:{}", self.type_name);
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
    use std::panic;

    use crate::any::value::AnyValue;

    #[test]
    fn test() {
        assert_eq!(AnyValue::new(1).take::<i32>(), 1);
        assert_eq!(AnyValue::new("2").take::<&str>(), "2");
        assert_ne!(AnyValue::new(1).take::<i32>(), 2);
        assert_ne!(AnyValue::new("1").take::<&str>(), "2");
        assert!(panic::catch_unwind(|| {
            AnyValue::new("2").take::<i32>();
        })
        .is_err());
        assert!(panic::catch_unwind(|| {
            let v = AnyValue::new("abc");
            assert_eq!(v.take::<&str>(), "abc");
            assert_eq!(v.take::<&str>(), "abc");
        })
        .is_err());
    }
}
