use std::{marker::PhantomData, mem};

use futures::task::FutureObj;

/// 用来代替dyn FnOnce(E) -> （dyn Future<Output = R> + Send + 'static) + Send + 'static)的工具
/// 适用于在代码中执行异步回调实现中set_hook(xxx)：
///   |_| async move { do_sth() }
pub struct Handler<'a, E, R> {
    handler: *mut (dyn FnOnce(E) -> FutureObj<'static, R> + Send + 'static),
    _marker: PhantomData<&'a ()>,
}

unsafe impl<E, R> Send for Handler<'_, E, R> {}
unsafe impl<E, R> Sync for Handler<'_, E, R> {}

impl<'a, E, R> Handler<'a, E, R> {
    pub fn new(f: Box<dyn FnOnce(E) -> FutureObj<'a, R> + Send + 'a>) -> Self {
        let ptr: *mut (dyn FnOnce(E) -> FutureObj<'static, R> + Send + 'static) =
            unsafe { mem::transmute(Box::into_raw(f)) };
        Handler {
            handler: ptr,
            _marker: PhantomData,
        }
    }

    pub async fn invoke(&self, event: E) -> R {
        let f = unsafe { Box::from_raw(self.handler) };
        f(event).await
    }
}
