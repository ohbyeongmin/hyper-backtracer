use anyhow::Result;

use crate::worker::worker_manager::Config;

pub async fn sync_candles(coin_symbol: &str, intervals: &str) -> Result<()> {
    let config = Config::new(coin_symbol.to_string(), intervals.try_into()?);
    todo!("Create worker manager and start sync and get data");
    Ok(())
}
