# Data Model: Windows 11 Gamer Autoclicker

## Entity: ClickProfile
- Description: Reusable user-defined clicker configuration.
- Fields:
  - `id` (string, UUID-like)
  - `name` (string, 1..64 chars, unique per device)
  - `click_action` (enum: `left|right|middle|double`)
  - `cps` (integer, 1..50)
  - `scope_mode` (enum: `focused_only|global`)
  - `position_mode` (enum: `cursor|fixed`)
  - `fixed_position_x` (integer, nullable; required when `position_mode=fixed`)
  - `fixed_position_y` (integer, nullable; required when `position_mode=fixed`)
  - `jitter_percent` (integer, 0..30)
  - `stationary_start_enabled` (boolean)
  - `stationary_seconds` (integer, 1..300, required if enabled)
  - `stop_after_enabled` (boolean)
  - `stop_after_seconds` (integer, 1..300, required if enabled)
  - `max_session_enabled` (boolean)
  - `max_session_seconds` (integer, 1..300, required if enabled)
  - `cooldown_enabled` (boolean)
  - `cooldown_seconds` (integer, 1..300, required if enabled)
  - `start_hotkey` (string)
  - `stop_hotkey` (string)
  - `panic_hotkey` (string)
  - `theme_variant` (enum: `mad_scientist_neon|high_contrast`)
  - `created_at` (datetime)
  - `updated_at` (datetime)
- Validation rules:
  - Hotkeys must be unique across start/stop/panic bindings.
  - `scope_mode` default is `focused_only`.
  - `name` must be unique case-insensitively.

## Entity: RunSession
- Description: Runtime execution record for one clicker run.
- Fields:
  - `session_id` (string)
  - `profile_id` (string, FK -> ClickProfile.id)
  - `started_at` (datetime)
  - `ended_at` (datetime, nullable while active)
  - `state` (enum: `idle|waiting_condition|running|stopped|error`)
  - `stop_reason` (enum: `manual_stop|timed_stop|panic_stop|validation_block|interruption|focus_loss_pause`)
  - `effective_cps` (integer)
  - `total_clicks` (integer)
  - `scope_mode` (enum)
  - `stationary_wait_observed_seconds` (integer, nullable)
  - `error_code` (string, nullable)
- Validation rules:
  - `ended_at >= started_at` when present.
  - `stop_reason` required when state transitions to `stopped|error`.

## Entity: ActivationCondition
- Description: Runtime condition set derived from profile and environment.
- Fields:
  - `session_id` (string, FK -> RunSession.session_id)
  - `requires_stationary` (boolean)
  - `stationary_seconds_required` (integer, nullable)
  - `stationary_seconds_accumulated` (integer)
  - `requires_focus` (boolean)
  - `target_window_focused` (boolean)
  - `eligible_to_start` (boolean)
- Validation rules:
  - `eligible_to_start` true only when all enabled conditions are satisfied.

## Entity: SafetyPolicy
- Description: Enforced safety limits for runs.
- Fields:
  - `profile_id` (string, FK -> ClickProfile.id)
  - `max_session_enabled` (boolean)
  - `max_session_seconds` (integer, nullable)
  - `cooldown_enabled` (boolean)
  - `cooldown_seconds` (integer, nullable)
  - `panic_hotkey` (string)
- Validation rules:
  - Bounds must be 1..300 when enabled.

## Entity: ThemePreference
- Description: Visual styling selection and accessibility fallback.
- Fields:
  - `profile_id` (string, FK -> ClickProfile.id)
  - `variant` (enum: `mad_scientist_neon|high_contrast`)
  - `neon_intensity` (integer, 0..100)
  - `reduced_effects` (boolean)
- Validation rules:
  - `high_contrast` must remain available regardless of other settings.

## Entity: ReleaseArtifactManifest
- Description: Metadata needed for deterministic release verification.
- Fields:
  - `version` (string)
  - `git_tag` (string)
  - `git_commit` (string)
  - `target_triple` (string)
  - `rust_toolchain` (string)
  - `build_timestamp_policy` (string)
  - `artifact_name` (string)
  - `sha256` (string)
  - `repro_instructions_url` (string)
- Validation rules:
  - `git_tag` and `git_commit` must match release source.
  - checksum format must be valid SHA-256 hex.

## Relationships
- ClickProfile 1..* RunSession
- RunSession 1..1 ActivationCondition
- ClickProfile 1..1 SafetyPolicy
- ClickProfile 1..1 ThemePreference
- ReleaseArtifactManifest is per release version, independent from profile/session runtime data.

## State Transitions
- RunSession:
  - `idle -> waiting_condition` (start requested with unmet condition)
  - `idle -> running` (start requested and conditions already met)
  - `waiting_condition -> running` (conditions satisfied)
  - `running -> stopped` (manual/timed/panic/guardrail stop)
  - `running -> waiting_condition` (focused-only mode loses focus)
  - `any -> error` (runtime fault)
  - `error -> idle` (user acknowledge/reset)
