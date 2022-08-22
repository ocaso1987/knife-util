//! 字符串处理工具类
use globset::Glob;
use regex::Regex;

use crate::{error::ERR_CONVERT, Result};

/// 字符串工具类
pub trait StringExt {
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
    fn str_to_bool(&self) -> Result<bool>;

    /// 根据正则匹配并进行字符替换
    fn regex_replace_all(&self, pattern: String, replacement: String) -> String;

    /// 检查正则是否匹配
    fn regex_match(&self, pattern: String) -> bool;
    
    /// 检查glob规则是否匹配
    fn glob_match(&self, pattern: String) -> bool;
}

impl StringExt for String{
    fn str_to_bool(&self) -> Result<bool> {
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

    fn regex_replace_all(&self, pattern: String, replacement: String) -> String {
        let regex = Regex::new(pattern.as_str()).expect("错误的正则表达式");
        regex.replace_all(self.as_str(), replacement).to_string()
    }

    fn regex_match(&self, pattern: String) -> bool {
        Regex::new(pattern.as_str()).unwrap().is_match(self)
    }

    fn glob_match(&self, pattern: String) -> bool {
        Glob::new(pattern.as_str()).unwrap().compile_matcher().is_match(self)
    }
}
    
impl StringExt for &str{
    fn str_to_bool(&self) -> Result<bool> {
        self.to_string().str_to_bool()
    }

    fn regex_replace_all(&self, pattern: String, replacement: String) -> String {
        self.to_string().regex_replace_all(pattern,replacement)
    }

    fn regex_match(&self, pattern: String) -> bool {
        self.to_string().regex_match(pattern)
    }

    fn glob_match(&self, pattern: String) -> bool {
        self.to_string().glob_match(pattern)
    }
}
