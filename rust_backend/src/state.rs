use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub system_status: Mutex<SystemStatus>,
    pub sensor_data: Mutex<SensorData>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            system_status: Mutex::new(self.system_status.lock().unwrap().clone()),
            sensor_data: Mutex::new(self.sensor_data.lock().unwrap().clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SystemStatus {
    pub is_on: bool,
    pub is_irrigating: bool,
}

#[derive(Clone, Debug)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: f32,
    pub light_brightness: f32,
    pub soil_moisture: f32,
}
