use actix_web::{get, web};
use serde::Deserialize;

use crate::AppData;
use crate::MethodResponse;

#[derive(Deserialize, Debug)]
pub struct GetRequest {
    room_id: Option<String>,
    user_id: Option<String>,
    begins: Option<String>,
    ends: Option<String>,
}

#[get("/room_status")]
pub async fn main_get(info: web::Query<GetRequest>, data: web::Data<AppData>) -> MethodResponse {
    unimplemented!()
}
