/// T028 — Failing unit tests for timed-stop countdown (US2 Red phase).
///
/// Tests cover:
/// - Stop at exact duration
/// - stop_reason=timed_stop
/// - Validates bounds 1–300s
/// - Proptest for timer range
use nos_autoclicker::domain::profile::ClickProfile;
use nos_autoclicker::domain::session::RunSession;
use nos_autoclicker::domain::settings::*;
use nos_autoclicker::engine::click_engine::ClickEngine;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[test]
fn test_timed_stop_triggers_at_duration() {
    let engine = ClickEngine::new();

    let mut profile = ClickProfile::new("Timed Stop Test".into());
    profile.cps = 10;
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(1); // Stop after 1 second

    let click_count = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let count_ref = Arc::clone(&click_count);
    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> =
        Arc::new(move || {
            count_ref.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            true
        });

    engine.start(&profile, click_fn).unwrap();

    // Wait for timed stop to trigger
    std::thread::sleep(Duration::from_millis(1500));

    assert!(
        !engine.is_running(),
        "Engine should have auto-stopped after timed duration"
    );

    // Verify stop reason is timed_stop
    let session = engine.session.lock().unwrap();
    let session = session.as_ref().expect("Session should exist");
    assert_eq!(session.state, RunState::Stopped);
    assert_eq!(session.stop_reason, Some(StopReason::TimedStop));
}

#[test]
fn test_timed_stop_reason_is_timed_stop() {
    let engine = ClickEngine::new();

    let mut profile = ClickProfile::new("Reason Test".into());
    profile.cps = 5;
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(1);

    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(|| true);

    engine.start(&profile, click_fn).unwrap();
    std::thread::sleep(Duration::from_millis(1500));

    let events = engine.recent_events(100);
    let has_timed_stop_event = events.iter().any(|e| e.event_type == "timed_stop");
    assert!(
        has_timed_stop_event,
        "Should have emitted a timed_stop event"
    );
}

#[test]
fn test_no_stop_when_timer_disabled() {
    let engine = ClickEngine::new();

    let mut profile = ClickProfile::new("No Timer".into());
    profile.cps = 10;
    profile.stop_after_enabled = false;
    profile.stop_after_seconds = None;

    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(|| true);

    engine.start(&profile, click_fn).unwrap();
    std::thread::sleep(Duration::from_millis(500));

    assert!(
        engine.is_running(),
        "Engine should still be running when timed stop is disabled"
    );

    engine.stop().unwrap();
}

#[test]
fn test_timed_stop_duration_approximately_correct() {
    let engine = ClickEngine::new();

    let mut profile = ClickProfile::new("Duration Accuracy".into());
    profile.cps = 20;
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(2);

    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(|| true);

    let start = Instant::now();
    engine.start(&profile, click_fn).unwrap();

    // Wait for timed stop
    std::thread::sleep(Duration::from_millis(2500));

    let elapsed = start.elapsed();
    assert!(
        !engine.is_running(),
        "Engine should have stopped"
    );
    assert!(
        elapsed >= Duration::from_secs(2),
        "Should have run for at least 2 seconds"
    );
    assert!(
        elapsed < Duration::from_secs(3),
        "Should not run much longer than configured duration"
    );
}

#[test]
fn test_stop_after_bounds_min() {
    let mut profile = ClickProfile::new("Bounds Min".into());
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(TIMER_MIN_SECONDS);
    assert!(profile.validate().is_ok());
}

#[test]
fn test_stop_after_bounds_max() {
    let mut profile = ClickProfile::new("Bounds Max".into());
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(TIMER_MAX_SECONDS);
    assert!(profile.validate().is_ok());
}

#[test]
fn test_stop_after_below_min_rejected() {
    let mut profile = ClickProfile::new("Below Min".into());
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(0);
    assert!(profile.validate().is_err());
}

#[test]
fn test_stop_after_above_max_rejected() {
    let mut profile = ClickProfile::new("Above Max".into());
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(TIMER_MAX_SECONDS + 1);
    assert!(profile.validate().is_err());
}

// ── Proptest ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn stop_after_valid_range(secs in 1u32..=300u32) {
            let mut profile = ClickProfile::new("PropTest".into());
            profile.stop_after_enabled = true;
            profile.stop_after_seconds = Some(secs);
            prop_assert!(profile.validate().is_ok());
        }

        #[test]
        fn stop_after_invalid_range(secs in 301u32..=1000u32) {
            let mut profile = ClickProfile::new("PropTest".into());
            profile.stop_after_enabled = true;
            profile.stop_after_seconds = Some(secs);
            prop_assert!(profile.validate().is_err());
        }
    }
}
