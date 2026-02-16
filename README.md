# ⚗ NOS Autoclicker

A Windows 11 gaming autoclicker with configurable click actions, profiles, and safety controls. Built in Rust with an immediate-mode GUI (egui/eframe).

## Features

- **Click Configuration**: Left, right, middle, double click at 1–50 CPS with 0–30% jitter
- **Conditional Start**: Delay clicking until mouse is stationary for X seconds
- **Timed Stop**: Auto-stop after X seconds for precise timing control
- **Profiles**: Save/load/switch click profiles with TOML persistence
- **Global Hotkeys**: Start, stop, and panic-stop via configurable hotkeys (default: F6/F7/F8)
- **Safety Controls**: Max session duration, cooldown between runs, panic stop
- **Theme**: Neon green "mad scientist" theme with high-contrast fallback
- **Event Log**: Real-time scrollable event viewer

## Installation

### From GitHub Releases

1. Go to [Releases](https://github.com/nos/nos-autoclicker/releases)
2. Download `nos-autoclicker.exe`
3. Verify the checksum: `certutil -hashfile nos-autoclicker.exe SHA256`
4. Compare with the `.sha256` file from the release

### Build from Source

**Prerequisites**: Rust stable toolchain (1.84+)

```bash
# Clone the repository
git clone https://github.com/nos/nos-autoclicker.git
cd nos-autoclicker

# Build optimized release binary
cargo build --release --locked

# Binary will be at target/release/nos-autoclicker(.exe)
```

### Cross-compile for Windows (from Linux)

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt-get install mingw-w64
cargo build --release --locked --target x86_64-pc-windows-gnu
```

## Usage

```bash
# Run the autoclicker
./target/release/nos-autoclicker

# Or via cargo
cargo run --release
```

### Quick Start

1. Launch the application
2. Select click action (Left/Right/Middle/Double)
3. Set CPS (1–50) and optional jitter percentage
4. Configure scope (Focused Only or Global)
5. Press **▶ START** or the Start hotkey (default: F6)
6. Press **⏹ STOP** or F7 to stop
7. Use **⚡ PANIC STOP** or F8 for emergency halt

### Profiles

- Create named profiles with different configurations
- Switch between profiles instantly
- Profiles are saved as TOML files in the system app data directory
- Last-active profile is restored on launch

### Safety Controls

- **Max Session**: Auto-stop after a configurable duration (1–300s)
- **Cooldown**: Enforce a waiting period between runs (1–300s)
- **Panic Stop**: Immediate halt that overrides all active timers

## Development

```bash
# Run tests
cargo test

# Run clippy lints
cargo clippy --all-targets

# Format code
cargo fmt

# Run a specific test suite
cargo test --test test_profile_validation
cargo test --test test_click_engine
cargo test --test test_scheduler
```

## Reproducible Builds

Verify that builds produce identical binaries:

```bash
chmod +x scripts/verify-reproducible-build.sh
./scripts/verify-reproducible-build.sh
```

The release workflow generates a `release-manifest.json` with git tag, commit, toolchain version, and SHA-256 checksum for each release.

## Architecture

```
src/
├── app/           # UI layer (egui/eframe)
│   ├── ui.rs      # Main application panels
│   ├── theme.rs   # Neon + high-contrast themes
│   └── view_model.rs  # Engine-UI bridge
├── domain/        # Core types
│   ├── profile.rs # ClickProfile with validation
│   ├── session.rs # RunSession state machine
│   ├── events.rs  # EventRecord with tracing
│   ├── settings.rs # Enums and constants
│   └── errors.rs  # Typed error variants
├── engine/        # Click engine
│   ├── click_engine.rs # State machine + click loop
│   ├── scheduler.rs    # CPS-to-interval conversion
│   ├── conditions.rs   # Stationary-start detection
│   └── safety.rs       # Max session + cooldown
├── infra/         # Infrastructure
│   ├── input_driver.rs # Mouse click synthesis (enigo)
│   ├── hotkeys.rs      # Global hotkey management
│   ├── persistence.rs  # TOML profile storage
│   └── telemetry.rs    # Structured logging
└── release/       # Build metadata
    └── reproducibility.rs
```

## License

MIT
