# Design Document
# Rust Mobile Sensor POC

**Version:** 1.0
**Date:** 2025-11-18
**Status:** Draft

## 1. UI/UX Design

### 1.1 Design Principles
- **Simplicity**: Clear, uncluttered interface focused on sensor data
- **Responsiveness**: Real-time updates with smooth transitions
- **Feedback**: Clear status indicators for sensor availability
- **Accessibility**: Large touch targets, readable text sizes
- **Native Feel**: Follow Android Material Design guidelines where possible

### 1.2 Color Scheme
```css
Primary: #2196F3 (Blue)
Secondary: #FFC107 (Amber)
Success: #4CAF50 (Green)
Warning: #FF9800 (Orange)
Error: #F44336 (Red)
Background: #FAFAFA (Light Gray)
Surface: #FFFFFF (White)
Text Primary: #212121 (Dark Gray)
Text Secondary: #757575 (Gray)
```

### 1.3 Typography
- **Headers**: Roboto, 20-24px, Bold
- **Body**: Roboto, 16px, Regular
- **Labels**: Roboto, 14px, Medium
- **Data Values**: Roboto Mono, 18px, Medium (for sensor readings)

## 2. Screen Layouts

### 2.1 Main Screen (Sensor List)

```
┌──────────────────────────────────────┐
│  ┌────┐  Rust Sensor POC             │  <- App Bar
│  └────┘                               │
├──────────────────────────────────────┤
│                                       │
│  ┌─────────────────────────────────┐ │
│  │ 📱 Accelerometer            ✓   │ │  <- Sensor Card
│  │ X: -0.23 m/s²                   │ │
│  │ Y:  9.81 m/s²                   │ │
│  │ Z:  0.05 m/s²                   │ │
│  │ ────────────────────────────    │ │
│  │ Last updated: 0.1s ago          │ │
│  └─────────────────────────────────┘ │
│                                       │
│  ┌─────────────────────────────────┐ │
│  │ 🧭 Magnetometer             ✓   │ │
│  │ Heading: 245° (SW)              │ │
│  │ X: -12.3 μT                     │ │
│  │ Y:  45.6 μT                     │ │
│  │ Z: -23.1 μT                     │ │
│  │ ────────────────────────────    │ │
│  │ Last updated: 0.2s ago          │ │
│  └─────────────────────────────────┘ │
│                                       │
│  ┌─────────────────────────────────┐ │
│  │ 📍 GPS Location             ✓   │ │
│  │ Lat: 37.7749° N                 │ │
│  │ Lon: 122.4194° W                │ │
│  │ Accuracy: ±5m                   │ │
│  │ ────────────────────────────    │ │
│  │ Last updated: 1.2s ago          │ │
│  └─────────────────────────────────┘ │
│                                       │
│  ┌─────────────────────────────────┐ │
│  │ 🌡️  Temperature             ✗   │ │
│  │ Sensor not available on device  │ │
│  └─────────────────────────────────┘ │
│                                       │
└──────────────────────────────────────┘
```

### 2.2 WiFi Scanner Screen

```
┌──────────────────────────────────────┐
│  ┌────┐  WiFi Networks               │  <- App Bar
│  │ ← │                   [Refresh]   │
│  └────┘                               │
├──────────────────────────────────────┤
│  Found 5 networks                     │  <- Status Bar
├──────────────────────────────────────┤
│                                       │
│  ┌─────────────────────────────────┐ │
│  │ MyHomeWiFi           ▂▃▅▆█      │ │  <- Network Item
│  │ WPA2 • 2.4 GHz • -45 dBm        │ │
│  └─────────────────────────────────┘ │
│                                       │
│  ┌─────────────────────────────────┐ │
│  │ CoffeeShop-Guest     ▂▃▅▆_      │ │
│  │ Open • 5 GHz • -62 dBm          │ │
│  └─────────────────────────────────┘ │
│                                       │
│  ┌─────────────────────────────────┐ │
│  │ Neighbor_5G          ▂▃__        │ │
│  │ WPA3 • 5 GHz • -78 dBm          │ │
│  └─────────────────────────────────┘ │
│                                       │
└──────────────────────────────────────┘
```

### 2.3 Error States

**Permission Denied:**
```
┌─────────────────────────────────┐
│ 📍 GPS Location              ⚠  │
│                                  │
│ Location permission required     │
│                                  │
│      [Grant Permission]          │
└─────────────────────────────────┘
```

**Sensor Unavailable:**
```
┌─────────────────────────────────┐
│ 🌡️  Temperature             ✗   │
│                                  │
│ This sensor is not available on  │
│ your device.                     │
└─────────────────────────────────┘
```

## 3. Component Design

### 3.1 Sensor Card Component

**Visual Structure:**
```
┌───────────────────────────────────┐
│ [Icon] Sensor Name         [Status]│  <- Header
├───────────────────────────────────┤
│ Data Display Area                 │  <- Body
│ • Primary value(s)                │
│ • Secondary info                  │
│ • Units clearly labeled           │
├───────────────────────────────────┤
│ Metadata (timestamp, accuracy)    │  <- Footer
└───────────────────────────────────┘
```

