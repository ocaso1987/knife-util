//! 字符串处理工具类
use regex::Regex;

use crate::error::{AppError, ERR_CONVERT};

/// 字符串工具类
pub trait StringUtil {
    /// 字符串转bool类型
    ///
    /// ```
    /// #[test]
    /// fn test_str_to_bool() {
    ///     assert_eq!("true".str_to_bool().unwrap(), true);
    ///     assert_eq!("FALSE".str_to_bool().unwrap(), false);
    ///     assert_eq!(str_to_bool("on".to_string()).unwrap(), true);
    ///     assert_eq!("yes".str_to_bool().unwrap(), true);
    ///     assert_eq!("no".to_string().str_to_bool().unwrap(), false);
    ///     "no2".str_to_bool().unwrap_err().should_panic("ERR_CONVERT");
    /// }
    /// ```
    fn str_to_bool(&self) -> Result<bool, AppError>;

    /// 根据正则匹配并进行字符替换
    fn replace_pattern_all(&self, pattern: String, replacement: String) -> String;

    /// 检查正则是否匹配
    fn match_pattern(&self, pattern: String) -> bool;
}

impl StringUtil for String {
    fn str_to_bool(&self) -> Result<bool, AppError> {
        return match self.to_lowercase().as_str() {
            "true" => Ok(true),
            "t" => Ok(true),
            "false" => Ok(false),
            "f" => Ok(false),
            "yes" => Ok(true),
            "y" => Ok(true),
            "no" => Ok(false),
            "n" => Ok(false),
            "on" => Ok(true),
            "off" => Ok(false),
            "1" => Ok(true),
            "0" => Ok(false),
            _ => Err(ERR_CONVERT.msg_detail(format!("字符串[{}]不能转换为bool类型", self))),
        };
    }

    fn replace_pattern_all(&self, pattern: String, replacement: String) -> String {
        let regex = Regex::new(pattern.as_str()).expect("错误的正则表达式");
        regex.replace_all(self.as_str(), replacement).to_string()
    }

    fn match_pattern(&self, pattern: String) -> bool {
        Regex::new(pattern.as_str()).unwrap().is_match(self)
    }
}

impl StringUtil for &str {
    fn str_to_bool(&self) -> Result<bool, AppError> {
        self.to_string().str_to_bool()
    }

    fn replace_pattern_all(&self, pattern: String, replacement: String) -> String {
        self.to_string().replace_pattern_all(pattern, replacement)
    }

    fn match_pattern(&self, pattern: String) -> bool {
        Regex::new(pattern.as_str()).unwrap().is_match(self)
    }
}
