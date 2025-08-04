use crate::{
    client::InfoClient,
    common::{
        constants,
        dto::client::{
            CANDLE_SNAPSHOT_BODY_TYPE, CandleSnapshotBody, CandleSnapshotPayload,
            CandleSnapshotResponse,
        },
        helper::{AppDateTime, CandleIntervals},
    },
    db::core::{get_last_close_time, init_postgres_pool, insert_candle_batches},
};
use anyhow::Result;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Config {
    coin_symbol: String,
    intervals: CandleIntervals,
    db_url: String,
}

impl Config {
    pub fn new(coin_symbol: String, intervals: CandleIntervals, db_url: String) -> Self {
        Self {
            coin_symbol,
            intervals,
            db_url,
        }
    }
}

pub struct WorkerManager;

impl WorkerManager {
    pub async fn start(config: Config) -> Result<()> {
        let info_client = InfoClient::new(constants::API_URL);
        let db_pool = init_postgres_pool(&config.db_url).await?;

        let intervals = config.intervals.clone();
        let (candle_tx, mut candle_rx) = mpsc::channel::<Vec<CandleSnapshotResponse>>(20);

        let pool_clone = db_pool.clone();

        let handle_recv_candle = tokio::spawn(async move {
            while let Some(mut candle) = candle_rx.recv().await {
                candle.pop();
                if !candle.is_empty() {
                    insert_candle_batches(&pool_clone, candle.try_into().unwrap())
                        .await
                        .unwrap();
                }
            }
        });

        for interval in intervals {
            let coin_symbol = config.coin_symbol.clone();
            let candle_tx = candle_tx.clone();
            let clone_info_client = info_client.clone();
            let start_time = match get_last_close_time(&db_pool, &coin_symbol, &interval).await? {
                Some(last_close_time) => last_close_time + 1,
                None => 0,
            };

            let _handle = tokio::spawn(async move {
                let payload = CandleSnapshotPayload {
                    coin: coin_symbol,
                    interval,
                    start_time,
                    end_time: AppDateTime::now().to_milliseconds(),
                };

                let body = CandleSnapshotBody {
                    body_type: CANDLE_SNAPSHOT_BODY_TYPE.to_string(),
                    payload,
                };
                match clone_info_client.get_candle_snapshot(body).await {
                    Ok(res) => candle_tx.send(res).await.unwrap(),
                    Err(e) => eprintln!("{e:?}"),
                }
            });
        }

        drop(candle_tx);
        handle_recv_candle.await?;

        Ok(())
    }
}
