/// T037 — Failing unit tests for profile CRUD and TOML persistence (US3 Red phase).
///
/// Tests cover:
/// - Create, rename, duplicate, delete profiles
/// - Unique name enforcement (case-insensitive)
/// - TOML round-trip serialization
use nos_autoclicker::domain::profile::ClickProfile;
use nos_autoclicker::domain::settings::*;
use nos_autoclicker::infra::persistence::ProfileStore;
use std::path::PathBuf;

fn temp_store() -> (ProfileStore, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let store = ProfileStore::new(dir.path().to_path_buf());
    (store, dir)
}

#[test]
fn test_save_and_load_profile() {
    let (store, _dir) = temp_store();
    let profile = ClickProfile::new("Test Profile".into());

    store.save(&profile).unwrap();
    let loaded = store.load(&profile.id).unwrap();

    assert_eq!(loaded.name, "Test Profile");
    assert_eq!(loaded.cps, profile.cps);
    assert_eq!(loaded.click_action, profile.click_action);
}

#[test]
fn test_list_all_profiles() {
    let (store, _dir) = temp_store();

    let p1 = ClickProfile::new("Alpha".into());
    let p2 = ClickProfile::new("Beta".into());
    let p3 = ClickProfile::new("Gamma".into());

    store.save(&p1).unwrap();
    store.save(&p2).unwrap();
    store.save(&p3).unwrap();

    let all = store.list_all().unwrap();
    assert_eq!(all.len(), 3);
}

#[test]
fn test_delete_profile() {
    let (store, _dir) = temp_store();
    let profile = ClickProfile::new("Deletable".into());

    store.save(&profile).unwrap();
    assert!(store.load(&profile.id).is_ok());

    store.delete(&profile.id).unwrap();
    assert!(store.load(&profile.id).is_err(), "Should not find deleted profile");
}

#[test]
fn test_rename_profile() {
    let (store, _dir) = temp_store();
    let mut profile = ClickProfile::new("Old Name".into());

    store.save(&profile).unwrap();

    profile.name = "New Name".to_string();
    store.save(&profile).unwrap();

    let loaded = store.load(&profile.id).unwrap();
    assert_eq!(loaded.name, "New Name");
}

#[test]
fn test_duplicate_profile() {
    let original = ClickProfile::new("Original".into());
    let dup = original.duplicate("Original (Copy)".to_string());

    assert_ne!(dup.id, original.id, "Duplicate should have a new ID");
    assert_eq!(dup.name, "Original (Copy)");
    assert_eq!(dup.cps, original.cps);
    assert_eq!(dup.click_action, original.click_action);
}

#[test]
fn test_unique_name_enforcement_case_insensitive() {
    let (store, _dir) = temp_store();

    let p1 = ClickProfile::new("Gaming Profile".into());
    store.save(&p1).unwrap();

    let all = store.list_all().unwrap();
    let name_exists = all
        .iter()
        .any(|p| p.name.to_lowercase() == "gaming profile");
    assert!(name_exists, "Should find the profile by case-insensitive name");

    // Check that a duplicate name is detected
    let p2 = ClickProfile::new("gaming profile".into());
    let is_dup = store.has_name_conflict(&p2.name, &p2.id).unwrap();
    assert!(is_dup, "Should detect case-insensitive name conflict");
}

#[test]
fn test_toml_roundtrip_preserves_all_fields() {
    let (store, _dir) = temp_store();

    let mut profile = ClickProfile::new("Full Profile".into());
    profile.cps = 25;
    profile.click_action = ClickAction::Double;
    profile.scope_mode = ScopeMode::Global;
    profile.jitter_percent = 15;
    profile.stationary_start_enabled = true;
    profile.stationary_seconds = Some(3);
    profile.stop_after_enabled = true;
    profile.stop_after_seconds = Some(60);
    profile.max_session_enabled = true;
    profile.max_session_seconds = Some(120);
    profile.cooldown_enabled = true;
    profile.cooldown_seconds = Some(5);
    profile.start_hotkey = "F1".to_string();
    profile.stop_hotkey = "F2".to_string();
    profile.panic_hotkey = "F3".to_string();
    profile.theme_variant = ThemeVariant::HighContrast;

    store.save(&profile).unwrap();
    let loaded = store.load(&profile.id).unwrap();

    assert_eq!(loaded.cps, 25);
    assert_eq!(loaded.click_action, ClickAction::Double);
    assert_eq!(loaded.scope_mode, ScopeMode::Global);
    assert_eq!(loaded.jitter_percent, 15);
    assert!(loaded.stationary_start_enabled);
    assert_eq!(loaded.stationary_seconds, Some(3));
    assert!(loaded.stop_after_enabled);
    assert_eq!(loaded.stop_after_seconds, Some(60));
    assert!(loaded.max_session_enabled);
    assert_eq!(loaded.max_session_seconds, Some(120));
    assert!(loaded.cooldown_enabled);
    assert_eq!(loaded.cooldown_seconds, Some(5));
    assert_eq!(loaded.start_hotkey, "F1");
    assert_eq!(loaded.stop_hotkey, "F2");
    assert_eq!(loaded.panic_hotkey, "F3");
    assert_eq!(loaded.theme_variant, ThemeVariant::HighContrast);
}

#[test]
fn test_load_nonexistent_profile_fails() {
    let (store, _dir) = temp_store();
    let result = store.load("nonexistent-id");
    assert!(result.is_err());
}

#[test]
fn test_delete_nonexistent_profile_fails() {
    let (store, _dir) = temp_store();
    let result = store.delete("nonexistent-id");
    assert!(result.is_err());
}
