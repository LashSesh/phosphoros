# PHOSPHOROS Dashboard

**Living Lab Interface for Autonomous Infogenetic System**

A sophisticated GUI dashboard built with the [iced](https://github.com/iced-rs/iced) framework, implementing the PHOSPHOROS unified agent blueprint for autonomous operation, real-time visualization, and modular panel-based interaction.

## Overview

PHOSPHOROS Dashboard is not a simple UI tool—it's a **living, autonomous laboratory** that continuously operates in the background while providing an intuitive interface for monitoring, control, and interaction.

### Key Features

✅ **Living System Architecture**
- Autonomous background services (Scraper, Analyzer, Cluster Engine)
- Continuous learning and data accumulation
- System grows more powerful with usage

✅ **Modular Panel System**
- Live Overview (Home) - Real-time system status
- Seed & Wallet Management - Import and manage cryptographic seeds
- Resonance & Spectrography - 5D-spectral analysis visualization
- Cluster Explorer - Forensic cluster discovery
- System Log - Complete activity logging
- Settings & Tasks - Service configuration and control

✅ **Professional UX/UI**
- Dark mode by default (#1E1E1E background, #C0C0C0 text)
- Responsive layout (min 1280x720, optimal 1600x900)
- Instant feedback for all actions
- Contextual tooltips (planned)
- Toast notifications
- Progress indicators

✅ **Autonomous Services**
- Services run independently in background
- User can pause/resume/configure at any time
- System continues to learn even when idle
- Persistent state across sessions

✅ **Integration with PHOSPHOROS Ecosystem**
- Full integration with `phosphoros-core` (5D geometry, resonance)
- Multichain wallet support via `phosphoros-bip39`
- Blockchain forensics via `phosphoros-satellite`
- Topological optimization via `cryptogenetik-core`

## Architecture

### Elm Architecture Pattern

The dashboard follows the **Model-Update-View** pattern:

```
┌──────────────┐
│    Model     │ ← Application State
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Update     │ ← Message Handlers
└──────┬───────┘
       │
       ▼
┌──────────────┐
│     View     │ ← UI Rendering
└──────────────┘
```

### Component Structure

```
phosphoros-dashboard/
├── src/
│   ├── main.rs           # Entry point
│   ├── app.rs            # Main application logic
│   ├── messages.rs       # Message types (Elm Update)
│   ├── state.rs          # Application state (Elm Model)
│   ├── config.rs         # Configuration management
│   ├── panels/           # Panel widgets
│   │   └── mod.rs        # Panel state definitions
│   ├── services/         # Background services
│   │   └── mod.rs        # Service manager
│   ├── theme/            # Visual theme
│   │   └── mod.rs        # Colors and styles
│   └── widgets/          # Custom widgets
│       └── mod.rs        # Widget helpers
└── Cargo.toml
```

### Message Flow

All interactions follow a message-based architecture:

```
User Action → Message → Update Handler → State Change → View Refresh
```

## Quick Start

### Building

```bash
# Build the dashboard
cargo build -p phosphoros-dashboard --release

# Run the dashboard
cargo run -p phosphoros-dashboard --release
```

### Development

```bash
# Run in debug mode with logging
RUST_LOG=debug cargo run -p phosphoros-dashboard

# Format code
cargo fmt -p phosphoros-dashboard

# Run clippy
cargo clippy -p phosphoros-dashboard -- -D warnings
```

## Panels

### 1. Live Overview (Home) 🏠

Real-time system status dashboard:
- Active tasks indicator
- Seeds collected counter
- Clusters discovered counter
- Coverage percentage
- Activity history graph
- Service status (Scraper, Analyzer, Cluster Engine)

### 2. Seed & Wallet Management 🔑

Cryptographic seed and wallet operations:
- Import mnemonics, seeds, or private keys
- Multichain address generation
- Seed masking for security
- Copy addresses to clipboard
- Import history with timestamps

**Security**: Seeds are masked in UI, never logged in plain text.

### 3. Resonance & Spectrography 📊

5D-spectral resonance analysis:
- Start/stop resonance analysis
- Real-time progress tracking
- Best resonance score display
- Operator configuration (WT, SW, DK, PI)
- Resonance history visualization
- Spectral signature (ψ, ρ, ω) display

### 4. Cluster Explorer 🔍

Forensic cluster discovery and analysis:
- Search and filter clusters
- Cluster member count
- Resonance magnitude
- Discovery timestamps
- Export/import capabilities

### 5. Search Space Explorer 🗺️

**NEW**: Manual navigation through cryptographic seed/address space:
- **Navigation Modes**:
  - Manual: Point-and-click exploration
  - Sequential: Step through space systematically
  - Random Walk: Stochastic exploration
  - Directed Search: Resonance-gradient guided navigation
- **View Modes**:
  - Tree: Hierarchical view of search space
  - Graph: Network visualization of relationships
  - List: Simple linear view
  - 5D Projection: High-dimensional visualization
- **Features**:
  - Exploration history with breadcrumb trail
  - Resonance threshold filtering
  - Jump to high-resonance regions
  - Export exploration paths
  - Real-time visible node display

### 6. Network Topology Explorer 🕸️

**NEW**: Interactive blockchain network visualization and analysis:
- **Layout Modes**:
  - Force-Directed: Physics-based layout
  - Hierarchical: Tree-like structure
  - Circular: Ring layout
  - Geographic: Geo-coordinated (if available)
- **Analysis Tools**:
  - Community detection (clustering)
  - Critical node identification (centrality)
  - Path finding between nodes
  - Edge weight filtering
- **Features**:
  - Node type filtering
  - Interactive node selection
  - Network metrics display
  - Export network data

### 7. Infogenetic Database Browser 🧬

**NEW**: Advanced querying and exploration of the spectral database:
- **Query Types**:
  - Address search
  - Spectral signature range queries
  - Cluster membership search
  - Full-text search
  - Advanced custom queries
- **Features**:
  - Multi-criteria filtering
  - Flexible sorting (timestamp, resonance, address, cluster size)
  - Pagination support
  - Detailed entry inspection
  - Export search results
- **Display**:
  - Spectral signatures (ψ, ρ, ω)
  - Resonance scores
  - Chain identification
  - Discovery timestamps
  - Metadata viewing

### 8. Anomaly Investigation 🚨

**NEW**: Deep forensic analysis of detected blockchain anomalies:
- **Anomaly Types**:
  - Sybil attacks
  - Money laundering patterns
  - Volume anomalies
  - Temporal anomalies
  - Structural anomalies
- **Investigation Tools**:
  - Related entity discovery
  - Timeline analysis
  - Similar case matching
  - Investigation notes
- **Features**:
  - Severity filtering (Low, Medium, High, Critical)
  - Type-based filtering
  - Affected entity tracking
  - Visual timeline display
  - Export investigation reports

### 9. Forensic Workflows 🔬

**NEW**: Guided step-by-step investigation processes:
- **Pre-built Workflows**:
  1. **Sybil Attack Investigation** (30-45 min)
     - Identify seed cluster
     - Analyze temporal patterns
     - Map network topology
     - Calculate resonance signatures
     - Generate report
  2. **Money Laundering Trace** (45-60 min)
     - Identify origin transaction
     - Trace fund flow
     - Identify mixing services
     - Map destination clusters
  3. **Comprehensive Address Profiling** (20-30 min)
     - Gather address data
     - Analyze transaction patterns
     - Compute spectral signature
     - Find similar addresses
  4. **Cluster Forensics Analysis** (30-40 min)
     - Load cluster data
     - Analyze internal structure
     - Identify external connections
     - Compute cluster resonance
- **Features**:
  - Step-by-step guidance
  - Progress tracking
  - Data collection at each step
  - Workflow history
  - Export workflow results

### 10. Stealth/Privacy 🔒

Advanced privacy and stealth networking controls:
- Traffic mimicry modes
- Proxy rotation
- Temporal jitter
- Request logging controls
- Compliance notices

### 11. System Log 📝

Complete activity logging:
- Filterable log entries
- Log levels (Debug, Info, Warning, Error)
- Timestamps and sources
- Auto-scroll option
- Clear logs functionality

### 12. Settings & Tasks ⚙️

System configuration and control:
- Dark/Light theme toggle
- Language selection
- Service priorities
- Auto-start configuration
- Pause/resume all services

## Services

### Autonomous Background Services

Three independent services run continuously:

1. **Scraper Service**
   - Crawls seedspace and addressspace
   - Collects cryptographic data
   - Builds database continuously
   
2. **Analyzer Service**
   - Pattern recognition
   - Anomaly detection
   - Cluster formation
   
3. **Cluster Engine**
   - KNN graph construction
   - Resonance hotspot detection
   - Topological analysis

All services:
- Run independently in background threads
- Can be paused/resumed individually or collectively
- Report status via message passing
- Persist state across sessions

## Theme

### Dark Mode (Default)

Following blueprint specifications:

- **Background**: `#1E1E1E` (rgb: 0.118, 0.118, 0.118)
- **Text**: `#C0C0C0` (rgb: 0.753, 0.753, 0.753)
- **Primary**: Blue accent for active elements
- **Success**: Green for positive states
- **Warning**: Orange for caution
- **Error**: Red for errors
- **Info**: Blue for information

### Responsive Breakpoints

- **Minimum**: 1280x720 (panels may collapse)
- **Optimal**: 1600x900 (full layout)
- **Maximum**: Scales to any size

## Integration

### Using PHOSPHOROS Core

```rust
use phosphoros_core::{HolisticMatrix, ResonanceEngine};

// Create resonance engine
let mut engine = HolisticMatrix::default_config();

// Evaluate resonance
let eval = engine.evaluate(t, perception, intention, gradient, theta);
```

### Using BIP39

```rust
use phosphoros_bip39::{Mnemonic, Language};

// Generate mnemonic
let mnemonic = Mnemonic::generate(128, Language::English)?;

// Derive multichain addresses
#[cfg(feature = "btc")]
let btc_address = derive_bitcoin_address(&mnemonic)?;
```

### Using Satellite Forensics

```rust
use phosphoros_satellite::{SatelliteEngine, AnalyticsPipeline};

// Analyze blockchain data
let engine = SatelliteEngine::default();
let report = engine.analyze(snapshot_id, request)?;
```

## Configuration

Configuration is stored in JSON format:

**Default path**: `~/.config/phosphoros/config.json` (Linux/macOS) or `%APPDATA%\phosphoros\config.json` (Windows)

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

## Future Enhancements

### Planned Features

- [ ] 5D field visualization with WebGL/Canvas
- [ ] Drag-and-drop panel rearrangement
- [ ] Undo/redo for all actions
- [ ] Enhanced tooltips with contextual help
- [ ] Data export (JSON, CSV, YAML)
- [ ] Import/export cluster configurations
- [ ] Real-time spectrography visualization
- [ ] Guided onboarding tour
- [ ] Keyboard shortcuts
- [ ] Plugin system for custom operators

### Performance Goals

- [ ] 60 FPS rendering
- [ ] < 100ms response time for all actions
- [ ] Efficient memory usage (< 200MB idle)
- [ ] Background services with minimal CPU impact

## Development Guidelines

### Adding New Panels

1. Define panel ID in `panels/mod.rs`
2. Add panel state structure
3. Create message types in `messages.rs`
4. Implement panel view in `app.rs`
5. Add message handlers
6. Update sidebar navigation

### Adding New Services

1. Define service state in `services/mod.rs`
2. Implement service logic
3. Add message types
4. Integrate with `ServiceManager`
5. Add UI controls in Settings panel

### Styling

- Use theme colors from `theme/colors`
- Apply consistent padding (12-20px)
- Use card widgets for grouped content
- Follow iced widget patterns

## Security Considerations

⚠️ **IMPORTANT**:
- Seeds are masked in UI
- Never log sensitive data
- Clipboard operations are temporary
- Configuration files don't store secrets
- All services run locally (no external connections without explicit user action)

## License

Dual-licensed under MIT or Apache 2.0.

## Acknowledgments

Built with:
- [iced](https://github.com/iced-rs/iced) - Cross-platform GUI framework
- [tokio](https://tokio.rs/) - Async runtime
- PHOSPHOROS ecosystem (core, bip39, satellite, cryptogenetik)

---

**Version**: 1.0.0  
**Status**: Initial Implementation  
**Framework**: iced 0.12  

*Part of the PHOSPHOROS project - Living Lab for Infogenetic Research*
