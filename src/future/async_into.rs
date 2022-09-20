use async_trait::async_trait;

/// 支持异步Into操作
#[async_trait]
pub trait AsyncInto<T>: Sized {
    async fn async_into(self) -> T;
}
