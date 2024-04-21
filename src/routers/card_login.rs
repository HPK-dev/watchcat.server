use actix_web::{get, web, HttpResponse};
use chrono::NaiveDateTime;
use futures_util::TryStreamExt;
use log::debug;

use serde::Deserialize;
use sqlx::MySql;
use std::error::Error;

use crate::database::{AppData, Card, Reservation, Room, RE_CARD_ID, RE_MAC};

#[derive(Deserialize, Debug)]
pub struct Info {
    card_id: String,
    device_mac: String,
}

#[get("/card_login")]
pub async fn main(
    info: web::Query<Info>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let requested_card = &info.card_id;
    let device_mac = &info.device_mac;

    // Check if the card_id is valid
    if !RE_CARD_ID.is_match(requested_card) {
        debug!("Invalid card: {}", requested_card);
        return Ok(HttpResponse::BadRequest().into());
    }

    // Check if the device_mac is valid
    if !RE_MAC.is_match(device_mac) {
        debug!("Invalid device: {}", device_mac);
        return Ok(HttpResponse::BadRequest().into());
    }

    // Get room_id from device_mac
    let mut rows = sqlx::query_as::<MySql, Room>("SELECT * FROM Rooms WHERE device_mac = ?")
        .bind(device_mac)
        .fetch(&data.db_conn);

    let room_id = match rows.try_next().await? {
        Some(room) => room.room_id,
        None => {
            debug!("Unknown device: {}", device_mac);
            return Ok(HttpResponse::BadRequest()
                .body(format!("No room found for device: {}", device_mac)));
        }
    };

    // Get current time
    let current: NaiveDateTime = chrono::Utc::now().naive_local();

    // Check if there is a reservation
    let rows = sqlx::query_as::<MySql, Reservation>(
        "SELECT * FROM Reservations WHERE room_id = ? AND begin >= ? AND ends <= ? ",
    )
    .bind(room_id)
    .bind(current)
    .bind(current)
    .fetch(&data.db_conn);

    // No existing reservation, return 403
    if rows.try_collect::<Vec<Reservation>>().await?.is_empty() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // Check if the card is existing and not expired
    let rows = sqlx::query_as::<MySql, Card>("SELECT * FROM Cards WHERE id = ? AND expire >= ?")
        .bind(requested_card)
        .bind(current)
        .fetch(&data.db_conn);

    // No valid card, return 403
    if rows.try_collect::<Vec<Card>>().await?.is_empty() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
