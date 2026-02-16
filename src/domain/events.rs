use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing;

// ── Event Level ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for EventLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug => write!(f, "DEBUG"),
            Self::Info => write!(f, "INFO"),
            Self::Warn => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

// ── Event Record ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub timestamp: DateTime<Utc>,
    pub level: EventLevel,
    pub event_type: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

impl EventRecord {
    pub fn new(level: EventLevel, event_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            event_type: event_type.into(),
            message: message.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Emit this event via structured tracing at the appropriate level.
    pub fn emit(&self) {
        let meta_str = if self.metadata.is_empty() {
            String::new()
        } else {
            serde_json::to_string(&self.metadata).unwrap_or_default()
        };

        match self.level {
            EventLevel::Debug => {
                tracing::debug!(
                    event_type = %self.event_type,
                    metadata = %meta_str,
                    "{}", self.message
                );
            }
            EventLevel::Info => {
                tracing::info!(
                    event_type = %self.event_type,
                    metadata = %meta_str,
                    "{}", self.message
                );
            }
            EventLevel::Warn => {
                tracing::warn!(
                    event_type = %self.event_type,
                    metadata = %meta_str,
                    "{}", self.message
                );
            }
            EventLevel::Error => {
                tracing::error!(
                    event_type = %self.event_type,
                    metadata = %meta_str,
                    "{}", self.message
                );
            }
        }
    }

    // ── Convenience constructors ──────────────────────────────────────────

    pub fn run_started(profile_id: &str, cps: u32) -> Self {
        Self::new(EventLevel::Info, "run_started", format!("Run started with CPS={cps}"))
            .with_metadata("profile_id", profile_id)
            .with_metadata("cps", cps.to_string())
    }

    pub fn run_stopped(reason: &str) -> Self {
        Self::new(EventLevel::Info, "run_stopped", format!("Run stopped: {reason}"))
            .with_metadata("stop_reason", reason)
    }

    pub fn panic_stop() -> Self {
        Self::new(EventLevel::Warn, "panic_stop", "Panic stop triggered")
    }

    pub fn validation_error(message: &str) -> Self {
        Self::new(EventLevel::Error, "validation_error", message)
    }

    pub fn condition_waiting(condition: &str) -> Self {
        Self::new(
            EventLevel::Info,
            "condition_waiting",
            format!("Waiting for condition: {condition}"),
        )
    }

    pub fn condition_met(condition: &str) -> Self {
        Self::new(
            EventLevel::Info,
            "condition_met",
            format!("Condition satisfied: {condition}"),
        )
    }

    pub fn timed_stop(duration_secs: u32) -> Self {
        Self::new(
            EventLevel::Info,
            "timed_stop",
            format!("Auto-stopped after {duration_secs}s"),
        )
        .with_metadata("duration_seconds", duration_secs.to_string())
    }

    pub fn profile_created(name: &str) -> Self {
        Self::new(EventLevel::Info, "profile_created", format!("Profile created: {name}"))
    }

    pub fn profile_switched(name: &str) -> Self {
        Self::new(
            EventLevel::Info,
            "profile_switched",
            format!("Switched to profile: {name}"),
        )
    }

    pub fn profile_deleted(name: &str) -> Self {
        Self::new(
            EventLevel::Info,
            "profile_deleted",
            format!("Profile deleted: {name}"),
        )
    }

    pub fn max_session_reached(seconds: u32) -> Self {
        Self::new(
            EventLevel::Warn,
            "max_session_reached",
            format!("Max session duration reached: {seconds}s"),
        )
    }

    pub fn cooldown_blocked(remaining_secs: u32) -> Self {
        Self::new(
            EventLevel::Warn,
            "cooldown_blocked",
            format!("Cooldown active, {remaining_secs}s remaining"),
        )
    }

    pub fn focus_lost() -> Self {
        Self::new(EventLevel::Info, "focus_lost", "Target window lost focus — pausing clicks")
    }

    pub fn focus_regained() -> Self {
        Self::new(EventLevel::Info, "focus_regained", "Target window regained focus — resuming clicks")
    }

    pub fn engine_error(message: &str) -> Self {
        Self::new(EventLevel::Error, "engine_error", message)
    }
}
