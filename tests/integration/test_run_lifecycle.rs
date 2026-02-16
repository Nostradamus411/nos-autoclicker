//! Integration test for full start/stop lifecycle (T015)
//! RED PHASE: These tests are written before implementation.

use nos_autoclicker::domain::profile::ClickProfile;
use nos_autoclicker::domain::session::RunSession;
use nos_autoclicker::domain::settings::*;

#[test]
fn test_full_run_lifecycle_manual_stop() {
    // 1. Create a valid profile
    let profile = ClickProfile::new("Lifecycle Test".to_string());
    assert!(profile.validate().is_ok(), "Profile should be valid");

    // 2. Start a run session
    let mut session = RunSession::new(
        profile.id.clone(),
        profile.cps,
        profile.scope_mode,
    );
    assert_eq!(session.state, RunState::Idle);

    // 3. Transition to running
    session.transition_to(RunState::Running).unwrap();
    assert_eq!(session.state, RunState::Running);
    assert!(session.is_active());

    // 4. Record some clicks
    for _ in 0..5 {
        session.record_click();
    }
    assert_eq!(session.total_clicks, 5);

    // 5. Stop with manual reason
    session.stop(StopReason::ManualStop).unwrap();
    assert_eq!(session.state, RunState::Stopped);
    assert_eq!(session.stop_reason, Some(StopReason::ManualStop));
    assert!(!session.is_active());
    assert!(session.ended_at.is_some());
}

#[test]
fn test_run_lifecycle_with_profile_settings() {
    let mut profile = ClickProfile::new("Full Settings".to_string());
    profile.click_action = ClickAction::Right;
    profile.cps = 30;
    profile.scope_mode = ScopeMode::Global;
    assert!(profile.validate().is_ok());

    let mut session = RunSession::new(
        profile.id.clone(),
        profile.cps,
        profile.scope_mode,
    );
    assert_eq!(session.effective_cps, 30);
    assert_eq!(session.scope_mode, ScopeMode::Global);

    session.transition_to(RunState::Running).unwrap();
    session.record_click();
    session.stop(StopReason::ManualStop).unwrap();

    assert_eq!(session.total_clicks, 1);
    assert_eq!(session.state, RunState::Stopped);
}

#[test]
fn test_cannot_start_completed_session() {
    let mut session = RunSession::new("p1".to_string(), 10, ScopeMode::FocusedOnly);
    session.transition_to(RunState::Running).unwrap();
    session.stop(StopReason::ManualStop).unwrap();

    // Should not be able to go back to running
    assert!(session.transition_to(RunState::Running).is_err());
}

#[test]
fn test_error_recovery_to_idle() {
    let mut session = RunSession::new("p1".to_string(), 10, ScopeMode::FocusedOnly);
    session.transition_to(RunState::Running).unwrap();
    session.set_error("unexpected_crash".to_string()).unwrap();

    assert_eq!(session.state, RunState::Error);
    assert_eq!(session.error_code, Some("unexpected_crash".to_string()));

    // Recover to idle
    session.transition_to(RunState::Idle).unwrap();
    assert_eq!(session.state, RunState::Idle);
}
