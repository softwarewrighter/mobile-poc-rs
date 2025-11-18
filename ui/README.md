## UI Demo - Rust Mobile Sensor POC

This directory contains a static HTML/CSS/JavaScript demonstration of the sensor UI.

### Overview

The UI demonstrates how sensor data would be displayed in a real mobile application. It uses mock data to simulate real-time sensor updates.

### Features

- **Accelerometer Display**: Shows X, Y, Z acceleration values and total magnitude
- **Magnetometer/Compass**: Visual compass display with heading and cardinal direction
- **GPS Location**: Displays coordinates, altitude, accuracy, and speed
- **Barometric Pressure**: Shows pressure with high/normal/low indicator
- **Temperature**: Shows unavailable state (common on mobile devices)
- **WiFi Scanner**: Lists nearby networks with signal strength and security

### Running the Demo

Simply open `index.html` in a web browser:

```bash
# From the ui directory
open index.html
# or
firefox index.html
# or
google-chrome index.html
```

### Files

- `index.html` - Main HTML structure with sensor cards
- `css/styles.css` - Complete styling with responsive design
- `js/app.js` - JavaScript for simulating sensor data updates

### Mock Data

The demo uses mock data that mirrors the Rust library's mock data providers:

- Accelerometer alternates between "at rest" and "shaking"
- Magnetometer rotates through compass headings
- GPS shows slight movement simulation
- Pressure shows natural variations
- WiFi networks update with fluctuating signal strengths

### Responsive Design

The UI is fully responsive and adapts to different screen sizes:

- **Mobile (< 600px)**: Single column layout
- **Tablet (600-768px)**: Optimized spacing
- **Desktop (> 768px)**: Two-column grid layout

### Integration Notes

In a real Tauri mobile application, the JavaScript would:

1. Call Rust backend via Tauri IPC commands
2. Receive actual sensor data from Android/iOS
3. Handle permissions and error states
4. Support user interactions (refresh, settings, etc.)

### Technology Stack

- Pure HTML5/CSS3/JavaScript (no frameworks required)
- Material Design inspired styling
- CSS Grid and Flexbox for layout
- CSS Custom Properties for theming
- Responsive design with media queries

### Future Enhancements

- Dark mode support
- Historical data charts
- Export data functionality
- Settings page
- Real Tauri integration
