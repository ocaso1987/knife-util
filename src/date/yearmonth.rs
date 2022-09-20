use chrono::Datelike;

use crate::{error::AppError, Ok, Result, Value};

use super::main::{is_leap_year, last_day_of_month};

#[derive(Debug, Clone)]
pub struct YearMonth {
    date: chrono::NaiveDate,
}

impl ToString for YearMonth {
    fn to_string(&self) -> String {
        self.date.format("%Y-%m").to_string()
    }
}

impl YearMonth {
    pub fn is_leap_year(&self) -> bool {
        is_leap_year(self.date.year())
    }

    pub fn last_day(&self) -> chrono::NaiveDate {
        let day = last_day_of_month(self.date.year(), self.date.month());
        self.date.with_day(day).unwrap()
    }

    pub fn from_date(date: &chrono::NaiveDate) -> Result<YearMonth> {
        Ok(YearMonth {
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
}
