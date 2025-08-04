use anyhow::Result;

use crate::worker::worker_manager::{Config, WorkerManager};

pub async fn sync_candles(coin_symbol: &str, intervals: &str, db_url: &str) -> Result<()> {
    let config = Config::new(
        coin_symbol.to_string(),
        intervals.try_into()?,
        db_url.to_string(),
    );
    WorkerManager::start(config).await
}
