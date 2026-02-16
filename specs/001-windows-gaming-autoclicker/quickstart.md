# Quickstart: Windows 11 Gamer Autoclicker (Rust on WSL2)

## Prerequisites
- WSL2 with Rust toolchain installed (`rustup`, stable toolchain)
- Git and GitHub access
- Windows 11 host for runtime verification
- Optional: MinGW or CI-based Windows builds for release artifacts

## 1) Bootstrap project
```bash
cargo init --bin .
rustup toolchain install stable
rustup default stable
```

## 2) Add core dependencies
```bash
cargo add eframe egui enigo global-hotkey device_query serde serde_json toml thiserror tracing tracing-subscriber
cargo add --dev proptest
```

## 3) Run tests first (TDD red phase)
Create failing tests for:
- CPS bounds (1..50)
- Timer bounds (1..300)
- Focused-only default scope
- Stationary-start state transitions
- Timed stop and panic-stop precedence

```bash
cargo test
# Expect failures before implementation
```

## 4) Implement minimal passing slice (green phase)
Start with User Story 1:
- `ClickProfile` model
- Start/stop state machine
- CPS validation and click loop scheduler
- UI status updates for idle/running/stopped

```bash
cargo test
# Expect core tests to pass
```

## 5) Add conditional and safety behavior
Implement User Story 2 and User Story 3:
- Stationary-start logic
- Stop-after timer
- Profile save/load
- Panic-stop and session guardrails
- Lightweight event log

```bash
cargo test
```

## 6) Run app locally in dev
```bash
cargo run
```

## 7) Verify release reproducibility workflow
For release builds, enforce:
- locked dependencies (`--locked`)
- pinned rust toolchain
- checksum generation for artifacts
- publish manifest + reproducibility instructions

Example verification flow:
```bash
cargo build --release --locked
sha256sum target/release/* > checksums.txt
```

## 8) CI expectations
- Linux CI: unit/integration tests, linting, formatting
- Windows CI: runtime-sensitive integration checks and release artifact production
- Release job: publish executable, checksum, and reproducibility manifest/instructions

## 9) UX acceptance smoke checks
- Neon-green theme applied to primary controls and status indicators
- High-contrast fallback available and switchable
- Start/Stop/Panic state changes are immediately visible
- Validation messages are in-context and user-readable
