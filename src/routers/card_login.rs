use crate::MethodResponse;
use actix_web::{get, web, HttpResponse};
use chrono::NaiveDateTime;
use futures_util::TryStreamExt;
use log::debug;

use serde::Deserialize;
use sqlx::MySql;

use crate::{AppData, Card, Reservation, Room, RE_CARD_ID, RE_MAC};

#[derive(Deserialize, Debug)]
pub struct Info {
    card_id: String,
    device_mac: String,
}

#[get("/card_login")]
pub async fn main(info: web::Query<Info>, data: web::Data<AppData>) -> MethodResponse {
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
    let current: NaiveDateTime = chrono::Utc::now().naive_utc();
    debug!("Current time1: {}", current);

    let current: NaiveDateTime = current
        .checked_sub_signed(chrono::Duration::hours(-8))
        .unwrap();
    debug!("Current time2: {}", current);

    // Check if there is a reservation and it is approved
    let rows = sqlx::query_as::<MySql, Reservation>(
        "SELECT * FROM Reservations WHERE room_id = ? AND approval_pending IS FALSE AND begins <= ? AND ends >= ? ",
    )
    .bind(&room_id)
    .bind(current)
    .bind(current)
    .fetch(&data.db_conn);

    // No existing reservation, return 403
    if rows.try_collect::<Vec<Reservation>>().await?.is_empty() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // Check if the card is existing and not expired
    let rows = sqlx::query_as::<MySql, Card>(
        "SELECT * FROM Cards WHERE id = ? AND ( expire is NULL OR expire >= ? )",
    )
    .bind(requested_card)
    .bind(current)
    .fetch(&data.db_conn);

    // No valid card, return 403
    if rows.try_collect::<Vec<Card>>().await?.is_empty() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // Approved. Log the record
    let _ =
        sqlx::query("INSERT INTO Records (room_id, device_mac, card_id, at) VALUES (?, ?, ?, ?)")
            .bind(room_id)
            .bind(device_mac)
            .bind(requested_card)
            .bind(current)
            .execute(&data.db_conn)
            .await?;

    Ok(HttpResponse::Ok().finish())
}
