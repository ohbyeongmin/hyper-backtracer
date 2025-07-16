use hyper_backtracer::manager::Manager;

#[tokio::main]
async fn main() {
    Manager::sync().await;
}
