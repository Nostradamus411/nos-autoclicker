/// T039 — Failing unit tests for max session and cooldown (US3 Red phase).
///
/// Tests cover:
/// - Max session duration enforcement
/// - Cooldown blocks re-start
/// - Bounds validation 1–300s
use nos_autoclicker::domain::profile::ClickProfile;
use nos_autoclicker::domain::settings::*;
use nos_autoclicker::engine::safety::SafetyPolicy;
use std::time::Duration;

#[test]
fn test_max_session_enforced() {
    let policy = SafetyPolicy::new(Some(1), None); // 1 second max
    let profile = ClickProfile::new("Max Session".into());

    assert!(
        policy.check_max_session(Duration::from_secs(2)),
        "Should flag exceeded max session"
    );
    assert!(
        !policy.check_max_session(Duration::from_millis(500)),
        "Should not flag within max session"
    );
}

#[test]
fn test_max_session_disabled() {
    let policy = SafetyPolicy::new(None, None);
    assert!(
        !policy.check_max_session(Duration::from_secs(9999)),
        "Should never flag when disabled"
    );
}

#[test]
fn test_cooldown_blocks_restart() {
    let mut policy = SafetyPolicy::new(None, Some(5)); // 5 second cooldown

    policy.record_session_end();
    assert!(
        policy.is_cooldown_active(),
        "Cooldown should be active immediately after session end"
    );
}

#[test]
fn test_cooldown_expires() {
    let mut policy = SafetyPolicy::new(None, Some(1)); // 1 second cooldown

    policy.record_session_end();
    std::thread::sleep(Duration::from_millis(1100));

    assert!(
        !policy.is_cooldown_active(),
        "Cooldown should have expired"
    );
}

#[test]
fn test_cooldown_disabled() {
    let policy = SafetyPolicy::new(None, None);
    assert!(
        !policy.is_cooldown_active(),
        "Cooldown should not be active when disabled"
    );
}

#[test]
fn test_cooldown_remaining_seconds() {
    let mut policy = SafetyPolicy::new(None, Some(10));
    policy.record_session_end();

    let remaining = policy.cooldown_remaining_secs();
    assert!(
        remaining > 0 && remaining <= 10,
        "Remaining cooldown should be between 1 and 10, got {remaining}"
    );
}

// ── Bounds validation (these test the ClickProfile validation) ────────────────

#[test]
fn test_max_session_bounds_min() {
    let mut profile = ClickProfile::new("Min".into());
    profile.max_session_enabled = true;
    profile.max_session_seconds = Some(TIMER_MIN_SECONDS);
    assert!(profile.validate().is_ok());
}

#[test]
fn test_max_session_bounds_max() {
    let mut profile = ClickProfile::new("Max".into());
    profile.max_session_enabled = true;
    profile.max_session_seconds = Some(TIMER_MAX_SECONDS);
    assert!(profile.validate().is_ok());
}

#[test]
fn test_max_session_below_min_rejected() {
    let mut profile = ClickProfile::new("Below".into());
    profile.max_session_enabled = true;
    profile.max_session_seconds = Some(0);
    assert!(profile.validate().is_err());
}

#[test]
fn test_max_session_above_max_rejected() {
    let mut profile = ClickProfile::new("Above".into());
    profile.max_session_enabled = true;
    profile.max_session_seconds = Some(TIMER_MAX_SECONDS + 1);
    assert!(profile.validate().is_err());
}

#[test]
fn test_cooldown_bounds_min() {
    let mut profile = ClickProfile::new("CD Min".into());
    profile.cooldown_enabled = true;
    profile.cooldown_seconds = Some(TIMER_MIN_SECONDS);
    assert!(profile.validate().is_ok());
}

#[test]
fn test_cooldown_bounds_max() {
    let mut profile = ClickProfile::new("CD Max".into());
    profile.cooldown_enabled = true;
    profile.cooldown_seconds = Some(TIMER_MAX_SECONDS);
    assert!(profile.validate().is_ok());
}

#[test]
fn test_cooldown_below_min_rejected() {
    let mut profile = ClickProfile::new("CD Below".into());
    profile.cooldown_enabled = true;
    profile.cooldown_seconds = Some(0);
    assert!(profile.validate().is_err());
}

#[test]
fn test_cooldown_above_max_rejected() {
    let mut profile = ClickProfile::new("CD Above".into());
    profile.cooldown_enabled = true;
    profile.cooldown_seconds = Some(TIMER_MAX_SECONDS + 1);
    assert!(profile.validate().is_err());
}

// ── Proptest ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn max_session_valid_range(secs in 1u32..=300u32) {
            let mut profile = ClickProfile::new("Prop".into());
            profile.max_session_enabled = true;
            profile.max_session_seconds = Some(secs);
            prop_assert!(profile.validate().is_ok());
        }

        #[test]
        fn cooldown_valid_range(secs in 1u32..=300u32) {
            let mut profile = ClickProfile::new("Prop".into());
            profile.cooldown_enabled = true;
            profile.cooldown_seconds = Some(secs);
            prop_assert!(profile.validate().is_ok());
        }
    }
}
