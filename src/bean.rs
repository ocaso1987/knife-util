use crate::Result;

/// 可遍历特征
/// 支持以/a/b/c/2的方式获取指定层级上的对象
/// 特殊字行可转义采用~1代替/，采用~0代替~
/// 更多信息可参考：[RFC6901](https://tools.ietf.org/html/rfc6901)
pub trait PointerExt {
    type Context;
    fn p(&self, pointer: &str) -> Option<&Self::Context>;
}

impl PointerExt for serde_json::Value {
    type Context = serde_json::Value;
    fn p(&self, pointer: &str) -> Option<&Self::Context> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                serde_json::Value::Object(map) => map.get(&token),
                serde_json::Value::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }
}

impl PointerExt for serde_yaml::Value {
    type Context = serde_yaml::Value;
    fn p(&self, pointer: &str) -> Option<&Self::Context> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                serde_yaml::Value::Mapping(map) => map.get(&token),
                serde_yaml::Value::Sequence(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }
}

impl PointerExt for toml::Value {
    type Context = toml::Value;
    fn p(&self, pointer: &str) -> Option<&Self::Context> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                toml::Value::Table(map) => map.get(&token),
                toml::Value::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }
}

impl PointerExt for bson::Bson {
    type Context = bson::Bson;
    fn p(&self, pointer: &str) -> Option<&Self::Context> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                bson::Bson::Document(map) => map.get(&token),
                bson::Bson::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }
}

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

/// 支持两个相同的Object对象进行合并
/// 如果是存放在Json/Yaml/Toml中的数据，合并后将变为基本类型
pub trait MergeExt {
    type Context;
    fn merge(&self, target: &Self) -> Result<&Self::Context>;
}
