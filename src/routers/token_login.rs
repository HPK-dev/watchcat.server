use tracing::{instrument, Level};

use actix_web::{post, web, Either, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GoogleUser {
    idtoken: String,
}

type AnyResult<T = ()> = anyhow::Result<T>;
type RegisterResult = Either<HttpResponse, String>;

#[instrument(level = Level::DEBUG)]
#[post("/token_login")]
pub async fn main(item: web::Json<GoogleUser>) -> RegisterResult {
    println!("{:?}",&item);
    RegisterResult::Left(HttpResponse::Ok().into())
}
