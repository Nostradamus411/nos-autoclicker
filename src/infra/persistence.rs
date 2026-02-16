// Profile persistence — TOML-based file storage.

use std::fs;
use std::path::PathBuf;

use crate::domain::errors::PersistenceError;
use crate::domain::profile::ClickProfile;

/// File-per-profile TOML-based storage.
pub struct ProfileStore {
    base_dir: PathBuf,
}

impl ProfileStore {
    /// Create a new store rooted at the given directory.
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Ensure the storage directory exists.
    fn ensure_dir(&self) -> Result<(), PersistenceError> {
        fs::create_dir_all(&self.base_dir).map_err(PersistenceError::Io)
    }

    /// Path to a profile file.
    fn profile_path(&self, id: &str) -> PathBuf {
        self.base_dir.join(format!("{id}.toml"))
    }

    /// Path to the last-active marker file.
    fn last_active_path(&self) -> PathBuf {
        self.base_dir.join("_last_active.txt")
    }

    /// Save a profile to disk.
    pub fn save(&self, profile: &ClickProfile) -> Result<(), PersistenceError> {
        self.ensure_dir()?;
        let toml_str =
            toml::to_string_pretty(profile).map_err(|e| PersistenceError::Serialization(e.to_string()))?;
        fs::write(self.profile_path(&profile.id), toml_str).map_err(PersistenceError::Io)
    }

    /// Load a profile by ID.
    pub fn load(&self, id: &str) -> Result<ClickProfile, PersistenceError> {
        let path = self.profile_path(id);
        if !path.exists() {
            return Err(PersistenceError::NotFound(id.to_string()));
        }
        let content = fs::read_to_string(&path).map_err(PersistenceError::Io)?;
        let profile: ClickProfile =
            toml::from_str(&content).map_err(|e| PersistenceError::Deserialization(e.to_string()))?;
        Ok(profile)
    }

    /// Delete a profile by ID.
    pub fn delete(&self, id: &str) -> Result<(), PersistenceError> {
        let path = self.profile_path(id);
        if !path.exists() {
            return Err(PersistenceError::NotFound(id.to_string()));
        }
        fs::remove_file(&path).map_err(PersistenceError::Io)
    }

    /// List all saved profiles.
    pub fn list_all(&self) -> Result<Vec<ClickProfile>, PersistenceError> {
        self.ensure_dir()?;
        let mut profiles = Vec::new();
        for entry in fs::read_dir(&self.base_dir).map_err(PersistenceError::Io)? {
            let entry = entry.map_err(PersistenceError::Io)?;
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "toml") {
                let content = fs::read_to_string(&path).map_err(PersistenceError::Io)?;
                if let Ok(profile) = toml::from_str::<ClickProfile>(&content) {
                    profiles.push(profile);
                }
            }
        }
        Ok(profiles)
    }

    /// Check if a profile name already exists (case-insensitive), excluding a given ID.
    pub fn has_name_conflict(&self, name: &str, exclude_id: &str) -> Result<bool, PersistenceError> {
        let all = self.list_all()?;
        let lower = name.to_lowercase();
        Ok(all
            .iter()
            .any(|p| p.id != exclude_id && p.name.to_lowercase() == lower))
    }

    /// Set the last-active profile ID.
    pub fn set_last_active(&self, id: &str) -> Result<(), PersistenceError> {
        self.ensure_dir()?;
        fs::write(self.last_active_path(), id).map_err(PersistenceError::Io)
    }

    /// Get the last-active profile ID (None if not set).
    pub fn get_last_active(&self) -> Result<Option<String>, PersistenceError> {
        let path = self.last_active_path();
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&path)
            .map_err(PersistenceError::Io)?
            .trim()
            .to_string();
        if content.is_empty() {
            Ok(None)
        } else {
            Ok(Some(content))
        }
    }
}
