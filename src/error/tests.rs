#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::error::{ERR_CAST, ERR_INTERNAL};

    #[test]
    fn test() {
        let err = File::open("not_exist.txt").err().unwrap();
        let app_err = ERR_CAST.msg_detail("other".to_string()).cause(err);
        let app_err_str = format!("{:?}", app_err);
        assert!(app_err_str.contains("other"));

        let app_err2 = ERR_INTERNAL
            .msg_detail("unknown".to_string())
            .cause(app_err);
        let app_err_str2 = format!("{:?}", app_err2);
        assert!(app_err_str2.contains("unknown"));
    }
}
