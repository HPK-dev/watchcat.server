use actix_web::{post, web, HttpResponse};
use futures_util::StreamExt;
use serde::Deserialize;
use sqlx::MySql;

use crate::{Admin, AppData, MethodResponse};

#[derive(Deserialize, Debug)]
pub struct Request {
    password: String,
    username: String,
}

#[post("/console_login")]
pub async fn login(item: web::Json<Request>, data: web::Data<AppData>) -> MethodResponse {
    let mut rows =
        sqlx::query_as::<MySql, Admin>("SELECT * FROM Admins WHERE password = ? AND username = ?")
            .bind(&item.password)
            .bind(&item.username)
            .fetch(&data.db_conn);

    while let Some(admin) = rows.next().await {
        if let Ok(v) = admin {
            return Ok(HttpResponse::Ok().json(v));
        }
    }

    Ok(HttpResponse::BadRequest().into())
}
