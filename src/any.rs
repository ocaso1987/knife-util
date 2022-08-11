//! 简化对象类型处理的工具
use std::{
    alloc::{alloc_zeroed, Layout},
    any::type_name,
    mem::MaybeUninit,
    ptr::{read, read_volatile, write, write_volatile},
};

/// 用于代替Box<dyn Any>的工具
///
/// 与Box不同的是，可以执行一次take操作。
/// 用于代替Box<dyn Any>，支持以指定类型存入数据，又以指定类型进行取出
/// 存入与取出数据时类型需保持一致
/// 需要注意的是，AnyValue被简化为了Send+Sync类型的，但存入的数据并不做检查
/// 在多线程环境下的使用，其数据安全性由开发者自身确认
///
/// ```
/// #[test]
/// fn test() {
///     assert_eq!(AnyValue::new(1).take::<i32>(), 1);
///     assert_eq!(AnyValue::new("2").take::<&str>(), "2");
///     assert_ne!(AnyValue::new(1).take::<i32>(), 2);
///     assert_ne!(AnyValue::new("1").take::<&str>(), "2");
///     assert!(panic::catch_unwind(|| {
///         AnyValue::new("2").take::<i32>();
///     })
///    .is_err());
/// }
/// ```
#[derive(Clone)]
pub struct AnyValue {
    /// 用于存放实际对象
    pointer: *mut u8,

    /// 数据类型，用于取出时进行检查
    type_name: String,

    /// 数据是否已取出
    is_taken: bool,
}

impl std::fmt::Debug for AnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnyValue").field(&self.type_name).finish()
    }
}
unsafe impl Send for AnyValue {}
unsafe impl Sync for AnyValue {}

impl AnyValue {
    /// 存入数据，需指定数据类型V
    pub fn new<V>(v: V) -> Self {
        Self::new_with_write(v, |dst, src| unsafe { write(dst, src) })
    }

    /// 存入数据，需指定数据类型V
    pub fn new_volatile<V>(v: V) -> Self {
        Self::new_with_write(v, |dst, src| unsafe { write_volatile(dst, src) })
    }

    /// 存入数据，需指定数据类型V
    pub fn new_with_write<V, F>(v: V, f: F) -> Self
    where
        F: Fn(*mut V, V),
    {
        let type_name = type_name::<V>().to_string();
        let layout = Layout::new::<MaybeUninit<V>>();
        let pointer = unsafe { alloc_zeroed(layout) };
        f(pointer.cast::<V>(), v);
        AnyValue {
            pointer,
            type_name,
            is_taken: false,
        }
    }

    /// 取出可变数据引用，可采用继承类型的特征，不需要与原始类型完全一致
    pub fn as_mut<V>(&self) -> &mut V {
        unsafe { &mut *(self.pointer as *const V as *mut V) }
    }

    /// 取出数据引用，可采用继承类型的特征，不需要与原始类型完全一致
    pub fn as_ref<V>(&self) -> &V {
        unsafe { &*(self.pointer as *const V as *mut V) }
    }

    /// 取出数据，只能执行一次
    pub fn take<V>(&self) -> V {
        self.take_with_read(|src| unsafe { read(src) })
    }

    /// 取出数据，只能执行一次
    pub fn take_volatile<V>(&self) -> V {
        self.take_with_read(|src| unsafe { read_volatile(src) })
    }

    /// 取出数据，只能执行一次
    ///
    /// ```
    /// #[test]
    /// fn test() {
    ///     assert!(panic::catch_unwind(|| {
    ///         let mut v = AnyValue::new("abc");
    ///         assert_eq!(v.take::<&str>(), "abc");
    ///         assert_eq!(v.take::<&str>(), "abc");
    ///     })
    ///     .is_err());
    /// }
    /// ```
    pub fn take_with_read<V, F>(&self, f: F) -> V
    where
        F: Fn(*const V) -> V,
    {
        if self.is_taken {
            panic!("该值已取出:{}", self.type_name);
        }
        let type_name = type_name::<V>();
        if type_name != self.type_name {
            panic!("类型不一致，期望:{},实际:{}", type_name, self.type_name);
        }
        let ptr = self.pointer as *const V;
        let v = f(ptr);
        unsafe { &mut *(self as *const AnyValue as *mut AnyValue) }.is_taken = true;
        v
    }

