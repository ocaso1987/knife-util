use crate::{Result, Value, ERR_MERGE};

/// 支持两个相同的Object对象进行合并
pub trait ValueMergeExt {
    type Context;
    fn merge<'a>(&'a mut self, target: &'a Self) -> Result<Self::Context>;
}

impl ValueMergeExt for Value {
    type Context = Value;
    fn merge<'a>(&'a mut self, target: &'a Self) -> Result<Self::Context> {
        match self {
            Value::Array(arr) => {
                match target {
                    Value::Array(arr2) => {
                        arr.extend_from_slice(arr2);
                    }
                    v => {
                        arr.push(v.clone());
                    }
                };
                Ok(self.clone())
            }
            Value::Object(obj) => match target {
                Value::Object(obj2) => {
                    for (k2, v2) in obj2 {
                        if obj.contains_key(k2) {
                            let v2 = obj.get_mut(k2).unwrap().merge(v2).unwrap().clone();
                            obj.insert(k2.to_string(), v2);
                        } else {
                            obj.insert(k2.to_string(), v2.clone());
                        }
                    }
                    Ok(self.clone())
                }
                _ => Err(ERR_MERGE
                    .msg_detail("Bson之Document数据不接受来自于其它类型的MERGE操作".to_string())),
            },
            _ => Ok(target.clone()),
        }
    }
}

impl ValueMergeExt for bson::Bson {
    type Context = bson::Bson;
    fn merge<'a>(&'a mut self, target: &'a Self) -> Result<Self::Context> {
        match self {
            bson::Bson::Array(arr) => {
                match target {
                    bson::Bson::Array(arr2) => {
                        arr.extend_from_slice(arr2);
                    }
                    v => {
                        arr.push(v.clone());
                    }
                };
                Ok(self.clone())
            }
            bson::Bson::Document(doc) => match target {
                bson::Bson::Document(doc2) => {
                    for (k2, v2) in doc2 {
                        if doc.contains_key(k2) {
                            let v2 = doc.get_mut(k2).unwrap().merge(v2).unwrap().clone();
                            doc.insert(k2, v2);
                        } else {
                            doc.insert(k2, v2);
                        }
                    }
                    Ok(self.clone())
                }
                _ => Err(ERR_MERGE
                    .msg_detail("Bson之Document数据不接受来自于其它类型的MERGE操作".to_string())),
            },
            _ => Ok(target.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ValueMergeExt, ValuePointerExt};
    use bson::bson;

    #[test]
    fn test() {
        let mut a = bson!({"a":[1,2],"b":3,"e":{"f":"456","h":23}});
        let b = bson!({"a":[3],"d":4,"e":{"f":"123","g":4}});
        let res = a.merge(&b).unwrap();
        assert_eq!(res.p("/e/f").unwrap().as_str().unwrap(), "123");
    }
}
