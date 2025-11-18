# Development Process
# Rust Mobile Sensor POC

**Version:** 1.0
**Date:** 2025-11-18
**Status:** Active

## 1. Development Methodology

### 1.1 Test-Driven Development (TDD)

This project follows the **Red-Green-Refactor** TDD cycle:

```
┌─────────────┐
│   RED       │  Write a failing test
│   (Test)    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   GREEN     │  Write minimal code to pass
│   (Code)    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  REFACTOR   │  Clean up and optimize
│  (Improve)  │
└──────┬──────┘
       │
       ▼
    (Repeat)
```

#### TDD Rules:
1. **Write test first** - No production code without a failing test
2. **Minimal code** - Write only enough code to make the test pass
3. **Refactor** - Clean up code while keeping tests green
4. **One test at a time** - Focus on single functionality
5. **All tests must pass** - Before moving to next feature

### 1.2 TDD Workflow Example

**Feature**: Implement accelerometer sensor

**Step 1 - RED**: Write failing test
```rust
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
```
Run: `cargo test` → **FAIL** (struct doesn't exist)

**Step 2 - GREEN**: Implement minimal code
```rust
#[derive(Debug, Clone)]
pub struct AccelerometerData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub timestamp: i64,
    pub accuracy: i32,
}
```
Run: `cargo test` → **PASS**

**Step 3 - REFACTOR**: Add Serialize/Deserialize
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelerometerData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub timestamp: i64,
    pub accuracy: i32,
}
```
Run: `cargo test` → **PASS**

**Repeat** for next feature (command handler, UI, etc.)

---

## 2. Code Quality Process

### 2.1 Quality Gates

Every commit must pass these quality gates:

1. **Format Check** - Code is formatted with `rustfmt`
2. **Lint Check** - No `clippy` warnings
3. **Test Check** - All tests pass
4. **Build Check** - Code compiles successfully
5. **Git Check** - .gitignore is correct

### 2.2 Pre-Commit Hook Configuration

**Install pre-commit:**
```bash
pip install pre-commit
# or
brew install pre-commit
```

**Create `.pre-commit-config.yaml`:**
```yaml
repos:
  - repo: local
    hooks:
      # Rust formatting
      - id: rustfmt
        name: rustfmt
        entry: cargo fmt --
        language: system
        types: [rust]
        pass_filenames: false

      # Rust linting
      - id: clippy
        name: clippy
        entry: cargo clippy --all-targets --all-features -- -D warnings
        language: system
        types: [rust]
        pass_filenames: false

      # Run tests
      - id: cargo-test
        name: cargo test
        entry: cargo test
        language: system
        types: [rust]
        pass_filenames: false

      # Check Cargo.toml
      - id: cargo-check
        name: cargo check
        entry: cargo check
        language: system
        types: [rust]
        pass_filenames: false

  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.5.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-added-large-files
      - id: check-merge-conflict
```

**Install hooks:**
```bash
pre-commit install
```

**Test hooks:**
```bash
pre-commit run --all-files
```

### 2.3 Quality Commands

**Before every commit:**
```bash
# Format code
cargo fmt

# Check linting
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test

# Build check
cargo build

# If all pass, commit
git commit -m "Your message"
```

**Manual quality check:**
```bash
# Run all quality checks
./scripts/quality-check.sh
```

---

## 3. Git Workflow

### 3.1 Branch Strategy

**For this POC:**
- Main branch: `main` or `master`
- Development branch: `claude/read-readme-01LRMMmzK3WvvV6kLbmG1wxF`
- Feature branches: As needed (e.g., `feature/accelerometer`)

### 3.2 Commit Guidelines

**Commit Message Format:**
```
<type>: <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding tests
- `refactor`: Code refactoring
- `style`: Formatting changes
- `chore`: Build process, dependencies

**Example:**
```
feat: implement accelerometer sensor plugin

- Create AccelerometerPlugin.kt for Android sensor access
- Add Rust command handler for accelerometer data
- Implement UI component to display X, Y, Z values
- Add tests for data model and command handler

All tests passing, code formatted with rustfmt, no clippy warnings.
```

### 3.3 Commit Process

1. **Make changes**
2. **Run quality checks** (pre-commit will do this automatically)
3. **Stage changes**: `git add .`
4. **Commit**: `git commit -m "message"`
5. **Pre-commit hooks run**:
   - If PASS → Commit created
   - If FAIL → Fix issues and retry
6. **Push**: `git push`

**Never:**
- Commit with failing tests
- Commit with clippy warnings
- Commit unformatted code
- Skip pre-commit hooks
- Disable linter warnings

---

## 4. Testing Process

### 4.1 Test Categories

