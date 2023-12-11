use std::collections::HashMap;
use std::str::FromStr;

use anyhow::anyhow;
use serde_json::Value;
use tracing::{instrument, Level};

use actix_web::{post, web, Either, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GoogleUser {
    credential: String,
    g_csrf_token: String,
}
type AnyResult<T = ()> = anyhow::Result<T>;

use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{decode, decode_header, jwk, Algorithm, DecodingKey, TokenData, Validation};

type RegisterResult = Either<HttpResponse, String>;

#[instrument(level = Level::DEBUG)]
#[post("/token_login")]
pub async fn main(req: HttpRequest, item: web::Form<GoogleUser>) -> RegisterResult {
    let cookie_token = req.cookie("g_csrf_token");
    let cookie_post = &item.g_csrf_token;

    if cookie_token.is_none() || cookie_token.unwrap().value() != cookie_post {
        return RegisterResult::Left(HttpResponse::BadRequest().body("Failed to verify user."));
    }

    let token = &item.credential;
    let decoded_cred = match jwt_decoder(token).await {
        Err(e) => {
            println!("{}", e.to_string());
            return RegisterResult::Left(HttpResponse::InternalServerError().into());
        }
        Ok(val) => val,
    };

    let payload = decoded_cred.claims;
    let issuer = payload.get("iss");

    if let Some(iss) = issuer {
        if iss != "accounts.google.com" || iss != "https://accounts.google.com" {
            return RegisterResult::Left(HttpResponse::BadRequest().body("Unknown JWT issuer!"));
        }
    }

    RegisterResult::Left(HttpResponse::Ok().into())
}

async fn jwt_decoder(token: &String) -> AnyResult<TokenData<HashMap<String, Value>>> {
    let jwt_reply = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
        .await?
        .text()
        .await?;

    let jwks: jwk::JwkSet = serde_json::from_str(&jwt_reply)?;

    let header = decode_header(token)?;
    let kid = match header.kid {
        Some(k) => k,
        None => return Err(anyhow!("Token doesn't have a `kid` header field")),
    };
    if let Some(j) = jwks.find(&kid) {
        match &j.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?;

                let mut validation = Validation::new(Algorithm::from_str(
                    j.common.key_algorithm.unwrap().to_string().as_str(),
                )?);
                let aud = std::env::var("google_oauth_id")?;
                validation.set_audience(&[aud]);

                validation.validate_exp = false;
                let decoded_token = decode::<HashMap<String, serde_json::Value>>(
                    token,
                    &decoding_key,
                    &validation,
                )?;

                return Ok(decoded_token);
            }
            _ => return Err(anyhow!("This should be a RSA")),
        }
    } else {
        return Err(anyhow!("No matching JWK found for the given kid"));
    }
}
