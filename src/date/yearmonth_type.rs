use chrono::Datelike;

use crate::{
    bean::{AsValueTrait, MergeValueTrait},
    error::AppError,
    Result, Value, OK,
};

use super::{
    main::{is_leap_year, last_day_of_month},
    Date,
};

/// 年月工具
#[derive(Debug, Clone, Copy, Default)]
pub struct YearMonth {
    date: chrono::NaiveDate,
}

impl ToString for YearMonth {
    fn to_string(&self) -> String {
        self.date.format("%Y-%m").to_string()
    }
}

impl AsValueTrait for YearMonth {
    fn as_value(&self) -> Result<Value> {
        OK(Value::YearMonth(*self))
    }
}

impl MergeValueTrait for YearMonth {
    fn merge_value(&mut self, target: Option<&Value>) -> Result<Self> {
        if let Some(v) = target {
            *self = v.as_year_month()?;
        }
        OK(*self)
    }
}

impl serde::ser::Serialize for YearMonth {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.date.format("%Y-%m").to_string().as_str())
    }
}

struct YearMonthVisitor;

impl<'de> serde::de::Visitor<'de> for YearMonthVisitor {
    type Value = YearMonth;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("年月格式字符串")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        YearMonth::parse_str(value).map_err(E::custom)
    }
}

impl<'de> serde::de::Deserialize<'de> for YearMonth {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(YearMonthVisitor)
    }
}

impl YearMonth {
    pub fn from_chrono_date(date: &chrono::NaiveDate) -> Result<YearMonth> {
        OK(YearMonth {
            date: date.with_day(1).unwrap(),
        })
    }

    pub fn parse_str(str: &str) -> Result<YearMonth> {
        let str = format!("{}-01", str);
        chrono::NaiveDate::parse_from_str(str.as_str(), "%Y-%m-%d")
            .map_err(|e| {
                AppError::from(e)
                    .msg_detail("年月格式从字符解析时发生异常")
                    .context_value("source".to_string(), Value::String(str.to_string()))
            })
            .map(|x| YearMonth { date: x })
    }

    pub fn is_leap_year(&self) -> bool {
        is_leap_year(self.date.year())
    }

    pub fn last_day(&self) -> Date {
        let day = last_day_of_month(self.date.year(), self.date.month());
        Date {
            date: self.date.with_day(day).unwrap(),
        }
    }
}
