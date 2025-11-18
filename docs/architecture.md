# Architecture Document
# Rust Mobile Sensor POC

**Version:** 1.0
**Date:** 2025-11-18
**Status:** Draft

## 1. System Overview

This application is built using **Tauri 2.0**, a framework that enables building native mobile applications using Rust for the backend logic and web technologies (HTML/CSS/JavaScript) for the UI.

### 1.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Android Device                       │
│  ┌───────────────────────────────────────────────────┐  │
│  │              Tauri Application                     │  │
│  │  ┌─────────────────────────────────────────────┐  │  │
│  │  │          WebView (UI Layer)                 │  │  │
│  │  │  - HTML/CSS/JavaScript                      │  │  │
│  │  │  - Sensor data visualization                │  │  │
│  │  │  - User interaction handling                │  │  │
│  │  └─────────────────┬───────────────────────────┘  │  │
│  │                    │ Tauri IPC                    │  │
│  │  ┌─────────────────▼───────────────────────────┐  │  │
│  │  │      Rust Core (Business Logic)            │  │  │
│  │  │  - Command handlers                         │  │  │
│  │  │  - Data processing                          │  │  │
│  │  │  - State management                         │  │  │
│  │  │  - Sensor data models                       │  │  │
│  │  └─────────────────┬───────────────────────────┘  │  │
│  │                    │ Plugin Interface            │  │
│  │  ┌─────────────────▼───────────────────────────┐  │  │
│  │  │    Tauri Mobile Plugins (Kotlin)           │  │  │
│  │  │  - Accelerometer Plugin                     │  │  │
│  │  │  - Magnetometer Plugin                      │  │  │
│  │  │  - GPS Plugin                               │  │  │
│  │  │  - Pressure Plugin                          │  │  │
│  │  │  - Temperature Plugin                       │  │  │
│  │  │  - WiFi Scanner Plugin                      │  │  │
│  │  └─────────────────┬───────────────────────────┘  │  │
│  │                    │ Android SDK                  │  │
│  │  ┌─────────────────▼───────────────────────────┐  │  │
│  │  │         Android System Services             │  │  │
│  │  │  - SensorManager                            │  │  │
│  │  │  - LocationManager                          │  │  │
│  │  │  - WifiManager                              │  │  │
│  │  └─────────────────┬───────────────────────────┘  │  │
│  └────────────────────┼─────────────────────────────┘  │
│                       │                                 │
│  ┌────────────────────▼───────────────────────────┐    │
│  │           Hardware Sensors                     │    │
│  │  - Accelerometer  - Barometer                  │    │
│  │  - Magnetometer   - Temperature                │    │
│  │  - GPS            - WiFi Radio                 │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## 2. Component Architecture

### 2.1 Layer Descriptions

#### Layer 1: UI Layer (WebView)
**Technology:** HTML, CSS, JavaScript
**Responsibilities:**
- Render sensor data in user-friendly format
- Handle user interactions (navigation, refresh, etc.)
- Display sensor status and availability
- Provide responsive layout for different screen sizes

**Key Components:**
- `index.html` - Main application shell
- `sensor-view.js` - Sensor data display components
- `styles.css` - Application styling
- `app.js` - Application logic and Tauri communication

#### Layer 2: Rust Core
**Technology:** Rust
**Responsibilities:**
- Handle IPC commands from UI
- Process and validate sensor data
- Manage application state
- Coordinate sensor plugin calls
- Implement business logic
- Data transformation and formatting

**Key Modules:**
```rust
src/
├── main.rs           # Application entry point
├── commands/         # Tauri command handlers
│   ├── mod.rs
│   ├── sensors.rs    # Sensor-related commands
│   └── wifi.rs       # WiFi-related commands
├── models/           # Data models
│   ├── mod.rs
│   ├── sensor_data.rs
│   └── location.rs
├── services/         # Business logic
│   ├── mod.rs
│   └── sensor_service.rs
└── utils/            # Utility functions
    ├── mod.rs
    └── formatters.rs
```

#### Layer 3: Mobile Plugins (Kotlin)
**Technology:** Kotlin
**Responsibilities:**
- Interface with Android SDK
- Register sensor listeners
- Handle Android permissions
- Convert Android data types to Rust-compatible types
- Manage sensor lifecycle

**Key Plugins:**
```
src-tauri/gen/android/app/src/main/java/com/mobile_poc/
├── plugins/
│   ├── AccelerometerPlugin.kt
│   ├── MagnetometerPlugin.kt
│   ├── GpsPlugin.kt
│   ├── PressurePlugin.kt
│   ├── TemperaturePlugin.kt
│   └── WifiScannerPlugin.kt
└── MainActivity.kt
```

