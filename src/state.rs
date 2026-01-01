//! State persistence module
//!
//! Handles saving and loading application state to/from disk.

use crate::desk_object::DeskObject;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application state that gets persisted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Version of the state format
    pub version: u32,
    /// All desk objects
    pub objects: Vec<DeskObject>,
    /// Global collision radius multiplier
    pub collision_radius_multiplier: f32,
    /// Global collision height multiplier
    pub collision_height_multiplier: f32,
    /// Next object ID to use
    pub next_object_id: u64,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            version: 1,
            objects: Vec::new(),
            collision_radius_multiplier: 1.0,
            collision_height_multiplier: 1.0,
            next_object_id: 1,
        }
    }
}

impl AppState {
    /// Create a new empty state
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the path to the state file
    fn state_file_path() -> Option<PathBuf> {
        dirs::data_dir().map(|mut path| {
            path.push("focus-desktop-simulator");
            fs::create_dir_all(&path).ok();
            path.push("desk-state.json");
            path
        })
    }

    /// Load state from disk
    pub fn load() -> Self {
        let path = match Self::state_file_path() {
            Some(p) => p,
            None => {
                log::warn!("Could not determine data directory, using default state");
                return Self::default();
            }
        };

        if !path.exists() {
            log::info!("No saved state found, using default");
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => {
                match serde_json::from_str::<AppState>(&content) {
                    Ok(state) => {
                        log::info!("Loaded state with {} objects", state.objects.len());
                        state
                    }
                    Err(e) => {
                        log::warn!(
                            "State file format is outdated or corrupted: {}. \
                            Creating backup and using default state.",
                            e
                        );
                        // Try to backup the corrupted file for potential recovery
                        Self::backup_corrupted_state(&path);
                        Self::default()
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to read state file: {}", e);
                Self::default()
            }
        }
    }

    /// Backup a corrupted state file so user doesn't lose data
    fn backup_corrupted_state(path: &PathBuf) {
        let backup_path = path.with_extension("json.backup");
        if let Err(e) = fs::copy(path, &backup_path) {
            log::warn!("Could not backup corrupted state file: {}", e);
        } else {
            log::info!(
                "Backed up old state file to {:?}. \
                You can try to recover data from this file manually.",
                backup_path
            );
        }
    }

    /// Save state to disk
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::state_file_path()
            .ok_or("Could not determine data directory")?;

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;

        log::info!("Saved state with {} objects to {:?}", self.objects.len(), path);
        Ok(())
    }

    /// Generate a new unique object ID
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_object_id;
        self.next_object_id += 1;
        id
    }

    /// Add an object to the state
    pub fn add_object(&mut self, object: DeskObject) {
        self.objects.push(object);
    }

    /// Remove an object by ID
    pub fn remove_object(&mut self, id: u64) -> Option<DeskObject> {
        if let Some(pos) = self.objects.iter().position(|o| o.id == id) {
            Some(self.objects.remove(pos))
        } else {
            None
        }
    }

    /// Get an object by ID
    pub fn get_object(&self, id: u64) -> Option<&DeskObject> {
        self.objects.iter().find(|o| o.id == id)
    }

    /// Get a mutable reference to an object by ID
    pub fn get_object_mut(&mut self, id: u64) -> Option<&mut DeskObject> {
        self.objects.iter_mut().find(|o| o.id == id)
    }

    /// Clear all objects
    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }
}
