use actix_web::{get, http::StatusCode, web, Either, HttpResponse};
use serde::Deserialize;
use std::env;
use tracing::{instrument, Level};

#[instrument(level = Level::DEBUG)]
#[get("/user_login")]
pub async fn main() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!(
            include_str!("./../../static/login.html"),
            env::var("google_oauth_id").unwrap()
        ))
}
