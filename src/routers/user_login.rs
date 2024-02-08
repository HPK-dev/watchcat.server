use actix_web::{get, http::StatusCode, HttpResponse};
use std::env;

#[get("/user_login")]
pub async fn main() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!(
            include_str!("./../../static/login.html"),
            env::var("GOOGLE_OAUTH_CLIENT_ID").unwrap()
        ))
}
