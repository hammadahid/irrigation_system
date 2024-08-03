use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct StatusResponse {
    pub is_on: Option<bool>,
    pub is_irrigating: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct SensorDataResponse {
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub light_brightness: Option<f64>,
    pub soil_moisture: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorDataInput {
    pub temperature: f32,
    pub humidity: f32,
    pub light_brightness: f32,
    pub soil_moisture: f32,
}

#[derive(Deserialize, Serialize)]
pub struct ToggleRequest {
    pub gpio: u8,
    pub state: bool,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct SensorData {
    pub id: i64,
    pub temperature: f64,
    pub humidity: f64,
    pub light_brightness: f64,
    pub soil_moisture: f64,
    pub timestamp: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct SystemStatus {
    pub id: i64,
    pub is_on: bool,
    pub is_irrigating: bool,
    pub timestamp: String,
}
