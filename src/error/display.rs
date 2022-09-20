use std::fmt::Display;

use backtrace::Backtrace;

use super::{backtrace::enable_backtrace, AppError};

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppError")
            .field("name", &self.name_ref())
            .field("code", &self.code_ref())
            .field("msg", &self.msg_ref())
            .field("msg_detail", &self.msg_detail_ref())
            .finish()
            .unwrap();

        if self.context_map_ref().is_some() {
            write!(f, "\nContext Value:")?;
            for (k, v) in self.context_map_ref().unwrap() {
                write!(f, "\n\t{}: {:?}", k, v)?;
            }
        }

        if self.cause_ref().is_some() {
            let cause = self.cause_ref().unwrap();
            write!(f, "\nCaused by: {:?}", cause)?;
        }

        if enable_backtrace() {
            if self.backtrace_ref().is_some() {
                let backtrace = self.backtrace_ref().unwrap();
                write!(f, "\nBacktrace:\n{:?}", backtrace)?;
            } else {
                write!(f, "\nBacktrace:\n{:?}", Backtrace::new())?;
            }
        }

        std::result::Result::Ok(())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppError")
            .field("name", &self.name_ref())
            .field("code", &self.code_ref())
            .field("msg", &self.msg_ref())
            .field("msg_detail", &self.msg_detail_ref())
            .finish()
    }
}
