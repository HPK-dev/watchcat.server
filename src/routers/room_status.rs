use actix_web::{get, web, HttpResponse};
use futures_util::StreamExt;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::MySql;

use crate::AppData;
use crate::MethodResponse;

#[derive(Deserialize, Debug)]
pub struct GetRequest {
    room_id: Option<String>,
    user_id: Option<String>,
    begins: Option<String>,
    ends: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct GetResponse {
    room_id: String,
    derive_mac: String,
    card_id: String,
    at: String,
    username: String,
    user_id: String,
}

#[get("/room_status")]
pub async fn main_get(info: web::Query<GetRequest>, data: web::Data<AppData>) -> MethodResponse {
    let mut query = "
    SELECT Records.room_id,
           Records.device_mac,
           Records.card_id,
           Records.at
           Users.id AS user_id,
           Users.name AS username
    FROM Reservations
    INNER JOIN Users ON Reservations.user_id=Users.id
    WHERE "
        .to_string();

    let mut params: Vec<String> = Vec::new();

    // Check if the room_id is provided
    if let Some(room_id) = &info.room_id {
        query.push_str("room_id=? AND ");
        params.push(room_id.to_string());
    }

    // Check if the user_id is provided
    if let Some(user_id) = &info.user_id {
        query.push_str("user_id=? AND ");
        params.push(user_id.to_string());
    }

    // Check if the begin is provided
    if let Some(begin) = &info.begins {
        query.push_str("begins=? AND ");
        params.push(begin.to_string());
    }

    // Check if the ends is provided
    if let Some(ends) = &info.ends {
        query.push_str("ends=? AND ");
        params.push(ends.to_string());
    }

    // If params is empty, remove the last WHERE
    if params.is_empty() {
        query.drain(query.len() - 6..query.len());
    } else {
        // Remove the last AND
        query.drain(query.len() - 4..query.len());
    }

    debug!("Query: {:#?}", query);

    // Execute the query
    let rows = sqlx::query_as::<MySql, GetResponse>(&query);
    let rows = params.iter().fold(rows, |rows, p| rows.bind(p));
    let mut rows = rows.fetch(&data.db_conn);

    let mut result = Vec::new();

    while let Some(record) = rows.next().await {
        if let Err(e) = record {
            error!("Something went wrong!");
            error!("{:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }

        result.push(record.unwrap())
    }

    Ok(HttpResponse::Ok().json(result))
}