    /// 取出字符数据，如果存入数据不是字符类型将抛出异常
    pub fn get_string(&self) -> String {
        let type_name = type_name::<String>();
        if type_name != self.type_name {
            panic!("类型不一致，期望:{},实际:{}", type_name, self.type_name);
        }
        self.as_ref::<String>().to_string()
    }

    /// 取出字符数据，如果存入数据不是字符类型尝试进行转换
    pub fn as_string(&self) -> String {
        if self.type_name == "XXX" {
            panic!("xxx");
        } else {
            panic!("类型无法转换为字符串，实际:{}", self.type_name);
        }
    }
}

/// 用于代替&指针的工具
///
/// 与&指针不同的是，可以在一定程度简化对生命周期的定义。
/// 用于代替&指针，支持以指定类型存入数据，又以指定类型进行取出
/// 存入与取出数据时类型需保持一致
/// 需要注意的是，AnyRef被简化为了Send+Sync类型的，但存入的数据并不做检查
/// 在多线程环境下的使用，其数据安全性由开发者自身确认
///
/// ```
/// #[test]
/// fn test() {
// assert_eq!(*AnyRef::new(&1).as_ref::<i32>(), 1);
// assert_eq!(*AnyRef::new(&"2").as_mut::<&str>(), "2");
// assert_ne!(*AnyRef::new(&1).as_ref::<i32>(), 2);
// assert_ne!(*AnyRef::new(&"1").as_mut::<&str>(), "2");
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AnyRef {
    /// 用于存放实际对象
    pointer: *mut u8,

    /// 数据类型，用于取出时进行检查
    type_name: String,
}
unsafe impl Send for AnyRef {}
unsafe impl Sync for AnyRef {}

impl AnyRef {
    /// 存入数据，需指定数据类型V
    pub fn new<V>(v: &V) -> Self {
        let type_name = type_name::<V>().to_string();
        let pointer = v as *const V as *mut V as *mut u8;
        AnyRef { pointer, type_name }
    }

    /// 取出可变数据引用，可采用继承类型的特征，不需要与原始类型完全一致
    pub fn as_mut<V>(&self) -> &mut V {
        unsafe { &mut *(self.pointer as *const V as *mut V) }
    }

    /// 取出数据引用，可采用继承类型的特征，不需要与原始类型完全一致
    pub fn as_ref<V>(&self) -> &V {
        unsafe { &*(self.pointer as *const V as *mut V) }
    }

    /// 取出字符数据，如果存入数据不是字符类型将抛出异常
    pub fn get_string(&self) -> String {
        let type_name = type_name::<String>();
        if type_name != self.type_name {
            panic!("类型不一致，期望:{},实际:{}", type_name, self.type_name);
        }
        self.as_ref::<String>().to_string()
    }

    /// 取出字符数据，如果存入数据不是字符类型尝试进行转换
    pub fn as_string(&self) -> String {
        if self.type_name == "XXX" {
            panic!("xxx");
        } else {
            panic!("类型无法转换为字符串，实际:{}", self.type_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::any::AnyRef;

    #[test]
    fn test() {
        assert_eq!(*AnyRef::new(&1).as_ref::<i32>(), 1);
        assert_eq!(*AnyRef::new(&"2").as_mut::<&str>(), "2");
        assert_ne!(*AnyRef::new(&1).as_ref::<i32>(), 2);
        assert_ne!(*AnyRef::new(&"1").as_mut::<&str>(), "2");
    }
}
