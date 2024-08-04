use actix_web::{web, HttpResponse, Responder};
use crate::state::{AppState, PinState};
use crate::models::{StatusResponse, SensorDataResponse, SensorDataInput, ToggleRequest, PinStateRequest};
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


pub async fn post_sensor_data(state: web::Data<AppState>, input: web::Json<SensorDataInput>) -> impl Responder {
    info!("Received sensor data: {:?}", input);

    let mut data = state.sensor_data.lock().unwrap();

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

    HttpResponse::Ok().json("Sensor data updated")
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