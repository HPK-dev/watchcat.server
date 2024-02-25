use crate::database::AppData;
use crate::database::Card;
use crate::database::RE_USER_ID;
use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use log::debug;
use serde::Deserialize;
use sqlx::MySql;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Fields {
    user_id: String,
}

#[post("/fetch_cards")]
pub async fn main(
    data: web::Data<AppData>,
    item: web::Json<Fields>,
) -> Result<HttpResponse, Box<dyn Error>> {
    if !RE_USER_ID.is_match(&item.user_id) {
        debug!("Invalid user id: {}", &item.user_id);
        return Ok(HttpResponse::BadRequest().into());
    }

    let rows = sqlx::query_as::<MySql, Card>("SELECT * FROM Cards WHERE id=\"?\"")
        .bind(&item.user_id)
        .fetch(&data.db_conn);

    unimplemented!()
}
