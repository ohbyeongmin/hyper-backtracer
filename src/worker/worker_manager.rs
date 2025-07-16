use crate::{
    commander::ClientCommand,
    helpers::{CandleIntervals, InputCandleIntervals},
    worker::client_worker::{Action, ClientWorker},
};
use anyhow::{Context, Result};
use tokio::{sync::mpsc, task::JoinHandle};

pub struct WorkerManager;

#[derive(Debug)]
pub struct Config {
    coin_symbol: String,
    intervals: InputCandleIntervals,
    tx: mpsc::Sender<ClientCommand>,
}

impl Config {
    pub fn new(
        coin_symbol: String,
        intervals: InputCandleIntervals,
        tx: mpsc::Sender<ClientCommand>,
    ) -> Self {
        Self {
            coin_symbol,
            intervals,
            tx,
        }
    }
}

impl WorkerManager {
    pub async fn start(config: Config) -> Result<Vec<JoinHandle<()>>> {
        let intervals = match config.intervals {
            InputCandleIntervals::Default => CandleIntervals::default(),
            InputCandleIntervals::Custom(input) => {
                CandleIntervals::new(&input).context("failed parse custom intervals")?
            }
        };

        let mut worker_handles: Vec<JoinHandle<()>> = Vec::new();

        for interval in intervals.into_iter() {
            let tx = config.tx.clone();
            let coin_symbol = config.coin_symbol.clone();

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

        Ok(worker_handles)
    }
}
