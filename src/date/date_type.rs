use crate::{
    bean::{AsValueTrait, MergeValueTrait},
    error::AppError,
    Ok, Result, Value,
};

/// 日期工具，同chrono:NativeDate
#[derive(Debug, Clone, Copy, Default)]
pub struct Date {
    pub(super) date: chrono::NaiveDate,
}

impl ToString for Date {
    fn to_string(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }
}

impl AsValueTrait for Date {
    fn as_value(&self) -> Result<Value> {
        Ok(Value::Date(*self))
    }
}

impl MergeValueTrait for Date {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            *self = v.as_date()?;
        }
        Ok(*self)
    }
}

impl serde::ser::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.date.format("%Y-%m-%d").to_string().as_str())
    }
}

struct DateVisitor;

impl<'de> serde::de::Visitor<'de> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("日期格式字符串")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Date::parse_str(value).map_err(E::custom)
    }
}

impl<'de> serde::de::Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(DateVisitor)
    }
}

impl Date {
    pub fn from_date(v: chrono::NaiveDate) -> Result<Date> {
        Ok(Date { date: v })
    }

    pub fn parse_str(str: &str) -> Result<Date> {
        chrono::NaiveDate::parse_from_str(str, "%Y-%m-%d")
            .map_err(|e| {
                AppError::from(e)
                    .msg_detail("日期格式从字符解析时发生异常")
                    .context_value("source".to_string(), Value::String(str.to_string()))
            })
            .map(|x| Date { date: x })
    }

    pub fn to_chrono_date(&self) -> chrono::NaiveDate {
        self.date
    }
}
