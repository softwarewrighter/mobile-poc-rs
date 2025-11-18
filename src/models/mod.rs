// Data models for mobile sensor data

use serde::{Deserialize, Serialize};

/// Represents data from the accelerometer sensor
///
/// The accelerometer measures acceleration forces in m/s²
/// along three axes (X, Y, Z).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccelerometerData {
    /// Acceleration force along the X axis in m/s²
    pub x: f32,
    /// Acceleration force along the Y axis in m/s²
    pub y: f32,
    /// Acceleration force along the Z axis in m/s²
    pub z: f32,
    /// Timestamp when the data was recorded (Unix timestamp in milliseconds)
    pub timestamp: i64,
    /// Sensor accuracy level (0-3, where 3 is highest)
    pub accuracy: i32,
}

/// Represents data from the magnetometer sensor
///
/// The magnetometer measures magnetic field strength in μT (microtesla)
/// and can be used to determine compass heading.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagnetometerData {
    /// Magnetic field strength along X axis in μT
    pub x: f32,
    /// Magnetic field strength along Y axis in μT
    pub y: f32,
    /// Magnetic field strength along Z axis in μT
    pub z: f32,
    /// Compass heading in degrees from magnetic north (0-359)
    pub heading: f32,
    /// Timestamp when the data was recorded (Unix timestamp in milliseconds)
    pub timestamp: i64,
    /// Sensor accuracy level (0-3, where 3 is highest)
    pub accuracy: i32,
}

/// Represents GPS location data
///
/// Provides geographical coordinates, altitude, and accuracy information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpsData {
    /// Latitude in degrees (-90 to 90)
    pub latitude: f64,
    /// Longitude in degrees (-180 to 180)
    pub longitude: f64,
    /// Altitude in meters above sea level (if available)
    pub altitude: Option<f64>,
    /// Horizontal accuracy estimate in meters
    pub accuracy: f32,
    /// Speed in meters per second (if available)
    pub speed: Option<f32>,
    /// Timestamp when the data was recorded (Unix timestamp in milliseconds)
    pub timestamp: i64,
}

/// Represents pressure sensor data
///
/// The barometric pressure sensor measures atmospheric pressure in hPa.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PressureData {
    /// Atmospheric pressure in hPa (hectopascals) or mbar
    pub pressure: f32,
    /// Timestamp when the data was recorded (Unix timestamp in milliseconds)
    pub timestamp: i64,
}

/// Represents temperature sensor data
///
/// Measures ambient temperature in Celsius.
/// Note: Most mobile devices do not have ambient temperature sensors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemperatureData {
    /// Temperature in degrees Celsius
    pub temperature: f32,
    /// Timestamp when the data was recorded (Unix timestamp in milliseconds)
    pub timestamp: i64,
}

/// Represents a WiFi network
///
/// Information about a detected WiFi access point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WifiNetwork {
    /// Network SSID (Service Set Identifier)
    pub ssid: String,
    /// BSSID (Basic Service Set Identifier) - MAC address
    pub bssid: String,
    /// Signal strength in dBm (typically -100 to 0)
    pub signal_strength: i32,
    /// Frequency in MHz (e.g., 2412 for channel 1, 5180 for 5GHz)
    pub frequency: i32,
    /// Security type (e.g., "WPA2", "WPA3", "Open")
    pub security: String,
}

/// Errors that can occur when working with sensors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorError {
    /// Sensor is not available on this device
    NotAvailable(String),
    /// Permission to access sensor was denied
    PermissionDenied(String),
    /// Hardware error occurred
    HardwareError(String),
    /// Error communicating with sensor plugin
    PluginError(String),
    /// Invalid data received from sensor
    DataError(String),
}

impl std::fmt::Display for SensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorError::NotAvailable(msg) => write!(f, "Sensor not available: {}", msg),
            SensorError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            SensorError::HardwareError(msg) => write!(f, "Hardware error: {}", msg),
            SensorError::PluginError(msg) => write!(f, "Plugin error: {}", msg),
            SensorError::DataError(msg) => write!(f, "Data error: {}", msg),
        }
    }
}

impl std::error::Error for SensorError {}

// Utility functions for calculations

/// Calculate compass heading from magnetometer X and Y values
///
/// Returns heading in degrees from magnetic north (0-359)
///
/// # Arguments
/// * `x` - Magnetic field strength along X axis in μT
/// * `y` - Magnetic field strength along Y axis in μT
///
/// # Returns
/// Heading in degrees (0-359) where 0 is magnetic north
pub fn calculate_heading(x: f32, y: f32) -> f32 {
    // Calculate angle from X and Y components
    // atan2(y, x) gives angle in radians, convert to degrees
    // In compass terms: Y points north, X points east
    let mut heading = x.atan2(y).to_degrees();
    if heading < 0.0 {
        heading += 360.0;
    }
    heading
}

