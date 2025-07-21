use crate::{
    client::InfoClient,
    common::{
        constants,
        dto::client::{
            CandleSnapshotBody, CandleSnapshotPayload, CandleSnapshotResponse, ClientResponse,
            CANDLE_SNAPSHOT_BODY_TYPE,
        },
        helper::{AppDateTime, CandleIntervals},
    },
    worker::client_worker::ClientWorker,
};
use anyhow::Result;
use tokio::sync::{mpsc, oneshot};

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

pub type Responder<T> = oneshot::Sender<T>;

pub enum ClientInterface {
    GetCandleSnapshot {
        req: CandleSnapshotBody,
        resp: Responder<ClientResponse>,
    },
}

pub enum DbInterface {
    WriteCandleSnapshot,
}

pub struct WorkerManager;

impl WorkerManager {
    pub async fn start(config: Config) -> Result<()> {
        let info_client = InfoClient::new(constants::API_URL);

        let (client_tx, mut client_rx) = mpsc::channel(20);

        println!("start client");
        let info_client_handle = tokio::spawn(async move {
            while let Some(resp) = client_rx.recv().await {
                match resp {
                    ClientInterface::GetCandleSnapshot { req, resp } => {
                        println!("received req: {req:#?}");
                        let response = info_client.get_candle_snapshot(req).await;
                        resp.send(response).unwrap();
                    }
                }
            }
        });

        let intervals = config.intervals.clone();
        let (candle_tx, mut candle_rx) = mpsc::channel::<Vec<CandleSnapshotResponse>>(20);

        let handle_recv_candle = tokio::spawn(async move {
            while let Some(candle) = candle_rx.recv().await {
                println!("get candle from client: {}", candle[0].interval);
                println!("count: {}", candle.len());
            }
        });

        for interval in intervals {
            let new_client_tx = client_tx.clone();
            let new_client_worker = ClientWorker::new(new_client_tx);
            let coin_symbol = config.coin_symbol.clone();
            let candle_tx = candle_tx.clone();

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
                let res = new_client_worker.action(body).await.unwrap();
                println!("get candle info: {}", res.len());
                candle_tx.send(res).await.unwrap();
            });
        }

        drop(client_tx);
        drop(candle_tx);

        info_client_handle.await?;
        handle_recv_candle.await?;

        Ok(())
    }
}
