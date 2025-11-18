# Research: Android Development with Rust

**Date:** 2025-11-18
**Status:** Complete

## Executive Summary

This research investigates approaches for building an Android mobile application using Rust. The goal is to create a sensor-based demo app that displays data from various device sensors (temperature, pressure, accelerometer, magnetic direction, GPS, WiFi SSIDs).

## Key Findings

### 1. Rust on Android - Current State (2025)

- **Official Support**: Android platform now officially supports Rust for native OS components
- **Major Milestone**: 2025 was the first year more lines of Rust code were added to Android than C++ code
- **Security Benefits**: 1000x reduction in memory safety vulnerability density compared to C/C++
- **Production Ready**: Android 6.12 kernel includes Rust support with first production Rust driver
- **Primary Use**: Rust is primarily used for system-level components, libraries, and performance-critical code

### 2. Framework Options

#### Option A: Tauri 2.0 (Recommended)
**Website:** https://v2.tauri.app/

**Pros:**
- Full Android and iOS support from single codebase
- Uses web technologies for UI (HTML/CSS/JS) with Rust backend
- Native plugin system using Kotlin/Swift for platform-specific features
- Tool: `cargo-mobile2` for easy Android setup
- Active development and excellent documentation
- Built-in debugging with Chrome DevTools (chrome://inspect)
- Commands: `cargo android run` for development

**Cons:**
- WebView-based (not fully native UI)
- Requires knowledge of web technologies for UI

**Best For:** Our use case - needs native sensor access with modern UI

#### Option B: Dioxus
**Website:** https://dioxuslabs.com/

**Pros:**
- Fullstack cross-platform (Web, Desktop, Mobile, SSR)
- Pure Rust with React-like syntax
- Built-in CLI: `dx serve --platform android`
- Mobile as first-class target
- WebView or WGPU rendering

**Cons:**
- Larger APK size (~15MB initial)
- Newer framework, smaller community than Tauri
- WebView-based for mobile

**Best For:** Pure Rust developers who prefer React-like development

#### Option C: Slint
**Website:** https://slint.dev/

**Pros:**
- Only Rust GUI toolkit with official Android support
- Pure Rust, native rendering
- Supports embedded devices to mobile/desktop
- Declarative UI language

**Cons:**
- More focused on embedded/desktop use cases
- Less documentation for Android sensor integration
- Smaller community

**Best For:** Embedded systems or pure native Rust GUI

#### Option D: Yew + WASM
**Website:** https://yew.rs/

**Pros:**
- Mature Rust/WASM framework
- Component-based (React-like)
- Good for Progressive Web Apps (PWAs)

**Cons:**
- Primarily for web apps, not native mobile
- Limited native sensor access
- Requires service workers for offline support

**Best For:** Web applications, not ideal for native mobile sensors

### 3. Sensor Access on Android

#### Available Sensors:
- **Common**: Accelerometer, Magnetometer, GPS
- **Less Common**: Barometer, Thermometer
- **WiFi**: Network scanning APIs
- **Location**: GPS/GNSS with various providers

#### Rust Libraries:
- `android_sensor-sys`: Rust bindings to Android Sensor Library (older, from 2017)
- `rust-i2cdev`: Examples for hardware sensor access
- **Recommended Approach**: Use Tauri plugins written in Kotlin to access Android SDK sensor APIs

#### Android API Restrictions:
- Android 9+ (API 28): Background apps have restrictions on continuous sensors
- Sensors like accelerometer/gyroscope don't receive events when app is in background

### 4. Development Workflow

#### Recommended Setup (Tauri 2.0):
```bash
# Install Rust targets for Android
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# Install cargo-mobile2
cargo install cargo-mobile2

# Initialize Android project
cargo mobile init

# Run on device/emulator
cargo android run
```

#### Required Tools:
- Android Studio (for SDK and emulator)
- Android SDK and NDK
- Rust with Android targets
- cargo-mobile2 or Tauri CLI

### 5. Best Practices

1. **Architecture**:
   - Separate Rust core logic from UI layer
   - Use native plugins for platform-specific features (sensors)
   - Keep UI responsive with async/await for sensor data

2. **Testing**:
   - Use TDD with Rust's built-in test framework
   - Mock sensor data for unit tests
   - Integration tests on actual devices/emulators

3. **Code Quality**:
   - Use `rustfmt` for formatting
   - Use `clippy` for linting
   - Pre-commit hooks for quality checks
   - Never disable linter warnings

4. **Security**:
   - Request minimum necessary permissions
   - Handle sensor data privacy
   - Validate all inputs from sensors

### 6. Hybrid Approach (Alternative)

For maximum native performance, consider:
- Core business logic in Rust (shared library)
- UI in Kotlin/Jetpack Compose
- FFI/JNI bindings using `uniffi` or `jni` crate
- More complex but fully native

## Recommendation

**Use Tauri 2.0** for this POC because:
1. Full Android support with easy setup
2. Web-based UI is quick to develop and iterate
3. Kotlin plugins provide full access to Android sensors
4. Active development and good documentation
5. Single codebase for future iOS support
6. Debugging tools built-in

## References

- [Android Rust Introduction](https://source.android.com/docs/setup/build/rust/building-rust-modules/overview)
- [Google Comprehensive Rust Android](https://google.github.io/comprehensive-rust/android.html)
- [Tauri 2.0 Documentation](https://v2.tauri.app/)
- [Dioxus Mobile Guide](https://dioxuslabs.com/learn/0.6/guides/mobile/)
- [Slint Android Support](https://slint.dev/blog/slint-1.5-released)
- [cargo-mobile2 GitHub](https://github.com/tauri-apps/cargo-mobile2)
- [Android Sensors Overview](https://developer.android.com/develop/sensors-and-location/sensors/sensors_overview)
- [Rust for Native Mobile Development](https://medium.com/@lipa_btc/rust-for-native-mobile-development-b8a4b82720bf)
- [Cross-Platform Development with Rust](https://www.rapidinnovation.io/post/cross-platform-development-with-rust-desktop-mobile-and-web)

## Next Steps

1. Set up Tauri 2.0 project structure
2. Configure Android development environment
3. Create basic app shell
4. Implement sensor plugins in Kotlin
5. Build UI for sensor displays
6. Implement TDD tests
7. Set up pre-commit hooks
