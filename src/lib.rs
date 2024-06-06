pub mod routers;

use std::error::Error;

use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use google_oauth::AsyncClient;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{FromRow, MySql};

lazy_static! {
    pub static ref RE_CARD_ID: Regex = Regex::new(r"^[a-fA-F0-9]{8}$").unwrap();
    pub static ref RE_MAC: Regex = Regex::new(r"[a-fA-F0-9]{2}(:[a-fA-F0-9]{2}){5}").unwrap();
    pub static ref RE_USER_ID: Regex = Regex::new(r"^[0-9]*$").unwrap();
}

type MethodResponse = Result<HttpResponse, Box<dyn Error>>;

#[derive(Debug)]
pub struct AppData {
    pub db_conn: sqlx::Pool<MySql>,
    pub google_oauth_client: AsyncClient,
}

impl AppData {
    pub async fn new() -> AppData {
        let db_conn = MySqlPoolOptions::new()
            // NOTE: We checked this env var so just unwarp it.
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap_or_else(|e| panic!("Cannot initalize database!\nError message:\n{:#?}", e));

        AppData {
            db_conn,
            google_oauth_client: AsyncClient::new(""),
        }
    }
}

// CREATE TABLE Users (
//     id text NOT NULL,
//     email text,
//     name text
// );
#[derive(Deserialize, Debug, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
}

// IMPORTANT: All time fields are UTC+0 ( NO TIMEZONE OFFSET!!!)
// Also, the definition of NaiveDateTime is 'ISO 8601 combined date and time without timezone'.
//
// CREATE TABLE Cards (
//   expire DATETIME NULL,
//   owner text,
//   id char(8) PRIMARY KEY NOT NULL
// );
#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Card {
    pub expire: Option<NaiveDateTime>,
    pub id: String,
    pub owner: String,
}

// IMPORTANT: All time fields are UTC+0 ( NO TIMEZONE OFFSET!!! )
// Also, the definition of NaiveDateTime is 'ISO 8601 combined date and time without timezone'.
//
// CREATE TABLE Reservations (
//   reservation_id INT AUTO_INCREMENT PRIMARY KEY,
//   room_id text NOT NULL,
//   user_id text NOT NULL,
//   description LONGTEXT NULL,
//   begins DATETIME NOT NULL,
//   ends DATETIME NOT NULL,
//   approval_pending BOOLEAN DEFAULT TRUE
//   );
#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Reservation {
    pub reservation_id: i32,
    pub room_id: String,
    pub user_id: String,
    pub description: Option<String>,
    pub begins: NaiveDateTime,
    pub ends: NaiveDateTime,
    pub approval_pending: bool,
}

// CREATE TABLE Rooms (
//   room_id text NOT NULL PRIMARY KEY,
//   device_mac char(17) NOT NULL UNIQUE
//   );
#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Room {
    pub room_id: String,
    pub device_mac: String,
}

// IMPORTANT: All time fields are UTC+0 ( NO TIMEZONE OFFSET!!!)
// Also, the definition of NaiveDateTime is 'ISO 8601 combined date and time without timezone'.
//
// CREATE TABLE Records (
//   room_id text NOT NULL PRIMARY KEY,
//   device_mac char(17) NOT NULL UNIQUE
//   card_id char(8) PRIMARY KEY NOT NULL
//   at DATETIME NOT NULL,
// )
#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Record {
    room_id: String,
    device_mac: String,
    card_id: String,
    at: NaiveDateTime,
}
