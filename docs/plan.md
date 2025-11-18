# Implementation Plan
# Rust Mobile Sensor POC

**Version:** 1.0
**Date:** 2025-11-18
**Status:** Active

## 1. Project Overview

### 1.1 Objective
Build a proof-of-concept Android application using Rust that demonstrates sensor access and data visualization.

### 1.2 Approach
- Use Tauri 2.0 framework for cross-platform mobile development
- Implement Test-Driven Development (TDD) methodology
- Follow quality-first approach with pre-commit hooks
- Iterative development with continuous testing

### 1.3 Timeline
**Estimated Duration:** 5 days
**Target MVP Date:** Day 5

## 2. Development Phases

### Phase 1: Environment Setup (Day 1)
**Duration:** 4-6 hours
**Status:** Not Started

#### Tasks:
1. **Install Dependencies**
   - [ ] Install Rust (latest stable)
   - [ ] Install Android Studio
   - [ ] Install Android SDK and NDK
   - [ ] Install cargo-mobile2
   - [ ] Add Android Rust targets

2. **Initialize Project**
   - [ ] Create new Tauri project: `cargo create-tauri-app`
   - [ ] Configure for mobile: `cargo mobile init`
   - [ ] Verify build: `cargo android build`
   - [ ] Test on emulator: `cargo android run`

3. **Setup Development Tools**
   - [ ] Configure IDE (VS Code or IntelliJ)
   - [ ] Install Rust analyzer
   - [ ] Install Android tools
   - [ ] Setup emulator or test device

4. **Initialize Git Workflow**
   - [ ] Verify git repository
   - [ ] Create .gitignore (Rust, Android, IDE files)
   - [ ] Initial commit with project structure

**Deliverable:** Working Tauri project that builds and runs on Android

---

### Phase 2: Core Infrastructure (Day 1-2)
**Duration:** 6-8 hours
**Status:** Not Started

#### Tasks:
1. **Project Structure**
   - [ ] Create Rust module structure
   - [ ] Set up data models (src/models/)
   - [ ] Create command handlers (src/commands/)
   - [ ] Setup services (src/services/)
   - [ ] Configure Cargo.toml dependencies

2. **Define Data Models**
   ```rust
   - [ ] AccelerometerData
   - [ ] MagnetometerData
   - [ ] LocationData
   - [ ] PressureData
   - [ ] TemperatureData
   - [ ] WifiNetwork
   - [ ] SensorError enum
   ```

3. **Create Test Framework**
   - [ ] Setup test module structure
   - [ ] Create mock data generators
   - [ ] Write test utilities
   - [ ] Configure test runner

4. **Basic UI Shell**
   - [ ] Create index.html structure
   - [ ] Add styles.css with design system
   - [ ] Create app.js with Tauri integration
   - [ ] Implement sensor card component template

**Deliverable:** Project structure with models, tests, and basic UI shell

---

### Phase 3: Sensor Implementation (Day 2-3)
**Duration:** 12-16 hours
**Status:** Not Started

#### Implementation Order (by priority):

#### 3.1 Accelerometer (Priority 1)
**Duration:** 2-3 hours

**TDD Steps:**
1. [ ] Write test: `test_accelerometer_data_model()`
2. [ ] Implement: AccelerometerData struct
3. [ ] Write test: `test_get_accelerometer_command()`
4. [ ] Create Kotlin plugin: AccelerometerPlugin.kt
5. [ ] Implement: Rust command handler
6. [ ] Write test: `test_accelerometer_ui_update()`
7. [ ] Implement: UI component
8. [ ] Verify: Run tests, manual test on device

#### 3.2 Magnetometer (Priority 2)
**Duration:** 2-3 hours

