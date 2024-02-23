mod database;
mod routers;

use crate::database::AppData;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer};
use actix_web::{web, HttpResponse};
use dotenv::dotenv;
use log::{debug, error};
use routers::{card_login, token_login, user_login};
use serde::Deserialize;
use std::env;

type AnyResult<T = ()> = anyhow::Result<T>;

fn check_needed_env() -> AnyResult {
    const REQUIRED_ENV_FIELD: [&str; 4] = [
        "BIND_IP",
        "BIND_PORT",
        "GOOGLE_OAUTH_CLIENT_ID",
        "DATABASE_URL",
    ];

    let mut missing: Vec<&str> = Vec::new();
    let mut should_crash = false;

    debug!("Checking env vars...");
    for f in REQUIRED_ENV_FIELD {
        match env::var(f) {
            Ok(v) => {
                debug!("{}: {}", f, v);
            }
            Err(e) => {
                should_crash = true;
                missing.push(f);
            }
        }
    }

    if should_crash {
        println!("Some env var are missing!");
        for val in missing {
            println!("    {:?}", val);
        }

        return Err(anyhow::anyhow!("missing env vars"));
    }

    Ok(())
}

#[actix_web::main]
pub async fn main() -> AnyResult {
    dotenv().ok();

    check_needed_env()?;

    env_logger::init();

    let bind_ip = env::var("BIND_IP")?;
    let bind_port: u16 = env::var("BIND_PORT")?.parse()?;

    let app_data = web::Data::new(AppData::new().await);

    let server = HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_methods(vec!["POST"])
            .allow_any_origin()
            .max_age(3600);

        App::new()
            // middleware
            .wrap(Logger::default())
            .wrap(cors)
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
