# PHOSPHOROS Dashboard - Quick Start Guide

## Installation & Running

### Build from Source

```bash
# Clone the repository
git clone https://github.com/LashSesh/phosphoros.git
cd phosphoros

# Build the dashboard
cargo build -p phosphoros-dashboard --release

# Run the dashboard
cargo run -p phosphoros-dashboard --release
```

### Development Mode

```bash
# Run in debug mode with logging
RUST_LOG=debug cargo run -p phosphoros-dashboard

# Build only
cargo build -p phosphoros-dashboard

# Check for errors
cargo check -p phosphoros-dashboard
```

## First Launch

When you first launch PHOSPHOROS Dashboard, you'll see:

1. **Sidebar** (left) with navigation to all 6 panels
2. **Main Content Area** (right) showing the currently active panel
3. **Active Tasks Indicator** at the bottom of sidebar showing running services

### Default Configuration

The dashboard starts with these defaults:
- **Theme**: Dark Mode (#1E1E1E background)
- **Window Size**: 1600x900 (minimum 1280x720)
- **Services**: Auto-start enabled
- **Active Panel**: Home (Live Overview)

## Panel Guide

### 1. 🏠 Live Overview (Home)

The default landing panel showing:
- **Seeds Collected**: Total number of imported seeds
- **Clusters Found**: Discovered resonance clusters  
- **Coverage**: Search space coverage percentage
- **System Status**: Real-time status of all three services (Scraper, Analyzer, Cluster Engine)

**Usage**: Monitor overall system health and activity

### 2. 🔑 Seed & Wallet Management

Import and manage cryptographic seeds:

**To Import a Seed**:
1. Enter mnemonic, seed phrase, or private key in text box
2. Click "Import Seed" button
3. System will generate multichain addresses
4. View imported seeds in the list below

**Features**:
- Masked seed display for security
- Address count per seed
- Import timestamps
- Clear input functionality

**Security Note**: Seeds are masked in UI and never logged in plain text

### 3. 📊 Resonance & Spectrography

Run 5D-spectral resonance analysis:

**To Start Analysis**:
1. Click "Start Analysis" button
2. Watch progress bar for real-time status
3. Monitor "Best Resonance" score updates
4. Click "Stop Analysis" to pause

**Features**:
- Real-time progress tracking
- Best resonance score display
- Operator configuration (WT, SW, DK, PI)
- Analysis history

### 4. 🔍 Cluster Explorer

Search and explore discovered clusters:

**To Search**:
1. Enter search terms in search box
2. View matching clusters below
3. Click cluster to view details (planned)

**Features**:
- Real-time search
- Cluster member count
- Resonance magnitude display
- Export functionality (planned)

### 5. 📝 System Log

View all system activity:

**Features**:
- Real-time log streaming
- Filter by keyword
- Log levels: Debug, Info, Warning, Error
- Timestamps and source tracking
- Clear logs functionality

**To Filter Logs**:
1. Enter keyword in filter box
2. Logs automatically filter as you type

### 6. ⚙️ Settings & Tasks

Configure dashboard and services:

**Theme Settings**:
- Toggle Dark/Light mode
- Changes apply immediately

**Services** (planned):
- Pause/resume individual services
- Set task priorities
- Configure auto-start

## Autonomous Services

Three background services run independently:

### Scraper Service 🔍
- Crawls seedspace and addressspace
- Collects cryptographic data
- Builds database continuously
- **Status**: Green when running, Gray when paused

### Analyzer Service 🧠
- Pattern recognition
- Anomaly detection
- Cluster formation
- **Status**: Green when running, Gray when paused

### Cluster Engine ⚙️
- KNN graph construction
- Resonance hotspot detection
- Topological analysis
- **Status**: Green when running, Gray when paused

**All services**:
- Run in background automatically
- Can be paused/resumed
- Report status to UI
- Persist state across sessions

## Keyboard Shortcuts (Planned)

Future keyboard shortcuts will include:
- `Ctrl+1-6`: Switch between panels
- `Ctrl+L`: Focus search/filter
- `Ctrl+Shift+C`: Clear current panel
- `Ctrl+/`: Toggle sidebar
- `F11`: Toggle fullscreen

## Configuration

Configuration file location:
- **Linux/macOS**: `~/.config/phosphoros/config.json`
- **Windows**: `%APPDATA%\phosphoros\config.json`

Example `config.json`:
```json
{
  "window": {
    "width": 1600.0,
    "height": 900.0,
    "min_width": 1280.0,
    "min_height": 720.0
  },
  "theme": {
    "dark_mode": true,
    "language": "English"
  },
  "services": {
    "auto_start": true,
    "scraper_enabled": true,
    "analyzer_enabled": true,
    "cluster_engine_enabled": true
  }
}
```

## Troubleshooting

### Dashboard won't start
- Check that you have a compatible graphics environment
- On Linux, ensure you have Wayland or X11
- Try running with `RUST_LOG=debug` for detailed output

### Services not running
- Check "System Status" card on Home panel
- Services may be paused - they're gray when paused
- Enable auto-start in Settings panel

### High CPU usage
- Normal during analysis operations
- Pause services if needed
- Check service status indicators

### Window too small
- Minimum window size is 1280x720
- Resize window or maximize
- Some UI elements collapse on smaller screens

## Integration with PHOSPHOROS Core

The dashboard integrates with:

### phosphoros-core
- 5D geometry operations
- Resonance engine execution
- Spectral signature analysis

### phosphoros-bip39  
- Multichain wallet generation
- Seed validation
- HD derivation paths

### phosphoros-satellite
- Blockchain forensics
- Cluster analysis
- Anomaly detection

### cryptogenetik-core
- Topological operators (WT, SW, DK, PI)
- Search optimization
- Pattern matching

## Security Best Practices

⚠️ **IMPORTANT**:
- Never share screenshots with visible seeds
- Clear sensitive data before screenshots
- Seeds are masked but still sensitive
- Use for research/forensics only
- Not for production wallets

## Getting Help

- **Documentation**: See `crates/phosphoros-dashboard/README.md`
- **Architecture**: See `DELTA_BLUEPRINT.md` in repo root
- **Issues**: Open issue on GitHub
- **Logs**: Check System Log panel for details

## Advanced Usage

### Running Tests
```bash
cargo test -p phosphoros-dashboard
```

### Building Release
```bash
cargo build -p phosphoros-dashboard --release
```

### Custom Configuration
```bash
# Specify config file
PHOSPHOROS_CONFIG=/path/to/config.json cargo run -p phosphoros-dashboard
```

### Logging Levels
```bash
# Debug mode (verbose)
RUST_LOG=debug cargo run -p phosphoros-dashboard

# Info mode (default)
RUST_LOG=info cargo run -p phosphoros-dashboard

# Warning only
RUST_LOG=warn cargo run -p phosphoros-dashboard
```

## What's Next?

Check the main README for:
- Full PHOSPHOROS ecosystem documentation
- Integration guides
- Advanced features
- Development roadmap

---

**Version**: 1.0.0  
**Framework**: iced 0.13  
**Status**: Fully Functional ✅

*Built with ❤️ for the PHOSPHOROS project*
