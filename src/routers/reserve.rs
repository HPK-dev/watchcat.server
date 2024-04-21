use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDateTime;
use futures_util::{StreamExt, TryStreamExt};
use log::error;

use serde::Deserialize;
use sqlx::MySql;
use std::error::Error;

use crate::database::{AppData, Reservation};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PostRequest {
    room_id: String,
    user_id: String,
    description: String,
    // Both begin and ends are in the format of "YYYY-mm-DD HH:MM"
    begins: String,
    ends: String,
}

// #[derive(Deserialize, Debug)]
// #[allow(non_snake_case)]
// pub struct GetRequest {
//     room_id: Option<String>,
//     user_id: Option<String>,
//     begin: Option<String>,
//     ends: Option<String>,
// }

#[post("/reserve")]
pub async fn main_post(
    info: web::Query<PostRequest>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let room_id = &info.room_id;
    let user_id = &info.user_id;
    let description = &info.description;
    let begins = match NaiveDateTime::parse_from_str(&info.begins, "%Y-%m-%d %H:%M") {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let ends = match NaiveDateTime::parse_from_str(&info.ends, "%Y-%m-%d %H:%M") {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    // Check if the room is available
    let rows = sqlx::query_as::<MySql, Reservation>("SELECT * FROM Reservations WHERE room_id=? AND (begin BETWEEN ? AND ? OR ends BETWEEN ? AND ?)")
        .bind(room_id)
        .bind(begins)
        .bind(ends)
        .bind(begins)
        .bind(ends)
        .fetch(&data.db_conn);

    if rows.try_collect::<Vec<Reservation>>().await?.is_empty() {
        // Insert the reservation
        sqlx::query("INSERT INTO Reservations (room_id, user_id, description, begins, ends) VALUES (?, ?, ?, ?, ?)")
            .bind(room_id)
            .bind(user_id)
            .bind(description)
            .bind(begins)
            .bind(ends)
            .execute(&data.db_conn)
            .await?;

        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Conflict().finish())
    }
}

// Return all reservations
#[get("/reserve")]
pub async fn main_get(data: web::Data<AppData>) -> Result<HttpResponse, Box<dyn Error>> {
    let rows =
        sqlx::query_as::<MySql, Reservation>("SELECT * FROM Reservations").fetch(&data.db_conn);

    let mut result = Vec::new();
    let mut rows = rows;

    while let Some(reservation) = rows.next().await {
        if let Err(e) = reservation {
            error!("Something went wrong!");
            error!("{:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }

        let reservation = reservation.unwrap();
        result.push(reservation);
    }

    Ok(HttpResponse::Ok().json(result))
}
