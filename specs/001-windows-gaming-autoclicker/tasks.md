# Tasks: Windows 11 Gamer Autoclicker

**Input**: Design documents from `/specs/001-windows-gaming-autoclicker/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are MANDATORY. For each user story, write tests first, verify they fail (Red), then implement until they pass (Green).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root (Rust binary with internal modules)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, dependency configuration, and directory scaffolding

- [x] T001 Create project directory structure per plan.md (`src/app/`, `src/engine/`, `src/domain/`, `src/infra/`, `src/release/`, `tests/unit/`, `tests/integration/`, `tests/contract/`, `scripts/`)
- [x] T002 Initialize Rust project with `cargo init --bin` and configure Cargo.toml (edition 2024, name, version, binary target)
- [x] T003 [P] Add production dependencies to Cargo.toml (`eframe`, `egui`, `enigo`, `global-hotkey`, `device_query`, `serde`, `serde_json`, `toml`, `thiserror`, `tracing`, `tracing-subscriber`, `uuid`, `chrono`)
- [x] T004 [P] Add dev dependencies to Cargo.toml (`proptest`) and create `rust-toolchain.toml` with pinned stable toolchain

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain types, shared infrastructure, and module wiring that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Define domain enums (`ClickAction`, `ScopeMode`, `PositionMode`, `ThemeVariant`, `RunState`, `StopReason`) and shared constants (CPS bounds 1–50, timer bounds 1–300) in src/domain/settings.rs
- [x] T006 [P] Create `ClickProfile` struct with all field definitions from data-model.md in src/domain/profile.rs
- [x] T007 [P] Create `RunSession` struct with state, timing, and metrics fields in src/domain/session.rs
- [x] T008 [P] Create `EventRecord` struct with timestamp, level, event type, message, and metadata in src/domain/events.rs
- [x] T009 [P] Define typed error variants with `thiserror` (`ValidationError`, `EngineError`, `PersistenceError`, `HotkeyError`, `InputError`) in src/domain/errors.rs
- [x] T010 Setup `tracing-subscriber` initialization and JSONL file appender for session logs in src/infra/telemetry.rs
- [x] T011 Create module declarations (`mod app`, `mod engine`, `mod domain`, `mod infra`, `mod release`) and minimal app skeleton in src/main.rs

**Checkpoint**: Foundation ready — user story implementation can now begin

---

## Phase 3: User Story 1 — Start/Stop Core Auto-Clicking (Priority: P1) 🎯 MVP

**Goal**: Users can configure click button and rate, then start/stop auto-clicking via UI controls and global hotkeys during active gameplay

**Independent Test**: Select click button → set CPS → press Start → verify clicking occurs at configured rate → press Stop (UI or hotkey) → verify clicking stops and status updates

### Tests for User Story 1 (MANDATORY) ⚠️

> **NOTE: Write these tests FIRST, capture failure evidence (Red phase), then implement until they pass (Green phase)**

- [x] T012 [P] [US1] Write failing unit tests for ClickProfile CPS validation: reject <1 and >50, accept 1, 25, 50, and verify default scope is `focused_only` in tests/unit/test_profile_validation.rs
- [x] T013 [P] [US1] Write failing unit tests for click engine state transitions: `idle→running`, `running→stopped` (manual), invalid transitions rejected, and state query correctness in tests/unit/test_click_engine.rs
- [x] T014 [P] [US1] Write failing unit tests for scheduler interval calculation: CPS→interval ms conversion, jitter bounds within configured percentage, and proptest for CPS range in tests/unit/test_scheduler.rs
- [x] T015 [P] [US1] Write failing integration test for full start/stop lifecycle: create profile → start run → verify state=running → stop run → verify state=stopped with stop_reason=manual_stop in tests/integration/test_run_lifecycle.rs

### Implementation for User Story 1

- [x] T016 [US1] Implement ClickProfile validation logic: CPS bounds enforcement (1–50), required field checks, scope default to `focused_only`, hotkey uniqueness validation in src/domain/profile.rs
- [x] T017 [US1] Implement click scheduler: CPS-to-interval conversion, optional jitter percentage application (0–30%), sleep-based tick loop with configurable precision in src/engine/scheduler.rs
- [x] T018 [US1] Implement click engine state machine: `idle→running→stopped` transitions, start/stop command handling, scheduler integration, and click dispatch loop in src/engine/click_engine.rs
- [x] T019 [US1] Implement mouse click synthesis via `enigo`: left/right/middle/double click actions, focused-only scope check before each click in src/infra/input_driver.rs
- [x] T020 [US1] Implement window focus detection for default focused-only click scope using platform APIs in src/infra/input_driver.rs
- [x] T021 [US1] Implement global hotkey registration and listener for start/stop bindings via `global-hotkey` in src/infra/hotkeys.rs
- [x] T022 [US1] Implement view model: bridge between engine state and UI, expose current state/profile/run status as observable properties in src/app/view_model.rs
- [x] T023 [US1] Implement neon-green "mad scientist" theme: define color palette, accent colors, widget styling for egui `Visuals` in src/app/theme.rs
- [x] T024 [US1] Implement main UI panel: click action selector, CPS input with validation feedback, start/stop buttons, real-time status display (Idle/Running/Stopped), scope mode indicator, and stop reason in src/app/ui.rs
- [x] T025 [US1] Wire `eframe::run_native` app launch, tracing init, module bootstrapping, and engine thread spawn in src/main.rs
- [x] T026 [US1] Implement event emission for run start, stop, and error events with structured tracing spans in src/domain/events.rs

**Checkpoint**: User Story 1 is fully functional — user can configure click button/rate, start/stop via UI and hotkey, see real-time status, with neon theme applied

---

## Phase 4: User Story 2 — Conditional Start and Timed Stop (Priority: P2)

**Goal**: Users can delay clicking until mouse is stationary for X seconds and/or auto-stop after X seconds for precise timing control

**Independent Test**: Enable stationary-start (2s) → press Start → move mouse → stop moving → verify clicking begins after 2s stationary → enable stop-after (30s) → verify clicking stops at 30s with reason=timed_stop

### Tests for User Story 2 (MANDATORY) ⚠️

- [x] T027 [P] [US2] Write failing unit tests for stationary detection: position polling detects no-change window, resets on movement, fires eligible after threshold, validates bounds 1–300s in tests/unit/test_conditions.rs
- [x] T028 [P] [US2] Write failing unit tests for timed-stop countdown: stops at exact duration, stop_reason=timed_stop, validates bounds 1–300s, proptest for timer range in tests/unit/test_timed_stop.rs
- [x] T029 [P] [US2] Write failing integration test for conditional start → timed stop: enable both conditions → start → satisfy stationary → verify running → verify auto-stop at configured duration in tests/integration/test_conditional_run.rs

### Implementation for User Story 2

- [x] T030 [US2] Implement `ActivationCondition` model: stationary threshold tracking, focus requirement, eligibility evaluation in src/engine/conditions.rs
- [x] T031 [US2] Implement mouse position polling via `device_query`: fixed-cadence sampling, no-change window accumulation, movement reset in src/infra/input_driver.rs
- [x] T032 [US2] Implement stationary-start state machine: `idle→waiting_condition` transition when conditions unmet, `waiting_condition→running` when stationary threshold satisfied in src/engine/conditions.rs
- [x] T033 [US2] Implement stop-after countdown timer: start countdown on run begin, trigger `running→stopped` with `stop_reason=timed_stop` at expiry in src/engine/click_engine.rs
- [x] T034 [US2] Extend RunSession with `waiting_condition` state, `stationary_wait_observed_seconds` tracking, and condition-related stop reasons in src/domain/session.rs
- [x] T035 [US2] Update UI to show condition status: "Waiting for stationary…" indicator, countdown timer display, condition settings group with labeled inputs in src/app/ui.rs
- [x] T036 [US2] Add event logging for condition-waiting start, stationary threshold met, timed-stop trigger, and stop-before-condition-met edge case in src/domain/events.rs

**Checkpoint**: User Stories 1 AND 2 both work independently — conditional start and timed stop layer cleanly onto core clicking

---

## Phase 5: User Story 3 — Gamer Profiles and Safety Controls (Priority: P3)

**Goal**: Users can save/switch profiles with different settings, use panic stop to immediately halt clicking, and enforce session guardrails (max duration, cooldown)

**Independent Test**: Create profile → save → switch to different profile → verify settings update → start run → trigger panic stop → verify immediate halt → enable max session → verify enforcement

### Tests for User Story 3 (MANDATORY) ⚠️

- [x] T037 [P] [US3] Write failing unit tests for profile CRUD: create, rename, duplicate, delete, unique name enforcement (case-insensitive), TOML round-trip serialization in tests/unit/test_profile_persistence.rs
- [x] T038 [P] [US3] Write failing unit tests for panic-stop override: panic stops running session regardless of conditions, overrides timed-stop, overrides stationary-wait in tests/unit/test_safety.rs
- [x] T039 [P] [US3] Write failing unit tests for max session and cooldown: max duration enforced, cooldown blocks re-start, bounds validation 1–300s in tests/unit/test_safety_policy.rs
- [x] T040 [P] [US3] Write failing integration test for profile switch → run: load profile → switch → start run → verify effective settings match selected profile in tests/integration/test_profiles.rs

### Implementation for User Story 3

- [x] T041 [US3] Implement `SafetyPolicy` model: max session duration, cooldown duration, panic hotkey binding, and validation rules (bounds 1–300 when enabled) in src/engine/safety.rs
- [x] T042 [US3] Implement TOML-based profile persistence: save to user app data directory, load all profiles, delete by ID, file-per-profile storage in src/infra/persistence.rs
- [x] T043 [US3] Implement profile CRUD operations: create with defaults, rename with uniqueness check, duplicate, delete with confirmation, list all in src/domain/profile.rs
- [x] T044 [US3] Implement panic-stop handler: immediate state transition to stopped with `stop_reason=panic_stop`, override all active timers and conditions in src/engine/click_engine.rs
- [x] T045 [US3] Implement max session duration enforcement and cooldown timer between runs in src/engine/safety.rs
- [x] T046 [US3] Implement last-active profile persistence: save active profile ID on switch, restore on app launch in src/infra/persistence.rs
- [x] T047 [US3] Register global panic hotkey binding via `global-hotkey`, ensure it is always active regardless of app focus in src/infra/hotkeys.rs
- [x] T048 [US3] Update UI with profile list/selector, create/rename/duplicate/delete controls, safety settings panel, and high-contrast fallback theme toggle in src/app/ui.rs
- [x] T049 [US3] Implement high-contrast fallback theme as alternative to neon theme, switchable per profile in src/app/theme.rs
- [x] T050 [US3] Add event logging for profile create/switch/delete, panic-stop trigger, max session enforcement, and cooldown block in src/domain/events.rs

**Checkpoint**: All three user stories are independently functional — profiles persist, safety controls enforce, panic stop overrides all

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Integration polish, release infrastructure, documentation, and validation across all user stories

- [x] T051 [P] Implement scrollable event log viewer panel showing recent run/stop/condition/error events in src/app/ui.rs
- [x] T052 [P] Implement release artifact manifest generation (version, git tag, commit, toolchain, SHA-256) in src/release/reproducibility.rs
- [x] T053 [P] Create reproducible build verification script with checksum comparison in scripts/verify-reproducible-build.sh
- [x] T054 [P] Create CI workflow for `cargo test`, `cargo clippy`, `cargo fmt --check` on Linux runner in .github/workflows/ci.yml
- [x] T055 [P] Create release workflow: Windows cross-compile, `--locked` build, checksum generation, manifest publishing to GitHub Releases in .github/workflows/release.yml
- [x] T056 Add README.md with project overview, installation from GitHub Releases, build-from-source instructions, and reproducibility verification guide
- [x] T057 Run quickstart.md validation: execute all 9 steps end-to-end, verify TDD workflow, runtime launch, and release reproducibility

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational phase completion
- **User Story 2 (Phase 4)**: Depends on Foundational phase completion; integrates with engine from US1
- **User Story 3 (Phase 5)**: Depends on Foundational phase completion; integrates with engine from US1
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) — No dependencies on other stories. This IS the MVP.
- **User Story 2 (P2)**: Extends engine from US1 (conditions and timed-stop layer on click_engine). Recommended after US1 but independently testable.
- **User Story 3 (P3)**: Profile persistence and safety are independent of conditional start. Recommended after US1 but independently testable.

### Within Each User Story

1. Tests MUST be written and FAIL before implementation (Red phase)
2. Models/domain before engine logic
3. Engine logic before infrastructure adapters
4. Infrastructure before UI integration
5. UI wiring last
6. Story complete and passing before moving to next priority

### Parallel Opportunities

**Phase 1**: T003 and T004 can run in parallel (different files)
**Phase 2**: T006, T007, T008, T009 can all run in parallel (different source files)
**Phase 3 tests**: T012, T013, T014, T015 can all run in parallel (different test files)
**Phase 4 tests**: T027, T028, T029 can all run in parallel (different test files)
**Phase 5 tests**: T037, T038, T039, T040 can all run in parallel (different test files)
**Phase 6**: T051–T055 can all run in parallel (different files/workflows)
**Cross-story**: After Foundational completes, US2 and US3 can theoretically run in parallel if US1 engine interfaces are stable

---

## Parallel Example: User Story 1

```bash
# Launch all US1 tests together (Red phase):
T012: "Write failing unit tests for CPS validation in tests/unit/test_profile_validation.rs"
T013: "Write failing unit tests for click engine state transitions in tests/unit/test_click_engine.rs"
T014: "Write failing unit tests for scheduler interval in tests/unit/test_scheduler.rs"
T015: "Write failing integration test for start/stop lifecycle in tests/integration/test_run_lifecycle.rs"

