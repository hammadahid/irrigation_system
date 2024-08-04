use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use std::sync::Mutex;
use crate::state::{AppState, SystemStatus, SensorData, PinState};
use crate::handlers::{get_status, get_sensor_data, post_sensor_data, set_gpio, send_gpio};
use std::collections::HashMap;
use crate::db::Database;


mod state;
mod handlers;
mod models;
mod db;
//mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize application state
    let mut pin_states: HashMap<u8, PinState> = HashMap::new();
    pin_states.insert(2, PinState { pin_number: 2, state: false }); // DHTPIN
    pin_states.insert(32, PinState { pin_number: 32, state: false }); // LDRPIN
    pin_states.insert(33, PinState { pin_number: 33, state: false }); // SSPIN


    let database = Database::connect(true).await;
    let app_state = web::Data::new(AppState {
        system_status: Mutex::new(SystemStatus {
            is_on: false,
            is_irrigating: false,
            is_wifi_connected: false,
        }),
        sensor_data: Mutex::new(SensorData {
            temperature: 0.0,
            humidity: 0.0,
            light_brightness: 0.0,
            soil_moisture: 0.0,
        }),
        pin_states: Mutex::new(pin_states),
        db: database
    });

    // Set up server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/api/status", web::get().to(get_status))
            .route("/api/sensor-data", web::get().to(get_sensor_data))
            .route("/api/sensor-data", web::post().to(post_sensor_data))
            .route("/api/toggle-gpio", web::post().to(set_gpio))
            .route("/api/toggle-gpio", web::get().to(send_gpio))
    })
    .bind("192.168.105.229:8080")?
    .run()
    .await
}