**TDD Steps:**
1. [ ] Write test: `test_magnetometer_data_model()`
2. [ ] Implement: MagnetometerData struct
3. [ ] Write test: `test_heading_calculation()`
4. [ ] Implement: Heading calculation logic
5. [ ] Write test: `test_get_magnetometer_command()`
6. [ ] Create Kotlin plugin: MagnetometerPlugin.kt
7. [ ] Implement: Rust command handler
8. [ ] Write test: `test_magnetometer_ui_update()`
9. [ ] Implement: UI component with compass
10. [ ] Verify: Run tests, manual test on device

#### 3.3 GPS Location (Priority 3)
**Duration:** 3-4 hours

**TDD Steps:**
1. [ ] Write test: `test_location_data_model()`
2. [ ] Implement: LocationData struct
3. [ ] Write test: `test_get_location_command()`
4. [ ] Create Kotlin plugin: GpsPlugin.kt
5. [ ] Implement: Permission handling
6. [ ] Implement: Rust command handler
7. [ ] Write test: `test_location_ui_update()`
8. [ ] Implement: UI component
9. [ ] Write test: `test_permission_denied_handling()`
10. [ ] Implement: Error handling UI
11. [ ] Verify: Run tests, manual test on device

#### 3.4 WiFi Scanner (Priority 4)
**Duration:** 3-4 hours

**TDD Steps:**
1. [ ] Write test: `test_wifi_network_model()`
2. [ ] Implement: WifiNetwork struct
3. [ ] Write test: `test_scan_wifi_command()`
4. [ ] Create Kotlin plugin: WifiScannerPlugin.kt
5. [ ] Implement: Permission handling
6. [ ] Implement: Rust command handler
7. [ ] Write test: `test_wifi_list_ui_update()`
8. [ ] Implement: UI component with list
9. [ ] Write test: `test_refresh_functionality()`
10. [ ] Implement: Refresh button
11. [ ] Verify: Run tests, manual test on device

#### 3.5 Pressure Sensor (Priority 5)
**Duration:** 1-2 hours

**TDD Steps:**
1. [ ] Write test: `test_pressure_data_model()`
2. [ ] Implement: PressureData struct
3. [ ] Write test: `test_get_pressure_command()`
4. [ ] Create Kotlin plugin: PressurePlugin.kt
5. [ ] Write test: `test_sensor_unavailable_handling()`
6. [ ] Implement: Rust command handler with availability check
7. [ ] Implement: UI component with fallback
8. [ ] Verify: Run tests, manual test on device

#### 3.6 Temperature Sensor (Priority 6)
**Duration:** 1-2 hours

**TDD Steps:**
1. [ ] Write test: `test_temperature_data_model()`
2. [ ] Implement: TemperatureData struct
3. [ ] Write test: `test_get_temperature_command()`
4. [ ] Create Kotlin plugin: TemperaturePlugin.kt
5. [ ] Write test: `test_sensor_unavailable_handling()`
6. [ ] Implement: Rust command handler with availability check
7. [ ] Implement: UI component with fallback message
8. [ ] Verify: Run tests, manual test on device

**Deliverable:** All 6 sensor types implemented with tests and UI

---

### Phase 4: Integration and Polish (Day 4)
**Duration:** 6-8 hours
**Status:** Not Started

#### Tasks:
1. **End-to-End Integration**
   - [ ] Test all sensors together
   - [ ] Verify data updates in real-time
   - [ ] Check memory usage and performance
   - [ ] Test on multiple devices/emulators

2. **Error Handling**
   - [ ] Test permission denials
   - [ ] Test sensor unavailability
   - [ ] Test network errors (WiFi)
   - [ ] Verify error messages are clear

3. **UI Polish**
   - [ ] Add loading states
   - [ ] Add error states
   - [ ] Improve visual design
   - [ ] Add animations
   - [ ] Verify responsive layout

4. **Performance Optimization**
   - [ ] Profile sensor update rate
   - [ ] Optimize UI rendering
   - [ ] Check battery impact
   - [ ] Minimize memory usage

