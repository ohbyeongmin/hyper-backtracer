#[macro_use]
extern crate dotenv_codegen;

use hyper_backtracer::service::sync;

#[tokio::main]
async fn main() {
    let db_url = dotenv!("DATABASE_URL");
    let _ = sync::sync_candles("BTC", "default", db_url).await;
}
