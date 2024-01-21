use crate::application::ports::input_port::InputPort;
use crate::{application::service::Bingokta, read_file_content};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct FileInfo {
    filename: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct FileContent {
    content: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FileInfoError {
    error: String,
}

#[tracing::instrument(skip(info), name = "web::get_file")]
pub async fn get_file(State(state): State<Bingokta>, Query(info): Query<FileInfo>) -> Response {
    match read_file_content(&info.filename).await {
        Ok(content) => {
            let _ = state.process().await;
            (StatusCode::OK, Json(FileContent { content })).into_response()
        }
        Err(_) => {
            tracing::error!("file {} not found", info.filename);
            (
                StatusCode::NOT_FOUND,
                Json(HashMap::new().insert("error", "file not found")),
            )
                .into_response()
        }
    }
}

#[tracing::instrument(name = "web::ping")]
pub async fn ping() -> StatusCode {
    StatusCode::NO_CONTENT
}
