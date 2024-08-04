import React, { useEffect, useState } from 'react';
import axios from 'axios';
import './App.css';

const App = () => {
  const [status, setStatus] = useState({
    is_on: false,
    is_irrigating: false,
    is_wifi_connected: false,
  });
  const [sensorData, setSensorData] = useState({
    temperature: '',
    humidity: '',
    light_brightness: '',
    soil_moisture: ''
  });
  const [pinStates, setPinStates] = useState({
    2: false,  // Example GPIO pin for temperature
    32: false, // Example GPIO pin for light brightness
    33: false  // Example GPIO pin for soil moisture
  });

  useEffect(() => {
    fetchStatus();
    fetchSensorData();
    checkPinStatus();
  }, []);

  const fetchStatus = async () => {
    try {
      const response = await axios.get('http://192.168.105.229:8080/api/status');
      setStatus(response.data);
    } catch (error) {
      console.error('Error fetching status:', error);
    }
  };

  const fetchSensorData = async () => {
    try {
      const response = await axios.get('http://192.168.105.229:8080/api/sensor-data');
      setSensorData(response.data);
    } catch (error) {
      console.error('Error fetching sensor data:', error);
    }
  };

  const checkPinStatus = async () => {
    try {
      const response = await axios.get('http://192.168.105.229:8080/api/toggle-gpio');
      const pinStatesData = response.data.reduce((acc, pin) => {
        acc[pin.pin_number] = pin.state;
        return acc;
      }, {});
      setPinStates(pinStatesData);
    } catch (error) {
      console.error('Error fetching pin status:', error);
    }
  };

  const toggleGPIO = async (pinNumber) => {
    const newState = !pinStates[pinNumber];
    try {
      await axios.post('http://192.168.105.229:8080/api/toggle-gpio', {
        pins: [{ pin: pinNumber, state: newState }]
      });
      setPinStates((prevStates) => ({
        ...prevStates,
        [pinNumber]: newState
      }));
    } catch (error) {
      console.error('Error toggling GPIO:', error);
    }
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
          <p>WiFi: <span className={status.is_wifi_connected ? 'wifi-on' : 'wifi-off'}>{status.is_wifi_connected ? 'Connected' : 'Disconnected'}</span></p>
        </div>
      </header>
      <main>
        <div className="controls">
          {Object.keys(pinStates).map((pin) => {
            const sensor = {
              2: 'Temperature',
              32: 'Light Brightness',
              33: 'Soil Moisture'
            }[pin];
            return (
              <div key={pin} className="sensor-control">
                <button
                  className={`toggle-button ${pinStates[pin] ? 'on' : 'off'}`}
                  onClick={() => toggleGPIO(parseInt(pin))}
                >
                  {pinStates[pin] ? 'Turn Off' : 'Turn On'} {sensor}
                </button>
                {pinStates[pin] && (
                  <p className="sensor-value">
                    {sensor}: {sensorData[sensor.toLowerCase().replace(' ', '_')]}
                  </p>
                )}
              </div>
            );
          })}
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