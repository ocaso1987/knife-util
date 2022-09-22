use crate::{error::ERR_VALIDATION, Result, OK};

/// 根据Web请求分页参数获取数据需要的参数offset
pub fn get_offset(page: u64, limit: u64) -> Result<u64> {
    if page < 1 {
        Err(ERR_VALIDATION.msg_detail("page参数必须大于0"))
    } else {
        OK((page - 1) * limit)
    }
}
