use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest;
use std::sync::Mutex;
use std::collections::HashMap;

struct AppState {
    pin_states: Mutex<HashMap<u8, bool>>, // To track the state of each pin
}

async fn read_pin(data: web::Data<AppState>) -> impl Responder {
    // Example URL to your ESP32
    let esp32_url = "http://your_esp32_ip/read_pin";

    let response = reqwest::get(esp32_url).await.unwrap();
    let pin4_state: String = response.text().await.unwrap();

    HttpResponse::Ok().body(format!("Pin 4 state: {}", pin4_state))
}

async fn control_pin(info: web::Path<(u8, bool)>, data: web::Data<AppState>) -> impl Responder {
    let (pin_number, pin_state) = info.into_inner();
    data.pin_states.lock().unwrap().insert(pin_number, pin_state);

    // Example URL to your ESP32
    let esp32_url = format!("http://your_esp32_ip/control_pin?pin={}&state={}", pin_number, pin_state as u8);

    let client = reqwest::Client::new();
    let response = client.get(&esp32_url).send().await.unwrap();

    HttpResponse::Ok().body(response.text().await.unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        pin_states: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .route("/read_pin", web::get().to(read_pin))
            .route("/control_pin/{pin}/{state}", web::get().to(control_pin))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
