/// T040 — Failing integration test for profile switch → run (US3 Red phase).
///
/// Tests cover:
/// - Load profile → switch → start run → verify effective settings match
use nos_autoclicker::domain::profile::ClickProfile;
use nos_autoclicker::domain::settings::*;
use nos_autoclicker::engine::click_engine::ClickEngine;
use nos_autoclicker::infra::persistence::ProfileStore;
use std::sync::Arc;

#[test]
fn test_switch_profile_and_run_with_new_settings() {
    let dir = tempfile::tempdir().unwrap();
    let store = ProfileStore::new(dir.path().to_path_buf());

    // Create two profiles with different settings
    let mut profile_a = ClickProfile::new("Profile A".into());
    profile_a.cps = 5;
    profile_a.click_action = ClickAction::Left;

    let mut profile_b = ClickProfile::new("Profile B".into());
    profile_b.cps = 20;
    profile_b.click_action = ClickAction::Right;

    store.save(&profile_a).unwrap();
    store.save(&profile_b).unwrap();

    // Load profile B and start engine
    let loaded_b = store.load(&profile_b.id).unwrap();
    let engine = ClickEngine::new();

    let click_fn: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(|| true);
    engine.start(&loaded_b, click_fn).unwrap();

    // Verify the session reflects profile B settings (scope drop to avoid deadlock)
    {
        let session_guard = engine.session.lock().unwrap();
        let session = session_guard.as_ref().unwrap();
        assert_eq!(session.effective_cps, 20, "Should use Profile B's CPS");
    } // MutexGuard drops here

    engine.stop().unwrap();
}

#[test]
fn test_persist_and_reload_profile_settings() {
    let dir = tempfile::tempdir().unwrap();
    let store = ProfileStore::new(dir.path().to_path_buf());

    let mut profile = ClickProfile::new("Persistent".into());
    profile.cps = 42;
    profile.scope_mode = ScopeMode::Global;
    profile.jitter_percent = 15;

    store.save(&profile).unwrap();

    // Create a new store instance (simulating app restart)
    let store2 = ProfileStore::new(dir.path().to_path_buf());
    let reloaded = store2.load(&profile.id).unwrap();

    assert_eq!(reloaded.cps, 42);
    assert_eq!(reloaded.scope_mode, ScopeMode::Global);
    assert_eq!(reloaded.jitter_percent, 15);
}

#[test]
fn test_last_active_profile_persistence() {
    let dir = tempfile::tempdir().unwrap();
    let store = ProfileStore::new(dir.path().to_path_buf());

    let profile = ClickProfile::new("Active".into());
    store.save(&profile).unwrap();

    // Set as last active
    store.set_last_active(&profile.id).unwrap();

    // Retrieve from a new store instance
    let store2 = ProfileStore::new(dir.path().to_path_buf());
    let last = store2.get_last_active().unwrap();
    assert_eq!(last, Some(profile.id));
}

#[test]
fn test_no_last_active_returns_none() {
    let dir = tempfile::tempdir().unwrap();
    let store = ProfileStore::new(dir.path().to_path_buf());

    let last = store.get_last_active().unwrap();
    assert_eq!(last, None);
}
