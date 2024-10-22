use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer};
use actix_web::{web, HttpResponse};
use env_logger::Env;
use log::info;
use serde::Deserialize;
use std::env;
use std::io::Write;

use watchcat_server::routers::{card_login, console, reserve, room_status, token_login};
use watchcat_server::AppData;

type AnyResult<T = ()> = anyhow::Result<T>;

fn check_needed_env() {
    const REQUIRED_ENV_FIELD: [&str; 4] = [
        "BIND_IP",
        "BIND_PORT",
        "GOOGLE_OAUTH_CLIENT_ID",
        "DATABASE_URL",
    ];

    let mut missing: Vec<&str> = Vec::new();

    info!("Checking env vars...");
    for f in REQUIRED_ENV_FIELD {
        match env::var(f) {
            Ok(v) => {
                info!("    {}: {}", f, v);
            }
            Err(_) => {
                missing.push(f);
            }
        }
    }

    if !missing.is_empty() {
        eprintln!("Some env var are missing!");
        for val in missing {
            eprintln!("    {:?}", val);
        }

        panic!("missing env vars");
    }
}

#[actix_web::main]
pub async fn main() {
    #[cfg(debug_assertions)]
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("debug"));
    #[cfg(not(debug_assertions))]
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("info"));

    let time_style = anstyle::Style::new()
        .fg_color(Some(anstyle::AnsiColor::Cyan.into()))
        .bold();

    builder.format(move |buf, record| {
        let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let level = buf.default_level_style(record.level());

        writeln!(
            buf,
            "[{time_style}{}{time_style:#}][{level}{}{level:#}] {}",
            current_time,
            record.level(),
            record.args()
        )
    });

    builder.init();

    // Try to load `.env` file
    let _ = dotenvy::dotenv();

    check_needed_env();

    let bind_ip = env::var("BIND_IP").unwrap();
    let bind_port: u16 = env::var("BIND_PORT").unwrap().parse().unwrap();

    let app_data = web::Data::new(AppData::new().await);

    let server = HttpServer::new(move || {
        #[cfg(not(debug_assertions))]
        let cors = Cors::default()
            .allowed_methods(vec!["POST"])
            .allow_any_origin()
            .max_age(3600);

        // WARN: Do not use this in production!!!
        #[cfg(debug_assertions)]
        let cors = Cors::permissive();

        App::new()
            // App data
            .app_data(app_data.clone())
            // middleware
            .wrap(Logger::default())
            .wrap(cors)
            // routers
            .service(hello)
            .service(teapot)
            .service(card_login::main)
            .service(token_login::main)
            .service(reserve::main_put)
            .service(reserve::main_get)
            .service(reserve::main_patch)
            .service(reserve::main_delete)
            .service(room_status::main_get)
            .service(console::login)
    })
    .bind((bind_ip, bind_port))
    .unwrap();
    server.run().await.unwrap();
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