#### Layer 4: Android System Services
**Technology:** Android SDK APIs
**Components:**
- `SensorManager` - Access to hardware sensors
- `LocationManager` - GPS and location services
- `WifiManager` - WiFi network scanning
- Permission system

## 3. Data Flow

### 3.1 Sensor Data Flow

```
Hardware Sensor
      ↓
Android SensorManager
      ↓
Kotlin Plugin (Listener)
      ↓
Plugin -> Rust Interface (via JNI)
      ↓
Rust Command Handler
      ↓
Data Processing/Validation
      ↓
Tauri IPC (Event Emission)
      ↓
JavaScript Event Handler
      ↓
Update UI (DOM)
```

### 3.2 User Interaction Flow

```
User Action (UI)
      ↓
JavaScript Event Handler
      ↓
Tauri Invoke Command
      ↓
Rust Command Handler
      ↓
Call Plugin Method
      ↓
Kotlin Plugin Execution
      ↓
Return Result
      ↓
Rust Processing
      ↓
Return to JavaScript
      ↓
Update UI
```

## 4. Key Design Patterns

### 4.1 Observer Pattern (Sensor Updates)
- Kotlin plugins register as sensor listeners
- Emit events when sensor data changes
- UI subscribes to data update events
- Decouples sensor reading from display

### 4.2 Command Pattern (Tauri Commands)
- UI invokes commands on Rust backend
- Each command is a discrete operation
- Results returned via promises
- Supports async operations

### 4.3 Repository Pattern (Optional for future)
- Abstract sensor data access
- Easy to mock for testing
- Swap implementations if needed

## 5. API Design

### 5.1 Tauri Commands (Rust → Exposed to JS)

```rust
// Get accelerometer data
#[tauri::command]
async fn get_accelerometer_data() -> Result<AccelerometerData, String>

// Get magnetometer data with heading
#[tauri::command]
async fn get_magnetometer_data() -> Result<MagnetometerData, String>

// Get current location
#[tauri::command]
async fn get_location() -> Result<LocationData, String>

// Get pressure sensor data
#[tauri::command]
async fn get_pressure() -> Result<PressureData, String>

// Get temperature sensor data
#[tauri::command]
async fn get_temperature() -> Result<TemperatureData, String>

// Scan for WiFi networks
#[tauri::command]
async fn scan_wifi() -> Result<Vec<WifiNetwork>, String>

// Check if sensor is available
#[tauri::command]
async fn is_sensor_available(sensor_type: String) -> Result<bool, String>
```

