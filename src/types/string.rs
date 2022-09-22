use globset::Glob;
use regex::Regex;

use crate::{error::ERR_CONVERT, Result, OK};

/// 用于字符串处理的工具类
pub trait StringExt {
    /// 字符串转bool类型
    fn str_to_bool(&self) -> Result<bool>;

    /// 当内容为""时设置默认值
    fn if_blank(&self, default_value: String) -> String;

    /// 显示紧凑格式
    fn compact(&self) -> String;

    /// 去除首尾空格
    fn trim_both(&self) -> String;

    /// 根据正则匹配并进行字符替换
    fn regex_replace_all(&self, pattern: String, replacement: String) -> String;

    /// 检查正则是否匹配
    fn regex_match(&self, pattern: String) -> bool;

    /// 检查glob规则是否匹配
    fn glob_match(&self, pattern: String) -> bool;

    /// 检查是否包含指定字符串，且忽略其大小写
    fn contains_ignore_case(&self, pat: String) -> bool;
}

impl StringExt for &str {
    fn str_to_bool(&self) -> Result<bool> {
        return match self.to_lowercase().as_str() {
            "true" => OK(true),
            "t" => OK(true),
            "false" => OK(false),
            "f" => OK(false),
            "yes" => OK(true),
            "y" => OK(true),
            "no" => OK(false),
            "n" => OK(false),
            "on" => OK(true),
            "off" => OK(false),
            "1" => OK(true),
            "0" => OK(false),
            _ => {
                Err(ERR_CONVERT.msg_detail(format!("字符串[{}]不能转换为bool类型", self).as_str()))
            }
        };
    }

    fn if_blank(&self, default_value: String) -> String {
        if !self.is_empty() {
            self.to_string()
        } else {
            default_value
        }
    }

    fn compact(&self) -> String {
        self.regex_replace_all("[ \t\r\n]+".to_string(), " ".to_string())
            .trim_both()
    }

    fn trim_both(&self) -> String {
        self.trim_start().trim_end().to_string()
    }

    fn regex_replace_all(&self, pattern: String, replacement: String) -> String {
        let regex = Regex::new(pattern.as_str()).expect("错误的正则表达式");
        regex.replace_all(self, replacement).to_string()
    }

    fn regex_match(&self, pattern: String) -> bool {
        Regex::new(pattern.as_str()).unwrap().is_match(self)
    }

    fn glob_match(&self, pattern: String) -> bool {
        Glob::new(pattern.as_str())
            .unwrap()
            .compile_matcher()
            .is_match(self)
    }

    fn contains_ignore_case(&self, pat: String) -> bool {
        self.to_lowercase().contains(&pat.to_lowercase())
    }
}

impl StringExt for String {
    fn str_to_bool(&self) -> Result<bool> {
        self.as_str().str_to_bool()
    }

    fn if_blank(&self, default_value: String) -> String {
        self.as_str().if_blank(default_value)
    }

    fn compact(&self) -> String {
        self.as_str().compact()
    }

    fn trim_both(&self) -> String {
        self.as_str().trim_both()
    }

    fn regex_replace_all(&self, pattern: String, replacement: String) -> String {
        self.as_str().regex_replace_all(pattern, replacement)
    }

    fn regex_match(&self, pattern: String) -> bool {
        self.as_str().regex_match(pattern)
    }

    fn glob_match(&self, pattern: String) -> bool {
        self.as_str().glob_match(pattern)
    }

    fn contains_ignore_case(&self, pat: String) -> bool {
        self.as_str().contains_ignore_case(pat)
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
