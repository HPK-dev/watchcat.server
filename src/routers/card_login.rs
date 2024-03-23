use actix_web::{get, web, HttpResponse};
use chrono::FixedOffset;
use futures_util::StreamExt;
use log::{debug, error};

use serde::Deserialize;
use sqlx::MySql;
use std::error::Error;

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

    // let mut rows = sqlx::query_as::<MySql, Card>("SELECT * FROM Cards WHERE ?=1 AND id=?")
    let mut rows = sqlx::query_as::<MySql, Card>("SELECT * FROM Cards WHERE id=?")
        // .bind(device_mac)
        .bind(requested_card)
        .fetch(&data.db_conn);

    while let Some(card) = rows.next().await {
        if let Err(e) = card {
            error!("Something went wrong!");
            error!("{:?}", e);
            return Ok(HttpResponse::InternalServerError().finish()); // WARN: need some investigation
        }

        let card = card.unwrap();

        let expire = match card.expire {
            Some(ex) => ex.and_utc(),
            None => return Ok(HttpResponse::Ok().finish()),
        };

        let current = chrono::Utc::now();

        debug!("CUR: {:?}", current);
        debug!("EXP: {:?}", expire);

        return match expire > current {
            true => Ok(HttpResponse::Ok().finish()),
            false => continue,
        };
    }

    Ok(HttpResponse::Forbidden().finish())
}
