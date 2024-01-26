use bingokta::{infrastructure::web::start_server, init_otel};

#[tokio::main]
async fn main() {
    let _guard = init_otel();
    tracing::info!("Bingoktà started");
    start_server().await;
}
