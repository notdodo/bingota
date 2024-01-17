use axum::{routing::get, Router};

use crate::{
    application::service::MyService,
    infrastructure::web::controller::{get_file, ping},
};

pub fn routes_v1() -> Router<MyService> {
    Router::<MyService>::new().route("/get_file", get(get_file))
}

pub fn routes_default() -> Router<MyService> {
    Router::<MyService>::new().route("/ping", get(ping))
}
