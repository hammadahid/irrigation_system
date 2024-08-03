use actix_web::{web, HttpResponse, Responder};
use crate::state::AppState;
use crate::models::{StatusResponse, SensorDataResponse, SensorDataInput, ToggleRequest};
use log::info;

pub async fn get_status(state: web::Data<AppState>) -> impl Responder {
    let status = state.system_status.lock().unwrap();
    HttpResponse::Ok().json(StatusResponse {
        is_on: status.is_on,
        is_irrigating: status.is_irrigating,
    })
}


pub async fn get_sensor_data(state: web::Data<AppState>) -> impl Responder {
    let data = state.sensor_data.lock().unwrap();
    HttpResponse::Ok().json(SensorDataResponse {
        temperature: data.temperature,
        humidity: data.humidity,
        light_brightness: data.light_brightness,
        soil_moisture: data.soil_moisture,
    })
}


pub async fn post_sensor_data(state: web::Data<AppState>, input: web::Json<SensorDataInput>) -> impl Responder {
    info!("Received sensor data: {:?}", input);

    let mut data = state.sensor_data.lock().unwrap();
    data.temperature = input.temperature;
    data.humidity = input.humidity;
    data.light_brightness = input.light_brightness;
    data.soil_moisture = input.soil_moisture;

    HttpResponse::Ok().json("Sensor data updated")
}

pub async fn toggle_gpio(state: web::Data<AppState>, _info: web::Json<ToggleRequest>) -> impl Responder {
    let _info = _info;
    let mut status = state.system_status.lock().unwrap();
    status.is_on = _info.state;

    HttpResponse::Ok().body("GPIO toggled")
}
