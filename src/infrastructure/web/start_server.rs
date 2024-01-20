use axum::Router;

use crate::application::service::MyService;
use crate::infrastructure::web::routes::{routes_default, routes_v1};

#[tracing::instrument(name = "web::start_server")]
pub async fn start_server() {
    let state = MyService::new();
    let app: Router = Router::new()
        .nest("/api/v1", routes_v1())
        .nest("/", routes_default())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to run server")
}