#### Unit Tests
**Location:** `src/**/*_test.rs` or `#[cfg(test)]` modules
**Purpose:** Test individual functions and structs
**Coverage:** Should cover all business logic

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnetometer_heading_calculation() {
        let data = MagnetometerData {
            x: 0.0,
            y: 10.0,
            z: 0.0,
            heading: 0.0,
            timestamp: 0,
            accuracy: 3,
        };
        let heading = calculate_heading(data.x, data.y);
        assert_eq!(heading, 0.0); // Points north
    }
}
```

#### Integration Tests
**Location:** `tests/` directory
**Purpose:** Test interaction between components
**Coverage:** Critical paths and workflows

**Example:**
```rust
// tests/sensor_commands_test.rs
#[tokio::test]
async fn test_accelerometer_command_flow() {
    // Test that command returns valid data
    let result = get_accelerometer_data().await;
    assert!(result.is_ok());
}
```

#### Manual Tests
**Location:** Test plan document
**Purpose:** Verify functionality on actual device
**Process:**
1. Install app on device
2. Test each sensor
3. Test error cases
4. Test permissions
5. Verify UI behavior

### 4.2 Test Execution

**Run all tests:**
```bash
cargo test
```

**Run specific test:**
```bash
cargo test test_accelerometer_data_creation
```

**Run with output:**
```bash
cargo test -- --nocapture
```

**Run tests for specific module:**
```bash
cargo test models::
```

**Check test coverage (optional):**
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### 4.3 Mock Data

For testing without actual sensors:

```rust
pub fn mock_accelerometer_data() -> AccelerometerData {
    AccelerometerData {
        x: 0.0,
        y: 9.81,
        z: 0.0,
        timestamp: 1234567890,
        accuracy: 3,
    }
}

#[test]
fn test_with_mock_data() {
    let data = mock_accelerometer_data();
    assert_eq!(data.y, 9.81);
}
```

---

## 5. Code Review Process

### 5.1 Self-Review Checklist

Before committing, review:

- [ ] Code follows Rust idioms and best practices
- [ ] All functions have documentation comments
- [ ] Complex logic is commented
- [ ] No unnecessary `unwrap()` or `expect()`
- [ ] Errors are properly handled
- [ ] No hardcoded values (use constants)
- [ ] Variable names are descriptive
- [ ] Code is DRY (Don't Repeat Yourself)
- [ ] Security considerations addressed
- [ ] Performance is acceptable
- [ ] Tests cover new functionality
- [ ] No debugging code left in
- [ ] Dependencies are necessary and minimal

### 5.2 Code Standards

**Naming Conventions:**
- Types: `PascalCase` (e.g., `AccelerometerData`)
- Functions: `snake_case` (e.g., `get_sensor_data`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_SENSORS`)
- Modules: `snake_case` (e.g., `sensor_commands`)

**File Organization:**
```
src/
├── main.rs                 # Entry point, app initialization
├── commands/               # Tauri command handlers
│   ├── mod.rs
│   ├── sensors.rs
│   └── wifi.rs
├── models/                 # Data structures
│   ├── mod.rs
│   ├── sensor_data.rs
│   └── errors.rs
├── services/               # Business logic
│   ├── mod.rs
│   └── sensor_service.rs
├── utils/                  # Helper functions
│   ├── mod.rs
│   └── formatters.rs
└── tests/                  # Unit tests (when not inline)
    └── mod.rs
```

**Error Handling:**
```rust
// Good - Proper error propagation
pub fn get_sensor_data() -> Result<SensorData, SensorError> {
    let data = read_sensor()?;
    Ok(data)
}

// Bad - Unwrap can panic
pub fn get_sensor_data() -> SensorData {
    read_sensor().unwrap()
}
```

**Documentation:**
```rust
/// Gets accelerometer data from the device sensor
///
/// # Returns
/// - `Ok(AccelerometerData)` - Sensor data with X, Y, Z values
/// - `Err(SensorError)` - If sensor is unavailable or data read fails
///
/// # Example
/// ```
/// let data = get_accelerometer_data().await?;
/// println!("X: {}", data.x);
/// ```
#[tauri::command]
pub async fn get_accelerometer_data() -> Result<AccelerometerData, String> {
    // Implementation
}
```

---

## 6. Build Process

### 6.1 Development Build

```bash
# Build for Android
cargo android build

# Run on emulator/device
cargo android run

# Watch for changes (if available)
cargo watch -x "android build"
```

### 6.2 Release Build

```bash
# Build optimized APK
cargo android build --release

# Output location:
# src-tauri/gen/android/app/build/outputs/apk/release/
```

### 6.3 Clean Build

```bash
# Clean Rust build
cargo clean

# Clean Android build
cd src-tauri/gen/android
./gradlew clean
cd ../../..

# Full clean
rm -rf target/
rm -rf src-tauri/gen/android/app/build/
```

---

## 7. Debugging Process

### 7.1 Rust Debugging

**Print debugging:**
```rust
println!("Debug: value = {:?}", value);
dbg!(value);
```

**Logging:**
```rust
use log::{info, warn, error};

info!("Sensor data updated");
error!("Failed to read sensor: {}", e);
```

### 7.2 Android Debugging

**Logcat:**
```bash
# View all logs
adb logcat

# Filter by tag
adb logcat -s RustSensorPOC

# Clear logs
adb logcat -c
```

**Chrome DevTools:**
1. Open Chrome browser
2. Navigate to `chrome://inspect`
3. Select your device/app
4. Debug JavaScript/WebView

### 7.3 Common Issues

