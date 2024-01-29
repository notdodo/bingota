use axum::BoxError;
use axum::{
    routing::{get, put},
    Router,
};

use crate::{
    application::service::Bingokta,
    infrastructure::web::controller::{get_file, number, ping},
};

pub fn routes_v1() -> Router<Bingokta> {
    Router::<Bingokta>::new()
        .route("/get_file", get(get_file))
        .route("/number/:number", put(number))
}

pub fn routes_default() -> Router {
    Router::new().route("/ping", get(ping))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn the_real_deal() {
        let server = TestServer::new(routes_default()).unwrap();
        let response = server.get("/ping").await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }
}
