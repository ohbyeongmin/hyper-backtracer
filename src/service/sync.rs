use anyhow::Result;

use crate::worker::worker_manager::{Config, WorkerManager};

pub async fn sync_candles(coin_symbol: &str, intervals: &str) -> Result<()> {
    let config = Config::new(coin_symbol.to_string(), intervals.try_into()?);
    WorkerManager::start(config).await
}
