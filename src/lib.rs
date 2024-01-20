mod application;
mod domain;
pub mod infrastructure;
use std::str::FromStr;

use prima_tracing::{configure_subscriber, init_subscriber, Country, Environment, Uninstall};
use tokio::{fs, io};

async fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path).await
}

pub fn init_otel() -> Uninstall {
    let service_name = "bingoktà".to_string();
    let opentelemetry_ip = std::env::var("OPENTELEMETRY_IP")
        .ok()
        .unwrap_or("192.168.178.29".into());
    let app_env = std::env::var("APP_ENV")
        .ok()
        .and_then(|e| Environment::from_str(&e).ok())
        .unwrap_or(Environment::Dev);
    let subscriber = configure_subscriber(
        prima_tracing::builder(&service_name)
            .with_env(app_env)
            .with_country(Country::Common)
            .with_telemetry(
                format!("http://{}:55681/v1/traces", opentelemetry_ip),
                service_name.clone(),
            )
            .build(),
    );
    init_subscriber(subscriber)
}
