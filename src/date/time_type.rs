use crate::{
    bean::{AsValueTrait, MergeValueTrait},
    error::AppError,
    Result, Value, OK,
};

/// 时间工具，同chrono:NativeTime
#[derive(Debug, Clone, Copy, Default)]
pub struct Time {
    time: chrono::NaiveTime,
}

impl ToString for Time {
    fn to_string(&self) -> String {
        self.time.format("%H:%M:%S").to_string()
    }
}

impl AsValueTrait for Time {
    fn as_value(&self) -> Result<Value> {
        OK(Value::Time(*self))
    }
}

impl MergeValueTrait for Time {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            *self = v.as_time()?;
        }
        OK(*self)
    }
}

impl serde::ser::Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.time.format("%H:%M:%S").to_string().as_str())
    }
}

struct TimeVisitor;

impl<'de> serde::de::Visitor<'de> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("时间格式字符串")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Time::parse_str(value).map_err(E::custom)
    }
}

impl<'de> serde::de::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeVisitor)
    }
}

impl Time {
    pub fn from_time(v: chrono::NaiveTime) -> Result<Time> {
        OK(Time { time: v })
    }

    pub fn parse_str(str: &str) -> Result<Time> {
        chrono::NaiveTime::parse_from_str(str, "%H:%M:%S")
            .map_err(|e| {
                AppError::from(e)
                    .msg_detail("时间格式从字符解析时发生异常")
                    .context_value("source".to_string(), Value::String(str.to_string()))
            })
            .map(|x| Time { time: x })
    }
}
