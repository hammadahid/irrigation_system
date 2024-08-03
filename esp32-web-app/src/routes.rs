use actix_web::web;
use crate::handlers::{get_status, get_sensor_data, toggle_gpio};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/status", web::get().to(get_status))
            .route("/sensor-data", web::get().to(get_sensor_data))
            .route("/toggle-gpio", web::post().to(toggle_gpio)),
    );
}
