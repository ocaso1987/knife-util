use indexmap::IndexMap;

use crate::{number::cast_u64_to_i64, Result, VecExt, ERR_MERGE};

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
    fn merge<'a>(&'a mut self, target: &'a Self) -> Result<Self::Context>;
}

impl MergeExt for bson::Bson {
    type Context = bson::Bson;
    fn merge<'a>(&'a mut self, target: &'a Self) -> Result<Self::Context> {
        match self {
            bson::Bson::Array(a1) => {
                match target {
                    bson::Bson::Array(a2) => {
                        a1.extend_from_slice(a2);
                    }
                    v => {
                        a1.push(v.clone());
                    }
                };
                Ok(self.clone())
            }
            bson::Bson::Document(d1) => {
                match target {
                    bson::Bson::Document(d2) => {
                        for (k2, v2) in d2 {
                            if d1.contains_key(k2) {
                                let v2 = d1.get_mut(k2).unwrap().merge(v2).unwrap().clone();
                                d1.insert(k2, v2);
                            } else {
                                d1.insert(k2, v2);
                            }
                        }
                        Ok(self.clone())
                    }
                    _ => Err(ERR_MERGE
                        .msg_detail("Document数据不接受来自于其它类型的MERGE操作".to_string())),
                }
            }
            _ => Ok(target.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MergeExt, PointerExt};
    use bson::bson;

    #[test]
    fn test() {
        let mut a = bson!({"a":[1,2],"b":3,"e":{"f":"456","h":23}});
        let b = bson!({"a":[3],"d":4,"e":{"f":"123","g":4}});
        let res = a.merge(&b).unwrap();
        assert_eq!(res.p("/e/f").unwrap().as_str().unwrap(), "123");
    }
}

pub trait BsonConvertExt {
    fn as_bson(&self) -> bson::Bson;
}

impl BsonConvertExt for serde_json::Value {
    fn as_bson(&self) -> bson::Bson {
        match self {
            handlebars::JsonValue::Null => bson::Bson::Null,
            handlebars::JsonValue::Bool(v) => bson::Bson::Boolean(v.clone()),
            handlebars::JsonValue::Number(v) => {
                if v.is_f64() {
                    return bson::Bson::Double(v.as_f64().unwrap());
                } else if v.is_i64() {
                    return bson::Bson::Int64(v.as_i64().unwrap());
                } else if v.is_u64() {
                    return bson::Bson::Int64(cast_u64_to_i64(v.as_u64().unwrap()).unwrap());
                } else {
                    panic!("无法到达的代码");
                }
            }
            handlebars::JsonValue::String(v) => bson::Bson::String(v.clone()),
            handlebars::JsonValue::Array(v) => bson::Bson::Array(v.map(|x| x.as_bson())),
            handlebars::JsonValue::Object(o) => {
                let mut map = IndexMap::new();
                for (k, v) in o {
                    map.insert(k.to_string(), v.as_bson());
                }
                bson::Bson::Document(bson::Document::from_iter(map.into_iter()))
            }
        }
    }
}

impl BsonConvertExt for serde_yaml::Value {
    fn as_bson(&self) -> bson::Bson {
        match self {
            serde_yaml::Value::Null => bson::Bson::Null,
            serde_yaml::Value::Bool(v) => bson::Bson::Boolean(v.clone()),
            serde_yaml::Value::Number(v) => {
                if v.is_f64() {
                    return bson::Bson::Double(v.as_f64().unwrap());
                } else if v.is_i64() {
                    return bson::Bson::Int64(v.as_i64().unwrap());
                } else if v.is_u64() {
                    return bson::Bson::Int64(cast_u64_to_i64(v.as_u64().unwrap()).unwrap());
                } else {
                    panic!("无法到达的代码");
                }
            }
            serde_yaml::Value::String(v) => bson::Bson::String(v.clone()),
            serde_yaml::Value::Sequence(v) => bson::Bson::Array(v.map(|x| x.as_bson())),
            serde_yaml::Value::Mapping(o) => {
                let mut map = IndexMap::new();
                for (k, v) in o {
                    map.insert(format!("{:?}", k), v.as_bson());
                }
                bson::Bson::Document(bson::Document::from_iter(map.into_iter()))
            }
            serde_yaml::Value::Tagged(_) => panic!("暂不支持Yaml使用Tag类型"),
        }
    }
}

impl BsonConvertExt for toml::Value {
    fn as_bson(&self) -> bson::Bson {
        match self {
            toml::Value::String(v) => bson::Bson::String(v.clone()),
            toml::Value::Integer(v) => bson::Bson::Int64(v.clone()),
            toml::Value::Float(v) => bson::Bson::Double(v.clone()),
            toml::Value::Boolean(v) => bson::Bson::Boolean(v.clone()),
            toml::Value::Datetime(_v) => {
                panic!("暂不支持Toml使用时间类型");
            }
            toml::Value::Array(v) => bson::Bson::Array(v.map(|x| x.as_bson())),
            toml::Value::Table(o) => {
                let mut map = IndexMap::new();
                for (k, v) in o {
                    map.insert(k.to_string(), v.as_bson());
                }
                bson::Bson::Document(bson::Document::from_iter(map.into_iter()))
            }
        }
    }
}
