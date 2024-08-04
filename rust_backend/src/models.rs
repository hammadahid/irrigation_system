use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    pub is_on: bool,
    pub is_irrigating: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SensorDataResponse {
    pub temperature: f64,
    pub humidity: f64,
    pub light_brightness: f64,
    pub soil_moisture: f64,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorDataInput {
    pub temperature: f64,
    pub humidity: f64,
    pub light_brightness: f64,
    pub soil_moisture: f64,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct ToggleRequest {
    pub gpio: u8,
    pub state: bool,
}
