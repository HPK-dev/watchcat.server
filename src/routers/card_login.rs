use actix_web::{get, web, HttpResponse};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use log::{error, debug};
use regex::Regex;
use serde::Deserialize;
use sqlx::Sqlite;
use std::error::Error;

use crate::database::{AppData, Card};

lazy_static! {
    static ref REGEX_SUB: Regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();
}
#[derive(Deserialize, Debug)]
pub struct Info {
    card_id: String,
}

#[get("/card_login")]
pub async fn main(
    info: web::Query<Info>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let requested_card = &info.card_id;

    if REGEX_SUB.is_match(requested_card) {
        debug!("Invalid card: {}",requested_card);
        return Ok(HttpResponse::BadRequest().into());
    }

    let rows = sqlx::query_as::<Sqlite, Card>("SELECT id, owner FROM card").fetch(&data.db_conn);

    if rows
        .any(|val| async {
            match val {
                Ok(val) => val.id == info.card_id,
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
        Ok(HttpResponse::BadRequest().into())
    }
}
