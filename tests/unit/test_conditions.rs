/// T027 — Failing unit tests for stationary detection (US2 Red phase).
///
/// Tests cover:
/// - Position polling detects no-change window
/// - Resets on movement
/// - Fires eligible after threshold
/// - Validates bounds 1–300s
use nos_autoclicker::domain::settings::{TIMER_MAX_SECONDS, TIMER_MIN_SECONDS};
use nos_autoclicker::engine::conditions::{ActivationCondition, PositionSample};

// ── Stationary detection ──────────────────────────────────────────────────────

#[test]
fn test_stationary_detected_after_threshold() {
    let mut cond = ActivationCondition::new(2); // 2 seconds
    let pos = PositionSample { x: 100, y: 200 };

    // Feed same position for 2+ seconds worth of polls
    for _ in 0..21 {
        cond.feed_position(pos, 100); // 100ms apart
    }

    assert!(
        cond.is_eligible(),
        "Should be eligible after stationary for threshold duration"
    );
}

#[test]
fn test_not_eligible_before_threshold() {
    let mut cond = ActivationCondition::new(2);
    let pos = PositionSample { x: 100, y: 200 };

    // Feed same position for only 1 second
    for _ in 0..10 {
        cond.feed_position(pos, 100);
    }

    assert!(
        !cond.is_eligible(),
        "Should NOT be eligible before reaching threshold"
    );
}

#[test]
fn test_movement_resets_stationary_timer() {
    let mut cond = ActivationCondition::new(2);
    let pos_a = PositionSample { x: 100, y: 200 };
    let pos_b = PositionSample { x: 150, y: 250 };

    // Accumulate 1.5s of stationary
    for _ in 0..15 {
        cond.feed_position(pos_a, 100);
    }

    // Move — resets
    cond.feed_position(pos_b, 100);

    // Accumulate another 1.5s — not enough for 2s threshold since reset
    for _ in 0..15 {
        cond.feed_position(pos_b, 100);
    }

    assert!(
        !cond.is_eligible(),
        "Timer should have reset on movement; 1.5s is not enough"
    );
}

#[test]
fn test_eligible_after_reset_and_full_threshold() {
    let mut cond = ActivationCondition::new(1); // 1 second threshold
    let pos_a = PositionSample { x: 100, y: 200 };
    let pos_b = PositionSample { x: 150, y: 250 };

    // Some stationary
    for _ in 0..5 {
        cond.feed_position(pos_a, 100);
    }

    // Move
    cond.feed_position(pos_b, 100);

    // Now stay stationary at new position for full 1s
    for _ in 0..11 {
        cond.feed_position(pos_b, 100);
    }

    assert!(
        cond.is_eligible(),
        "Should be eligible after re-accumulating full threshold at new position"
    );
}

#[test]
fn test_initial_state_not_eligible() {
    let cond = ActivationCondition::new(2);
    assert!(
        !cond.is_eligible(),
        "Should not be eligible with no position data"
    );
}

#[test]
fn test_observed_seconds_tracking() {
    let mut cond = ActivationCondition::new(3);
    let pos = PositionSample { x: 0, y: 0 };

    for _ in 0..15 {
        cond.feed_position(pos, 100); // 1.5s
    }

    let observed = cond.observed_stationary_ms();
    assert!(
        observed >= 1400 && observed <= 1600,
        "Observed stationary should be approximately 1500ms, got {observed}"
    );
}

// ── Bounds validation ─────────────────────────────────────────────────────────

#[test]
fn test_stationary_threshold_min_valid() {
    let cond = ActivationCondition::new(TIMER_MIN_SECONDS);
    assert_eq!(cond.threshold_seconds(), TIMER_MIN_SECONDS);
}

#[test]
fn test_stationary_threshold_max_valid() {
    let cond = ActivationCondition::new(TIMER_MAX_SECONDS);
    assert_eq!(cond.threshold_seconds(), TIMER_MAX_SECONDS);
}

#[test]
fn test_elapsed_resets_to_zero_on_movement() {
    let mut cond = ActivationCondition::new(5);
    let pos_a = PositionSample { x: 10, y: 20 };
    let pos_b = PositionSample { x: 30, y: 40 };

    for _ in 0..10 {
        cond.feed_position(pos_a, 100);
    }

    assert!(cond.observed_stationary_ms() > 0);

    cond.feed_position(pos_b, 100);

    assert_eq!(
        cond.observed_stationary_ms(),
        0,
        "Observed should reset to 0 after movement"
    );
}

#[test]
fn test_reset_clears_eligibility() {
    let mut cond = ActivationCondition::new(1);
    let pos = PositionSample { x: 5, y: 5 };

    for _ in 0..20 {
        cond.feed_position(pos, 100);
    }
    assert!(cond.is_eligible());

    cond.reset();
    assert!(
        !cond.is_eligible(),
        "Reset should clear eligibility"
    );
}