**Deliverable:** Polished, integrated application ready for testing

---

### Phase 5: Quality Assurance and Delivery (Day 5)
**Duration:** 4-6 hours
**Status:** Not Started

#### Tasks:
1. **Setup Pre-Commit Hooks**
   - [ ] Install pre-commit framework
   - [ ] Configure rustfmt hook
   - [ ] Configure clippy hook
   - [ ] Configure test runner hook
   - [ ] Test hooks work correctly

2. **Code Quality**
   - [ ] Run rustfmt on all code
   - [ ] Fix all clippy warnings
   - [ ] Ensure all tests pass
   - [ ] Review code for issues
   - [ ] Check code coverage

3. **Documentation**
   - [ ] Update status.md with final status
   - [ ] Document any known issues
   - [ ] Update README.md with build instructions
   - [ ] Add code comments where needed
   - [ ] Create user guide (optional)

4. **Final Testing**
   - [ ] Manual acceptance testing
   - [ ] Test on physical device
   - [ ] Verify all requirements met
   - [ ] Check against acceptance criteria

5. **Delivery**
   - [ ] Create final commit
   - [ ] Push to repository
   - [ ] Tag release: v1.0.0-mvp
   - [ ] Generate APK for distribution
   - [ ] Update project status to "Complete"

**Deliverable:** MVP ready for acceptance testing

---

## 3. Technical Tasks Breakdown

### 3.1 Kotlin Plugin Development

Each sensor plugin follows this template:

```kotlin
// File: AccelerometerPlugin.kt
package com.mobile_poc.plugins

import android.app.Activity
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import com.google.gson.Gson

@TauriPlugin
class AccelerometerPlugin(private val activity: Activity) : SensorEventListener {

    private val sensorManager: SensorManager =
        activity.getSystemService(Context.SENSOR_SERVICE) as SensorManager

    private var sensor: Sensor? = null
    private var lastData: AccelerometerData? = null

    data class AccelerometerData(
        val x: Float,
        val y: Float,
        val z: Float,
        val timestamp: Long,
        val accuracy: Int
    )

    init {
        sensor = sensorManager.getDefaultSensor(Sensor.TYPE_ACCELEROMETER)
    }

    @Command
    fun isAvailable(): Boolean {
        return sensor != null
    }

    @Command
    fun getData(): String {
        return if (lastData != null) {
            Gson().toJson(lastData)
        } else {
            "{}"
        }
    }

    @Command
    fun startListening() {
        sensor?.let {
            sensorManager.registerListener(
                this, it, SensorManager.SENSOR_DELAY_NORMAL
            )
        }
    }

    @Command
    fun stopListening() {
        sensorManager.unregisterListener(this)
    }

    override fun onSensorChanged(event: SensorEvent?) {
        event?.let {
            lastData = AccelerometerData(
                x = it.values[0],
                y = it.values[1],
                z = it.values[2],
                timestamp = System.currentTimeMillis(),
                accuracy = it.accuracy
            )
        }
    }

    override fun onAccuracyChanged(sensor: Sensor?, accuracy: Int) {
        // Handle accuracy changes if needed
    }
}
```

**Tasks per plugin:**
1. [ ] Create plugin file
2. [ ] Implement SensorEventListener
3. [ ] Add Tauri commands
4. [ ] Handle permissions if needed
5. [ ] Test on device

### 3.2 Rust Command Handlers

Template for each sensor:

```rust
// File: src/commands/sensors.rs

use crate::models::AccelerometerData;
use tauri::command;

#[command]
pub async fn get_accelerometer_data() -> Result<AccelerometerData, String> {
    // Call Kotlin plugin
    // Parse JSON response
    // Return data or error
}

#[command]
pub async fn is_accelerometer_available() -> Result<bool, String> {
    // Call Kotlin plugin
    // Return availability
}
```

