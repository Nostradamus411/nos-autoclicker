// Activation conditions — stationary detection and eligibility evaluation.

/// A 2D position sample for stationary detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionSample {
    pub x: i32,
    pub y: i32,
}

/// Tracks whether the mouse has been stationary long enough to satisfy
/// the activation condition (stationary-start).
pub struct ActivationCondition {
    /// Required stationary duration in seconds.
    threshold_seconds: u32,
    /// Accumulated stationary time in milliseconds.
    accumulated_ms: u64,
    /// Last recorded position (None if no data yet).
    last_position: Option<PositionSample>,
    /// Whether the threshold has been met.
    eligible: bool,
}

impl ActivationCondition {
    /// Create a new condition with the given stationary threshold (in seconds).
    pub fn new(threshold_seconds: u32) -> Self {
        Self {
            threshold_seconds,
            accumulated_ms: 0,
            last_position: None,
            eligible: false,
        }
    }

    /// Feed a new mouse position sample.
    ///
    /// `delta_ms` is the time since the last sample in milliseconds.
    /// If the position hasn't changed, we accumulate stationary time.
    /// If it changed, we reset the accumulator.
    pub fn feed_position(&mut self, pos: PositionSample, delta_ms: u64) {
        match self.last_position {
            Some(last) if last == pos => {
                // Same position — accumulate
                self.accumulated_ms += delta_ms;
                if self.accumulated_ms >= (self.threshold_seconds as u64) * 1000 {
                    self.eligible = true;
                }
            }
            _ => {
                // First sample or position changed — reset
                self.accumulated_ms = 0;
                self.eligible = false;
                self.last_position = Some(pos);
            }
        }
    }

    /// Check if the stationary threshold has been met.
    pub fn is_eligible(&self) -> bool {
        self.eligible
    }

    /// Get the configured threshold in seconds.
    pub fn threshold_seconds(&self) -> u32 {
        self.threshold_seconds
    }

    /// Get how long the mouse has been observed stationary (in ms).
    pub fn observed_stationary_ms(&self) -> u64 {
        self.accumulated_ms
    }

    /// Reset the condition state (clear eligibility and accumulated time).
    pub fn reset(&mut self) {
        self.accumulated_ms = 0;
        self.last_position = None;
        self.eligible = false;
    }
}
