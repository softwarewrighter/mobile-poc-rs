// Service layer for sensor management
//
// This module provides a high-level API for managing sensor access,
// data validation, and formatting.

use crate::models::*;

/// High-level sensor service for managing sensor data
///
/// This service provides convenience methods for accessing sensor data,
/// validating it, and formatting it for display.
pub struct SensorService;

impl SensorService {
    /// Create a new sensor service instance
    pub fn new() -> Self {
        SensorService
    }

    /// Validate accelerometer data is within reasonable bounds
    ///
    /// Checks that acceleration values are within expected ranges
    /// (typically -20 to +20 m/s² for mobile devices)
    pub fn validate_accelerometer(&self, data: &AccelerometerData) -> Result<(), SensorError> {
        const MAX_ACCEL: f32 = 20.0;
        if data.x.abs() > MAX_ACCEL || data.y.abs() > MAX_ACCEL || data.z.abs() > MAX_ACCEL {
            return Err(SensorError::DataError(
                "Acceleration value out of range".to_string(),
            ));
        }
        Ok(())
    }

    /// Format accelerometer data for display
    pub fn format_accelerometer(&self, data: &AccelerometerData) -> String {
        format!(
            "X: {:.2} m/s², Y: {:.2} m/s², Z: {:.2} m/s²",
            data.x, data.y, data.z
        )
    }

    /// Calculate total acceleration magnitude
    pub fn calculate_acceleration_magnitude(&self, data: &AccelerometerData) -> f32 {
        (data.x * data.x + data.y * data.y + data.z * data.z).sqrt()
    }

    /// Validate GPS coordinates are within valid ranges
    pub fn validate_gps(&self, data: &GpsData) -> Result<(), SensorError> {
        if !(-90.0..=90.0).contains(&data.latitude) {
            return Err(SensorError::DataError("Invalid latitude".to_string()));
        }
        if !(-180.0..=180.0).contains(&data.longitude) {
            return Err(SensorError::DataError("Invalid longitude".to_string()));
        }
        Ok(())
    }

    /// Format GPS coordinates for display
    pub fn format_gps(&self, data: &GpsData) -> String {
        let lat_dir = if data.latitude >= 0.0 { "N" } else { "S" };
        let lon_dir = if data.longitude >= 0.0 { "E" } else { "W" };
        format!(
            "{:.4}° {}, {:.4}° {}",
            data.latitude.abs(),
            lat_dir,
            data.longitude.abs(),
            lon_dir
        )
    }

    /// Format magnetometer heading with cardinal direction
    pub fn format_heading(&self, data: &MagnetometerData) -> String {
        let direction = get_cardinal_direction(data.heading);
        format!("{:.1}° ({})", data.heading, direction)
    }

    /// Validate pressure data is within reasonable range
    pub fn validate_pressure(&self, data: &PressureData) -> Result<(), SensorError> {
        // Typical range: 870 hPa (top of Mt. Everest) to 1084 hPa (record high)
        if !(800.0..=1100.0).contains(&data.pressure) {
            return Err(SensorError::DataError("Pressure out of range".to_string()));
        }
        Ok(())
    }

    /// Format pressure with description
    pub fn format_pressure(&self, data: &PressureData) -> String {
        let description = match data.pressure {
            p if p < 1000.0 => "(Low)",
            p if p > 1020.0 => "(High)",
            _ => "(Normal)",
        };
        format!("{:.2} hPa {}", data.pressure, description)
    }

    /// Validate temperature is within reasonable range
    pub fn validate_temperature(&self, data: &TemperatureData) -> Result<(), SensorError> {
        // Typical range: -40°C to +85°C for electronic sensors
        if !(-50.0..=100.0).contains(&data.temperature) {
            return Err(SensorError::DataError(
                "Temperature out of range".to_string(),
            ));
        }
        Ok(())
    }

    /// Format temperature in Celsius and Fahrenheit
    pub fn format_temperature(&self, data: &TemperatureData) -> String {
        let fahrenheit = data.temperature * 9.0 / 5.0 + 32.0;
        format!("{:.1}°C ({:.1}°F)", data.temperature, fahrenheit)
    }

    /// Sort WiFi networks by signal strength (strongest first)
    pub fn sort_wifi_by_signal(&self, networks: &mut [WifiNetwork]) {
        networks.sort_by(|a, b| b.signal_strength.cmp(&a.signal_strength));
    }

    /// Get signal strength description
    pub fn get_signal_description(&self, rssi: i32) -> &'static str {
        match rssi {
            r if r >= -50 => "Excellent",
            r if r >= -60 => "Good",
            r if r >= -70 => "Fair",
            _ => "Weak",
        }
    }

    /// Format WiFi network for display
    pub fn format_wifi_network(&self, network: &WifiNetwork) -> String {
        let signal = self.get_signal_description(network.signal_strength);
        format!(
            "{} - {} ({} dBm) - {}",
            network.ssid, signal, network.signal_strength, network.security
        )
    }
}

