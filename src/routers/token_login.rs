use tracing::{instrument, Level};

use actix_web::{post, web, Either, HttpRequest, HttpResponse };
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GoogleUser {
    credential: String,
    g_csrf_token: String,
}

use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{decode, decode_header, jwk, Algorithm, DecodingKey, Validation};

type RegisterResult = Either<HttpResponse, String>;

#[instrument(level = Level::DEBUG)]
#[post("/token_login")]
pub async fn main(req: HttpRequest, item: web::Form<GoogleUser>) -> RegisterResult {
    let cookie_token = req.cookie("g_csrf_token");
    let cookie_post = &item.g_csrf_token;

    if cookie_token.is_none() || cookie_token.unwrap().value() != cookie_post {
        return RegisterResult::Left(HttpResponse::BadRequest().body("Failed to verify user."));
    }

    let token = item.credential;
    let jwt_reply = ""

    RegisterResult::Left(HttpResponse::Ok().into())
}
