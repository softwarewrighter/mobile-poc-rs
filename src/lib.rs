// Core library for Rust Mobile Sensor POC
//
// This library provides data models and business logic for accessing
// mobile device sensors including accelerometer, magnetometer, GPS,
// pressure, temperature, and WiFi scanning.

pub mod models;

// Re-export main types for convenience
pub use models::{
    AccelerometerData, GpsData, MagnetometerData, PressureData, SensorError, TemperatureData,
    WifiNetwork,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // Placeholder test
        assert_eq!(2 + 2, 4);
    }
}
