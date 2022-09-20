use async_trait::async_trait;

/// 支持异步From操作
#[async_trait]
pub trait AsyncFrom<T>: Sized {
    async fn async_from(_: T) -> Self;
}
