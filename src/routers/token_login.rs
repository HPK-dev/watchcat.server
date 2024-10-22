use crate::MethodResponse;
use crate::{AppData, User};
use actix_web::cookie::time::Duration as CookieDuration;
use actix_web::{post, web, HttpRequest, HttpResponse};
use futures_util::TryStreamExt;
use log::debug;
use serde::Deserialize;
use sqlx::MySql;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Debug, Hash)]
struct AccessToken {
    access_token: String,
}

#[post("/token_login")]
pub async fn main(
    _req: HttpRequest,
    item: web::Json<AccessToken>,
    data: web::Data<AppData>,
) -> MethodResponse {
    let payload = match data
        .google_oauth_client
        .validate_access_token(&item.access_token)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            debug!("Failed to validate access token: {:#?}", e);
            return Ok(HttpResponse::BadRequest().body("Failed to validate access token."));
        }
    };

    debug!("Payload: {:#?}", payload);

    let id = payload.sub;
    let email = payload.email;
    let username = payload.name;

    // TODO: Cookie check
    //
    // // Build hash with posted data and current time
    // let mut s = DefaultHasher::new();
    // SystemTime::now().duration_since(UNIX_EPOCH)?.hash(&mut s);
    // item.hash(&mut s);
    // let hashed = s.finish().to_string(); // TODO: Should cached the result

    // Build cookie
    let cookie = actix_web::cookie::Cookie::build("user-logged", id.clone())
        .max_age(CookieDuration::days(1))
        .finish();

    // Check if the user is already registered
    let rows = sqlx::query_as::<MySql, User>("SELECT * from Users WHERE id=?")
        .bind(id.clone())
        .fetch(&data.db_conn);

    //This user doesn't register yet
    if rows.try_collect::<Vec<User>>().await?.is_empty() {
        let _ = sqlx::query("INSERT INTO Users (id, email, name) VALUES (?, ?, ?)")
            .bind(id)
            .bind(email)
            .bind(username)
            .execute(&data.db_conn)
            .await?;
    }

    Ok(HttpResponse::Found()
        .append_header(("Location", "/after_login")) // WARN: redierct url
        .cookie(cookie)
        .finish())
}