/// Get cardinal direction from heading in degrees
///
/// # Arguments
/// * `heading` - Heading in degrees (0-359)
///
/// # Returns
/// Cardinal direction as a string (N, NE, E, SE, S, SW, W, NW)
pub fn get_cardinal_direction(heading: f32) -> &'static str {
    let normalized = ((heading % 360.0) + 360.0) % 360.0;
    match normalized {
        h if !(22.5..337.5).contains(&h) => "N",
        h if (22.5..67.5).contains(&h) => "NE",
        h if (67.5..112.5).contains(&h) => "E",
        h if (112.5..157.5).contains(&h) => "SE",
        h if (157.5..202.5).contains(&h) => "S",
        h if (202.5..247.5).contains(&h) => "SW",
        h if (247.5..292.5).contains(&h) => "W",
        h if (292.5..337.5).contains(&h) => "NW",
        _ => "N", // Fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TDD: Test-Driven Development
    // RED: Write failing tests first
    // GREEN: Implement minimal code to pass
    // REFACTOR: Clean up and optimize

    #[test]
    fn test_accelerometer_data_creation() {
        let data = AccelerometerData {
            x: 0.0,
            y: 9.81,
            z: 0.0,
            timestamp: 1234567890,
            accuracy: 3,
        };
        assert_eq!(data.y, 9.81);
        assert_eq!(data.accuracy, 3);
    }

    #[test]
    fn test_accelerometer_serialization() {
        let data = AccelerometerData {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            timestamp: 1000,
            accuracy: 2,
        };
        let json = serde_json::to_string(&data).unwrap();
        let deserialized: AccelerometerData = serde_json::from_str(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_magnetometer_data_creation() {
        let data = MagnetometerData {
            x: 10.0,
            y: 20.0,
            z: 30.0,
            heading: 45.0,
            timestamp: 1234567890,
            accuracy: 3,
        };
        assert_eq!(data.heading, 45.0);
    }

    #[test]
    fn test_heading_calculation_north() {
        // When Y is positive and X is near zero, heading should be near 0 (north)
        let heading = calculate_heading(0.0, 10.0);
        assert!((heading - 0.0).abs() < 1.0);
    }

    #[test]
    fn test_heading_calculation_east() {
        // When X is positive and Y is near zero, heading should be near 90 (east)
        let heading = calculate_heading(10.0, 0.0);
        assert!((heading - 90.0).abs() < 1.0);
    }

    #[test]
    fn test_heading_calculation_south() {
        // When Y is negative and X is near zero, heading should be near 180 (south)
        let heading = calculate_heading(0.0, -10.0);
        assert!((heading - 180.0).abs() < 1.0);
    }

    #[test]
    fn test_heading_calculation_west() {
        // When X is negative and Y is near zero, heading should be near 270 (west)
        let heading = calculate_heading(-10.0, 0.0);
        assert!((heading - 270.0).abs() < 1.0);
    }

    #[test]
    fn test_cardinal_direction_north() {
        assert_eq!(get_cardinal_direction(0.0), "N");
        assert_eq!(get_cardinal_direction(10.0), "N");
        assert_eq!(get_cardinal_direction(350.0), "N");
    }

    #[test]
    fn test_cardinal_direction_east() {
        assert_eq!(get_cardinal_direction(90.0), "E");
        assert_eq!(get_cardinal_direction(85.0), "E");
    }

    #[test]
    fn test_cardinal_direction_south() {
        assert_eq!(get_cardinal_direction(180.0), "S");
    }

    #[test]
    fn test_cardinal_direction_west() {
        assert_eq!(get_cardinal_direction(270.0), "W");
    }

    #[test]
    fn test_cardinal_direction_northeast() {
        assert_eq!(get_cardinal_direction(45.0), "NE");
    }

    #[test]
    fn test_gps_data_creation() {
        let data = GpsData {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: Some(10.0),
            accuracy: 5.0,
            speed: Some(0.0),
            timestamp: 1234567890,
        };
        assert_eq!(data.latitude, 37.7749);
        assert_eq!(data.longitude, -122.4194);
    }

    #[test]
    fn test_gps_data_optional_fields() {
        let data = GpsData {
            latitude: 0.0,
            longitude: 0.0,
            altitude: None,
            accuracy: 10.0,
            speed: None,
            timestamp: 1234567890,
        };
        assert!(data.altitude.is_none());
        assert!(data.speed.is_none());
    }

    #[test]
    fn test_pressure_data_creation() {
        let data = PressureData {
            pressure: 1013.25,
            timestamp: 1234567890,
        };
        assert_eq!(data.pressure, 1013.25);
    }

    #[test]
    fn test_temperature_data_creation() {
        let data = TemperatureData {
            temperature: 22.5,
            timestamp: 1234567890,
        };
        assert_eq!(data.temperature, 22.5);
    }

    #[test]
    fn test_wifi_network_creation() {
        let network = WifiNetwork {
            ssid: "MyNetwork".to_string(),
            bssid: "00:11:22:33:44:55".to_string(),
            signal_strength: -50,
            frequency: 2412,
            security: "WPA2".to_string(),
        };
        assert_eq!(network.ssid, "MyNetwork");
        assert_eq!(network.signal_strength, -50);
    }

    #[test]
    fn test_sensor_error_not_available() {
        let error = SensorError::NotAvailable("Barometer not found".to_string());
        let msg = format!("{}", error);
        assert!(msg.contains("not available"));
        assert!(msg.contains("Barometer"));
    }

    #[test]
    fn test_sensor_error_permission_denied() {
        let error = SensorError::PermissionDenied("Location permission required".to_string());
        let msg = format!("{}", error);
        assert!(msg.contains("Permission denied"));
    }

    #[test]
    fn test_sensor_error_display() {
        let errors = vec![
            SensorError::NotAvailable("test".to_string()),
            SensorError::PermissionDenied("test".to_string()),
            SensorError::HardwareError("test".to_string()),
            SensorError::PluginError("test".to_string()),
            SensorError::DataError("test".to_string()),
        ];

        for error in errors {
            let msg = format!("{}", error);
            assert!(!msg.is_empty());
        }
    }
}
