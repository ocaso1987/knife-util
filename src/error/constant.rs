pub use lazy_static::lazy_static;

use crate::error::AppError;

lazy_static! {

    /// 读取环境变量出现异常
    pub static ref ERR_ENV_VAR: AppError = AppError::new("ERR_ENV_VAR", "100001", "读取环境变量出现异常");

    /// 读取IO出现异常
    pub static ref ERR_IO: AppError = AppError::new("ERR_IO", "100002", "读取IO操作出现异常");

    /// 全局数据格式转换异常
    pub static ref ERR_CONVERT: AppError = AppError::new("ERR_CONVERT", "100003", "数据转换出现异常");

    /// 全局数据内部格式转换异常
    pub static ref ERR_CAST: AppError = AppError::new("ERR_CAST", "100004", "数据内部转换出现异常");

    /// 序列化数据异常
    pub static ref ERR_SERIALIZE: AppError = AppError::new("ERR_SERIALIZE", "100005", "序列化数据出现异常");

    /// 反序列化数据异常
    pub static ref ERR_DESERIALIZE: AppError = AppError::new("ERR_DESERIALIZE", "100006", "反序列化数据出现异常");

    /// 全局数据处理异常
    pub static ref ERR_DATA: AppError = AppError::new("ERR_DATA", "100007", "数据处理出现异常");

    /// 全局数据解析异常
    pub static ref ERR_PARSE: AppError = AppError::new("ERR_PARSE", "100008", "数据解析出现异常");

    /// 全局数据解析异常
    pub static ref ERR_MERGE: AppError = AppError::new("ERR_MERGE", "100009", "数据进行合并处理出现异常");

    /// 全局数据格式化异常
    pub static ref ERR_FORMAT: AppError = AppError::new("ERR_FORMAT", "100010", "数据格式化出现异常");

    /// 请求参数错误
    pub static ref ERR_ARGUMENT: AppError = AppError::new("ERR_ARGUMENT", "100011", "请求参数错误");

    /// 参数校验失败
    pub static ref ERR_VALIDATION: AppError = AppError::new("ERR_VALIDATION", "100012", "参数校验失败");

    /// Web处理错误
    pub static ref ERR_WEB: AppError = AppError::new("ERR_WEB", "100013", "Web处理错误");

    /// 数据库操作异常
    pub static ref ERR_DB_ACTION: AppError = AppError::new("ERR_DB_ACTION", "100014", "数据库操作异常");

    /// 数据库数据错误
    pub static ref ERR_DB_DATA: AppError = AppError::new("ERR_DB_DATA", "100015", "数据库数据错误");

    /// 全局内部异常
    pub static ref ERR_INTERNAL: AppError = AppError::new("ERR_INTERNAL", "999999", "内部异常");

}
