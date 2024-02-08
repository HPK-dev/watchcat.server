use actix_web::{get, web, HttpResponse};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use log::{debug, error};
use regex::Regex;
use serde::Deserialize;
use sqlx::MySql;
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::database::{AppData, Card};

lazy_static! {
    static ref RE_CARD_ID: Regex = Regex::new(r"[a-fA-F0-9]{16}(.)+").unwrap();
}
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

    if !RE_CARD_ID.is_match(requested_card) {
        debug!("Invalid card: {}", requested_card);
        return Ok(HttpResponse::BadRequest().into());
    }

    let rows =
        sqlx::query_as::<MySql, Card>("SELECT id, owner, expire FROM Cards").fetch(&data.db_conn);

    match rows
        .any(|val| async {
            if let Err(e) = val {
                error!("Something went wrong!");
                error!("{:?}", e);
                panic!("err!") // WARN: need some investigation
            } else {
                let card = val.unwrap();
                if card.id != info.card_id {
                    return false;
                }

                if card.expire.is_none() {
                    return true;
                }

                let ex = card.expire.unwrap();

                let timestamp_ms = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();

                let expire_ms: u64 = ex
                    .timestamp_millis()
                    .try_into()
                    .expect("Time went backwards"); // ?? :thinking emoji:

                expire_ms > timestamp_ms
            }
        })
        .await
    {
        true => Ok(HttpResponse::Ok().finish()),
        false => Ok(HttpResponse::Forbidden().finish()),
    }
}
