use crate::constants;
use regex::Regex;
use thiserror::Error;
use tokio::time::{sleep, Duration};

pub struct ClientManager {
    pub name: String,
}

impl ClientManager {
    pub async fn get(&self, input: &str) -> String {
        println!("{} getting info...: {input}", self.name);
        sleep(Duration::from_secs(3)).await;
        format!("{input} is done try db")
    }
}

#[derive(Error, Debug)]
pub enum ClientManagerError {
    #[error("invalid interval: {0}")]
    InvalidInterval(String),
}

#[derive(Debug)]
pub enum InputCandleIntervals {
    Default,
    Custom(String),
}

#[derive(Debug, PartialEq)]
struct CandleIntervals {
    intervals: String,
}

impl IntoIterator for CandleIntervals {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.intervals
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter()
    }
}

impl Default for CandleIntervals {
    fn default() -> Self {
        Self::new(constants::DEFAULT_CANDLE_INTERVALS).unwrap()
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

    fn new(input: &str) -> Result<Self, &str> {
        let input = input.trim();

        if !Self::validate_format(input) {
            return Err("intervals format");
        }

        if !Self::validate_interval_value(input) {
            return Err("intervals value");
        }

        Ok(Self {
            intervals: String::from(input),
        })
    }
}

//pub struct ClientManager {
//    client: InfoClient,
//    intervals: CandleIntervals,
//    coin_symbol: String,
//}
//
//impl ClientManager {
//    pub fn new(
//        url: &str,
//        symbol: &str,
//        intervals: InputCandleIntervals,
//    ) -> Result<Self, ClientManagerError> {
//        let intervals = match intervals {
//            InputCandleIntervals::Default => CandleIntervals::default(),
//            InputCandleIntervals::Custom(intervals) => CandleIntervals::new(&intervals)
//                .map_err(|msg| ClientManagerError::InvalidInterval(msg.to_string()))?,
//        };
//
//        let client = InfoClient::new(url);
//
//        Ok(Self {
//            client,
//            intervals,
//            coin_symbol: symbol.to_string(),
//        })
//    }
//    //async fn get_all_candles() {}
//    //async fn get_candles_from_start_time() {}
//}

#[cfg(test)]
mod tests {
    use crate::client::manager::CandleIntervals;

    #[test]
    fn candleintervals_default() {
        let default_candle = CandleIntervals::default();
        assert_eq!(
            default_candle,
            CandleIntervals {
                intervals: "1m,5m,15m,30m,1h,4h,1d,1w,1M".to_string()
            }
        );

        assert!(CandleIntervals::validate_format(&default_candle.intervals))
    }

    #[test]
    fn candleintervals_new() {
        let valid_input = [
            "1m,3m",
            "15m,4h,1d",
            "5m,1h,3d,1w",
            "3m,4h,1d,1w,1M",
            "1h",
            "1d",
            "1w",
            "1M",
            "1m",
            " 1m,4h",
            "  1m,4h,1w  ",
            "1m,3m ",
        ];

        let invalid_input_format = ["1m, 4h", "1s,4h", "m1m,5m", "1hh,1m"];

        let invalid_input_value = [
            "1m,3h",
            "1M,2m,8h",
            "1m,3h",
            "2m,1m,4h",
            "1m,3m,4w",
            "1m,3m,11d",
            "4w,2M",
        ];

        for input in valid_input {
            let interval = CandleIntervals::new(input);
            assert_eq!(
                interval,
                Ok(CandleIntervals {
                    intervals: input.trim().to_string()
                })
            )
        }

        for input in invalid_input_format {
            let interval = CandleIntervals::new(input);
            assert_eq!(interval, Err("intervals format"));
        }

        for input in invalid_input_value {
            let interval = CandleIntervals::new(input);
            assert_eq!(interval, Err("intervals value"));
        }
    }
}
