use crate::{bean::MergeTrait, error::ERR_MERGE, Result, Value};

impl MergeTrait for Value {
    fn merge_self(&mut self, target: &Self) -> Result<Self> {
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
                            let v2 = obj.get_mut(k2).unwrap().merge_self(v2)?.clone();
                            obj.insert(k2.to_string(), v2);
                        } else {
                            obj.insert(k2.to_string(), v2.clone());
                        }
                    }
                    Ok(self.clone())
                }
                _ => Err(ERR_MERGE.msg_detail("Value之Object数据不接受来自于其它类型的MERGE操作")),
            },
            _ => Ok(target.clone()),
        }
    }
}
