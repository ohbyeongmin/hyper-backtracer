use crate::{
    common::dto::client::{CandleSnapshotBody, ClientResponse},
    worker::worker_manager::ClientInterface,
};
use anyhow::Context;
use tokio::sync::{mpsc, oneshot};

pub struct ClientWorker {
    tx: mpsc::Sender<ClientInterface>,
}

impl ClientWorker {
    pub fn new(tx: mpsc::Sender<ClientInterface>) -> Self {
        Self { tx }
    }

    pub async fn action(&self, payload: CandleSnapshotBody) -> ClientResponse {
        let (resp_tx, resp_rx) = oneshot::channel::<ClientResponse>();
        let cmd = ClientInterface::GetCandleSnapshot {
            req: payload,
            resp: resp_tx,
        };

        self.tx
            .send(cmd)
            .await
            .context("failed client worker: send")?;

        resp_rx.await.context("failed client worker: response")?
    }
}
