use axum::{routing::get, Router};

use crate::{
    application::service::Bingokta,
    infrastructure::web::controller::{get_file, ping},
};

pub fn routes_v1() -> Router<Bingokta> {
    Router::<Bingokta>::new().route("/get_file", get(get_file))
}

pub fn routes_default() -> Router<Bingokta> {
    Router::<Bingokta>::new().route("/ping", get(ping))
}
