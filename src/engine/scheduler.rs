use rand::Rng;
use std::time::Duration;

/// Convert clicks-per-second to interval in milliseconds.
pub fn cps_to_interval_ms(cps: u32) -> u64 {
    1000 / cps as u64
}

/// Apply jitter to an interval. Returns interval ± jitter_percent%.
/// If jitter_percent is 0, returns the exact interval.
pub fn apply_jitter(interval_ms: u64, jitter_percent: u32) -> u64 {
    if jitter_percent == 0 {
        return interval_ms;
    }
    let jitter_range = interval_ms as f64 * jitter_percent as f64 / 100.0;
    let mut rng = rand::thread_rng();
    let offset = rng.gen_range(-jitter_range..=jitter_range);
    let result = interval_ms as f64 + offset;
    result.round().max(1.0) as u64
}

/// Calculate the sleep duration for a single tick.
pub fn tick_duration(cps: u32, jitter_percent: u32) -> Duration {
    let base_ms = cps_to_interval_ms(cps);
    let actual_ms = apply_jitter(base_ms, jitter_percent);
    Duration::from_millis(actual_ms)
}
