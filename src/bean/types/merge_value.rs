use crate::{bean::MergeValueTrait, Ok, Result, Value};

impl<T> MergeValueTrait for Option<T>
where
    T: Default + Clone,
    T: MergeValueTrait,
{
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
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

impl<T> MergeValueTrait for Vec<T>
where
    T: Default + Clone,
    T: MergeValueTrait,
{
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(arr) = target {
            for v in arr.as_array()? {
                let mut defau_value: T = Default::default();
                self.push(defau_value.merge_value(Some(v))?.clone())
            }
        }
        Ok(self.clone())
    }
}

impl MergeValueTrait for bool {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_bool()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueTrait for i32 {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_i32()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueTrait for i64 {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_i64()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueTrait for u64 {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_u64()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueTrait for f64 {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_null()? {
                *self = v.as_f64()?;
            }
        }
        Ok(*self)
    }
}

impl MergeValueTrait for String {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            if !v.is_empty()? {
                *self = v.as_str()?.to_string();
            }
        }
        Ok(self.clone())
    }
}
