
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#include <Arduino.h>
#include <DHT.h>
#include <ESPAsyncWebServer.h>
#include <HTTPClient.h>
#include <WiFi.h>
#include <Wire.h>

const char *ssid = "Redmi Note 13";
const char *password = "codersarehere12";

AsyncWebServer server(80);

#define DHTPIN 2
#define LDRPIN 32
#define SSPIN 33
#define DHTTYPE DHT11
#define SCREEN_WIDTH 128
#define SCREEN_HEIGHT 64

DHT dht(DHTPIN, DHTTYPE);
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

const char *serverName = "http://192.168.105.229:8080/api/sensor-data";

void setup() {
  dht.begin();
  Serial.begin(115200);
  Serial.println("Dht begins");
  pinMode(LDRPIN, INPUT);
  pinMode(SSPIN, INPUT);

  WiFi.begin(ssid, password);
  WiFi.setSleep(false);

  long rssi = WiFi.RSSI();
  Serial.print("Signal strength (RSSI): ");
  Serial.println(rssi);

  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.println("Connecting to WiFi...");
  }

  Serial.println("Connected to WiFi");

  if (!display.begin(SSD1306_SWITCHCAPVCC, 0x3C)) {
    Serial.println(F("SSD1306 allocation failed"));
    for (;;)
      ;
  }
  delay(2000);

  server.on("/read_pin", HTTP_GET, [](AsyncWebServerRequest *request) {
    int pin2State = digitalRead(SSPIN);
    request->send(200, "text/plain", String(pin2State));
  });

  server.on("/control_pin", HTTP_GET, [](AsyncWebServerRequest *request) {
    if (request->hasParam("pin") && request->hasParam("state")) {
      int pin = request->getParam("pin")->value().toInt();
      int state = request->getParam("state")->value().toInt();
      digitalWrite(pin, state);
      request->send(200, "text/plain", "OK");
    } else {
      request->send(400, "text/plain", "Invalid request");
    }
  });

  server.begin();
}

void loop() {
  delay(1000);
  int value = analogRead(SSPIN);
  int brightness = analogRead(LDRPIN);
  float h = dht.readHumidity();
  float t = dht.readTemperature();

  if (isnan(h) || isnan(t)) {
    Serial.println(F("Failed to read from sensor!"));
    return;
  }

  Serial.print(F("Humidity: "));
  Serial.print(h);
  Serial.print(F("%  Temperature: "));
  Serial.print(t);
  Serial.print(F("°C "));
  Serial.print(F("% SOIL MOISTURE: "));
  Serial.print(value);
  Serial.print(F("% Brightness: "));
  Serial.print(brightness);
  Serial.print('\n');

  display.clearDisplay();
  display.setTextSize(2);
  display.setTextColor(WHITE);
  display.setCursor(10, 0);
  display.println(t);
  display.setCursor(10, 20);
  display.println(h);
  display.setCursor(10, 40);
  display.println(value);
  display.display();

  // Send sensor data to the server
  if (WiFi.status() == WL_CONNECTED) {
    HTTPClient http;
    http.begin(serverName);
    http.addHeader("Content-Type", "application/json");
    http.setTimeout(20000);

    String jsonPayload = "{\"temperature\": " + String(t) +
                         ", \"humidity\": " + String(h) +
                         ", \"light_brightness\": " + String(brightness) +
                         ", \"soil_moisture\": " + String(value) + "}";

    int httpResponseCode = http.POST(jsonPayload);
    int retries = 3;

    for (int i = 0; i < retries; i++) {
      if (httpResponseCode > 0) {
        String response = http.getString();
        Serial.println(httpResponseCode);
        Serial.println(response);
        break;
      } else {
        Serial.print("Error on sending POST: ");
        Serial.println(httpResponseCode);
        delay(2000);
      }
    }
    http.end();
  } else {
    Serial.println("Error in WiFi connection");
    WiFi.disconnect();
    WiFi.begin(ssid, password);
  }
}
