use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::routers::token_login::JwtCert;

#[derive(Debug)]
pub struct AppData {
    pub registering_pool: Mutex<Vec<HashMap<String, String>>>,
    pub db_conn: sqlx::Pool<Postgres>,
    pub jwt_cert: Mutex<JwtCert>,
}

impl AppData {
    // HINT: We should panic to make app stop when we cant init data.
    pub async fn new() -> AppData {
        AppData {
            registering_pool: Mutex::new(vec![]),
            db_conn: PgPoolOptions::new()
                .connect(&std::env::var("PG_DATABASE_URL").unwrap())
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
