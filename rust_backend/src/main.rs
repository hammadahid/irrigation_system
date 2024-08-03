use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use std::sync::Mutex;
use crate::state::{AppState, SystemStatus, SensorData};
use crate::handlers::{get_status, get_sensor_data, post_sensor_data, toggle_gpio};
use crate::db::Database;

mod state;
mod handlers;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize application state
    let database = Database::connect(true).await;
    let app_state = web::Data::new(AppState {
        system_status: Mutex::new(SystemStatus {
            is_on: false,
            is_irrigating: false,
        }),
        sensor_data: Mutex::new(SensorData {
            temperature: 0.0,
            humidity: 0.0,
            light_brightness: 0.0,
            soil_moisture: 0.0,
        }),
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
            .route("/api/toggle-gpio", web::post().to(toggle_gpio))
    })
    .bind("192.168.105.229:8080")?
    .run()
    .await
}
