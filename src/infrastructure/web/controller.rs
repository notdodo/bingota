use crate::application::{ports::input_port::InputPort, service::MyService};
use axum::{extract, extract::State, http::StatusCode, Json};
use tokio::{fs, io};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FileInfo {
    filename: String,
}

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

async fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path).await
}
