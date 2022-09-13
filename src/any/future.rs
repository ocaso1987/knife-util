use std::{marker::PhantomData, mem, pin::Pin};

use futures::Future;

/// 用来代替dyn Future<Output = R> + Send + 'static的工具
pub struct AnyFuture<'a, T> {
    future: *mut (dyn Future<Output = T> + Send + 'static),
    _marker: PhantomData<&'a ()>,
}

unsafe impl<T> Send for AnyFuture<'_, T> {}
unsafe impl<T> Sync for AnyFuture<'_, T> {}

impl<'a, T> AnyFuture<'a, T> {
    pub fn new(f: Box<dyn Future<Output = T> + Send + 'a>) -> Self {
        let ptr: *mut (dyn Future<Output = T> + Send + 'static) =
            unsafe { mem::transmute(Box::into_raw(f)) };
        AnyFuture {
            future: ptr,
            _marker: PhantomData,
        }
    }
}
impl<T> Future for AnyFuture<'_, T> {
    type Output = T;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        unsafe { Pin::new_unchecked(&mut *self.future).poll(cx) }
    }
}
