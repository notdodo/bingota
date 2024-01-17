use rust_demo::infrastructure::web::start_server::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
