use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Sqlite;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::routers::token_login::JwtCert;

#[derive(Debug)]
pub struct AppData {
    pub registering_pool: Mutex<Vec<HashMap<String, String>>>,
    pub db_conn: sqlx::Pool<Sqlite>,
    pub jwt_cert: Mutex<JwtCert>,
}

impl AppData {
    // HINT: We should panic to make app stop when we cant init data.
    pub async fn new() -> AppData {
        AppData {
            registering_pool: Mutex::new(vec![]),
            db_conn: SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
            jwt_cert: Mutex::new(JwtCert::new().await),
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
    pub expire: Option<String>,
    pub id: String,
    pub owner: String,
}