**States:**
- **Active**: White background, data updating
- **Unavailable**: Gray background, disabled appearance
- **Error**: Yellow/red border, error message
- **Loading**: Skeleton or spinner animation

**Interaction:**
- Tap to expand for more details (future enhancement)
- Pull-to-refresh gesture support
- Haptic feedback on interaction

### 3.2 Navigation

**Tab Navigation (Future):**
```
┌────────────────────────────────────┐
│  [Motion] [Environment] [Location] │
└────────────────────────────────────┘
```

**Current Approach:**
- Single scrollable list of all sensors
- Simple and straightforward for POC

### 3.3 Icons

| Sensor | Icon | Unicode |
|--------|------|---------|
| Accelerometer | 📱 | U+1F4F1 |
| Magnetometer | 🧭 | U+1F9ED |
| GPS | 📍 | U+1F4CD |
| Pressure | 🌤️ | U+1F324 |
| Temperature | 🌡️ | U+1F321 |
| WiFi | 📶 | U+1F4F6 |

Status indicators:
- Available: ✓ (Green checkmark)
- Unavailable: ✗ (Red X)
- Warning: ⚠ (Yellow warning)
- Loading: ⟳ (Spinning loader)

## 4. Interaction Design

### 4.1 Gestures
- **Scroll**: Navigate through sensor list
- **Pull-to-Refresh**: Update all sensors (future)
- **Tap**: Expand sensor details (future)
- **Swipe**: Navigate between views (future)

### 4.2 Animations
- **Sensor Value Updates**: Smooth number transitions
- **Card Loading**: Fade-in animation
- **Error Appearance**: Slide-in from top
- **State Changes**: 200ms ease-in-out transitions

### 4.3 Feedback
- **Visual**: Color changes, icons, animations
- **Textual**: Clear status messages
- **Temporal**: Timestamps for last update
- **Haptic**: Touch feedback on buttons (future)

## 5. Data Visualization

### 5.1 Sensor Value Display

**Accelerometer:**
```
X: -0.23 m/s²  ████░░░░░░
Y:  9.81 m/s²  ██████████
Z:  0.05 m/s²  █░░░░░░░░░
```

**Magnetometer:**
```
        N
        ↑
    W ← ⊕ → E    Heading: 245° (SW)
        ↓
        S
```

**GPS:**
```
Latitude:  37.7749° N
Longitude: 122.4194° W
Altitude:  10 m
Accuracy:  ±5 m
Speed:     0 km/h
```

**Pressure:**
```
Pressure: 1013.25 hPa
(Standard sea level)
```

**Temperature:**
```
Temperature: 22.5°C (72.5°F)
```

**WiFi:**
```
Signal Strength:
Strong  ▂▃▅▆█
Medium  ▂▃▅__
Weak    ▂____
```

### 5.2 Value Formatting

**Numbers:**
- Decimal places: 2 for most values
- Grouping: Use locale-appropriate separators
- Units: Always displayed with value
- Ranges: Show min/max if relevant

**Timestamps:**
- Recent: "X.Xs ago" (0-60s)
- Medium: "Xm ago" (1-60 min)
- Old: "HH:MM:SS" (> 1 hour)
- Never: "Never updated"

**Directions:**
- Degrees: 0-359°
- Cardinals: N, NE, E, SE, S, SW, W, NW
- Combined: "245° (SW)"

## 6. Responsive Design

### 6.1 Screen Size Support

**Small Phones (< 360dp width):**
- Single column layout
- Compact card spacing
- Smaller font sizes
- Stack data vertically

**Medium Phones (360-600dp):**
- Single column with comfortable spacing
- Standard font sizes
- Mix of horizontal and vertical data

**Large Phones/Tablets (> 600dp):**
- Two-column grid layout
- Larger cards
- More whitespace
- Side-by-side data display

### 6.2 Orientation

**Portrait (Primary):**
- Vertical scrolling list
- Full-width cards
- Optimized for one-handed use

**Landscape:**
- Two-column grid
- Horizontal scrolling if needed
- Better use of wide screen

## 7. Accessibility

### 7.1 Screen Reader Support
- Semantic HTML structure
- ARIA labels for all interactive elements
- Meaningful alt text for icons
- Announce sensor value updates

### 7.2 Text Accessibility
- Minimum font size: 14sp
- High contrast ratios (4.5:1 minimum)
- Scalable text (respect system font size)
- No text in images

### 7.3 Touch Targets
- Minimum size: 48x48dp
- Adequate spacing between targets
- Clear focus indicators
- Support for system navigation gestures

### 7.4 Color Accessibility
- Don't rely solely on color for information
- Use icons and text labels
- Support dark mode (future)
- High contrast mode support

## 8. Design System

### 8.1 Spacing Scale
```
xs:  4px
sm:  8px
md:  16px
lg:  24px
xl:  32px
xxl: 48px
```

### 8.2 Border Radius
```
small:  4px (buttons, chips)
medium: 8px (cards)
large:  16px (modals)
round:  50% (circular elements)
```

