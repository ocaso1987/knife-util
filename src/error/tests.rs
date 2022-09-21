#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::error::{ERR_CAST, ERR_INTERNAL};

    #[test]
    fn test() {
        let err = File::open("not_exist.txt").err().unwrap();
        let app_err = ERR_CAST.msg_detail("other").cause(err);
        let app_err_str = format!("{:?}", app_err);
        assert!(app_err_str.contains("other"));
        assert!(ERR_CAST.msg_detail_ref().is_none());
        assert!(app_err.msg_detail_ref().is_some());

        let app_err2 = ERR_INTERNAL.msg_detail("unknown").cause(app_err);
        let app_err_str2 = format!("{:?}", app_err2);
        assert!(app_err_str2.contains("unknown"));
    }
}
