use actix_web::{web, HttpResponse, Responder};
use crate::state::{AppState, PinState, SystemStatus};
use crate::models::{StatusResponse, SensorDataResponse, SensorDataInput, ToggleRequest, PinStateRequest, PaginationParams};
use log::info;
use chrono::prelude::*;
use sqlx;

pub async fn get_status(state: web::Data<AppState>) -> impl Responder {
    let status = state.system_status.lock().unwrap();
    HttpResponse::Ok().json(StatusResponse {
        is_on: status.is_on,
        is_irrigating: status.is_irrigating,
        is_wifi_connected: status.is_wifi_connected,
    })
}
pub async fn post_system_status(state: web::Data<AppState>, input: web::Json<StatusResponse>) -> impl Responder {
    info!("Received sensor data: {:?}", input);

    let mut data = state.system_status.lock().unwrap();

    println!("{:?}",data);

    data.is_on = input.is_on;
    data.is_irrigating = input.is_irrigating;
    data.is_wifi_connected = input.is_wifi_connected;

    HttpResponse::Ok().body("Sensor data updated")
}


pub async fn get_sensor_data(state: web::Data<AppState>) -> impl Responder {
    let data = state.sensor_data.lock().unwrap();
    HttpResponse::Ok().json(SensorDataResponse {
        temperature: data.temperature,
        humidity: data.humidity,
        light_brightness: data.light_brightness,
        soil_moisture: data.soil_moisture,
        updated_at: Some(Utc::now()),
        created_at: Some(Utc::now()),
    })
}

pub async fn read_sensor_data(
    state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);
    let offset = (page - 1) * limit as u32;

    let sensor_data: Vec<SensorDataResponse> = sqlx::query_as!(
        SensorDataResponse,
        r#"SELECT temperature, humidity, light_brightness, soil_moisture, created_at, updated_at FROM sensor_data ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
        limit as i64,
        offset as i64,
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_else(|_| Vec::new());

    HttpResponse::Ok().json(sensor_data)
}


pub async fn post_sensor_data(state: web::Data<AppState>, input: web::Json<SensorDataInput>) -> impl Responder {
    info!("Received sensor data: {:?}", input);

    let mut data = state.sensor_data.lock().unwrap();

    println!("{:?}",data);

    sqlx::query!(
        r#"INSERT INTO sensor_data (temperature, humidity, light_brightness, soil_moisture) 
        VALUES ($1, $2, $3, $4)"#,
        data.temperature,
        data.humidity,
        data.light_brightness,
        data.soil_moisture,
    )
    .execute(&state.db.pool)
    .await;

    data.temperature = input.temperature;
    data.humidity = input.humidity;
    data.light_brightness = input.light_brightness;
    data.soil_moisture = input.soil_moisture;

    HttpResponse::Ok().body("Sensor data updated")
}

pub async fn set_gpio(data: web::Data<AppState>, _info: web::Json<ToggleRequest>) -> impl Responder {
    let mut pin_states = data.pin_states.lock().unwrap();

    for req in &_info.pins {
        pin_states.insert(req.pin, PinState { pin_number: req.pin, state: req.state });
    }

    // Serialize all pin states to JSON
    let all_pin_states: Vec<_> = pin_states.values().cloned().collect();
    HttpResponse::Ok().json(all_pin_states)
}

pub async fn send_gpio(data: web::Data<AppState>) -> impl Responder {
    let pin_states = data.pin_states.lock().unwrap();
    let all_pin_states: Vec<_> = pin_states.values().cloned().collect();
    HttpResponse::Ok().json(all_pin_states)
}

pub async fn get_latest_sensor_data(state: web::Data<AppState>) -> impl Responder {
    let latest_sensor_data: Option<SensorDataResponse> = sqlx::query_as!(
        SensorDataResponse,
        r#"SELECT temperature, humidity, light_brightness, soil_moisture, created_at, updated_at FROM sensor_data ORDER BY created_at DESC LIMIT 1"#
    )
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    match latest_sensor_data {
        Some(data) => HttpResponse::Ok().json(data),
        None => HttpResponse::NotFound().json("No sensor data found"),
    }
}

pub async fn get_last_ten_sensor_data(state: web::Data<AppState>) -> impl Responder {
    let sensor_data: Vec<SensorDataResponse> = sqlx::query_as!(
        SensorDataResponse,
        r#"SELECT temperature, humidity, light_brightness, soil_moisture, created_at, updated_at 
        FROM sensor_data 
        ORDER BY created_at DESC 
        LIMIT 10"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_else(|_| Vec::new());

    HttpResponse::Ok().json(sensor_data)
}