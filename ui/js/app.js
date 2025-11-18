// Rust Sensor POC - Main Application
// This simulates sensor data updates for demonstration purposes

/**
 * Mock sensor data based on Rust library models
 * In a real Tauri app, this would call Rust backend via IPC
 */
const MockSensorData = {
    accelerometer: {
        atRest: { x: 0.0, y: 9.81, z: 0.0, timestamp: Date.now(), accuracy: 3 },
        shaking: { x: 2.5, y: 8.3, z: -1.2, timestamp: Date.now(), accuracy: 3 }
    },
    magnetometer: {
        north: { x: 0.0, y: 50.0, z: 20.0, heading: 0.0, timestamp: Date.now(), accuracy: 3 },
        southwest: { x: -35.0, y: -35.0, z: 20.0, heading: 225.0, timestamp: Date.now(), accuracy: 3 }
    },
    gps: {
        sanFrancisco: {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: 16.0,
            accuracy: 5.0,
            speed: 0.0,
            timestamp: Date.now()
        }
    },
    pressure: {
        seaLevel: { pressure: 1013.25, timestamp: Date.now() },
        altitude: { pressure: 950.0, timestamp: Date.now() }
    },
    wifi: [
        {
            ssid: "MyHomeWiFi",
            bssid: "00:11:22:33:44:55",
            signal_strength: -45,
            frequency: 2412,
            security: "WPA2"
        },
        {
            ssid: "Neighbor_5G",
            bssid: "AA:BB:CC:DD:EE:FF",
            signal_strength: -68,
            frequency: 5180,
            security: "WPA3"
        },
        {
            ssid: "CoffeeShop-Guest",
            bssid: "11:22:33:44:55:66",
            signal_strength: -52,
            frequency: 2437,
            security: "Open"
        }
    ]
};

/**
 * Format timestamp for display
 */
function formatTimestamp(timestamp) {
    const now = Date.now();
    const diff = Math.floor((now - timestamp) / 1000);

    if (diff < 60) return `${diff}s ago`;
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;

    const date = new Date(timestamp);
    return date.toLocaleTimeString();
}

/**
 * Calculate acceleration magnitude
 */
function calculateMagnitude(x, y, z) {
    return Math.sqrt(x * x + y * y + z * z).toFixed(2);
}

/**
 * Get cardinal direction from heading
 */
function getCardinalDirection(heading) {
    const normalized = ((heading % 360) + 360) % 360;

    if (normalized >= 337.5 || normalized < 22.5) return 'N';
    if (normalized >= 22.5 && normalized < 67.5) return 'NE';
    if (normalized >= 67.5 && normalized < 112.5) return 'E';
    if (normalized >= 112.5 && normalized < 157.5) return 'SE';
    if (normalized >= 157.5 && normalized < 202.5) return 'S';
    if (normalized >= 202.5 && normalized < 247.5) return 'SW';
    if (normalized >= 247.5 && normalized < 292.5) return 'W';
    if (normalized >= 292.5 && normalized < 337.5) return 'NW';
    return 'N';
}

/**
 * Update accelerometer display
 */
function updateAccelerometer(data) {
    document.getElementById('accel-x').textContent = data.x.toFixed(2);
    document.getElementById('accel-y').textContent = data.y.toFixed(2);
    document.getElementById('accel-z').textContent = data.z.toFixed(2);

    const mag = calculateMagnitude(data.x, data.y, data.z);
    document.getElementById('accel-mag').textContent = mag;

    document.getElementById('accel-time').textContent = formatTimestamp(data.timestamp);
}

/**
 * Update magnetometer display
 */
function updateMagnetometer(data) {
    document.getElementById('mag-x').textContent = data.x.toFixed(1);
    document.getElementById('mag-y').textContent = data.y.toFixed(1);
    document.getElementById('mag-z').textContent = data.z.toFixed(1);

    const direction = getCardinalDirection(data.heading);
    document.getElementById('mag-heading-large').textContent = `${data.heading.toFixed(1)}°`;
    document.getElementById('mag-direction').textContent = direction;

    document.getElementById('mag-time').textContent = formatTimestamp(data.timestamp);
}

/**
 * Update GPS display
 */
function updateGPS(data) {
    const latDir = data.latitude >= 0 ? 'N' : 'S';
    const lonDir = data.longitude >= 0 ? 'E' : 'W';

    document.getElementById('gps-lat').textContent =
        `${Math.abs(data.latitude).toFixed(4)}° ${latDir}`;
    document.getElementById('gps-lon').textContent =
        `${Math.abs(data.longitude).toFixed(4)}° ${lonDir}`;
    document.getElementById('gps-alt').textContent =
        data.altitude ? data.altitude.toFixed(1) : 'N/A';
    document.getElementById('gps-acc').textContent =
        `±${data.accuracy.toFixed(1)}`;
    document.getElementById('gps-speed').textContent =
        data.speed ? data.speed.toFixed(1) : '0.0';

    document.getElementById('gps-time').textContent = formatTimestamp(data.timestamp);
}

