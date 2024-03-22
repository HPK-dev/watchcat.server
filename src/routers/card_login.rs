use actix_web::{get, web, HttpResponse};
use futures_util::StreamExt;
use log::{debug, error};

use serde::Deserialize;
use sqlx::MySql;
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::database::{AppData, Card, RE_CARD_ID, RE_MAC};

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

    if !RE_CARD_ID.is_match(requested_card) {
        debug!("Invalid card: {}", requested_card);
        return Ok(HttpResponse::BadRequest().into());
    }

    if !RE_MAC.is_match(device_mac) {
        debug!("Invalid device: {}", device_mac);
        return Ok(HttpResponse::BadRequest().into());
    }

    let mut rows = sqlx::query_as::<MySql, Card>("SELECT * FROM Cards WHERE id=\"?\"")
        .bind(&info.card_id)
        .fetch(&data.db_conn);

    while let Some(card) = rows.next().await {
        if let Err(e) = card {
            error!("Something went wrong!");
            error!("{:?}", e);
            return Ok(HttpResponse::InternalServerError().finish()); // WARN: need some investigation
        }

        let card = card.unwrap();

        if card.expire.is_none() {
            return Ok(HttpResponse::Ok().finish());
        }

        let ex = card.expire.unwrap();

        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let expire_ms: u64 = ex
            .and_utc()
            .timestamp_millis()
            .try_into()
            .expect("Time went backwards"); // ?? 🤔

        return match expire_ms > timestamp_ms {
            true => Ok(HttpResponse::Ok().finish()),
            false => continue,
        };
    }

    Ok(HttpResponse::Forbidden().finish())
}
