use crate::database::{AppData, User};
use actix_web::cookie::time::Duration as CookieDuration;
use actix_web::{post, web, HttpRequest, HttpResponse};
use anyhow::anyhow;
use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{decode, decode_header, jwk, Algorithm, DecodingKey, TokenData, Validation};
use lazy_static::lazy_static;
use log::{error, warn};
use regex::Regex;
use serde::de::value::MapDeserializer;
use serde::Deserialize;
use serde_json::Value;
use sqlx::Sqlite;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Debug)]
pub struct GoogleUser {
    credential: String,
    g_csrf_token: String,
}
type AnyResult<T = ()> = anyhow::Result<T>;

lazy_static! {
    static ref RE_SUB: Regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    static ref RE_EMAIL: Regex = Regex::new(r"[^a-zA-Z0-9@._]").unwrap();
}

#[post("/token_login")]
pub async fn main(
    req: HttpRequest,
    item: web::Form<GoogleUser>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Box<dyn Error>> {
    // Obtain required data
    let cookie_token = req.cookie("g_csrf_token");
    let post_request_token = &item.g_csrf_token;

    // Check if post-request token and cookie token are met.
    if cookie_token.is_none() || cookie_token.unwrap().value() != post_request_token {
        return Ok(HttpResponse::BadRequest().body("Failed to verify user."));
    }

    // Get the encoded JWT
    let token = &item.credential;

    let mut jwt_cert = data.jwt_cert.lock().unwrap();
    let jwt_cert = jwt_cert.get_cert().await;

    // Decode
    let decoded_cred = match jwt_decoder(token, jwt_cert).await {
        Err(e) => {
            error!("{:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
        Ok(val) => val,
    };

    // Deserialize the Hashmap to `JwtToken`
    let payload = JwtToken::deserialize(MapDeserializer::new(decoded_cred.claims.into_iter()))?;

    // If the JWT is not issued by Google, should the token be considered as forged by others? 🤔
    let iss = &payload.iss;
    if !(iss == "accounts.google.com" || iss == "https://accounts.google.com") {
        warn!("Unknown JWT issuer! {:?}", iss);
        warn!("{:?}", payload);
        return Ok(HttpResponse::BadRequest().body("Invalid token."));
    }

    let sub = &payload.sub;
    let email = &payload.email;

    // IMPORTANT: Ensure `sub` and `email` both does not contain ANY specical characters.
    if RE_SUB.is_match(sub) || RE_EMAIL.is_match(email) {
        warn!("Suspicious values.");
        warn!("payload: {:?}", payload);
        return Ok(HttpResponse::BadRequest().body("Invalid token."));
    }

    // Update user sub
    let rows = sqlx::query_as::<Sqlite, User>("SELECT id, email from user")
        .fetch_all(&data.db_conn)
        .await?;

    // This user doesn't register yet
    if !rows.into_iter().any(|v| v.id == *sub) {
        let _ = sqlx::query("INSERT INTO user (id, email) VALUES ($1, $2)")
            .bind(sub)
            .bind(email)
            .execute(&data.db_conn)
            .await?;
    }

    // TODO: Save to cookie and redirect

    // Build hash with user's `sub` and current time
    let mut s = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH)?.hash(&mut s);
    sub.hash(&mut s);

    // TODO: Should cached the result
    let hashed = s.finish().to_string();

    let cookie = actix_web::cookie::Cookie::build("logged", hashed)
        .max_age(CookieDuration::days(14))
        .finish();

    Ok(HttpResponse::Found()
        .append_header(("Location", "/user_login"))
        .cookie(cookie)
        .finish())
}

#[derive(Debug)]
pub struct JwtCert {
    exp: Duration,
    val: String,
}

impl JwtCert {
    async fn refresh_data(&mut self) -> AnyResult {
        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;

        self.exp = since_the_epoch + Duration::from_secs(3600);
        self.val = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
            .await?
            .text()
            .await?;

        Ok(())
    }

    async fn refresh(&mut self) -> AnyResult {
        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;
        if since_the_epoch > self.exp {
            self.refresh_data().await?;
        }

        Ok(())
    }

    pub async fn new() -> JwtCert {
        let mut jwt_cert = JwtCert {
            exp: Duration::from_secs(1),
            val: "".to_string(),
        };

        let _ = jwt_cert.refresh_data().await;

        jwt_cert
    }

    pub async fn get_cert(&mut self) -> &String {
        let r = self.refresh().await;

        if r.is_err_and(|e| {
            error!("{:?}", e);
            true
        }) {}

        return &self.val;
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct JwtToken {
    iss: String, // The JWT's issuer
    nbf: u64,
    aud: String,          // Your server's client ID
    sub: String,          // The unique ID of the user's Google Account
    hd: Option<String>,   // If present, the host domain of the user's GSuite email address
    email: String,        // The user's email address
    email_verified: bool, // true, if Google has verified the email address
    azp: String,
    name: String,
    picture: Option<String>, // If present, a URL to user's profile picture
    given_name: String,
    family_name: String,
    iat: u64, // Unix timestamp of the assertion's creation time
    exp: u64, // Unix timestamp of the assertion's expiration time
    jti: String,
}

async fn jwt_decoder(
    token: &String,
    jwt_reply: &String,
) -> AnyResult<TokenData<HashMap<String, Value>>> {
    let jwks: jwk::JwkSet = serde_json::from_str(jwt_reply)?;

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
                let aud = std::env::var("GOOGLE_OAUTH_ID")?;
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