impl Default for SensorService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::*;

    #[test]
    fn test_validate_accelerometer_valid() {
        let service = SensorService::new();
        let data = mock_accelerometer_at_rest();
        assert!(service.validate_accelerometer(&data).is_ok());
    }

    #[test]
    fn test_validate_accelerometer_invalid() {
        let service = SensorService::new();
        let data = AccelerometerData {
            x: 100.0, // Way too high
            y: 0.0,
            z: 0.0,
            timestamp: 0,
            accuracy: 3,
        };
        assert!(service.validate_accelerometer(&data).is_err());
    }

    #[test]
    fn test_format_accelerometer() {
        let service = SensorService::new();
        let data = mock_accelerometer_at_rest();
        let formatted = service.format_accelerometer(&data);
        assert!(formatted.contains("9.81"));
        assert!(formatted.contains("m/s²"));
    }

    #[test]
    fn test_calculate_acceleration_magnitude() {
        let service = SensorService::new();
        let data = AccelerometerData {
            x: 3.0,
            y: 4.0,
            z: 0.0,
            timestamp: 0,
            accuracy: 3,
        };
        let magnitude = service.calculate_acceleration_magnitude(&data);
        assert!((magnitude - 5.0).abs() < 0.01); // 3-4-5 triangle
    }

    #[test]
    fn test_validate_gps_valid() {
        let service = SensorService::new();
        let data = mock_gps_san_francisco();
        assert!(service.validate_gps(&data).is_ok());
    }

    #[test]
    fn test_validate_gps_invalid_latitude() {
        let service = SensorService::new();
        let data = GpsData {
            latitude: 95.0, // Invalid
            longitude: 0.0,
            altitude: None,
            accuracy: 10.0,
            speed: None,
            timestamp: 0,
        };
        assert!(service.validate_gps(&data).is_err());
    }

    #[test]
    fn test_format_gps() {
        let service = SensorService::new();
        let data = mock_gps_san_francisco();
        let formatted = service.format_gps(&data);
        assert!(formatted.contains("N"));
        assert!(formatted.contains("W"));
    }

    #[test]
    fn test_format_heading() {
        let service = SensorService::new();
        let data = mock_magnetometer_north();
        let formatted = service.format_heading(&data);
        assert!(formatted.contains("0.0"));
        assert!(formatted.contains("N"));
    }

    #[test]
    fn test_validate_pressure_valid() {
        let service = SensorService::new();
        let data = mock_pressure_sea_level();
        assert!(service.validate_pressure(&data).is_ok());
    }

    #[test]
    fn test_format_pressure() {
        let service = SensorService::new();
        let data = mock_pressure_sea_level();
        let formatted = service.format_pressure(&data);
        assert!(formatted.contains("1013.25"));
        assert!(formatted.contains("hPa"));
    }

    #[test]
    fn test_validate_temperature_valid() {
        let service = SensorService::new();
        let data = mock_temperature_comfortable();
        assert!(service.validate_temperature(&data).is_ok());
    }

    #[test]
    fn test_format_temperature() {
        let service = SensorService::new();
        let data = TemperatureData {
            temperature: 0.0,
            timestamp: 0,
        };
        let formatted = service.format_temperature(&data);
        assert!(formatted.contains("0.0°C"));
        assert!(formatted.contains("32.0°F"));
    }

    #[test]
    fn test_sort_wifi_by_signal() {
        let service = SensorService::new();
        let mut networks = mock_wifi_networks();
        service.sort_wifi_by_signal(&mut networks);
        // Should be sorted strongest first
        assert!(networks[0].signal_strength >= networks[1].signal_strength);
        assert!(networks[1].signal_strength >= networks[2].signal_strength);
    }

    #[test]
    fn test_get_signal_description() {
        let service = SensorService::new();
        assert_eq!(service.get_signal_description(-45), "Excellent");
        assert_eq!(service.get_signal_description(-55), "Good");
        assert_eq!(service.get_signal_description(-65), "Fair");
        assert_eq!(service.get_signal_description(-80), "Weak");
    }

    #[test]
    fn test_format_wifi_network() {
        let service = SensorService::new();
        let network = WifiNetwork {
            ssid: "TestWiFi".to_string(),
            bssid: "00:11:22:33:44:55".to_string(),
            signal_strength: -45,
            frequency: 2412,
            security: "WPA2".to_string(),
        };
        let formatted = service.format_wifi_network(&network);
        assert!(formatted.contains("TestWiFi"));
        assert!(formatted.contains("Excellent"));
        assert!(formatted.contains("WPA2"));
    }
}