/**
 * Update pressure display
 */
function updatePressure(data) {
    const description = data.pressure < 1000.0 ? '(Low)' :
                       data.pressure > 1020.0 ? '(High)' : '(Normal)';

    document.getElementById('pressure-large').textContent =
        `${data.pressure.toFixed(2)} hPa`;
    document.getElementById('pressure-desc').textContent = description;
    document.getElementById('pressure-time').textContent = formatTimestamp(data.timestamp);
}

/**
 * Get signal description and CSS class
 */
function getSignalInfo(rssi) {
    if (rssi >= -50) return { desc: 'Excellent', class: 'signal-excellent', bars: '▂▃▅▆█' };
    if (rssi >= -60) return { desc: 'Good', class: 'signal-good', bars: '▂▃▅▆_' };
    if (rssi >= -70) return { desc: 'Fair', class: 'signal-fair', bars: '▂▃▅__' };
    return { desc: 'Weak', class: 'signal-weak', bars: '▂▃___' };
}

/**
 * Update WiFi networks display
 */
function updateWiFi(networks) {
    // Sort by signal strength (strongest first)
    const sorted = networks.slice().sort((a, b) => b.signal_strength - a.signal_strength);

    const wifiList = document.getElementById('wifi-list');
    wifiList.innerHTML = '';

    sorted.forEach(network => {
        const signalInfo = getSignalInfo(network.signal_strength);

        const networkDiv = document.createElement('div');
        networkDiv.className = 'wifi-network fade-in';
        networkDiv.innerHTML = `
            <div class="wifi-info">
                <div class="wifi-ssid">${network.ssid}</div>
                <div class="wifi-details">
                    ${network.security} • ${(network.frequency / 1000).toFixed(1)} GHz
                </div>
            </div>
            <div class="wifi-signal">
                <div class="signal-bars ${signalInfo.class}">${signalInfo.bars}</div>
                <div class="signal-strength">${network.signal_strength} dBm (${signalInfo.desc})</div>
            </div>
        `;

        wifiList.appendChild(networkDiv);
    });

    document.getElementById('wifi-time').textContent = formatTimestamp(Date.now());
}

/**
 * Simulate sensor data updates
 */
function startSensorUpdates() {
    let shaking = false;
    let heading = 0;

    // Update accelerometer every 100ms
    setInterval(() => {
        shaking = !shaking;
        const data = shaking ?
            MockSensorData.accelerometer.shaking :
            MockSensorData.accelerometer.atRest;
        data.timestamp = Date.now();
        updateAccelerometer(data);
    }, 100);

    // Update magnetometer every 500ms (rotating compass)
    setInterval(() => {
        heading = (heading + 5) % 360;
        const data = {
            x: Math.sin(heading * Math.PI / 180) * 50,
            y: Math.cos(heading * Math.PI / 180) * 50,
            z: 20.0,
            heading: heading,
            timestamp: Date.now(),
            accuracy: 3
        };
        updateMagnetometer(data);
    }, 500);

    // Update GPS every 2 seconds (slight movement)
    let gpsCount = 0;
    setInterval(() => {
        const data = MockSensorData.gps.sanFrancisco;
        data.latitude += (Math.random() - 0.5) * 0.0001;
        data.longitude += (Math.random() - 0.5) * 0.0001;
        data.speed = Math.random() * 2.0;
        data.timestamp = Date.now();
        updateGPS(data);
        gpsCount++;
    }, 2000);

    // Update pressure every 5 seconds
    setInterval(() => {
        const data = MockSensorData.pressure.seaLevel;
        data.pressure = 1013.25 + (Math.random() - 0.5) * 2.0;
        data.timestamp = Date.now();
        updatePressure(data);
    }, 5000);

    // Update WiFi networks every 10 seconds
    setInterval(() => {
        // Randomize signal strengths slightly
        const networks = MockSensorData.wifi.map(n => ({
            ...n,
            signal_strength: n.signal_strength + Math.floor((Math.random() - 0.5) * 10)
        }));
        updateWiFi(networks);
    }, 10000);

    // Initial updates
    updateAccelerometer(MockSensorData.accelerometer.atRest);
    updateMagnetometer(MockSensorData.magnetometer.north);
    updateGPS(MockSensorData.gps.sanFrancisco);
    updatePressure(MockSensorData.pressure.seaLevel);
    updateWiFi(MockSensorData.wifi);
}

/**
 * Initialize application
 */
function init() {
    console.log('Rust Sensor POC - Initializing...');
    console.log('Using mock sensor data for demonstration');

    // Update status
    document.querySelector('.status-text').textContent = 'Sensors Active';

    // Start sensor updates
    startSensorUpdates();

    console.log('Application initialized successfully!');
}

// Start the application when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
