use std::collections::HashMap;
use std::str::FromStr;

use actix_web::{post, web, Either, HttpRequest, HttpResponse};
use anyhow::anyhow;
use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{decode, decode_header, jwk, Algorithm, DecodingKey, TokenData, Validation};
use serde::Deserialize;
use serde_json::Value;
use tracing::{instrument, Level};

use crate::database::AppData;

#[derive(Deserialize, Debug)]
pub struct GoogleUser {
    credential: String,
    g_csrf_token: String,
}
type AnyResult<T = ()> = anyhow::Result<T>;

type RegisterResult = Either<HttpResponse, String>;

#[instrument(level = Level::DEBUG)]
#[post("/token_login")]
pub async fn main(
    req: HttpRequest,
    item: web::Form<GoogleUser>,
    data: web::Data<AppData>,
) -> RegisterResult {
    // Obtain required data
    let cookie_token = req.cookie("g_csrf_token");
    let post_request_token = &item.g_csrf_token;

    // Check if post-request token and cookie token are met.
    if cookie_token.is_none() || cookie_token.unwrap().value() != post_request_token {
        return RegisterResult::Left(HttpResponse::BadRequest().body("Failed to verify user."));
    }

    // Get the encoded JWT
    let token = &item.credential;

    // Decode
    let decoded_cred = match jwt_decoder(token).await {
        Err(e) => {
            println!("{:?}", e);
            return RegisterResult::Left(HttpResponse::InternalServerError().into());
        }
        Ok(val) => val,
    };

    let payload = decoded_cred.claims;

    // If the JWT is not issued by Google, should the token be considered as forged by others? 🤔
    if !payload
        .get("iss")
        .is_some_and(|iss| iss == "accounts.google.com" || iss == "https://accounts.google.com")
    {
        return RegisterResult::Left(HttpResponse::BadRequest().body("Unknown JWT issuer!"));
    }


    println!("{:?}",payload);

    // TODO: Add the unsigned user

    // TODO: This should return a redirect response
    RegisterResult::Left(HttpResponse::Ok().into())
}


async fn jwt_decoder(token: &String) -> AnyResult<TokenData<HashMap<String, Value>>> {
    //////////////////////////////////////////////////////////////////////////
    // WARN: This impl will send a request on each Google login request!
    // Should make a cache to save the cert.
    let jwt_reply = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
        .await?
        .text()
        .await?;
    //////////////////////////////////////////////////////////////////////////

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
