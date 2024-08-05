import React, { useEffect, useState } from "react";
import axios from "axios";
import moment from "moment-timezone"; // Import moment-timezone
import "./App.css";
import { WiThermometer, WiHumidity } from "weather-icons-react";
import { BsBrightnessHigh } from "react-icons/bs";
import { GiPlantWatering } from "react-icons/gi";
import { FaWifi } from "react-icons/fa";

const App = () => {
  const [status, setStatus] = useState({
    is_irrigating: false,
    is_wifi_connected: false,
  });
  const [sensorData, setSensorData] = useState({
    temperature: "",
    humidity: "",
    light_brightness: "",
    soil_moisture: "",
  });
  const [lastTenSensorData, setLastTenSensorData] = useState([]);

  useEffect(() => {
    fetchStatus();
    fetchSensorData();
    fetchLastTenSensorData();
  }, []);

  useEffect(() => {
    const fetchInterval = setInterval(fetchSensorData, 3000);
    const fetchStatusInterval = setInterval(fetchStatus, 3000);
    const fetchLastTenSensorDataInterval = setInterval(fetchLastTenSensorData, 30000);

    return () => {
      clearInterval(fetchInterval);
      clearInterval(fetchStatusInterval);
      clearInterval(fetchLastTenSensorDataInterval);
    };
  }, []);

  const fetchStatus = async () => {
    try {
      const response = await axios.get("http://192.168.8.101:8080/api/status");
      setStatus(response.data);
    } catch (error) {
      console.error("Error fetching status:", error);
    }
  };

  const fetchSensorData = async () => {
    try {
      const response = await axios.get("http://192.168.8.101:8080/api/sensor-data");
      setSensorData(response.data);
    } catch (error) {
      console.error("Error fetching sensor data:", error);
    }
  };

  const fetchLastTenSensorData = async () => {
    try {
      const response = await axios.get("http://192.168.8.101:8080/api/last-ten-sensor-data");
      if (Array.isArray(response.data)) {
        const formattedData = response.data.map(item => ({
          ...item,
          created_at: moment(item.created_at).tz('Africa/Lagos').format('YYYY-MM-DD HH:mm:ss'), // Format using moment-timezone
        }));
        setLastTenSensorData(formattedData);
      } else {
        console.error("Expected array but got:", response.data);
      }
    } catch (error) {
      console.error("Error fetching last ten sensor data:", error);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Smart Irrigation System</h1>
        <div className="status">
          <p>
            <GiPlantWatering />
            <span className={status.is_irrigating ? "on" : "off"}>
              {status.is_irrigating ? "Irrigating" : "Not Irrigating"}
            </span>
          </p>
          <p>
            <FaWifi />
            <span className={status.is_wifi_connected ? "wifi-on" : "wifi-off"}>
              {status.is_wifi_connected ? "Connected" : "Disconnected"}
            </span>
          </p>
        </div>
      </header>
      <main>
        <div className="controls">
          <div className="sensor-control">
            <button className="sensor-button">
              <WiThermometer size={60} />
              <br />
              {sensorData.temperature} °C
            </button>
          </div>
          <div className="sensor-control">
            <button className="sensor-button">
              <WiHumidity size={60} />
              <br />
              {sensorData.humidity} %rh
            </button>
          </div>
          <div className="sensor-control">
            <button className="sensor-button">
              <BsBrightnessHigh size={60} />
              <br />
              {sensorData.light_brightness} lux
            </button>
          </div>
          <div className="sensor-control">
            <button className="sensor-button">
              <GiPlantWatering size={60} />
              <br />
              {sensorData.soil_moisture} %
            </button>
          </div>
        </div>
        <div className="sensor-data">
          <h2>Data Collected</h2>
          <table>
            <thead>
              <tr>
                <th>Temperature (°C)</th>
                <th>Humidity </th>
                <th>Light Brightness </th>
                <th>Soil Moisture </th>
                <th>Time </th>
              </tr>
            </thead>
            <tbody>
              {lastTenSensorData.map((data, index) => (
                <tr key={index}>
                  <td>{data.temperature}</td>
                  <td>{data.humidity}</td>
                  <td>{data.light_brightness}</td>
                  <td>{data.soil_moisture}</td>
                  <td>{data.created_at}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
};

export default App;