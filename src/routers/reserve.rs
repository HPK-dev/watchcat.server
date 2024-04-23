use actix_web::{delete, get, patch, put, web, HttpResponse};
use chrono::NaiveDateTime;
use futures_util::{StreamExt, TryStreamExt};
use log::error;

use serde::Deserialize;
use sqlx::MySql;
use std::error::Error;

use crate::database::{AppData, Reservation};

#[derive(Deserialize, Debug)]
pub struct PutRequest {
    room_id: String,
    user_id: String,
    description: String,
    // Both begin and ends are in the format of "YYYY-mm-DD HH:MM"
    begins: String,
    ends: String,
}

#[derive(Deserialize, Debug)]
pub struct GetRequest {
    room_id: Option<String>,
    user_id: Option<String>,
    begin: Option<String>,
    ends: Option<String>,
    approval_pending: Option<bool>,
    description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PatchRequest {
    reservation_id: i32,
    room_id: Option<String>,
    user_id: Option<String>,
    begin: Option<String>,
    ends: Option<String>,
    approval_pending: Option<bool>,
    description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DeleteRequest {
    reservation_id: String,
}

// Reserve a room
#[put("/reserve")]
pub async fn main_put(
    info: web::Json<PutRequest>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let room_id = &info.room_id;
    let user_id = &info.user_id;
    let description = &info.description;
    let begins = match NaiveDateTime::parse_from_str(&info.begins, "%Y-%m-%d %H:%M") {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    let ends = match NaiveDateTime::parse_from_str(&info.ends, "%Y-%m-%d %H:%M") {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    // Insert the reservation
    sqlx::query("INSERT INTO Reservations (room_id, user_id, description, begins, ends) VALUES (?, ?, ?, ?, ?)")
            .bind(room_id)
            .bind(user_id)
            .bind(description)
            .bind(begins)
            .bind(ends)
            .execute(&data.db_conn)
            .await?;

    Ok(HttpResponse::Created().finish())
}

// Return reservations
#[get("/reserve")]
pub async fn main_get(
    info: web::Query<GetRequest>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let mut query = "SELECT * FROM Reservations WHERE ".to_string();
    let mut params: Vec<String> = Vec::new();

    // Check if the room_id is provided
    if let Some(room_id) = &info.room_id {
        query.push_str("room_id=? AND ");
        params.push(room_id.to_string());
    }

    // Check if the user_id is provided
    if let Some(user_id) = &info.user_id {
        query.push_str("user_id=? AND ");
        params.push(user_id.to_string());
    }

    // Check if the begin is provided
    if let Some(begin) = &info.begin {
        query.push_str("begins=? AND ");
        params.push(begin.to_string());
    }

    // Check if the ends is provided
    if let Some(ends) = &info.ends {
        query.push_str("ends=? AND ");
        params.push(ends.to_string());
    }

    // Check if the approval_pending is provided
    if let Some(approval_pending) = &info.approval_pending {
        query.push_str("approval_pending=? AND ");
        params.push(approval_pending.to_string());
    }

    // If params is empty, remove the last WHERE
    if params.is_empty() {
        query.drain(query.len() - 6..query.len());
    } else {
        // Remove the last AND
        query.drain(query.len() - 4..query.len());
    }

    // Execute the query
    let rows = sqlx::query_as::<MySql, Reservation>(&query);
    let rows = params.iter().fold(rows, |rows, p| rows.bind(p));
    let mut rows = rows.fetch(&data.db_conn);

    let mut result = Vec::new();

    while let Some(reservation) = rows.next().await {
        if let Err(e) = reservation {
            error!("Something went wrong!");
            error!("{:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }

        let reservation = reservation.unwrap();

        // Check if the description is provided
        if let Some(description) = &info.description {
            // If the description is not in the reservation, skip it
            match &reservation.description {
                Some(v) => {
                    if !v.contains(description) {
                        continue;
                    }
                }
                None => continue,
            }
        }

        result.push(reservation);
    }

    Ok(HttpResponse::Ok().json(result))
}

// Update a reservation
#[patch("/reserve")]
pub async fn main_patch(
    info: web::Json<PatchRequest>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Box<dyn Error>> {
    // Try to get the reservation first
    let mut rows =
        sqlx::query_as::<MySql, Reservation>("SELECT * FROM Reservations WHERE reservation_id=?")
            .bind(info.reservation_id)
            .fetch(&data.db_conn);

    let reservation = match rows.try_next().await {
        Ok(Some(v)) => v,
        Ok(None) => return Ok(HttpResponse::NotFound().finish()),
        Err(e) => {
            error!("Something went wrong!");
            error!("{:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // If all fields are None, return BadRequest
    if info.room_id.is_none()
        && info.user_id.is_none()
        && info.begin.is_none()
        && info.ends.is_none()
        && info.approval_pending.is_none()
        && info.description.is_none()
    {
        return Ok(HttpResponse::BadRequest().finish());
    }

    // The new reservation
    let reservation_id = info.reservation_id;
    let room_id = info.room_id.clone().unwrap_or(reservation.room_id);
    let user_id = info.user_id.clone().unwrap_or(reservation.user_id);
    let description = match &info.description {
        Some(v) => Some(v.to_string()),
        None => reservation.description,
    };
    let begins = match &info.begin {
        Some(v) => match NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M") {
            Ok(v) => v,
            Err(_) => return Ok(HttpResponse::BadRequest().finish()),
        },
        None => reservation.begins,
    };
    let ends = match &info.ends {
        Some(v) => match NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M") {
            Ok(v) => v,
            Err(_) => return Ok(HttpResponse::BadRequest().finish()),
        },
        None => reservation.ends,
    };
    let approval_pending = info
        .approval_pending
        .unwrap_or(reservation.approval_pending);

    // If the room meets the following conditions, consider it conflict:
    // 1. The reservation_id is not the same as the new reservation
    // 2. The room_id is the same as the new reservation
    // 3. The approval_pending is false
    // 4-1. The new reservation begins between the existing reservation
    //    (old begins <= new begins <= old ends)
    // 4-2. The new reservation ends between the existing reservation
    //    (old begins <= new ends <= old ends)
    let rows = sqlx::query_as::<MySql, Reservation>(
        "
        SELECT * 
        FROM Reservations 
        WHERE reservation_id != ? 
          AND room_id = ? 
          AND approval_pending = FALSE 
          AND (begins <= ? AND ends >= ?)
        ",
    )
    .bind(reservation_id)
    .bind(room_id.clone())
    .bind(ends)
    .bind(begins)
    .fetch(&data.db_conn);

    if rows.try_collect::<Vec<Reservation>>().await?.is_empty() {
        // Update the reservation
        sqlx::query("UPDATE Reservations SET room_id=?, user_id=?, description=?, begins=?, ends=?, approval_pending=? WHERE reservation_id=?")
            .bind(room_id)
            .bind(user_id)
            .bind(description)
            .bind(begins)
            .bind(ends)
            .bind(approval_pending)
            .bind(reservation_id)
            .execute(&data.db_conn)
            .await?;

        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Conflict().finish())
    }
}

// Delete a reservation
#[delete("/reserve")]
pub async fn main_delete(
    info: web::Json<DeleteRequest>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Box<dyn Error>> {
    sqlx::query("DELETE FROM Reservations WHERE reservation_id=?")
        .bind(info.reservation_id.to_string())
        .execute(&data.db_conn)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