# Verify all tests fail (Red evidence captured)

# Then implement sequentially (Green phase):
T016 → T017 → T018 → T019 → T020 → T021 → T022 → T023 → T024 → T025 → T026
```

## Parallel Example: User Story 3

```bash
# Launch all US3 tests together (Red phase):
T037: "Write failing unit tests for profile CRUD in tests/unit/test_profile_persistence.rs"
T038: "Write failing unit tests for panic-stop override in tests/unit/test_safety.rs"
T039: "Write failing unit tests for max session and cooldown in tests/unit/test_safety_policy.rs"
T040: "Write failing integration test for profile switch in tests/integration/test_profiles.rs"

# Verify all tests fail (Red evidence captured)

# Then implement sequentially (Green phase):
T041 → T042 → T043 → T044 → T045 → T046 → T047 → T048 → T049 → T050
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — blocks all stories)
3. Complete Phase 3: User Story 1 (core clicking + neon theme)
4. **STOP and VALIDATE**: Test US1 independently — configure, start, stop, hotkey, status display
5. Deploy/demo if ready — this is a usable autoclicker

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → **MVP!** (core auto-clicking works)
3. Add User Story 2 → Test independently → Conditional start + timed stop layered on
4. Add User Story 3 → Test independently → Profiles + safety + high-contrast theme
5. Polish → Release pipeline, event log viewer, README, CI/CD
6. Each story adds value without breaking previous stories

### Suggested MVP Scope

**User Story 1 alone** (Phases 1–3, tasks T001–T026) delivers a fully functional autoclicker with:
- Configurable click button and rate (1–50 CPS)
- Start/stop via UI and global hotkeys
- Real-time status display
- Focused-only default scope
- Neon green "mad scientist" theme
- Structured event logging

---

## Notes

- [P] tasks = different files, no dependencies on incomplete tasks
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Verify tests fail before implementing (Red phase evidence required per constitution)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- All file paths reference the project structure defined in plan.md
