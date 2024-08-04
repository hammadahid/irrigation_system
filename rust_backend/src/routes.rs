use actix_web::web;
use crate::handlers::{get_status, get_sensor_data, post_sensor_data, set_gpio, send_gpio};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/status", web::get().to(get_status))
            .route("/sensor-data", web::get().to(get_sensor_data))
            .route("/sensor-data", web::post().to(post_sensor_data))
            .route("/toggle-gpio", web::get().to(set_gpio))
            .route("/toggle-gpio", web::post().to(send_gpio)),
        )
    }
