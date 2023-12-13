mod database;
mod routers;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web;
use actix_web::Error;
use actix_web::{get, App, HttpServer};
use dotenv::dotenv;
use routers::{token_login, user_auth, user_login};
use serde::Deserialize;

use crate::database::AppData;
use std::env;
use tracing::Span;
use tracing_actix_web::{RootSpanBuilder, TracingLogger};

type AnyResult<T = ()> = anyhow::Result<T>;

/// We will define a custom root span builder to capture additional fields, specific
/// to our application, on top of the ones provided by `DefaultRootSpanBuilder` out of the box.
pub struct CustomRootSpanBuilder;

impl RootSpanBuilder for CustomRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        // Not sure why you'd be keen to capture this, but it's an example and we try to keep it simple
        let n_headers = request.headers().len();
        // We set `cloud_provider` to a constant value.
        //
        // `name` is not known at this point - we delegate the responsibility to populate it
        // to the `personal_hello` handler. We MUST declare the field though, otherwise
        // `span.record("caller_name", XXX)` will just be silently ignored by `tracing`.
        tracing_actix_web::root_span!(
            request,
            n_headers,
            cloud_provider = "localhost",
            caller_name = tracing::field::Empty
        )
    }

    fn on_request_end<M: MessageBody>(span: Span, outcome: &Result<ServiceResponse<M>, Error>) {
        // Capture the standard fields when the request finishes.
    }
}

const REQUIRED_ENV_FIELD: [&str; 5] = [
    "BIND_IP",
    "BIND_PORT",
    "GOOGLE_OAUTH_KEY",
    "GOOGLE_OAUTH_ID",
    "DATABASE_URL",
];

fn check_needed_env() -> AnyResult {
    for f in REQUIRED_ENV_FIELD {
        env::var(f).expect(&format!("Required env variable `{}` is missing!", f));
    }

    Ok(())
}

#[actix_web::main]
pub async fn main() -> AnyResult {
    dotenv().ok();

    check_needed_env()?;

    let bind_ip = env::var("BIND_IP")?;
    let bind_port = env::var("BIND_PORT")?;

    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "watchcat-server=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    let server = HttpServer::new(|| {
        App::new()
            // middleware
            .wrap(TracingLogger::default())
            // routers
            .service(hello)
            .service(user_auth::main)
            .service(token_login::main)
            // WARN: This page will be replace with foront-end webpage
            .service(user_login::main)
            // App data
            .app_data(web::Data::new(AppData::new()))
    })
    .bind(format!("{}:{}", bind_ip, bind_port))?;
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
