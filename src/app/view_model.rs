use std::sync::Arc;

use crate::domain::events::EventRecord;
use crate::domain::profile::ClickProfile;
use crate::engine::click_engine::ClickEngine;
use crate::engine::safety::SafetyPolicy;
use crate::infra::persistence::ProfileStore;

/// View model — bridges engine state to UI.
pub struct ViewModel {
    pub engine: Arc<ClickEngine>,
    pub active_profile: ClickProfile,
    pub profiles: Vec<ClickProfile>,
    pub status_text: String,
    pub stop_reason_text: String,
    pub validation_error: Option<String>,
    /// CPS text field (string for editing, parsed to u32)
    pub cps_text: String,
    /// Profile store for persistence.
    pub store: Option<ProfileStore>,
    /// Safety policy (max session, cooldown).
    pub safety: SafetyPolicy,
    /// Name for new profile creation.
    pub new_profile_name: String,
    /// Index of selected profile in the list.
    pub selected_profile_idx: usize,
}

impl ViewModel {
    pub fn new(engine: Arc<ClickEngine>) -> Self {
        let default_profile = ClickProfile::new("Default".to_string());
        let cps_text = default_profile.cps.to_string();
        Self {
            engine,
            active_profile: default_profile.clone(),
            profiles: vec![default_profile],
            status_text: "Idle".to_string(),
            stop_reason_text: String::new(),
            validation_error: None,
            cps_text,
            store: None,
            safety: SafetyPolicy::new(None, None),
            new_profile_name: String::new(),
            selected_profile_idx: 0,
        }
    }

    /// Initialize with a profile store and load saved profiles.
    pub fn with_store(mut self, store: ProfileStore) -> Self {
        if let Ok(profiles) = store.list_all() {
            if !profiles.is_empty() {
                self.profiles = profiles;
                // Restore last active
                if let Ok(Some(last_id)) = store.get_last_active() {
                    if let Some(idx) = self.profiles.iter().position(|p| p.id == last_id) {
                        self.selected_profile_idx = idx;
                        self.active_profile = self.profiles[idx].clone();
                        self.cps_text = self.active_profile.cps.to_string();
                    }
                }
            }
        }
        self.store = Some(store);
        self
    }

    /// Switch to a different profile by index.
    pub fn switch_profile(&mut self, idx: usize) {
        if idx < self.profiles.len() {
            self.selected_profile_idx = idx;
            self.active_profile = self.profiles[idx].clone();
            self.cps_text = self.active_profile.cps.to_string();
            // Persist last-active
            if let Some(ref store) = self.store {
                let _ = store.set_last_active(&self.active_profile.id);
            }
            // Update safety policy from profile
            self.safety = SafetyPolicy::new(
                if self.active_profile.max_session_enabled {
                    self.active_profile.max_session_seconds
                } else {
                    None
                },
                if self.active_profile.cooldown_enabled {
                    self.active_profile.cooldown_seconds
                } else {
                    None
                },
            );
        }
    }

    /// Create a new profile and add it to the list.
    pub fn create_profile(&mut self, name: String) {
        let profile = ClickProfile::new(name);
        if let Some(ref store) = self.store {
            let _ = store.save(&profile);
        }
        self.profiles.push(profile);
    }

    /// Delete a profile by index. Cannot delete the last remaining profile.
    pub fn delete_profile(&mut self, idx: usize) -> bool {
        if self.profiles.len() <= 1 || idx >= self.profiles.len() {
            return false;
        }
        let profile = self.profiles.remove(idx);
        if let Some(ref store) = self.store {
            let _ = store.delete(&profile.id);
        }
        // Adjust selected index
        if self.selected_profile_idx >= self.profiles.len() {
            self.selected_profile_idx = self.profiles.len() - 1;
        }
        self.active_profile = self.profiles[self.selected_profile_idx].clone();
        self.cps_text = self.active_profile.cps.to_string();
        true
    }

    /// Duplicate the active profile.
    pub fn duplicate_profile(&mut self) {
        let dup = self
            .active_profile
            .duplicate(format!("{} (Copy)", self.active_profile.name));
        if let Some(ref store) = self.store {
            let _ = store.save(&dup);
        }
        self.profiles.push(dup);
    }

    /// Save the current active profile to disk.
    pub fn save_active_profile(&mut self) {
        self.active_profile.updated_at = chrono::Utc::now();
        if let Some(ref store) = self.store {
            let _ = store.save(&self.active_profile);
        }
        // Update in list
        if self.selected_profile_idx < self.profiles.len() {
            self.profiles[self.selected_profile_idx] = self.active_profile.clone();
        }
    }

    /// Refresh status from engine state.
    pub fn refresh_status(&mut self) {
        let state = self.engine.current_state();
        self.status_text = state.to_string();

        if let Ok(session_guard) = self.engine.session.lock() {
            if let Some(ref session) = *session_guard {
                if let Some(reason) = session.stop_reason {
                    self.stop_reason_text = reason.to_string();
                } else {
                    self.stop_reason_text.clear();
                }
            }
        }
    }

    /// Validate profile and update error text.
    pub fn validate_and_update(&mut self) -> bool {
        // Parse CPS from text field
        match self.cps_text.parse::<u32>() {
            Ok(cps) => self.active_profile.cps = cps,
            Err(_) => {
                self.validation_error = Some("CPS must be a number between 1 and 50".to_string());
                return false;
            }
        }

        match self.active_profile.validate() {
            Ok(()) => {
                self.validation_error = None;
                true
            }
            Err(e) => {
                self.validation_error = Some(e.to_string());
                false
            }
        }
    }

    /// Get recent events for display.
    pub fn recent_events(&self, limit: usize) -> Vec<EventRecord> {
        self.engine.recent_events(limit)
    }

    /// Check if engine is currently running.
    pub fn is_running(&self) -> bool {
        self.engine.is_running()
    }
}
