use std::{future::Future, marker::PhantomData, mem, pin::Pin};

/// 用来代替dyn Future<Output = R> + Send + 'static的工具
pub struct FutureObj<'a, T> {
    future: *mut (dyn Future<Output = T> + Send + 'static),
    _marker: PhantomData<&'a ()>,
}

unsafe impl<T> Send for FutureObj<'_, T> {}
unsafe impl<T> Sync for FutureObj<'_, T> {}

impl<'a, T> FutureObj<'a, T> {
    pub fn new(f: Box<dyn Future<Output = T> + Send + 'a>) -> Self {
        let ptr: *mut (dyn Future<Output = T> + Send + 'static) =
            unsafe { mem::transmute(Box::into_raw(f)) };
        FutureObj {
            future: ptr,
            _marker: PhantomData,
        }
    }
}
impl<T> Future for FutureObj<'_, T> {
    type Output = T;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        unsafe { Pin::new_unchecked(&mut *self.future).poll(cx) }
    }
}

/// 用来代替dyn FnOnce(E) -> (dyn Future<Output = R> + Send + 'static) + Send + 'static)的工具
pub struct FutureHandler<'a, E, R> {
    handler: *mut (dyn FnOnce(E) -> FutureObj<'static, R> + Send + 'static),
    _marker: PhantomData<&'a ()>,
}

unsafe impl<E, R> Send for FutureHandler<'_, E, R> {}
unsafe impl<E, R> Sync for FutureHandler<'_, E, R> {}

impl<'a, E, R> FutureHandler<'a, E, R> {
    pub fn new(f: Box<dyn FnOnce(E) -> FutureObj<'a, R> + Send + 'a>) -> Self {
        let ptr: *mut (dyn FnOnce(E) -> FutureObj<'static, R> + Send + 'static) =
            unsafe { mem::transmute(Box::into_raw(f)) };
        FutureHandler {
            handler: ptr,
            _marker: PhantomData,
        }
    }

    pub async fn invoke(&self, event: E) -> R {
        let f = unsafe { Box::from_raw(self.handler) };
        f(event).await
    }
}
