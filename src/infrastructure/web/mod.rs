pub mod controller;
pub mod routes;

use axum::Router;

use crate::application::service::Bingokta;
use crate::infrastructure::web::routes::{routes_default, routes_v1};

use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

pub async fn start_server() {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::weeks(55),
        ));

    let state = Bingokta::new();
    let app: Router = Router::new()
        .nest("/api/v1", routes_v1())
        .with_state(state)
        .layer(session_layer)
        .nest("/", routes_default());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to run server")
}
