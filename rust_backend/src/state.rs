use std::sync::Mutex;
use std::collections::HashMap;
use crate::db::Database;
use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub struct AppState {
    pub system_status: Mutex<SystemStatus>,
    pub sensor_data: Mutex<SensorData>,
    pub pin_states: Mutex<HashMap<u8, PinState>>,
    pub db: Database,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            system_status: Mutex::new(self.system_status.lock().unwrap().clone()),
            sensor_data: Mutex::new(self.sensor_data.lock().unwrap().clone()),
            pin_states: Mutex::new(self.pin_states.lock().unwrap().clone()),
            db: self.db.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SystemStatus {
    pub is_on: bool,
    pub is_irrigating: bool,
    pub is_wifi_connected: bool,
}

#[derive(Clone, Debug)]
pub struct SensorData {
    pub temperature: f64,
    pub humidity: f64,
    pub light_brightness: f64,
    pub soil_moisture: f64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct PinState {
    pub pin_number: u8,
    pub state: bool,
}
