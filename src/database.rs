use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Sqlite;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::routers::token_login::JwtCert;

type AM<T> = Arc<Mutex<T>>;
type AnyResult<T = ()> = anyhow::Result<T>;

#[derive(Debug)]
pub struct AppData {
    pub registering_pool: AM<Vec<HashMap<String, String>>>,
    pub db_conn: sqlx::Pool<Sqlite>,
    pub jwt_cert: JwtCert,
}

impl AppData {
    // INFO: We should panic to make app stop when we cant init data.
    pub async fn new() -> AppData {
        AppData {
            registering_pool: Arc::new(Mutex::new(vec![])),
            db_conn: SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&std::env::var("DATABALSE_URL").unwrap())
                .await
                .unwrap(),
            jwt_cert: JwtCert::new().await,
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
    pub id: String,
    pub owner: String,
}
