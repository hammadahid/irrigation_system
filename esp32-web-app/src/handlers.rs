use actix_web::{web, HttpResponse, Responder};
use crate::state::AppState;
use crate::models::{StatusResponse, SensorDataResponse, SensorDataInput, ToggleRequest};
use std::sync::Arc;
use sqlx::SqlitePool;

pub async fn get_status(state: web::Data<AppState>, pool: web::Data<SqlitePool>) -> impl Responder {
    let status = sqlx::query_as!(
        StatusResponse,
        r#"SELECT is_on, is_irrigating FROM system_status ORDER BY id DESC LIMIT 1"#
    )
    .fetch_one(pool.get_ref())
    .await;

  
    match status {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch system status"),
    }
}

pub async fn get_sensor_data(state: web::Data<AppState>, pool: web::Data<SqlitePool>) -> impl Responder {
    let data = sqlx::query_as!(
        SensorDataResponse,
        r#"SELECT temperature, humidity, light_brightness, soil_moisture FROM sensor_data ORDER BY id DESC LIMIT 1"#
    )
    .fetch_one(pool.get_ref())
    .await;

    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch sensor data"),
    }
}

pub async fn post_sensor_data(state: web::Data<AppState>, pool: web::Data<SqlitePool>, new_data: web::Json<SensorDataInput>) -> impl Responder {
    let result = sqlx::query!(
        r#"INSERT INTO sensor_data (temperature, humidity, light_brightness, soil_moisture, timestamp) VALUES (?, ?, ?, ?, datetime('now'))"#,
        new_data.temperature,
        new_data.humidity,
        new_data.light_brightness,
        new_data.soil_moisture
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Sensor data updated"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update sensor data"),
    }
}

pub async fn toggle_gpio(state: web::Data<AppState>, pool: web::Data<SqlitePool>, info: web::Json<ToggleRequest>) -> impl Responder {
    // Implement the logic to toggle the specific GPIO pin based on `info.gpio`
    // Additionally, update the system status in the database if necessary

    let result = sqlx::query!(
        r#"UPDATE system_status SET is_on = ?, is_irrigating = ? WHERE id = (SELECT MAX(id) FROM system_status)"#,
        info.state, // Assuming this determines whether the system is on
        false // Adjust based on your logic for irrigation
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("GPIO toggled"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to toggle GPIO"),
    }
}





