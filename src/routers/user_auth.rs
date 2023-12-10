use tracing::{instrument, Level};

use actix_web::{get, web, Either, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Info {
    card_id: String,
}

type AnyResult<T = ()> = anyhow::Result<T>;
type RegisterResult = Either<HttpResponse, String>;

#[instrument(level = Level::DEBUG)]
#[get("/user_auth")]
pub async fn main(info: web::Query<Info>) -> RegisterResult {
    let user_id = get_user_id(&info.card_id);

    match user_id {
        Ok(v) => Either::Right(v),

        Err(e) => {
            tracing::error!("{:?}", e);
            Either::Left(
                HttpResponse::InternalServerError()
                .body("Something went wrong! The error has been log. Please contact with server administrator.")
            )
        }
    }
}

fn get_user_id(card_id: &String) -> AnyResult<String> {
    Ok(card_id.to_owned())
}
