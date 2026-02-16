use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::settings::{RunState, ScopeMode, StopReason};

// ── RunSession ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSession {
    pub session_id: String,
    pub profile_id: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub state: RunState,
    pub stop_reason: Option<StopReason>,
    pub effective_cps: u32,
    pub total_clicks: u64,
    pub scope_mode: ScopeMode,
    pub stationary_wait_observed_seconds: Option<u32>,
    pub error_code: Option<String>,
}

impl RunSession {
    /// Create a new run session in Idle state.
    pub fn new(profile_id: String, effective_cps: u32, scope_mode: ScopeMode) -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            profile_id,
            started_at: Utc::now(),
            ended_at: None,
            state: RunState::Idle,
            stop_reason: None,
            effective_cps,
            total_clicks: 0,
            scope_mode,
            stationary_wait_observed_seconds: None,
            error_code: None,
        }
    }

    /// Transition to a new state. Returns an error string if the transition is invalid.
    pub fn transition_to(&mut self, new_state: RunState) -> Result<(), String> {
        let valid = match (self.state, new_state) {
            // Valid transitions per data-model state diagram
            (RunState::Idle, RunState::WaitingCondition) => true,
            (RunState::Idle, RunState::Running) => true,
            (RunState::WaitingCondition, RunState::Running) => true,
            (RunState::Running, RunState::Stopped) => true,
            (RunState::Running, RunState::WaitingCondition) => true, // focus loss
            // Any state can transition to Error
            (_, RunState::Error) => true,
            // Error can reset to Idle
            (RunState::Error, RunState::Idle) => true,
            _ => false,
        };

        if !valid {
            return Err(format!(
                "Invalid state transition: {:?} -> {:?}",
                self.state, new_state
            ));
        }

        self.state = new_state;

        if matches!(new_state, RunState::Stopped | RunState::Error) {
            self.ended_at = Some(Utc::now());
        }

        Ok(())
    }

    /// Stop the session with a given reason.
    pub fn stop(&mut self, reason: StopReason) -> Result<(), String> {
        self.stop_reason = Some(reason);
        self.transition_to(RunState::Stopped)
    }

    /// Record an error and transition to Error state.
    pub fn set_error(&mut self, code: String) -> Result<(), String> {
        self.error_code = Some(code);
        self.transition_to(RunState::Error)
    }

    /// Increment the click counter.
    pub fn record_click(&mut self) {
        self.total_clicks += 1;
    }

    /// Check if the session is currently active (running or waiting).
    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            RunState::Running | RunState::WaitingCondition
        )
    }
}
