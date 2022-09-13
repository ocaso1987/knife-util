use globset::Glob;
use regex::Regex;

use crate::{error::ERR_CONVERT, Result};

/// 用于字符串处理的工具类
pub trait StringExt {
    /// 字符串转bool类型
    fn str_to_bool(&self) -> Result<bool>;

    /// 当内容为""时设置默认值
    fn if_blank(&self, default_value: String) -> String;

    /// 显示紧凑格式
    fn compact(&self) -> String;

    /// 根据正则匹配并进行字符替换
    fn regex_replace_all(&self, pattern: String, replacement: String) -> String;

    /// 检查正则是否匹配
    fn regex_match(&self, pattern: String) -> bool;

    /// 检查glob规则是否匹配
    fn glob_match(&self, pattern: String) -> bool;

    /// 检查是否包含指定字符串，且忽略其大小写
    fn contains_ignore_case(&self, pat: String) -> bool;
}

impl<T> StringExt for T
where
    T: Into<String> + Clone,
{
    fn str_to_bool(&self) -> Result<bool> {
        let str: String = self.clone().into();
        return match str.to_lowercase().as_str() {
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
            _ => Err(ERR_CONVERT.msg_detail(format!("字符串[{}]不能转换为bool类型", str))),
        };
    }

    fn if_blank(&self, default_value: String) -> String {
        let str: String = self.clone().into();
        if !str.is_empty() {
            str.clone()
        } else {
            default_value
        }
    }

    fn compact(&self) -> String {
        self.regex_replace_all("[ \t\r\n]+".to_string(), " ".to_string())
    }

    fn regex_replace_all(&self, pattern: String, replacement: String) -> String {
        let str: String = self.clone().into();
        let regex = Regex::new(pattern.as_str()).expect("错误的正则表达式");
        regex.replace_all(str.as_str(), replacement).to_string()
    }

    fn regex_match(&self, pattern: String) -> bool {
        let str: String = self.clone().into();
        Regex::new(pattern.as_str()).unwrap().is_match(str.as_str())
    }

    fn glob_match(&self, pattern: String) -> bool {
        let str: String = self.clone().into();
        Glob::new(pattern.as_str())
            .unwrap()
            .compile_matcher()
            .is_match(str)
    }

    fn contains_ignore_case(&self, pat: String) -> bool {
        let str: String = self.clone().into();
        str.to_lowercase().contains(&pat.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::StringExt;

    #[test]
    fn test() {
        assert_eq!("true".str_to_bool().unwrap(), true);
        assert_eq!("FALSE".str_to_bool().unwrap(), false);
        assert_eq!("on".to_string().str_to_bool().unwrap(), true);
        assert_eq!("yes".str_to_bool().unwrap(), true);
        assert_eq!("no".to_string().str_to_bool().unwrap(), false);
    }
}
