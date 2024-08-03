#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, json::Json};
use rusqlite::{params, Connection};
use std::sync::Mutex;
use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

#[derive(Serialize)]
struct SensorData {
    temperature: f32,
    humidity: f32,
    light: u32,
    moisture: u32,
}

struct State {
    db: Mutex<Connection>,
    gpio: Mutex<Gpio>,
}

#[get("/status")]
fn status(state: &rocket::State<State>) -> String {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT * FROM status").unwrap();
    let status: (String,) = stmt.query_row([], |row| Ok((row.get(0).unwrap(),))).unwrap();
    status.0
}

#[get("/sensor-data")]
fn sensor_data(state: &rocket::State<State>) -> Json<SensorData> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT temperature, humidity, light, moisture FROM sensor_data ORDER BY timestamp DESC LIMIT 1").unwrap();
    let data = stmt.query_row([], |row| {
        Ok(SensorData {
            temperature: row.get(0).unwrap(),
            humidity: row.get(1).unwrap(),
            ligh  pinMode(LDRPIN, INPUT);
: row.get(2).unwrap(),
            moisture: row.get(3).unwrap(),
        })
    }).unwrap();

    Json(data)
}

#[post("/gpio/<pin>/<state>")]
fn control_gpio(state: &rocket::State<State>, pin: u8, state: String) -> &'static str {
    let gpio = state.gpio.lock().unwrap();
    let pin = gpio.get(pin).unwrap().into_output();
    match state.as_str() {
        "on" => pin.set_high(),
        "off" => pin.set_low(),
        _ => return "Invalid state",
    }
    "Success"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = Connection::open("irrigation.db").unwrap();
    db.execute("CREATE TABLE IF NOT EXISTS sensor_data (timestamp TEXT, temperature REAL, humidity REAL, light INTEGER, moisture INTEGER)", []).unwrap();
    db.execute("CREATE TABLE IF NOT EXISTS status (status TEXT)", []).unwrap();

    let gpio = Gpio::new().unwrap();

    let state = State {
        db: Mutex::new(db),
        gpio: Mutex::new(gpio),
    };

    rocket::build()
        .manage(state)
        .mount("/", routes![status, sensor_data, control_gpio])
        .launch()
        .await?;

    Ok(())
}
