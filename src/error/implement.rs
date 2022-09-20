use std::fmt::Display;

use super::{AppError, ERR_DESERIALIZE, ERR_SERIALIZE};

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause_ref()
    }

    fn description(&self) -> &str {
        self.msg_ref()
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl serde::ser::Error for AppError {
    fn custom<T: Display>(msg: T) -> Self {
        ERR_SERIALIZE.msg_detail(msg.to_string().as_str())
    }
}

impl serde::de::Error for AppError {
    fn custom<T: Display>(msg: T) -> Self {
        ERR_DESERIALIZE.msg_detail(msg.to_string().as_str())
    }
}
