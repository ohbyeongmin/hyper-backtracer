use crate::common::dto::client::CandleSnapshotResponse;
use anyhow::{Error, Result, anyhow};

#[derive(Debug)]
pub struct CandleDataDTO {
    pub symbol: String,
    pub interval: String,
    pub datas: Vec<CandleSnapshotResponse>,
}

impl TryFrom<Vec<CandleSnapshotResponse>> for CandleDataDTO {
    type Error = Error;

    fn try_from(value: Vec<CandleSnapshotResponse>) -> Result<Self> {
        if value.is_empty() {
            return Err(anyhow!("candle snapshot responses are empty"));
        }

        let symbol = value[0].symbol.clone();
        let interval = value[0].symbol.clone();

        Ok(Self {
            symbol,
            interval,
            datas: value,
        })
    }
}
