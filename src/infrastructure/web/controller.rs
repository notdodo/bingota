use crate::{
    application::{ports::input_port::InputPort, service::MyService},
    read_file_content,
};
use axum::{extract, extract::State, http::StatusCode, Json};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FileInfo {
    filename: String,
}

#[tracing::instrument(skip(info), name = "web::get_file")]
pub async fn get_file(
    State(state): State<MyService>,
    info: extract::Query<FileInfo>,
) -> (StatusCode, Json<FileInfo>) {
    let _ = state.process().await;
    match read_file_content(&info.filename).await {
        Ok(content) => (StatusCode::OK, Json(FileInfo { filename: content })),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(FileInfo {
                filename: "J".to_string(),
            }),
        ),
    }
}

#[tracing::instrument(name = "web::ping")]
pub async fn ping() -> StatusCode {
    StatusCode::NO_CONTENT
}
