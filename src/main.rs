//use hyper_backtracer::manager::Manager;

use futures::future;

use tokio::sync::mpsc;

use hyper_backtracer::{
    client::InfoClient, commander::ClientCommand, constants, worker::worker_manager::WorkerManager,
};

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

    let worker_manager = WorkerManager::new(
        "HYPE".to_string(),
        vec!["5m".to_string(), "15m".to_string()],
    );

    let handle_workers = worker_manager.start(tx).await;

    handle_info_client.await.unwrap();
    future::join_all(handle_workers).await;
}
