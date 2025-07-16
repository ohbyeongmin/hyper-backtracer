use crate::{
    commander::ClientCommand,
    worker::client_worker::{Action, ClientWorker},
};
use tokio::{sync::mpsc, task::JoinHandle};

pub struct WorkerManager {
    coin_symbol: String,
    intervals: Vec<String>,
}

impl WorkerManager {
    pub fn new(coin_symbol: String, intervals: Vec<String>) -> Self {
        Self {
            coin_symbol,
            intervals,
        }
    }

    pub async fn start(&self, tx: mpsc::Sender<ClientCommand>) -> Vec<JoinHandle<()>> {
        let mut worker_handles: Vec<JoinHandle<()>> = Vec::new();

        for interval in self.intervals.clone() {
            let tx = tx.clone();
            let coin_symbol = self.coin_symbol.clone();

            let handle = tokio::spawn(async move {
                let action = Action::GetAllCandle {
                    coin_symbol,
                    interval,
                };
                let worker = ClientWorker::new(tx);

                let res = worker.start(action).await.unwrap();

                println!("received candle data: {}", res.len());
            });

            worker_handles.push(handle);
        }

        drop(tx);

        worker_handles
    }
}
