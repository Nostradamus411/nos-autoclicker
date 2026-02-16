// Safety policy — max session, cooldown, panic-stop enforcement.

use std::time::{Duration, Instant};

/// Safety guardrails: max session duration and cooldown between runs.
pub struct SafetyPolicy {
    /// Max session duration in seconds (None = disabled).
    max_session_seconds: Option<u32>,
    /// Cooldown between runs in seconds (None = disabled).
    cooldown_seconds: Option<u32>,
    /// When the last session ended (for cooldown tracking).
    last_session_end: Option<Instant>,
}

impl SafetyPolicy {
    /// Create a new safety policy.
    pub fn new(max_session_seconds: Option<u32>, cooldown_seconds: Option<u32>) -> Self {
        Self {
            max_session_seconds,
            cooldown_seconds,
            last_session_end: None,
        }
    }

    /// Check if the current session has exceeded the max duration.
    /// Returns true if max session is enabled and the elapsed time exceeds it.
    pub fn check_max_session(&self, elapsed: Duration) -> bool {
        match self.max_session_seconds {
            Some(max) => elapsed.as_secs() >= max as u64,
            None => false,
        }
    }

    /// Record that a session has ended (starts cooldown timer).
    pub fn record_session_end(&mut self) {
        self.last_session_end = Some(Instant::now());
    }

    /// Check if cooldown is currently active (blocks re-start).
    pub fn is_cooldown_active(&self) -> bool {
        match (self.cooldown_seconds, self.last_session_end) {
            (Some(cd), Some(end)) => end.elapsed() < Duration::from_secs(cd as u64),
            _ => false,
        }
    }

    /// Get remaining cooldown seconds (0 if not active).
    pub fn cooldown_remaining_secs(&self) -> u32 {
        match (self.cooldown_seconds, self.last_session_end) {
            (Some(cd), Some(end)) => {
                let elapsed = end.elapsed().as_secs() as u32;
                cd.saturating_sub(elapsed)
            }
            _ => 0,
        }
    }

    /// Get max session duration.
    pub fn max_session_seconds(&self) -> Option<u32> {
        self.max_session_seconds
    }

    /// Get cooldown duration.
    pub fn cooldown_seconds(&self) -> Option<u32> {
        self.cooldown_seconds
    }
}
