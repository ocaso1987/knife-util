use std::fmt::Display;

use super::AppError;

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyError")
            .field("name", &self.name)
            .field("code", &self.code)
            .field("msg", &self.msg)
            .field("msg_detail", &self.msg_detail)
            .finish()
            .unwrap();

        if self.context_map.is_some() && !self.context_map.as_ref().unwrap().is_empty() {
            write!(f, "\nContext Value:")?;
            for (k, v) in self.context_map.as_ref().unwrap() {
                write!(f, "\n\t{}: {:?}", k, v)?;
            }
        }

        if !self.cause.is_empty() {
            let cause = self
                .cause
                .as_ref::<Box<dyn std::error::Error + Send + Sync + 'static>>();
            write!(f, "\nCaused by: {:?}", cause)?;
        }

        std::result::Result::Ok(())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyError")
            .field("name", &self.name)
            .field("code", &self.code)
            .field("msg", &self.msg)
            .field("msg_detail", &self.msg_detail)
            .finish()
    }
}
