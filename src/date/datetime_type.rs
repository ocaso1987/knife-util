use crate::{
    bean::{AsValueTrait, MergeValueTrait},
    error::AppError,
    Result, Value,
};

/// 日期时间工具，同chrono:NativeDateTime
#[derive(Debug, Clone, Copy, Default)]
pub struct DateTime {
    datetime: chrono::NaiveDateTime,
}

impl ToString for DateTime {
    fn to_string(&self) -> String {
        self.datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

impl AsValueTrait for DateTime {
    fn as_value(&self) -> Result<Value> {
        Ok(Value::DateTime(*self))
    }
}

impl MergeValueTrait for DateTime {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            *self = v.as_datetime()?;
        }
        Ok(*self)
    }
}

impl serde::ser::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(
            self.datetime
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .as_str(),
        )
    }
}

struct DateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for DateTimeVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("日期时间格式字符串")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        DateTime::parse_str(value).map_err(E::custom)
    }
}

impl<'de> serde::de::Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(DateTimeVisitor)
    }
}

impl DateTime {
    pub fn from_datetime(v: chrono::NaiveDateTime) -> Result<DateTime> {
        Ok(DateTime { datetime: v })
    }

    pub fn parse_str(str: &str) -> Result<DateTime> {
        chrono::NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| {
                AppError::from(e)
                    .msg_detail("日期时间格式从字符解析时发生异常")
                    .context_value("source".to_string(), Value::String(str.to_string()))
            })
            .map(|x| DateTime { datetime: x })
    }

    pub fn to_chrono_date(&self) -> chrono::NaiveDate {
        self.datetime.date()
    }
}
