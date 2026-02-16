# Research: Windows 11 Gamer Autoclicker

## Decision 1: Desktop UI stack uses `eframe/egui`
- Decision: Use `eframe/egui` for the desktop UI shell and rendering.
- Rationale: Rust-native, fast iteration, cross-platform development from WSL2 while targeting Windows; supports custom neon-themed styling and immediate-mode state visibility.
- Alternatives considered:
  - `tauri` + web UI: stronger web ecosystem but adds web stack complexity and heavier cross-boundary integration.
  - `iced`: viable Rust UI option but less mature ecosystem for this team’s immediate need.

## Decision 2: Input simulation via `enigo`
- Decision: Use `enigo` for mouse button click synthesis on Windows.
- Rationale: Rust crate support for mouse events with straightforward API suitable for bounded CPS scheduling.
- Alternatives considered:
  - Direct Win32 API wrappers (`windows` crate): more control but significantly higher complexity for MVP.
  - `rdev`: useful for hooks, less focused for click synthesis itself.

## Decision 3: Global hotkeys via `global-hotkey`
- Decision: Register start/stop/panic hotkeys with `global-hotkey`.
- Rationale: Clear abstraction for global shortcuts and conflict handling that maps to FR-003 and FR-007.
- Alternatives considered:
  - Manual Win32 hotkey registration: low-level and error-prone for MVP.

## Decision 4: Mouse stationary detection via position polling (`device_query`)
- Decision: Sample pointer position on fixed cadence and evaluate no-change window for 1–300s thresholds.
- Rationale: Deterministic, testable state-machine behavior without heavy event-hook complexity.
- Alternatives considered:
  - Event-hook-only approach: can be efficient but harder to keep deterministic and test in isolation.

## Decision 5: Configuration persistence as local TOML with `serde`
- Decision: Store profiles/settings in TOML under user application data directory.
- Rationale: Human-readable, versionable schema, no database dependency, aligns with single-user offline constraints.
- Alternatives considered:
  - SQLite: robust but unnecessary for <=100 local profiles and adds migration overhead.

## Decision 6: Observability with `tracing` and JSONL run logs
- Decision: Emit structured events through `tracing`; persist run/session records to local JSONL.
- Rationale: Supports constitution observability principle and easy debugability for timing/state transitions.
- Alternatives considered:
  - Unstructured plain text logs: simpler initially but weaker for automated analysis and verification.

## Decision 7: Deterministic release strategy for GitHub Releases
- Decision: Use locked dependencies, pinned toolchain, normalized build environment, and reproducibility verification script before publishing artifacts.
- Rationale: Meets FR-022..FR-024 and SC-008..SC-009 requirements for user-verifiable binary provenance.
- Alternatives considered:
  - Standard CI release without reproducibility checks: simpler but fails explicit trust/provenance requirements.

## Decision 8: Local control contract exposed as loopback API
- Decision: Define a localhost control API contract (OpenAPI) between UI/control surface and engine orchestration.
- Rationale: Gives explicit contract artifacts for testing and future automation without introducing external service dependencies.
- Alternatives considered:
  - No formal contract (direct calls only): simpler, but weakens contract testing and evolvability.

## Decision 9: WSL2 development with Windows runtime target
- Decision: Develop from WSL2 and build/test core logic in Linux; run Windows-specific integration/release jobs in CI runners targeting Windows.
- Rationale: Matches developer environment while ensuring runtime correctness for Windows 11.
- Alternatives considered:
  - Linux-only runtime target: incompatible with required Windows click/hotkey behavior.
  - Windows-only local dev requirement: reduces contributor accessibility.

## Decision 10: TDD enforcement strategy
- Decision: Require test-first tasks per story with explicit red-phase evidence in task execution notes.
- Rationale: Aligns with constitution principle II and reduces timing/state regressions in engine logic.
- Alternatives considered:
  - Test-after implementation: faster short-term but violates constitution and increases rework risk.
