use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Sqlite;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppData {
    registering_pool: Mutex<Vec<HashMap<String, String>>>,
}

impl AppData {
    pub fn new() -> AppData {
        AppData {
            registering_pool: Mutex::new(vec![]),
        }
    }
}

type AnyResult<T = ()> = anyhow::Result<T>;

pub struct SqlDatabase {
    conn_pool: sqlx::Pool<Sqlite>,
}

impl SqlDatabase {
    pub async fn new(db_path: &String) -> AnyResult<SqlDatabase> {
        Ok(SqlDatabase {
            conn_pool: SqlitePoolOptions::new()
                .max_connections(5)
                .connect(db_path)
                .await?,
        })
    }
}

struct User {
    id: String,
    email: String,
}
struct Card {
    id: String,
    owner: String,
}
