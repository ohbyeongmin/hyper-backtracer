use anyhow::Result;
use serde::{Deserialize, Serialize};

pub type TimeRawData = i64;

pub type ClientResponse = Result<Vec<CandleSnapshotResponse>>;

pub const CANDLE_SNAPSHOT_BODY_TYPE: &str = "";

#[derive(Deserialize, Serialize, Debug)]
pub struct CandleSnapshotBody {
    #[serde(rename = "type")]
    pub body_type: String,
    #[serde(rename = "req")]
    pub payload: CandleSnapshotPayload,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CandleSnapshotPayload {
    pub coin: String,

    // 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 8h, 12h, 1d, 3d, 1w, 1M
    pub interval: String,

    // epoch milliseconds
    pub start_time: TimeRawData,
    pub end_time: TimeRawData,
}

#[derive(Deserialize, Debug)]
pub struct CandleSnapshotResponse {
    #[serde(rename = "T")]
    pub close_time: TimeRawData,

    #[serde(rename = "c")]
    pub close_price: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "l")]
    pub low_price: String,

    #[serde(rename = "n")]
    pub number_of_trades: u64,

    #[serde(rename = "o")]
    pub open_price: String,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "t")]
    pub open_time: TimeRawData,

    #[serde(rename = "v")]
    pub volume: String,
}
