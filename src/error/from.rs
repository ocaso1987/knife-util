use super::{AppError, ERR_CONVERT, ERR_DB_ACTION, ERR_ENV_VAR, ERR_IO, ERR_WEB};

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        ERR_IO.cause(err)
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        ERR_ENV_VAR.cause(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        ERR_CONVERT.cause(err)
    }
}

impl From<serde_yaml::Error> for AppError {
    fn from(err: serde_yaml::Error) -> Self {
        ERR_CONVERT.cause(err)
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        ERR_CONVERT.cause(err)
    }
}

impl From<rbatis::Error> for AppError {
    fn from(err: rbatis::Error) -> Self {
        ERR_DB_ACTION.cause(err)
    }
}

impl From<hyper::Error> for AppError {
    fn from(err: hyper::Error) -> Self {
        ERR_WEB.cause(err)
    }
}
