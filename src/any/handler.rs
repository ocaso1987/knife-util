use std::{marker::PhantomData, mem};

use super::future::AnyFuture;

/// 用来代替dyn FnOnce(E) -> (dyn Future<Output = R> + Send + 'static) + Send + 'static)的工具
pub struct AnyHandler<'a, E, R> {
    handler: *mut (dyn FnOnce(E) -> AnyFuture<'static, R> + Send + 'static),
    _marker: PhantomData<&'a ()>,
}

unsafe impl<E, R> Send for AnyHandler<'_, E, R> {}
unsafe impl<E, R> Sync for AnyHandler<'_, E, R> {}

impl<'a, E, R> AnyHandler<'a, E, R> {
    pub fn new(f: Box<dyn FnOnce(E) -> AnyFuture<'a, R> + Send + 'a>) -> Self {
        let ptr: *mut (dyn FnOnce(E) -> AnyFuture<'static, R> + Send + 'static) =
            unsafe { mem::transmute(Box::into_raw(f)) };
        AnyHandler {
            handler: ptr,
            _marker: PhantomData,
        }
    }

    pub async fn invoke(&self, event: E) -> R {
        let f = unsafe { Box::from_raw(self.handler) };
        f(event).await
    }
}
