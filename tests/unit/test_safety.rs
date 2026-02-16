/// T038 — Failing unit tests for panic-stop override (US3 Red phase).
///
/// Tests cover:
/// - Panic stops running session regardless of conditions
/// - Overrides timed-stop
/// - Overrides stationary-wait
use nos_autoclicker::domain::profile::ClickProfile;
use nos_autoclicker::domain::settings::*;
use nos_autoclicker::engine::click_engine::ClickEngine;
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_panic_stop_halts_running_session() {
    let engine = ClickEngine::new();
    let profile = ClickProfile::new("Panic Test".into());
    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(|| true);

    engine.start(&profile, click_fn).unwrap();
    assert!(engine.is_running());

    engine.panic_stop().unwrap();
    std::thread::sleep(Duration::from_millis(100));

    assert!(!engine.is_running(), "Engine should have stopped after panic");

    let session = engine.session.lock().unwrap();
    let session = session.as_ref().unwrap();
    assert_eq!(session.stop_reason, Some(StopReason::PanicStop));
}

#[test]
fn test_panic_stop_overrides_timed_stop() {
    let engine = ClickEngine::new();

    let mut profile = ClickProfile::new("Panic Override Timer".into());
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(60); // Would normally run 60s

    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(|| true);

    engine.start(&profile, click_fn).unwrap();
    std::thread::sleep(Duration::from_millis(100));

    // Panic stop immediately
    engine.panic_stop().unwrap();
    std::thread::sleep(Duration::from_millis(100));

    assert!(!engine.is_running());

    let session = engine.session.lock().unwrap();
    let session = session.as_ref().unwrap();
    assert_eq!(
        session.stop_reason,
        Some(StopReason::PanicStop),
        "Stop reason should be PanicStop, not TimedStop"
    );
}

#[test]
fn test_panic_stop_emits_event() {
    let engine = ClickEngine::new();
    let profile = ClickProfile::new("Panic Event".into());
    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(|| true);

    engine.start(&profile, click_fn).unwrap();
    std::thread::sleep(Duration::from_millis(50));

    engine.panic_stop().unwrap();
    std::thread::sleep(Duration::from_millis(100));

    let events = engine.recent_events(100);
    let has_panic = events.iter().any(|e| e.event_type == "panic_stop");
    assert!(has_panic, "Should have emitted a panic_stop event");
}

#[test]
fn test_panic_stop_when_not_running_is_ok() {
    let engine = ClickEngine::new();
    let result = engine.panic_stop();
    assert!(result.is_ok(), "Panic stop when idle should not error");
}

#[test]
fn test_panic_stop_sets_state_to_stopped() {
    let engine = ClickEngine::new();
    let profile = ClickProfile::new("Panic State".into());
    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(|| true);

    engine.start(&profile, click_fn).unwrap();
    std::thread::sleep(Duration::from_millis(50));

    engine.panic_stop().unwrap();
    std::thread::sleep(Duration::from_millis(100));

    let state = engine.current_state();
    assert_eq!(state, RunState::Stopped, "State should be Stopped after panic");
}
