use thiserror::Error;

// ── Validation Error ──────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Field '{0}' cannot be empty")]
    EmptyField(String),

    #[error("Field '{field}' exceeds maximum length of {max}")]
    FieldTooLong { field: String, max: usize },

    #[error("Field '{field}' value {actual} is out of range [{min}, {max}]")]
    OutOfRange {
        field: String,
        min: u32,
        max: u32,
        actual: u32,
    },

    #[error("Required field: {0}")]
    RequiredField(String),

    #[error("Duplicate hotkey binding: '{0}'")]
    DuplicateHotkey(String),

    #[error("Profile name '{0}' already exists")]
    DuplicateProfileName(String),
}

// ── Engine Error ──────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Invalid state transition: {0}")]
    InvalidTransition(String),

    #[error("Engine is not running")]
    NotRunning,

    #[error("Engine is already running")]
    AlreadyRunning,

    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),

    #[error("Cooldown active, {remaining_secs}s remaining")]
    CooldownActive { remaining_secs: u32 },
}

// ── Persistence Error ─────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Profile not found: {0}")]
    NotFound(String),
}

// ── Hotkey Error ──────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum HotkeyError {
    #[error("Failed to register hotkey '{0}': {1}")]
    Registration(String, String),

    #[error("Failed to unregister hotkey '{0}': {1}")]
    Unregistration(String, String),

    #[error("Hotkey conflict: '{0}' is already bound")]
    Conflict(String),
}

// ── Input Error ───────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Click simulation failed: {0}")]
    ClickFailed(String),

    #[error("Mouse position query failed: {0}")]
    PositionQueryFailed(String),

    #[error("Window focus detection failed: {0}")]
    FocusDetectionFailed(String),
}
