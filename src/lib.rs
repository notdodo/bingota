mod application;
mod domain;
pub mod infrastructure;
use tokio::{fs, io};

async fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path).await
}
