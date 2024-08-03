import React, { useEffect, useState } from 'react';
import axios from 'axios';
import './App.css';

const App = () => {
  const [status, setStatus] = useState({});
  const [sensorData, setSensorData] = useState({
    temperature: '',
    humidity: '',
    light_brightness: '',
    soil_moisture: ''
  });
  const [gpioStates, setGpioStates] = useState([true, true, true, true]);
  const [wifiConnected, setWifiConnected] = useState(false);

  useEffect(() => {
    fetchStatus();
    fetchSensorData();
    checkWifi();
  }, []);

  const fetchStatus = async () => {
    const response = await axios.get('http://192.168.105.229:8080/api/status');
    setStatus(response.data);
  };

  const fetchSensorData = async () => {
    const response = await axios.get('http://192.168.105.229:8080/api/sensor-data');
    setSensorData(response.data);
  };

  const toggleGPIO = async (gpio) => {
    const newGpioStates = [...gpioStates];
    newGpioStates[gpio - 1] = !newGpioStates[gpio - 1];
    setGpioStates(newGpioStates);
    await axios.post(`http://192.168.105.229:8080/api/toggle-gpio`, { gpio, state: newGpioStates[gpio - 1] });
    fetchStatus();
  };

  const checkWifi = async () => {
    // This would be replaced with the actual check for WiFi connectivity on the ESP32
    const response = await axios.get('http://192.168.105.229:8080/api/status'); // Example endpoint
    setWifiConnected(response.status === 200);
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Smart Irrigation System</h1>
        <div className="status">
          <p>System Status: <span className={status.is_on ? 'on' : 'off'}>{status.is_on ? 'ON' : 'OFF'}</span></p>
          <p>Irrigating: <span className={status.is_irrigating ? 'on' : 'off'}>{status.is_irrigating ? 'YES' : 'NO'}</span></p>
        </div>
        <div className="wifi-status">
          <p>WiFi: <span className={wifiConnected ? 'wifi-on' : 'wifi-off'}>{wifiConnected ? 'Connected' : 'Disconnected'}</span></p>
        </div>
      </header>
      <main>
        <div className="controls">
          {['Temperature', 'Humidity', 'Light Brightness', 'Soil Moisture'].map((sensor, index) => (
            <div key={index} className="sensor-control">
              <button
                className={`toggle-button ${gpioStates[index] ? 'on' : 'off'}`}
                onClick={() => toggleGPIO(index + 1)}
              >
                {gpioStates[index] ? 'Turn Off' : 'Turn On'} {sensor}
              </button>
              {gpioStates[index] && (
                <p className="sensor-value">
                  {sensor}: {sensorData[sensor.toLowerCase().replace(' ', '_')]}
                </p>
              )}
            </div>
          ))}
        </div>
        <div className="sensor-data">
          <h2>Sensor Data</h2>
          <table>
            <thead>
              <tr>
                <th>Temperature (°C)</th>
                <th>Humidity (%)</th>
                <th>Light Brightness</th>
                <th>Soil Moisture</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>{sensorData.temperature}</td>
                <td>{sensorData.humidity}</td>
                <td>{sensorData.light_brightness}</td>
                <td>{sensorData.soil_moisture}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
};

export default App;
