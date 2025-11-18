// Example: Basic usage of the Rust Mobile Sensor POC library
//
// This example demonstrates how to use the data models, mock data providers,
// and service layer in your own code.

use mobile_poc_core::mocks::*;
use mobile_poc_core::{calculate_heading, get_cardinal_direction, SensorService};

fn main() {
    println!("=== Rust Mobile Sensor POC - Example Usage ===\n");

    // Create a sensor service instance
    let service = SensorService::new();

    // Example 1: Working with Accelerometer Data
    println!("1. Accelerometer Example:");
    println!("   ----------------------");
    let accel_data = mock_accelerometer_at_rest();
    println!(
        "   Raw Data: X={}, Y={}, Z={}",
        accel_data.x, accel_data.y, accel_data.z
    );
    println!(
        "   Formatted: {}",
        service.format_accelerometer(&accel_data)
    );
    println!(
        "   Magnitude: {:.2} m/s²",
        service.calculate_acceleration_magnitude(&accel_data)
    );

    match service.validate_accelerometer(&accel_data) {
        Ok(_) => println!("   ✓ Data is valid\n"),
        Err(e) => println!("   ✗ Error: {}\n", e),
    }

    // Example 2: Working with Magnetometer Data
    println!("2. Magnetometer Example:");
    println!("   ----------------------");
    let mag_data = mock_magnetometer_southwest();
    println!(
        "   Raw Data: X={}, Y={}, Z={}",
        mag_data.x, mag_data.y, mag_data.z
    );

    // Calculate heading from X/Y components
    let heading = calculate_heading(mag_data.x, mag_data.y);
    let direction = get_cardinal_direction(heading);
    println!("   Calculated Heading: {:.1}° ({})", heading, direction);
    println!("   Formatted: {}\n", service.format_heading(&mag_data));

    // Example 3: Working with GPS Data
    println!("3. GPS Example:");
    println!("   -----------");
    let gps_data = mock_gps_san_francisco();
    println!("   Location: {}, {}", gps_data.latitude, gps_data.longitude);
    println!("   Formatted: {}", service.format_gps(&gps_data));

    match service.validate_gps(&gps_data) {
        Ok(_) => println!("   ✓ Coordinates are valid"),
        Err(e) => println!("   ✗ Error: {}", e),
    }

    if let Some(alt) = gps_data.altitude {
        println!("   Altitude: {} m", alt);
    }
    println!("   Accuracy: ±{} m\n", gps_data.accuracy);

    // Example 4: Working with Pressure Data
    println!("4. Pressure Example:");
    println!("   -----------------");
    let pressure_data = mock_pressure_sea_level();
    println!("   Raw Data: {} hPa", pressure_data.pressure);
    println!("   Formatted: {}", service.format_pressure(&pressure_data));

    match service.validate_pressure(&pressure_data) {
        Ok(_) => println!("   ✓ Pressure is valid\n"),
        Err(e) => println!("   ✗ Error: {}\n", e),
    }

    // Example 5: Working with Temperature Data
    println!("5. Temperature Example:");
    println!("   --------------------");
    let temp_data = mock_temperature_comfortable();
    println!("   Raw Data: {}°C", temp_data.temperature);
    println!("   Formatted: {}", service.format_temperature(&temp_data));

    match service.validate_temperature(&temp_data) {
        Ok(_) => println!("   ✓ Temperature is valid\n"),
        Err(e) => println!("   ✗ Error: {}\n", e),
    }

    // Example 6: Working with WiFi Networks
    println!("6. WiFi Scanner Example:");
    println!("   ---------------------");
    let mut wifi_networks = mock_wifi_networks();
    println!("   Found {} networks", wifi_networks.len());

    // Sort by signal strength
    service.sort_wifi_by_signal(&mut wifi_networks);

    for network in &wifi_networks {
        let signal_desc = service.get_signal_description(network.signal_strength);
        println!(
            "   • {} - {} ({} dBm)",
            network.ssid, signal_desc, network.signal_strength
        );
    }
    println!();

    // Example 7: Serialization to JSON
    println!("7. JSON Serialization Example:");
    println!("   ---------------------------");
    let json = serde_json::to_string_pretty(&accel_data).expect("Failed to serialize");
    println!("   Accelerometer as JSON:");
    println!("{}\n", json);

    // Example 8: Error Handling
    println!("8. Error Handling Example:");
    println!("   -----------------------");
    use mobile_poc_core::models::{AccelerometerData, SensorError};

    let invalid_accel = AccelerometerData {
        x: 100.0, // Invalid: too high
        y: 0.0,
        z: 0.0,
        timestamp: 0,
        accuracy: 3,
    };

    match service.validate_accelerometer(&invalid_accel) {
        Ok(_) => println!("   Unexpected: data is valid"),
        Err(SensorError::DataError(msg)) => {
            println!("   ✓ Caught expected error: {}", msg);
        }
        Err(e) => println!("   Unexpected error type: {}", e),
    }

    println!("\n=== Example Complete ===");
}
