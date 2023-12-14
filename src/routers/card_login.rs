use std::error::Error;

use actix_web::{get, web, HttpResponse};
use futures_util::StreamExt;
use serde::Deserialize;
use sqlx::Sqlite;
use tracing::{instrument, Level};

use crate::database::{AppData, Card};

#[derive(Deserialize, Debug)]
pub struct Info {
    card_id: String,
}

#[instrument(level = Level::DEBUG)]
#[get("/card_id")]
pub async fn main(
    info: web::Query<Info>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let requested_card = &info.card_id;

    let rows = sqlx::query_as::<Sqlite, Card>("SELECT id, owner FROM card").fetch(&data.db_conn);

    if !rows
        .any(|val| async {
            match val {
                Ok(val) => val.id == info.card_id,
                Err(e) => {
                    println!("Something went wrong!");
                    println!("{:?}", e);
                    false
                }
            }
        })
        .await
    {}

    todo!()
}
