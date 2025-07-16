use futures::future;
use hyper_backtracer::{
    client::InfoClient,
    commander::ClientCommand,
    constants, helpers,
    worker::worker_manager::{self, WorkerManager},
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<ClientCommand>(10);
    let info_client = InfoClient::new(constants::API_URL);

    let handle_info_client = tokio::spawn(async move {
        while let Some(resp) = rx.recv().await {
            match resp {
                ClientCommand::GetCandle { req, resp } => {
                    let response = info_client.get_candle_snapshot(req).await.unwrap();
                    resp.send(Ok(response)).unwrap();
                }
            };
        }
    });

    let config = worker_manager::Config::new(
        "HYPE".to_string(),
        helpers::InputCandleIntervals::Default,
        tx,
    );

    let worker_manager = WorkerManager::start(config).await.unwrap();

    handle_info_client.await.unwrap();
    future::join_all(worker_manager).await;
}
