use google_oauth::AsyncClient;
use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySql;

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

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct Card {
    pub expire: Option<chrono::NaiveDateTime>,
    pub id: String,
    pub owner: String,
}
