use crate::{Result, ERR_MERGE};

/// 支持两个相同的Object对象进行合并
pub trait ValueMergeExt {
    type Context;
    fn merge<'a>(&'a mut self, target: &'a Self) -> Result<Self::Context>;
}

impl ValueMergeExt for bson::Bson {
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
            bson::Bson::Document(d1) => match target {
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
                    .msg_detail("Bson之Document数据不接受来自于其它类型的MERGE操作".to_string())),
            },
            _ => Ok(target.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{PointerExt, ValueMergeExt};
    use bson::bson;

    #[test]
    fn test() {
        let mut a = bson!({"a":[1,2],"b":3,"e":{"f":"456","h":23}});
        let b = bson!({"a":[3],"d":4,"e":{"f":"123","g":4}});
        let res = a.merge(&b).unwrap();
        assert_eq!(res.p("/e/f").unwrap().as_str().unwrap(), "123");
    }
}
