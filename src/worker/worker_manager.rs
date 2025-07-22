use crate::{
    client::InfoClient,
    common::{
        constants,
        dto::client::{
            CandleSnapshotBody, CandleSnapshotPayload, CandleSnapshotResponse,
            CANDLE_SNAPSHOT_BODY_TYPE,
        },
        helper::{AppDateTime, CandleIntervals},
    },
};
use anyhow::Result;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Config {
    coin_symbol: String,
    intervals: CandleIntervals,
}

impl Config {
    pub fn new(coin_symbol: String, intervals: CandleIntervals) -> Self {
        Self {
            coin_symbol,
            intervals,
        }
    }
}

pub struct WorkerManager;

impl WorkerManager {
    pub async fn start(config: Config) -> Result<()> {
        let info_client = InfoClient::new(constants::API_URL);

        let intervals = config.intervals.clone();
        let (candle_tx, mut candle_rx) = mpsc::channel::<Vec<CandleSnapshotResponse>>(20);

        let handle_recv_candle = tokio::spawn(async move {
            while let Some(candle) = candle_rx.recv().await {
                println!("get candle from client: {}", candle[0].interval);
                println!("count: {}", candle.len());
            }
        });

        for interval in intervals {
            let coin_symbol = config.coin_symbol.clone();
            let candle_tx = candle_tx.clone();
            let clone_info_client = info_client.clone();

            let _handle = tokio::spawn(async move {
                let payload = CandleSnapshotPayload {
                    coin: coin_symbol,
                    interval,
                    start_time: 0,
                    end_time: AppDateTime::now().to_milliseconds(),
                };

                let body = CandleSnapshotBody {
                    body_type: CANDLE_SNAPSHOT_BODY_TYPE.to_string(),
                    payload,
                };
                println!("start client worker: {body:#?}");
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
