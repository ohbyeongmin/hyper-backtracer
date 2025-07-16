use anyhow::{Context, Result};
use tokio::sync::{mpsc, oneshot};

use crate::{
    client,
    commander::{self, commands::ClientCommand},
    helpers,
    models::{CandleSnapshotBody, CandleSnapshotPayload, CandleSnapshotResponse, TimeRawData},
};

const BODY_TYPE: &str = "candleSnapshot";

#[derive(Debug, Clone)]
pub enum Action {
    GetAllCandle {
        coin_symbol: String,
        interval: String,
    },
    GetPartCandle {
        coin_symbol: String,
        interval: String,
        start_time: TimeRawData,
    },
}

pub struct ClientWorker {
    tx_commander: mpsc::Sender<ClientCommand>,
}

impl ClientWorker {
    pub fn new(tx_commander: mpsc::Sender<ClientCommand>) -> Self {
        Self { tx_commander }
    }

    pub async fn start(&self, action: Action) -> Result<Vec<CandleSnapshotResponse>> {
        let (resp_tx, resp_rx) = oneshot::channel::<client::Response>();
        let req = Self::build_request(action);
        let cmd = commander::ClientCommand::GetCandle { req, resp: resp_tx };

        self.tx_commander
            .send(cmd)
            .await
            .context("failed client worker: send")?;

        resp_rx.await.context("failed client worker: response")?
    }

    fn build_request(action: Action) -> CandleSnapshotBody {
        let end_time = helpers::AppDateTime::now().to_milliseconds();

        let payload = match action {
            Action::GetAllCandle {
                coin_symbol,
                interval,
            } => CandleSnapshotPayload {
                coin: coin_symbol,
                interval,
                start_time: 0,
                end_time,
            },

            Action::GetPartCandle {
                coin_symbol,
                interval,
                start_time,
            } => CandleSnapshotPayload {
                coin: coin_symbol,
                interval,
                start_time,
                end_time,
            },
        };

        CandleSnapshotBody {
            body_type: BODY_TYPE.to_string(),
            payload,
        }
    }
}
