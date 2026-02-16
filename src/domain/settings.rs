use serde::{Deserialize, Serialize};

// ── Click Action ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClickAction {
    Left,
    Right,
    Middle,
    Double,
}

impl Default for ClickAction {
    fn default() -> Self {
        Self::Left
    }
}

impl std::fmt::Display for ClickAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "Left Click"),
            Self::Right => write!(f, "Right Click"),
            Self::Middle => write!(f, "Middle Click"),
            Self::Double => write!(f, "Double Click"),
        }
    }
}

// ── Scope Mode ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeMode {
    FocusedOnly,
    Global,
}

impl Default for ScopeMode {
    fn default() -> Self {
        Self::FocusedOnly
    }
}

impl std::fmt::Display for ScopeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FocusedOnly => write!(f, "Focused Only"),
            Self::Global => write!(f, "Global"),
        }
    }
}

// ── Position Mode ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionMode {
    Cursor,
    Fixed,
}

impl Default for PositionMode {
    fn default() -> Self {
        Self::Cursor
    }
}

// ── Theme Variant ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeVariant {
    MadScientistNeon,
    HighContrast,
}

impl Default for ThemeVariant {
    fn default() -> Self {
        Self::MadScientistNeon
    }
}

// ── Run State ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    Idle,
    WaitingCondition,
    Running,
    Stopped,
    Error,
}

impl Default for RunState {
    fn default() -> Self {
        Self::Idle
    }
}

impl std::fmt::Display for RunState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::WaitingCondition => write!(f, "Waiting for Condition"),
            Self::Running => write!(f, "Running"),
            Self::Stopped => write!(f, "Stopped"),
            Self::Error => write!(f, "Error"),
        }
    }
}

// ── Stop Reason ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    ManualStop,
    TimedStop,
    PanicStop,
    ValidationBlock,
    Interruption,
    FocusLossPause,
    MaxSessionReached,
}

impl std::fmt::Display for StopReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManualStop => write!(f, "Manual Stop"),
            Self::TimedStop => write!(f, "Timed Stop"),
            Self::PanicStop => write!(f, "Panic Stop"),
            Self::ValidationBlock => write!(f, "Validation Block"),
            Self::Interruption => write!(f, "Interruption"),
            Self::FocusLossPause => write!(f, "Focus Loss Pause"),
            Self::MaxSessionReached => write!(f, "Max Session Reached"),
        }
    }
}

// ── Shared Constants ──────────────────────────────────────────────────────────

/// Minimum clicks per second
pub const CPS_MIN: u32 = 1;
/// Maximum clicks per second
pub const CPS_MAX: u32 = 50;

/// Minimum timer duration in seconds (stationary-start, stop-after, max session, cooldown)
pub const TIMER_MIN_SECONDS: u32 = 1;
/// Maximum timer duration in seconds
pub const TIMER_MAX_SECONDS: u32 = 300;

/// Maximum jitter percentage (0–30%)
pub const JITTER_MIN_PERCENT: u32 = 0;
pub const JITTER_MAX_PERCENT: u32 = 30;

/// Maximum profile name length
pub const PROFILE_NAME_MAX_LEN: usize = 64;