### 8.3 Elevation (Shadows)
```
Level 1: 0 1px 3px rgba(0,0,0,0.12)  (cards)
Level 2: 0 2px 6px rgba(0,0,0,0.16)  (raised elements)
Level 3: 0 4px 12px rgba(0,0,0,0.20) (modals)
```

### 8.4 Animation Timing
```
Fast:   150ms (micro-interactions)
Medium: 250ms (standard transitions)
Slow:   400ms (complex animations)
```

## 9. HTML/CSS Structure

### 9.1 App Shell
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Sensor POC</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app">
        <header class="app-bar">
            <h1>Rust Sensor POC</h1>
        </header>
        <main class="sensor-list">
            <!-- Sensor cards inserted here -->
        </main>
    </div>
    <script src="app.js"></script>
</body>
</html>
```

### 9.2 Sensor Card Template
```html
<div class="sensor-card" data-sensor="accelerometer">
    <div class="card-header">
        <span class="sensor-icon">📱</span>
        <h2 class="sensor-name">Accelerometer</h2>
        <span class="sensor-status status-available">✓</span>
    </div>
    <div class="card-body">
        <div class="sensor-data">
            <div class="data-row">
                <span class="label">X:</span>
                <span class="value" id="accel-x">--</span>
                <span class="unit">m/s²</span>
            </div>
            <div class="data-row">
                <span class="label">Y:</span>
                <span class="value" id="accel-y">--</span>
                <span class="unit">m/s²</span>
            </div>
            <div class="data-row">
                <span class="label">Z:</span>
                <span class="value" id="accel-z">--</span>
                <span class="unit">m/s²</span>
            </div>
        </div>
    </div>
    <div class="card-footer">
        <span class="timestamp">Last updated: <span id="accel-time">never</span></span>
    </div>
</div>
```

### 9.3 CSS Classes
```css
/* Layout */
.app-bar { }
.sensor-list { }
.sensor-card { }

/* Components */
.card-header { }
.card-body { }
.card-footer { }

/* States */
.status-available { color: #4CAF50; }
.status-unavailable { color: #F44336; }
.status-warning { color: #FF9800; }

/* Typography */
.sensor-name { }
.label { }
.value { }
.unit { }

/* Data Display */
.data-row { }
.sensor-data { }
```

## 10. Dark Mode (Future)

### 10.1 Dark Color Scheme
```css
Background: #121212
Surface: #1E1E1E
Primary: #64B5F6
Text: #FFFFFF
Text Secondary: #B0B0B0
```

### 10.2 Implementation
```css
@media (prefers-color-scheme: dark) {
    :root {
        --bg-color: #121212;
        --surface-color: #1E1E1E;
        --text-color: #FFFFFF;
    }
}
```

## 11. Loading States

### 11.1 Initial Load
```
┌─────────────────────────────────┐
│ ┌───────────────────────────┐   │
│ │ ░░░░░░░░░░░░░░░░░░        │   │  <- Skeleton
│ │ ░░░░░░░░░░                │   │
│ │ ░░░░░░░░░░                │   │
│ └───────────────────────────┘   │
└─────────────────────────────────┘
```

### 11.2 Refreshing
- Pull-to-refresh spinner at top
- Individual card loading indicators
- Disable interactions during refresh

## 12. Error Handling UI

### 12.1 Error Types
1. **Inline Errors**: Show in sensor card
2. **Toast Messages**: Temporary notifications
3. **Dialog Errors**: Critical issues requiring action

### 12.2 Error Messages
- Clear and concise
- Actionable (suggest solutions)
- Non-technical language
- Include retry option when applicable

## 13. Performance Considerations

### 13.1 Rendering
- Virtual scrolling for long lists (future)
- Debounce rapid sensor updates
- Use CSS transforms for animations
- Minimize DOM manipulation

### 13.2 Data Updates
- Update only changed values
- Batch DOM updates
- Use requestAnimationFrame
- Throttle to 10-30 FPS max

## 14. Platform Integration

### 14.1 Android Material Design
- Ripple effects on touch
- FAB buttons (future)
- Bottom sheets for actions (future)
- Snackbar notifications (future)

### 14.2 Native Feel
- Respect system settings (font size, theme)
- Use platform fonts (Roboto on Android)
- Follow Android UI patterns
- Support system back button

## 15. Design Checklist

- [ ] All screens designed
- [ ] All states defined (loading, error, empty)
- [ ] Responsive breakpoints specified
- [ ] Color palette defined
- [ ] Typography scale established
- [ ] Icon set selected
- [ ] Animation timing defined
- [ ] Accessibility requirements met
- [ ] Dark mode considered (future)
- [ ] Error states designed
- [ ] Loading states designed
- [ ] Empty states designed

## 16. Future Enhancements

### 16.1 Visual Improvements
- Charts and graphs for sensor history
- 3D visualization for accelerometer
- Compass rose for magnetometer
- Map view for GPS
- Signal strength meter for WiFi

### 16.2 Interaction Improvements
- Gestures for navigation
- Customizable dashboard
- Widget support
- Notification support
- Share sensor data

### 16.3 Theming
- Multiple color themes
- Custom icon packs
- User preferences
- Dynamic theming based on time
