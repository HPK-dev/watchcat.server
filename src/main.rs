mod database;
mod routers;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer};
use actix_web::{web, HttpResponse};
use dotenv::dotenv;
use routers::{card_login, token_login, user_login};
use serde::Deserialize;

use crate::database::AppData;
use std::env;
use std::error::Error;

const REQUIRED_ENV_FIELD: [&str; 5] = [
    "BIND_IP",
    "BIND_PORT",
    "GOOGLE_OAUTH_KEY",
    "GOOGLE_OAUTH_ID",
    "DATABASE_URL",
];

fn check_needed_env() -> Result<(), Box<dyn Error>> {
    for f in REQUIRED_ENV_FIELD {
        env::var(f).unwrap_or_else(|_| panic!("Required env variable `{}` is missing!", f));
    }

    Ok(())
}

#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    check_needed_env()?;

    env_logger::init();

    let bind_ip = env::var("BIND_IP")?;
    let bind_port: u16 = env::var("BIND_PORT")?.parse()?;

    let app_data = web::Data::new(AppData::new().await);

    let server = HttpServer::new(move || {
        App::new()
            // middleware
            .wrap(Logger::default())
            // routers
            .service(hello)
            .service(teapot)
            .service(card_login::main)
            .service(token_login::main)
            // WARN: This page will be replace with foront-end webpage
            .service(user_login::main)
            // App data
            .app_data(app_data.clone())
    })
    .bind((bind_ip, bind_port))?;
    server.run().await?;

    Ok(())
}

#[get("/")]
pub async fn hello() -> String {
    "Hello, World!".to_string()
}

#[derive(Deserialize, Debug)]
struct Echo {
    msg: String,
}

#[get("/echo")]
async fn echo(info: web::Query<Echo>) -> String {
    info.msg.to_owned()
}

#[get("/teapot")]
async fn teapot() -> HttpResponse {
    HttpResponse::ImATeapot()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./../static/teapot.html"))
}
