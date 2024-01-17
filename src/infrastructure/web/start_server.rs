use axum::Router;

use crate::application::service::MyService;
use crate::infrastructure::web::routes::routes_v1;

pub async fn start_server() {
    let state = MyService::new();
    let app: Router = Router::new().nest("/api/v1", routes_v1()).with_state(state);
    let addr = "0.0.0.0:8080".parse().expect("Failed to parse address");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to run server")
}
