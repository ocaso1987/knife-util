use crate::error::AppError;

/// 实现对std::result::Result的代理操作
pub trait ResultExt<T> {
    fn into_future(self) -> futures::future::Ready<Result<T>>;
}

impl<T> ResultExt<T> for Result<T> {
    fn into_future(self) -> futures::future::Ready<Result<T>> {
        futures::future::ready(self)
    }
}

/// 可代替std::result::Result<T, AnyError>操作的工具
pub type Result<T> = std::result::Result<T, AppError>;

/// 默认返回成功
#[allow(non_snake_case)]
pub fn OK<T>(t: T) -> Result<T> {
    Ok(t)
}

#[cfg(test)]
mod tests {
    use futures::TryFutureExt;

    use rbatis::rbdc::block_on;

    use crate::{ResultExt, OK};

    #[test]
    fn test() {
        let func = async {
            let res = OK(1).into_future().and_then(|x| async move { OK(x + 1) });
            assert_eq!(res.await.unwrap(), 2);
        };
        block_on!(func);
    }
}
