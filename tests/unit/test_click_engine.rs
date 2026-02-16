//! Unit tests for click engine state transitions (T013)
//! RED PHASE: These tests are written before implementation.

use nos_autoclicker::domain::session::RunSession;
use nos_autoclicker::domain::settings::*;

#[test]
fn test_new_session_starts_idle() {
    let session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    assert_eq!(session.state, RunState::Idle);
}

#[test]
fn test_idle_to_running_valid() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    assert!(session.transition_to(RunState::Running).is_ok());
    assert_eq!(session.state, RunState::Running);
}

#[test]
fn test_idle_to_waiting_condition_valid() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    assert!(session.transition_to(RunState::WaitingCondition).is_ok());
    assert_eq!(session.state, RunState::WaitingCondition);
}

#[test]
fn test_running_to_stopped_valid() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    session.transition_to(RunState::Running).unwrap();
    assert!(session.stop(StopReason::ManualStop).is_ok());
    assert_eq!(session.state, RunState::Stopped);
    assert_eq!(session.stop_reason, Some(StopReason::ManualStop));
}

#[test]
fn test_running_to_idle_invalid() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    session.transition_to(RunState::Running).unwrap();
    assert!(session.transition_to(RunState::Idle).is_err());
}

#[test]
fn test_stopped_to_running_invalid() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    session.transition_to(RunState::Running).unwrap();
    session.stop(StopReason::ManualStop).unwrap();
    assert!(session.transition_to(RunState::Running).is_err());
}

#[test]
fn test_any_to_error_valid() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    assert!(session.set_error("test_error".to_string()).is_ok());
    assert_eq!(session.state, RunState::Error);
}

#[test]
fn test_error_to_idle_valid() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    session.set_error("test_error".to_string()).unwrap();
    assert!(session.transition_to(RunState::Idle).is_ok());
    assert_eq!(session.state, RunState::Idle);
}

#[test]
fn test_running_to_waiting_condition_valid() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    session.transition_to(RunState::Running).unwrap();
    // Focus loss transitions back to waiting
    assert!(session.transition_to(RunState::WaitingCondition).is_ok());
    assert_eq!(session.state, RunState::WaitingCondition);
}

#[test]
fn test_record_click_increments() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    assert_eq!(session.total_clicks, 0);
    session.record_click();
    assert_eq!(session.total_clicks, 1);
    session.record_click();
    assert_eq!(session.total_clicks, 2);
}

#[test]
fn test_is_active_when_running() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    assert!(!session.is_active());
    session.transition_to(RunState::Running).unwrap();
    assert!(session.is_active());
}

#[test]
fn test_is_active_when_waiting() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    session.transition_to(RunState::WaitingCondition).unwrap();
    assert!(session.is_active());
}

#[test]
fn test_not_active_when_stopped() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    session.transition_to(RunState::Running).unwrap();
    session.stop(StopReason::ManualStop).unwrap();
    assert!(!session.is_active());
}

#[test]
fn test_ended_at_set_on_stop() {
    let mut session = RunSession::new("profile-1".to_string(), 10, ScopeMode::FocusedOnly);
    assert!(session.ended_at.is_none());
    session.transition_to(RunState::Running).unwrap();
    session.stop(StopReason::ManualStop).unwrap();
    assert!(session.ended_at.is_some());
}
