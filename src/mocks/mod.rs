// Mock sensor data providers for testing
//
// This module provides mock implementations of sensor data
// for testing without requiring actual hardware.

use crate::models::*;

/// Generate mock accelerometer data simulating device at rest
pub fn mock_accelerometer_at_rest() -> AccelerometerData {
    AccelerometerData {
        x: 0.0,
        y: 9.81, // Gravity pointing down
        z: 0.0,
        timestamp: current_timestamp(),
        accuracy: 3,
    }
}

/// Generate mock accelerometer data simulating device shaking
pub fn mock_accelerometer_shaking() -> AccelerometerData {
    AccelerometerData {
        x: 2.5,
        y: 8.3,
        z: -1.2,
        timestamp: current_timestamp(),
        accuracy: 3,
    }
}

/// Generate mock magnetometer data pointing north
pub fn mock_magnetometer_north() -> MagnetometerData {
    MagnetometerData {
        x: 0.0,
        y: 50.0,
        z: 20.0,
        heading: 0.0,
        timestamp: current_timestamp(),
        accuracy: 3,
    }
}

/// Generate mock magnetometer data pointing southwest
pub fn mock_magnetometer_southwest() -> MagnetometerData {
    MagnetometerData {
        x: -35.0,
        y: -35.0,
        z: 20.0,
        heading: 225.0,
        timestamp: current_timestamp(),
        accuracy: 3,
    }
}

/// Generate mock GPS data for San Francisco
pub fn mock_gps_san_francisco() -> GpsData {
    GpsData {
        latitude: 37.7749,
        longitude: -122.4194,
        altitude: Some(16.0),
        accuracy: 5.0,
        speed: Some(0.0),
        timestamp: current_timestamp(),
    }
}

/// Generate mock GPS data while moving
pub fn mock_gps_moving() -> GpsData {
    GpsData {
        latitude: 37.7750,
        longitude: -122.4195,
        altitude: Some(18.0),
        accuracy: 8.0,
        speed: Some(5.5), // ~20 km/h
        timestamp: current_timestamp(),
    }
}

/// Generate mock pressure data at sea level
pub fn mock_pressure_sea_level() -> PressureData {
    PressureData {
        pressure: 1013.25,
        timestamp: current_timestamp(),
    }
}

/// Generate mock pressure data at altitude
pub fn mock_pressure_altitude() -> PressureData {
    PressureData {
        pressure: 950.0, // ~500m altitude
        timestamp: current_timestamp(),
    }
}

/// Generate mock temperature data (comfortable room temperature)
pub fn mock_temperature_comfortable() -> TemperatureData {
    TemperatureData {
        temperature: 22.5,
        timestamp: current_timestamp(),
    }
}

/// Generate mock temperature data (hot day)
pub fn mock_temperature_hot() -> TemperatureData {
    TemperatureData {
        temperature: 35.0,
        timestamp: current_timestamp(),
    }
}

/// Generate mock WiFi network list
pub fn mock_wifi_networks() -> Vec<WifiNetwork> {
    vec![
        WifiNetwork {
            ssid: "MyHomeWiFi".to_string(),
            bssid: "00:11:22:33:44:55".to_string(),
            signal_strength: -45,
            frequency: 2412,
            security: "WPA2".to_string(),
        },
        WifiNetwork {
            ssid: "Neighbor_5G".to_string(),
            bssid: "AA:BB:CC:DD:EE:FF".to_string(),
            signal_strength: -68,
            frequency: 5180,
            security: "WPA3".to_string(),
        },
        WifiNetwork {
            ssid: "CoffeeShop-Guest".to_string(),
            bssid: "11:22:33:44:55:66".to_string(),
            signal_strength: -52,
            frequency: 2437,
            security: "Open".to_string(),
        },
    ]
}

/// Get current timestamp in milliseconds
fn current_timestamp() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_accelerometer_at_rest() {
        let data = mock_accelerometer_at_rest();
        assert_eq!(data.y, 9.81);
        assert_eq!(data.accuracy, 3);
    }

    #[test]
    fn test_mock_magnetometer_north() {
        let data = mock_magnetometer_north();
        assert_eq!(data.heading, 0.0);
    }

    #[test]
    fn test_mock_gps_san_francisco() {
        let data = mock_gps_san_francisco();
        assert_eq!(data.latitude, 37.7749);
        assert_eq!(data.longitude, -122.4194);
    }

    #[test]
    fn test_mock_pressure_sea_level() {
        let data = mock_pressure_sea_level();
        assert_eq!(data.pressure, 1013.25);
    }

    #[test]
    fn test_mock_wifi_networks() {
        let networks = mock_wifi_networks();
        assert_eq!(networks.len(), 3);
        assert_eq!(networks[0].ssid, "MyHomeWiFi");
    }

    #[test]
    fn test_timestamp_is_recent() {
        let data = mock_accelerometer_at_rest();
        let now = current_timestamp();
        assert!(data.timestamp <= now);
        assert!(data.timestamp > now - 1000); // Within last second
    }
}
