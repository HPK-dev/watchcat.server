use actix_web::{get, web, HttpResponse};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use log::{debug, error};
use regex::Regex;
use serde::Deserialize;
use sqlx::Sqlite;
use std::{
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
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

    let rows = sqlx::query_as::<Sqlite, Card>("SELECT id, owner FROM card").fetch(&data.db_conn);

    if rows
        .any(|val| async {
            match val {
                Ok(val) => {
                    if val.id != info.card_id {
                        return false;
                    };

                    let expired_time = val.expire;
                    if !expired_time.is_some_and(|v| {
                        // Get the current time
                        let current_time = SystemTime::now();

                        // Calculate the duration since the Unix epoch
                        let duration_since_epoch = current_time
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");

                        // Extract the number of seconds as a u64
                        let timestamp_seconds = duration_since_epoch.as_secs();

                        v.parse::<u64>().expect("Invalid time") > timestamp_seconds
                    }) {
                        return false;
                    }

                    true
                }
                Err(e) => {
                    error!("Something went wrong!");
                    error!("{:?}", e);
                    false
                }
            }
        })
        .await
    {
        Ok(HttpResponse::Ok().into())
    } else {
        Ok(HttpResponse::Forbidden().into())
    }
}
