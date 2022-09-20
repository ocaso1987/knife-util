use std::fmt::{Debug, Display};

use crate::{
    bean::{traits::MergeValueExt, Value},
    error::ERR_CAST,
    Ok, Result,
};

impl<T> MergeValueExt for Option<T>
where
    T: Default + Clone + Sized,
    T: MergeValueExt,
    T: Display + Debug,
{
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        match target {
            Some(v) => {
                if v.is_null()? {
                    Ok(self.clone())
                } else {
                    let new_value = if let Some(o) = self {
                        o.merge_value(target)?
                    } else {
                        let mut defau_value: T = Default::default();
                        defau_value.merge_value(Some(v))?
                    };
                    self.replace(new_value);
                    Ok(self.clone())
                }
            }
            None => Ok(self.clone()),
        }
    }
}

impl<T> MergeValueExt for Vec<T>
where
    T: Default + Clone + Sized,
    T: MergeValueExt,
    T: Display + Debug,
{
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        if let Some(arr) = target {
            for v in arr.as_array()? {
                let mut defau_value: T = Default::default();
                self.push(defau_value.merge_value(Some(v))?.clone())
            }
        }
        Ok(self.clone())
    }
}

impl MergeValueExt for bool {
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_bool()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueExt for i32 {
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_i32()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueExt for i64 {
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_i64()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueExt for u64 {
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_u64()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueExt for f64 {
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_f64()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueExt for String {
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_empty()? {
                *self = v.as_str()?.to_string();
            }
        }
        Ok(self.clone())
    }
}

impl MergeValueExt for chrono::NaiveDate {
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_empty()? {
                let str = v.as_str()?;
                match chrono::NaiveDate::parse_from_str(str, "%Y-%m-%d") {
                    std::result::Result::Ok(v) => {
                        *self = v;
                    }
                    Err(e) => {
                        return Err(ERR_CAST
                            .msg_detail("日期类型转换失败")
                            .context_value("value".to_string(), Value::String(str.to_string()))
                            .cause(e));
                    }
                }
            }
        }
        Ok(*self)
    }
}

impl MergeValueExt for chrono::NaiveDateTime {
    fn merge_value(&mut self, target: Option<&crate::bean::Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_empty()? {
                let str = v.as_str()?;
                match chrono::NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S") {
                    std::result::Result::Ok(v) => {
                        *self = v;
                    }
                    Err(e) => {
                        return Err(ERR_CAST
                            .msg_detail("日期时间类型转换失败")
                            .context_value("value".to_string(), Value::String(str.to_string()))
                            .cause(e));
                    }
                }
            }
        }
        Ok(*self)
    }
}