**Tasks per command:**
1. [ ] Define command function
2. [ ] Call mobile plugin
3. [ ] Parse response
4. [ ] Handle errors
5. [ ] Write tests

### 3.3 UI Components

Template for each sensor:

```javascript
// File: src/ui/components/AccelerometerCard.js

class AccelerometerCard {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.isAvailable = false;
        this.updateInterval = null;
    }

    async init() {
        this.isAvailable = await invoke('is_accelerometer_available');
        this.render();
        if (this.isAvailable) {
            this.startUpdates();
        }
    }

    async startUpdates() {
        this.updateInterval = setInterval(async () => {
            try {
                const data = await invoke('get_accelerometer_data');
                this.updateDisplay(data);
            } catch (error) {
                this.showError(error);
            }
        }, 100); // 10Hz update rate
    }

    updateDisplay(data) {
        // Update DOM with sensor data
    }

    render() {
        // Create HTML structure
    }
}
```

**Tasks per component:**
1. [ ] Create component class
2. [ ] Implement render method
3. [ ] Add update logic
4. [ ] Handle errors
5. [ ] Add to main app

---

## 4. Testing Strategy

### 4.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn test_sensor_error_handling() {
        // Test error cases
    }
}
```

**Coverage Goals:**
- Data models: 100%
- Business logic: > 80%
- Error handling: 100%
- Command handlers: > 70%

### 4.2 Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    #[tokio::test]
    async fn test_get_accelerometer_end_to_end() {
        // Test full flow from command to plugin
    }
}
```

### 4.3 Manual Testing

**Test Cases:**
1. [ ] All sensors display data correctly
2. [ ] Unavailable sensors show appropriate message
3. [ ] Permission requests work
4. [ ] Permission denials handled gracefully
5. [ ] App doesn't crash on sensor errors
6. [ ] UI updates in real-time
7. [ ] Memory usage is reasonable
8. [ ] App works on different screen sizes
9. [ ] WiFi scanning works
10. [ ] GPS location updates

---

## 5. Dependencies Installation

### 5.1 System Dependencies

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# cargo-mobile2
cargo install cargo-mobile2

# Tauri CLI
cargo install tauri-cli
```

### 5.2 Rust Dependencies (Cargo.toml)

```toml
[dependencies]
tauri = { version = "2.0", features = ["mobile"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
```

---

## 6. Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Tauri 2.0 issues | Have Dioxus as backup plan |
| Sensor not available | Implement graceful fallbacks |
| Permission complexities | Research Android docs thoroughly |
| Time overrun | Focus on MVP, defer nice-to-haves |
| Build issues | Document setup process carefully |

---

## 7. Success Criteria

MVP is complete when:
- [ ] All 6 sensor types implemented
- [ ] At least 4 sensors functional on test device
- [ ] All tests passing
- [ ] No clippy warnings
- [ ] Code formatted with rustfmt
- [ ] Pre-commit hooks working
- [ ] Documentation complete
- [ ] App builds and runs successfully
- [ ] Acceptance testing passed

---

## 8. Next Steps After MVP

Future enhancements (post-MVP):
1. Add data logging
2. Implement charts/graphs
3. Add export functionality
4. Support more sensors (gyroscope, light)
5. iOS support
6. Background monitoring
7. Notifications
8. Settings screen

---

## 9. Daily Progress Tracking

Progress should be tracked in `status.md`:
- Tasks completed
- Blockers encountered
- Decisions made
- Next day's plan

---

## 10. Resources

### Documentation:
- Tauri Docs: https://v2.tauri.app/
- Android Sensors: https://developer.android.com/develop/sensors-and-location/sensors
- Rust Book: https://doc.rust-lang.org/book/
- cargo-mobile2: https://github.com/tauri-apps/cargo-mobile2

### Community:
- Tauri Discord
- Rust Forums
- Stack Overflow

### Tools:
- VS Code + rust-analyzer
- Android Studio
- Chrome DevTools (chrome://inspect)
