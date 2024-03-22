use google_oauth::AsyncClient;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySql;

lazy_static! {
    pub static ref RE_CARD_ID: Regex = Regex::new(r"^[a-fA-F0-9]{16}$").unwrap();
    pub static ref RE_MAC: Regex = Regex::new(r"[a-fA-F0-9]{2}(:[a-fA-F0-9]{2}){5}").unwrap();
    pub static ref RE_USER_ID: Regex = Regex::new(r"^[0-9]*$").unwrap();
}

#[derive(Debug)]
pub struct AppData {
    pub db_conn: sqlx::Pool<MySql>,
    pub google_oauth_client: AsyncClient,
}

impl AppData {
    pub async fn new() -> AppData {
        let db_conn = MySqlPoolOptions::new()
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await;

        match db_conn {
            Ok(db_conn) => AppData {
                db_conn,
                google_oauth_client: AsyncClient::new(""),
            },
            Err(e) => {
                panic!("Cannot initalize database!\nError message:\n{:#?}", e);
            }
        }
    }
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Card {
    pub expire: Option<chrono::NaiveDateTime>,
    pub id: String,
    pub owner: String,
}
