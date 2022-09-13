use serde::{Deserialize, Serialize};

use crate::types::VecExt;

/// 分页请求
#[derive(Deserialize, Debug, Clone)]
pub struct PageRequest<T> {
    /// 页码
    pub page: u64,

    /// 每页条数
    pub limit: u64,

    /// 请求参数
    #[serde(flatten)]
    pub target: T,
}

impl<T> PageRequest<T> {
    pub fn map<F, R>(&self, fun: F) -> PageRequest<R>
    where
        F: Fn(&T) -> R,
    {
        PageRequest {
            page: self.page,
            limit: self.limit,
            target: fun(&self.target),
        }
    }
}

/// 分页响应
#[derive(Serialize, Debug, Clone)]
pub struct PageResult<T> {
    /// 页码
    pub page: u64,

    /// 每页条数
    pub limit: u64,

    /// 总数
    pub total: u64,

    /// 返回列表
    pub list: Vec<T>,
}

impl<T> PageResult<T> {
    pub fn map<F, R>(&self, fun: F) -> PageResult<R>
    where
        F: Fn(&T) -> R,
    {
        PageResult {
            page: self.page,
            limit: self.limit,
            total: self.total,
            list: self.list.map(fun),
        }
    }
}
