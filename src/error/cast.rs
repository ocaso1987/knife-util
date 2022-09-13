use std::fmt::Display;

use super::{AppError, ERR_DESERIALIZE, ERR_SERIALIZE};

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if self.cause.is_empty() {
            None
        } else {
            let cause = self
                .cause
                .as_ref::<Box<dyn std::error::Error + Send + Sync + 'static>>();
            Some(cause.as_ref())
        }
    }

    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl serde::ser::Error for AppError {
    fn custom<T: Display>(msg: T) -> Self {
        ERR_SERIALIZE.msg_detail(msg.to_string())
    }
}

impl serde::de::Error for AppError {
    fn custom<T: Display>(msg: T) -> Self {
        ERR_DESERIALIZE.msg_detail(msg.to_string())
    }
}
