use hyper_backtracer::service::sync;

#[tokio::main]
async fn main() {
    let _ = sync::sync_candles("BTC", "default").await;
}
