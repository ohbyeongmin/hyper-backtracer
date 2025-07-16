use crate::constants;
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use regex::Regex;

#[derive(Debug)]
pub struct AppDateTime {
    datetime: DateTime<Local>,
}

impl AppDateTime {
    pub fn new(t: &str) -> Result<Self> {
        let datetime = NaiveDateTime::parse_from_str(t, "%Y-%m-%d %H:%M")
            .context(format!("invalid datetime format: {t}"))?;

        let local_datetime = Local
            .from_local_datetime(&datetime)
            .earliest()
            .context(format!("failed to convert local time: {t}"))?;

        Ok(Self {
            datetime: local_datetime,
        })
    }

    pub fn now() -> Self {
        Self {
            datetime: Local::now(),
        }
    }

    pub fn to_milliseconds(&self) -> i64 {
        self.datetime.timestamp_millis()
    }
}

#[derive(Debug)]
pub enum InputCandleIntervals {
    Default,
    Custom(String),
}

#[derive(Debug, PartialEq)]
pub struct CandleIntervals {
    intervals: Vec<String>,
}

impl Default for CandleIntervals {
    fn default() -> Self {
        Self::new(constants::DEFAULT_CANDLE_INTERVALS).unwrap()
    }
}

impl IntoIterator for CandleIntervals {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.intervals.into_iter()
    }
}

impl CandleIntervals {
    fn validate_format(input: &str) -> bool {
        let rege = Regex::new(r"^(\d+[mhdwM])(,\d+[mhdwM])*$").expect("faild regex compile");
        rege.is_match(input)
    }

    fn validate_interval_value(input: &str) -> bool {
        input.split(",").all(|interval| {
            matches!(
                interval,
                "1m" | "3m"
                    | "5m"
                    | "15m"
                    | "30m"
                    | "1h"
                    | "2h"
                    | "4h"
                    | "8h"
                    | "12h"
                    | "1d"
                    | "3d"
                    | "1w"
                    | "1M"
            )
        })
    }

    pub fn new(input: &str) -> Result<Self> {
        let input = input.trim();

        if !Self::validate_format(input) {
            return Err(anyhow!("intervals format: {input}"));
        }

        if !Self::validate_interval_value(input) {
            return Err(anyhow!("intervals value: {input}"));
        }

        let intervals = input.split(",").map(|s| s.to_string()).collect();

        Ok(Self { intervals })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datetime_create_vaild() {
        let inputs = vec!["2025-07-10 13:53", "1980-06-11 09:33", "2025-7-1 4:2"];
        for input in inputs {
            assert!(AppDateTime::new(input).is_ok());
        }
    }

    #[test]
    fn datetime_create_invalid() {
        let invalid_year = "299201929-02-10 13:21";
        let invalid_month = "2025-15-10 13:21";
        let invalid_day = "2025-12-33 12:22";
        let invalid_hour = "2021-04-29 29:31";
        let invalid_minutes = "2024-1-16 22:89";
        let invalid_contain_seconds = "2025-6-1 21:32:30";

        if let Err(msg) = AppDateTime::new(invalid_year) {
            assert_eq!(
                msg.to_string(),
                format!("invalid datetime format: {invalid_year}")
            );
        } else {
            panic!()
        }

        if let Err(msg) = AppDateTime::new(invalid_month) {
            assert_eq!(
                msg.to_string(),
                format!("invalid datetime format: {invalid_month}")
            );
        } else {
            panic!()
        }

        if let Err(msg) = AppDateTime::new(invalid_day) {
            assert_eq!(
                msg.to_string(),
                format!("invalid datetime format: {invalid_day}")
            );
        } else {
            panic!()
        }

        if let Err(msg) = AppDateTime::new(invalid_hour) {
            assert_eq!(
                msg.to_string(),
                format!("invalid datetime format: {invalid_hour}")
            );
        } else {
            panic!()
        }

        if let Err(msg) = AppDateTime::new(invalid_minutes) {
            assert_eq!(
                msg.to_string(),
                format!("invalid datetime format: {invalid_minutes}")
            );
        } else {
            panic!()
        }

        if let Err(msg) = AppDateTime::new(invalid_contain_seconds) {
            assert_eq!(
                msg.to_string(),
                format!("invalid datetime format: {invalid_contain_seconds}")
            );
        } else {
            panic!()
        }
    }

    #[test]
    fn datetime_convert_milliseconds() {
        let inputs = ["2025-07-10 13:53", "1980-06-11 09:33", "2025-7-1 4:2"];
        for input in inputs {
            assert_eq!(
                AppDateTime::new(input).unwrap().to_milliseconds(),
                AppDateTime::new(input)
                    .unwrap()
                    .datetime
                    .naive_utc()
                    .and_utc()
                    .timestamp_millis(),
            );
        }
    }
}
