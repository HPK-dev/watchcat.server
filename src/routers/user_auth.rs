use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use std::collections::HashMap;

type AnyResult<T = ()> = anyhow::Result<T>;

pub async fn main(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    match params.get("cid").take() {
        Some(v) => {
            let user_id = get_user_id(v);

            match user_id {
                Ok(v) => (StatusCode::OK, v),

                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unknow error".to_string(),
                ),
            }
        }
        None => (
            StatusCode::BAD_REQUEST,
            "Required card id not found".to_string(),
        ),
    }
}

fn get_user_id(card_id: &String) -> AnyResult<String> {
    Ok(card_id.to_owned())
}
