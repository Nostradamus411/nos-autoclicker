//! Unit tests for ClickProfile validation (T012)
//! RED PHASE: These tests are written before implementation.

use nos_autoclicker::domain::profile::ClickProfile;
use nos_autoclicker::domain::settings::*;

#[test]
fn test_valid_cps_minimum() {
    let mut p = ClickProfile::new("Test".to_string());
    p.cps = CPS_MIN;
    assert!(p.validate().is_ok(), "CPS at minimum ({CPS_MIN}) should be valid");
}

#[test]
fn test_valid_cps_maximum() {
    let mut p = ClickProfile::new("Test".to_string());
    p.cps = CPS_MAX;
    assert!(p.validate().is_ok(), "CPS at maximum ({CPS_MAX}) should be valid");
}

#[test]
fn test_valid_cps_midrange() {
    let mut p = ClickProfile::new("Test".to_string());
    p.cps = 25;
    assert!(p.validate().is_ok(), "CPS=25 should be valid");
}

#[test]
fn test_reject_cps_zero() {
    let mut p = ClickProfile::new("Test".to_string());
    p.cps = 0;
    assert!(p.validate().is_err(), "CPS=0 should be rejected");
}

#[test]
fn test_reject_cps_above_max() {
    let mut p = ClickProfile::new("Test".to_string());
    p.cps = CPS_MAX + 1;
    assert!(p.validate().is_err(), "CPS above max should be rejected");
}

#[test]
fn test_reject_cps_way_above_max() {
    let mut p = ClickProfile::new("Test".to_string());
    p.cps = 1000;
    assert!(p.validate().is_err(), "CPS=1000 should be rejected");
}

#[test]
fn test_default_scope_is_focused_only() {
    let p = ClickProfile::new("Test".to_string());
    assert_eq!(p.scope_mode, ScopeMode::FocusedOnly, "Default scope must be FocusedOnly");
}

#[test]
fn test_reject_empty_name() {
    let mut p = ClickProfile::new("Test".to_string());
    p.name = "".to_string();
    assert!(p.validate().is_err(), "Empty name should be rejected");
}

#[test]
fn test_reject_whitespace_only_name() {
    let mut p = ClickProfile::new("Test".to_string());
    p.name = "   ".to_string();
    assert!(p.validate().is_err(), "Whitespace-only name should be rejected");
}

#[test]
fn test_reject_name_too_long() {
    let mut p = ClickProfile::new("Test".to_string());
    p.name = "a".repeat(PROFILE_NAME_MAX_LEN + 1);
    assert!(p.validate().is_err(), "Name exceeding max length should be rejected");
}

#[test]
fn test_reject_duplicate_hotkeys() {
    let mut p = ClickProfile::new("Test".to_string());
    p.start_hotkey = "F6".to_string();
    p.stop_hotkey = "F6".to_string(); // same as start
    assert!(p.validate().is_err(), "Duplicate hotkeys should be rejected");
}

#[test]
fn test_accept_unique_hotkeys() {
    let mut p = ClickProfile::new("Test".to_string());
    p.start_hotkey = "F6".to_string();
    p.stop_hotkey = "F7".to_string();
    p.panic_hotkey = "F8".to_string();
    assert!(p.validate().is_ok(), "Unique hotkeys should be accepted");
}

#[test]
fn test_fixed_position_requires_coordinates() {
    let mut p = ClickProfile::new("Test".to_string());
    p.position_mode = PositionMode::Fixed;
    p.fixed_position_x = None;
    p.fixed_position_y = None;
    assert!(p.validate().is_err(), "Fixed position without coordinates should be rejected");
}

#[test]
fn test_fixed_position_with_coordinates_valid() {
    let mut p = ClickProfile::new("Test".to_string());
    p.position_mode = PositionMode::Fixed;
    p.fixed_position_x = Some(100);
    p.fixed_position_y = Some(200);
    assert!(p.validate().is_ok(), "Fixed position with coordinates should be valid");
}

#[test]
fn test_jitter_within_bounds() {
    let mut p = ClickProfile::new("Test".to_string());
    p.jitter_percent = JITTER_MAX_PERCENT;
    assert!(p.validate().is_ok(), "Jitter at max should be valid");
}

#[test]
fn test_jitter_above_bounds_rejected() {
    let mut p = ClickProfile::new("Test".to_string());
    p.jitter_percent = JITTER_MAX_PERCENT + 1;
    assert!(p.validate().is_err(), "Jitter above max should be rejected");
}

#[test]
fn test_stationary_enabled_without_seconds_rejected() {
    let mut p = ClickProfile::new("Test".to_string());
    p.stationary_start_enabled = true;
    p.stationary_seconds = None;
    assert!(p.validate().is_err(), "Stationary enabled without seconds should be rejected");
}

#[test]
fn test_stationary_enabled_with_valid_seconds() {
    let mut p = ClickProfile::new("Test".to_string());
    p.stationary_start_enabled = true;
    p.stationary_seconds = Some(5);
    assert!(p.validate().is_ok());
}

#[test]
fn test_stop_after_enabled_without_seconds_rejected() {
    let mut p = ClickProfile::new("Test".to_string());
    p.stop_after_enabled = true;
    p.stop_after_seconds = None;
    assert!(p.validate().is_err());
}

#[test]
fn test_stop_after_timer_out_of_range() {
    let mut p = ClickProfile::new("Test".to_string());
    p.stop_after_enabled = true;
    p.stop_after_seconds = Some(TIMER_MAX_SECONDS + 1);
    assert!(p.validate().is_err());
}
