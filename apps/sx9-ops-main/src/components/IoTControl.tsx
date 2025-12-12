import React, { useState } from 'react';
import { Lock, Unlock, Thermometer, Database } from 'lucide-react';

/**
 * IoTControl.tsx
 * Component for managing and controlling IoT devices in CTAS
 * Author: Charlie Payne
 * Date: June 15, 2023
 * 
 * This component provides an interface for managing IoT devices,
 * including status monitoring and control functionalities.
 * 
 * MVP:
 *  - Display list of connected IoT devices
 *  - Basic status information for each device
 * IOC:
 *  - Real-time updates on device status
 *  - Basic control interface for device management
 * Production:
 *  - Advanced IoT device management and orchestration
 *  - Integration with AI-driven anomaly detection for IoT devices
 */

interface IoTDevice {
  id: string;
  name: string;
  type: 'Security' | 'DataManagement' | 'Environmental';
  status: 'Online' | 'Offline';
  data: any;
}

const IoTControl: React.FC = () => {
  const [devices, setDevices] = useState<IoTDevice[]>([
    { id: '1', name: 'EntryCam-01', type: 'Security', status: 'Online', data: { locked: true } },
    { id: '2', name: 'ServerRoomSensor-01', type: 'Environmental', status: 'Online', data: { temperature: 20, humidity: 45 } },
    { id: '3', name: 'DataArchive-01', type: 'DataManagement', status: 'Online', data: { storageUsed: '70%' } },
    { id: '4', name: 'NetworkMonitor-01', type: 'Security', status: 'Online', data: { activeConnections: 42 } },
  ]);

  const toggleDeviceStatus = (id: string) => {
    setDevices(prevDevices =>
      prevDevices.map(device =>
        device.id === id
          ? { ...device, status: device.status === 'Online' ? 'Offline' : 'Online' }
          : device
      )
    );
  };

  const toggleLock = (id: string) => {
    setDevices(prevDevices =>
      prevDevices.map(device =>
        device.id === id && device.type === 'Security'
          ? { ...device, data: { ...device.data, locked: !device.data.locked } }
          : device
      )
    );
  };

  return (
    <div className="iot-control">
      <h1 className="text-lg font-bold mb-4">IoT Device Control</h1>
      <ul>
        {devices.map(device => (
          <li key={device.id} className="mb-2 p-2 border rounded">
            <p>
              <strong>{device.name}</strong>
              <button
                onClick={() => toggleDeviceStatus(device.id)}
                className={`ml-2 px-2 py-1 text-xs rounded ${
                  device.status === 'Online' ? 'bg-green-500' : 'bg-red-500'
                }`}
              >
                {device.status}
              </button>
            </p>
            <p>Type: {device.type}</p>
            {device.type === 'Security' && (
              <button
                onClick={() => toggleLock(device.id)}
                className="flex items-center text-xs bg-blue-500 hover:bg-blue-600 px-2 py-1 rounded mt-2"
              >
                {device.data.locked ? <Lock size={16} /> : <Unlock size={16} />}
                {device.data.locked ? 'Unlock' : 'Lock'}
              </button>
            )}
            {device.type === 'Environmental' && (
              <p>
                <Thermometer size={16} /> {device.data.temperature}°C, {device.data.humidity}% Humidity
              </p>
            )}
            {device.type === 'DataManagement' && (
              <p>
                <Database size={16} /> Storage Used: {device.data.storageUsed}
              </p>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default IoTControl;