| Issue | Solution |
|-------|----------|
| Build fails | Run `cargo clean`, check dependencies |
| Tests fail | Check mock data, verify logic |
| Sensor not working | Check permissions, device support |
| UI not updating | Check Tauri IPC, verify events |
| Permission denied | Check AndroidManifest.xml |

---

## 8. Documentation Process

### 8.1 Code Documentation

**Every public function should have:**
- Description of what it does
- Parameters explanation
- Return value description
- Error cases
- Example usage (when helpful)

**Generate docs:**
```bash
cargo doc --open
```

### 8.2 Project Documentation

**Documents to maintain:**
- `README.md` - Project overview, build instructions
- `docs/research.md` - Research findings (complete)
- `docs/prd.md` - Product requirements (complete)
- `docs/architecture.md` - System architecture (complete)
- `docs/design.md` - UI/UX design (complete)
- `docs/plan.md` - Implementation plan (complete)
- `docs/process.md` - This document (in progress)
- `docs/status.md` - Current status and progress

**Update frequency:**
- `status.md` - Daily or after significant progress
- Other docs - When significant changes occur

---

## 9. Continuous Integration (Future)

### 9.1 CI Pipeline (When available)

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo fmt -- --check
      - run: cargo clippy -- -D warnings
      - run: cargo test
      - run: cargo build
```

---

## 10. Quality Metrics

### 10.1 Success Metrics

Track these metrics:
- **Test Coverage**: > 70% for business logic
- **Clippy Warnings**: 0 (strict)
- **Build Time**: < 5 minutes
- **APK Size**: < 20 MB
- **Memory Usage**: < 100 MB runtime
- **Startup Time**: < 3 seconds

### 10.2 Code Quality Checks

**Before each commit:**
- [ ] Code compiles
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code is formatted
- [ ] Documentation is updated
- [ ] .gitignore is correct

---

## 11. Definition of Done

A feature is "Done" when:

1. **Code Complete**
   - [ ] Implementation matches requirements
   - [ ] Code follows style guidelines
   - [ ] No hardcoded values

2. **Tests Complete**
   - [ ] Unit tests written and passing
   - [ ] Integration tests passing
   - [ ] Manual testing completed

3. **Quality Checks**
   - [ ] Code formatted with rustfmt
   - [ ] No clippy warnings
   - [ ] Pre-commit hooks pass

4. **Documentation**
   - [ ] Code has doc comments
   - [ ] Status.md updated
   - [ ] README updated if needed

5. **Review**
   - [ ] Self-review completed
   - [ ] All checklist items addressed

6. **Integration**
   - [ ] Merged to development branch
   - [ ] Builds successfully
   - [ ] Works with other features

---

## 12. Issue Tracking

### 12.1 Issue Categories

**Bug**: Something is broken
**Enhancement**: Improvement to existing feature
**Feature**: New functionality
**Tech Debt**: Code quality improvements
**Documentation**: Doc updates

### 12.2 Issue Lifecycle

1. **Created** - Issue identified
2. **Triaged** - Priority assigned
3. **In Progress** - Being worked on
4. **Testing** - Verification
5. **Done** - Completed and verified

---

## 13. Release Process

### 13.1 Version Numbering

Use Semantic Versioning: `MAJOR.MINOR.PATCH`
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

**Example:**
- v1.0.0 - MVP Release
- v1.1.0 - Add gyroscope sensor
- v1.1.1 - Fix GPS accuracy bug

### 13.2 Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created
- [ ] Release notes written
- [ ] APK built and tested
- [ ] Pushed to repository

---

## 14. Emergency Procedures

### 14.1 Rollback Process

If a commit breaks things:
```bash
# Revert last commit
git revert HEAD

# Or reset (use cautiously)
git reset --hard HEAD~1
```

### 14.2 Critical Bug Process

1. Identify and document bug
2. Create fix branch
3. Write test that reproduces bug (RED)
4. Fix bug (GREEN)
5. Refactor if needed
6. Run all quality checks
7. Commit and push immediately
8. Tag as hotfix

---

## 15. Best Practices Summary

1. **Always write tests first** (TDD Red-Green-Refactor)
2. **Never disable linter warnings** - Fix them
3. **Run quality checks before commit** - Use pre-commit hooks
4. **Write clear commit messages** - Explain why, not just what
5. **Keep commits atomic** - One logical change per commit
6. **Update documentation** - Keep docs in sync with code
7. **Handle errors properly** - No unwrap() in production code
8. **Review your own code** - Before committing
9. **Test on real devices** - Emulators aren't enough
10. **Keep it simple** - KISS principle

---

## 16. Tools and Resources

### Development Tools:
- **IDE**: VS Code with rust-analyzer
- **Build**: cargo, cargo-mobile2
- **Format**: rustfmt
- **Lint**: clippy
- **Test**: cargo test
- **Debug**: Chrome DevTools, adb logcat

### Resources:
- Rust Book: https://doc.rust-lang.org/book/
- Tauri Docs: https://v2.tauri.app/
- Android Developer Guide: https://developer.android.com/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/

---

This process document ensures consistent, high-quality development throughout the project lifecycle.
