use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::settings::{
    ClickAction, PositionMode, ScopeMode, ThemeVariant, CPS_MAX, CPS_MIN, JITTER_MAX_PERCENT,
    PROFILE_NAME_MAX_LEN, TIMER_MAX_SECONDS, TIMER_MIN_SECONDS,
};

use super::errors::ValidationError;

// ── ClickProfile ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickProfile {
    pub id: String,
    pub name: String,
    pub click_action: ClickAction,
    pub cps: u32,
    pub scope_mode: ScopeMode,
    pub position_mode: PositionMode,
    pub fixed_position_x: Option<i32>,
    pub fixed_position_y: Option<i32>,
    pub jitter_percent: u32,

    // Stationary-start
    pub stationary_start_enabled: bool,
    pub stationary_seconds: Option<u32>,

    // Stop-after
    pub stop_after_enabled: bool,
    pub stop_after_seconds: Option<u32>,

    // Safety: max session
    pub max_session_enabled: bool,
    pub max_session_seconds: Option<u32>,

    // Safety: cooldown
    pub cooldown_enabled: bool,
    pub cooldown_seconds: Option<u32>,

    // Hotkeys (string representations like "F6", "Ctrl+Shift+S")
    pub start_hotkey: String,
    pub stop_hotkey: String,
    pub panic_hotkey: String,

    // Theme
    pub theme_variant: ThemeVariant,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ClickProfile {
    /// Create a new profile with sensible defaults.
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            click_action: ClickAction::default(),
            cps: 10,
            scope_mode: ScopeMode::default(),
            position_mode: PositionMode::default(),
            fixed_position_x: None,
            fixed_position_y: None,
            jitter_percent: 0,
            stationary_start_enabled: false,
            stationary_seconds: None,
            stop_after_enabled: false,
            stop_after_seconds: None,
            max_session_enabled: false,
            max_session_seconds: None,
            cooldown_enabled: false,
            cooldown_seconds: None,
            start_hotkey: "F6".to_string(),
            stop_hotkey: "F7".to_string(),
            panic_hotkey: "F8".to_string(),
            theme_variant: ThemeVariant::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Validate the profile fields. Returns Ok(()) if valid.
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Name validation
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyField("name".to_string()));
        }
        if self.name.len() > PROFILE_NAME_MAX_LEN {
            return Err(ValidationError::FieldTooLong {
                field: "name".to_string(),
                max: PROFILE_NAME_MAX_LEN,
            });
        }

        // CPS validation
        if self.cps < CPS_MIN || self.cps > CPS_MAX {
            return Err(ValidationError::OutOfRange {
                field: "cps".to_string(),
                min: CPS_MIN,
                max: CPS_MAX,
                actual: self.cps,
            });
        }

        // Jitter validation
        if self.jitter_percent > JITTER_MAX_PERCENT {
            return Err(ValidationError::OutOfRange {
                field: "jitter_percent".to_string(),
                min: 0,
                max: JITTER_MAX_PERCENT,
                actual: self.jitter_percent,
            });
        }

        // Fixed position required when position_mode is Fixed
        if self.position_mode == PositionMode::Fixed
            && (self.fixed_position_x.is_none() || self.fixed_position_y.is_none())
        {
            return Err(ValidationError::RequiredField(
                "fixed_position_x and fixed_position_y required when position_mode is Fixed"
                    .to_string(),
            ));
        }

        // Stationary-start timer validation
        if self.stationary_start_enabled {
            match self.stationary_seconds {
                Some(s) if s < TIMER_MIN_SECONDS || s > TIMER_MAX_SECONDS => {
                    return Err(ValidationError::OutOfRange {
                        field: "stationary_seconds".to_string(),
                        min: TIMER_MIN_SECONDS,
                        max: TIMER_MAX_SECONDS,
                        actual: s,
                    });
                }
                None => {
                    return Err(ValidationError::RequiredField(
                        "stationary_seconds required when stationary_start_enabled".to_string(),
                    ));
                }
                _ => {}
            }
        }

        // Stop-after timer validation
        if self.stop_after_enabled {
            match self.stop_after_seconds {
                Some(s) if s < TIMER_MIN_SECONDS || s > TIMER_MAX_SECONDS => {
                    return Err(ValidationError::OutOfRange {
                        field: "stop_after_seconds".to_string(),
                        min: TIMER_MIN_SECONDS,
                        max: TIMER_MAX_SECONDS,
                        actual: s,
                    });
                }
                None => {
                    return Err(ValidationError::RequiredField(
                        "stop_after_seconds required when stop_after_enabled".to_string(),
                    ));
                }
                _ => {}
            }
        }

        // Max session timer validation
        if self.max_session_enabled {
            match self.max_session_seconds {
                Some(s) if s < TIMER_MIN_SECONDS || s > TIMER_MAX_SECONDS => {
                    return Err(ValidationError::OutOfRange {
                        field: "max_session_seconds".to_string(),
                        min: TIMER_MIN_SECONDS,
                        max: TIMER_MAX_SECONDS,
                        actual: s,
                    });
                }
                None => {
                    return Err(ValidationError::RequiredField(
                        "max_session_seconds required when max_session_enabled".to_string(),
                    ));
                }
                _ => {}
            }
        }

        // Cooldown timer validation
        if self.cooldown_enabled {
            match self.cooldown_seconds {
                Some(s) if s < TIMER_MIN_SECONDS || s > TIMER_MAX_SECONDS => {
                    return Err(ValidationError::OutOfRange {
                        field: "cooldown_seconds".to_string(),
                        min: TIMER_MIN_SECONDS,
                        max: TIMER_MAX_SECONDS,
                        actual: s,
                    });
                }
                None => {
                    return Err(ValidationError::RequiredField(
                        "cooldown_seconds required when cooldown_enabled".to_string(),
                    ));
                }
                _ => {}
            }
        }

        // Hotkey uniqueness check
        let hotkeys = [&self.start_hotkey, &self.stop_hotkey, &self.panic_hotkey];
        for i in 0..hotkeys.len() {
            for j in (i + 1)..hotkeys.len() {
                if !hotkeys[i].is_empty() && hotkeys[i] == hotkeys[j] {
                    return Err(ValidationError::DuplicateHotkey(hotkeys[i].clone()));
                }
            }
        }

        Ok(())
    }

    /// Create a duplicate of this profile with a new ID and name suffix.
    pub fn duplicate(&self, new_name: String) -> Self {
        let now = Utc::now();
        let mut clone = self.clone();
        clone.id = uuid::Uuid::new_v4().to_string();
        clone.name = new_name;
        clone.created_at = now;
        clone.updated_at = now;
        clone
    }
}
