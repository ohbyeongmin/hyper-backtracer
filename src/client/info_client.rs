use anyhow::{Context, Result};
use reqwest::Client;

use crate::models::{CandleSnapshotBody, CandleSnapshotResponse};

pub type Response = Result<Vec<CandleSnapshotResponse>>;

#[derive(Clone)]
pub struct InfoClient {
    pub client: Client,
    pub base_url: String,
}

impl InfoClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: url.to_string(),
        }
    }

    pub async fn get_candle_snapshot(&self, req: CandleSnapshotBody) -> Response {
        let resp = self
            .client
            .post(format!("{}/info", self.base_url))
            .json(&req)
            .send()
            .await
            .context(format!("Failed to request: {req:?}"))?;

        let candles: Vec<CandleSnapshotResponse> = resp
            .json()
            .await
            .context(format!("Failed to convert response: reqest: {req:?}"))?;

        Ok(candles)
    }
}
