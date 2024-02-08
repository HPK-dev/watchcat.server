use crate::routers::token_login::JwtCert;
use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySql;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct AppData {
    pub registering_pool: Mutex<Vec<HashMap<String, String>>>,
    pub db_conn: sqlx::Pool<MySql>,
    pub jwt_cert: Mutex<JwtCert>,
}

impl AppData {
    // HINT: We should panic to make app stop when we cant init data.
    pub async fn new() -> AppData {
        AppData {
            registering_pool: Mutex::new(vec![]),
            db_conn: MySqlPoolOptions::new()
                .connect(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
            jwt_cert: Mutex::new(JwtCert::new().await.unwrap()),
        }
    }
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct Card {
    pub expire: Option<chrono::NaiveDateTime>,
    pub id: String,
    pub owner: String,
}
