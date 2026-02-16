/// T029 — Failing integration test for conditional start → timed stop (US2 Red phase).
///
/// Tests cover:
/// - Enable stationary-start + stop-after → start → satisfy stationary → verify running → auto-stop
use nos_autoclicker::domain::profile::ClickProfile;
use nos_autoclicker::domain::settings::*;
use nos_autoclicker::engine::click_engine::ClickEngine;
use nos_autoclicker::engine::conditions::{ActivationCondition, PositionSample};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_conditional_start_then_timed_stop() {
    // Create a profile with both stationary-start and stop-after
    let mut profile = ClickProfile::new("Conditional + Timed".into());
    profile.cps = 10;
    profile.stationary_start_enabled = true;
    profile.stationary_seconds = Some(1); // 1s stationary threshold
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(2); // Auto-stop after 2s of clicking

    // Verify profile validates
    assert!(profile.validate().is_ok());

    // Create engine and condition
    let engine = ClickEngine::new();
    let mut condition = ActivationCondition::new(1);

    // Simulate user pressing Start — engine enters WaitingCondition
    // The condition should not be satisfied yet
    assert!(!condition.is_eligible());

    // Feed stationary position for 1+ seconds
    let pos = PositionSample { x: 500, y: 300 };
    for _ in 0..12 {
        condition.feed_position(pos, 100); // ~1.2s of stationary
    }

    // Now condition should be satisfied
    assert!(
        condition.is_eligible(),
        "Stationary condition should be met after threshold"
    );

    // Start clicking (condition met)
    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(|| true);
    engine.start(&profile, click_fn).unwrap();

    assert!(engine.is_running(), "Engine should be running after conditional start");

    // Wait for timed stop to trigger
    std::thread::sleep(Duration::from_millis(2500));

    assert!(
        !engine.is_running(),
        "Engine should have auto-stopped after timed duration"
    );

    let session = engine.session.lock().unwrap();
    let session = session.as_ref().expect("Session should exist");
    assert_eq!(session.state, RunState::Stopped);
    assert_eq!(session.stop_reason, Some(StopReason::TimedStop));
    assert!(session.total_clicks > 0, "Should have recorded some clicks");
}

#[test]
fn test_stationary_condition_blocks_start_until_met() {
    let mut condition = ActivationCondition::new(3); // 3 second threshold
    let moving_positions = [
        PositionSample { x: 10, y: 20 },
        PositionSample { x: 50, y: 60 },
        PositionSample { x: 100, y: 120 },
        PositionSample { x: 200, y: 250 },
    ];

    // Mouse keeps moving
    for pos in moving_positions.iter().cycle().take(20) {
        condition.feed_position(*pos, 100);
    }

    assert!(
        !condition.is_eligible(),
        "Should NOT be eligible while mouse keeps moving"
    );
}

#[test]
fn test_conditional_run_observed_stationary_seconds() {
    let mut condition = ActivationCondition::new(2);
    let pos = PositionSample { x: 300, y: 400 };

    // Feed 2.5 seconds of stationary
    for _ in 0..25 {
        condition.feed_position(pos, 100);
    }

    assert!(condition.is_eligible());
    let observed = condition.observed_stationary_ms();
    assert!(
        observed >= 2400,
        "Observed stationary should be at least 2400ms, got {observed}"
    );
}

#[test]
fn test_profile_with_both_conditions_validates() {
    let mut profile = ClickProfile::new("Dual Condition".into());
    profile.stationary_start_enabled = true;
    profile.stationary_seconds = Some(2);
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(30);
    assert!(profile.validate().is_ok(), "Profile with both conditions should be valid");
}