### 5.2 Data Models

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct AccelerometerData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub timestamp: i64,
    pub accuracy: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MagnetometerData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub heading: f32,  // Degrees from magnetic north
    pub timestamp: i64,
    pub accuracy: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub accuracy: f32,
    pub speed: Option<f32>,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PressureData {
    pub pressure: f32,  // hPa
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemperatureData {
    pub temperature: f32,  // Celsius
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub bssid: String,
    pub signal_strength: i32,  // dBm
    pub frequency: i32,        // MHz
    pub security: String,
}
```

### 5.3 Plugin Interface (Kotlin)

```kotlin
interface SensorPlugin {
    fun isAvailable(): Boolean
    fun getData(): String  // JSON string
    fun startListening()
    fun stopListening()
}

class AccelerometerPlugin(private val activity: Activity) : SensorPlugin {
    private val sensorManager: SensorManager = ...
    private var sensor: Sensor? = null

    override fun isAvailable(): Boolean {
        sensor = sensorManager.getDefaultSensor(Sensor.TYPE_ACCELEROMETER)
        return sensor != null
    }

    override fun getData(): String {
        // Return JSON string of current sensor data
    }
}
```

## 6. State Management

### 6.1 Application State
- Managed in Rust using Tauri's state management
- Thread-safe using Mutex/RwLock when necessary
- State includes:
  - Current sensor values
  - Sensor availability status
  - Permission status
  - Error states

### 6.2 State Structure

```rust
pub struct AppState {
    pub sensors_available: HashMap<String, bool>,
    pub last_error: Option<String>,
}

// Managed by Tauri
fn main() {
    tauri::Builder::default()
        .manage(AppState {
            sensors_available: HashMap::new(),
            last_error: None,
        })
        .invoke_handler(tauri::generate_handler![
            get_accelerometer_data,
            get_magnetometer_data,
            // ... other commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## 7. Security Considerations

### 7.1 Permissions
- Request permissions at runtime (Android 6+)
- Handle permission denials gracefully
- Use minimum necessary permissions
- Explain permission usage to user

### 7.2 Data Privacy
- Sensor data is not stored persistently
- No data sent to external servers
- Location data only used for display
- WiFi SSIDs only shown, not logged

### 7.3 Input Validation
- Validate all data from sensors
- Handle null/undefined values
- Bounds checking on sensor values
- Sanitize display strings

## 8. Performance Considerations

### 8.1 Sensor Update Rate
- Throttle sensor updates to reasonable rate (e.g., 10Hz)
- Use appropriate sensor delay constant
  - `SENSOR_DELAY_NORMAL` for most sensors
  - `SENSOR_DELAY_UI` for less critical updates
- Batch updates when possible

### 8.2 UI Rendering
- Use requestAnimationFrame for smooth updates
- Debounce expensive operations
- Lazy load sensor views
- Virtual scrolling for WiFi list

### 8.3 Memory Management
- Unregister sensor listeners when not needed
- Clean up resources on component unmount
- Limit stored sensor history
- Use efficient data structures

## 9. Error Handling

### 9.1 Error Categories
1. **Sensor Not Available** - Device lacks sensor
2. **Permission Denied** - User denied permission
3. **Sensor Error** - Hardware/driver issue
4. **Plugin Error** - Communication failure
5. **Data Error** - Invalid data received

### 9.2 Error Handling Strategy
```rust
pub enum SensorError {
    NotAvailable(String),
    PermissionDenied(String),
    HardwareError(String),
    PluginError(String),
    DataError(String),
}

impl From<SensorError> for String {
    fn from(error: SensorError) -> Self {
        match error {
            SensorError::NotAvailable(msg) => format!("Sensor not available: {}", msg),
            SensorError::PermissionDenied(msg) => format!("Permission denied: {}", msg),
            // ... other cases
        }
    }
}
```

### 9.3 Fallback Behavior
- Show user-friendly error messages
- Provide retry mechanism for transient errors
- Gracefully degrade (hide unavailable sensors)
- Log errors for debugging

## 10. Testing Strategy

### 10.1 Unit Tests
- Test Rust business logic
- Test data transformations
- Test error handling
- Mock plugin responses

### 10.2 Integration Tests
- Test Tauri command handlers
- Test plugin integration
- Test end-to-end data flow

### 10.3 Testing Challenges
- Sensors require physical device or advanced emulation
- Mock sensor data for automated tests
- Manual testing on real devices required for validation

## 11. Build and Deployment

### 11.1 Build Process
```bash
# Development build
cargo android build

# Release build
cargo android build --release

# Run on device
cargo android run
```

### 11.2 Artifacts
- APK file: `src-tauri/gen/android/app/build/outputs/apk/`
- Build logs
- Test reports

### 11.3 Configuration
- `Cargo.toml` - Rust dependencies
- `tauri.conf.json` - Tauri configuration
- `AndroidManifest.xml` - Android permissions and metadata
- `build.gradle` - Android build configuration

## 12. Future Extensibility

### 12.1 Potential Enhancements
- Add more sensor types (gyroscope, light, proximity)
- Data logging and persistence
- Historical data visualization
- Export functionality
- Background sensor monitoring
- Notification support

### 12.2 Architectural Support
- Plugin system allows easy addition of new sensors
- Modular design supports feature additions
- Separation of concerns enables independent updates
- Data models can be extended without breaking existing code

## 13. Technology Stack Summary

| Layer | Technology | Purpose |
|-------|------------|---------|
| UI | HTML/CSS/JavaScript | User interface |
| Framework | Tauri 2.0 | Mobile app framework |
| Backend | Rust | Business logic |
| Mobile Plugin | Kotlin | Android sensor access |
| Build | cargo-mobile2 | Android build tooling |
| Platform | Android 8.0+ | Target platform |

## 14. Dependencies

### 14.1 Rust Dependencies
```toml
[dependencies]
tauri = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 14.2 Android Dependencies
- Android SDK API 26+
- Kotlin standard library
- AndroidX libraries

## 15. Glossary

- **Tauri**: Cross-platform app framework using web technologies
- **IPC**: Inter-Process Communication
- **JNI**: Java Native Interface
- **WebView**: Component for rendering web content in native apps
- **SensorManager**: Android system service for sensor access
- **Plugin**: Native code module for platform-specific functionality
