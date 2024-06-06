use crate::AppData;
use crate::Card;
use crate::MethodResponse;
use crate::RE_USER_ID;
use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use futures_util::StreamExt;
use log::debug;
use serde::Deserialize;
use serde::Serialize;
use sqlx::MySql;

#[derive(Deserialize, Debug)]
struct Fields {
    user_id: String,
}

#[derive(Serialize, Debug)]
struct Resp {
    cards: Vec<Card>,
}

#[post("/fetch_cards")]
pub async fn main(data: web::Data<AppData>, item: web::Json<Fields>) -> MethodResponse {
    if !RE_USER_ID.is_match(&item.user_id) {
        debug!("Invalid user id: {}", &item.user_id);
        return Ok(HttpResponse::BadRequest().into());
    }

    let mut rows = sqlx::query_as::<MySql, Card>("SELECT * FROM Cards WHERE id=\"?\"")
        .bind(&item.user_id)
        .fetch(&data.db_conn);

    let mut cards: Vec<Card> = Vec::new();
    while let Some(card) = rows.next().await {
        cards.push(card?);
    }

    Ok(HttpResponse::Ok().json(Resp { cards }))
}

