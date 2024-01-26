use crate::application::ports::input_port::InputPort;
use crate::{application::service::Bingokta, read_file_content};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_sessions::Session;

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

const COUNTER_KEY: &str = "counter";

#[derive(Default, Deserialize, Serialize)]
struct Counter(usize);

#[tracing::instrument(name = "web::number")]
pub async fn number(
    session: Session,
    State(state): State<Bingokta>,
    Path(number): Path<u32>,
) -> Response {
    let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
    session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();
    println!("Current count: {}", counter.0);
    println!("{}", number);
    StatusCode::NO_CONTENT.into_response()
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
