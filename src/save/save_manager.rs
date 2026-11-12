//! Reading and writing save slots on disk.
//!
//! Engine-independent (std only). Writing is crash-safe and never destroys
//! the previous save:
//! 1. the new save is written to `<slot>.ron.tmp`;
//! 2. the existing `<slot>.ron` (if any) is copied to `<slot>.ron.bak`;
//! 3. the temporary file is renamed over `<slot>.ron`.
//!
//! A save that fails to load (corrupted or too new) is left untouched.

use std::path::{Path, PathBuf};

use super::serialization::{SaveData, SaveError, decode, encode};

#[derive(Debug, Clone)]
pub struct SaveStorage {
    dir: PathBuf,
}

impl SaveStorage {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn slot_path(&self, slot: u32) -> PathBuf {
        self.dir.join(format!("slot{slot}.ron"))
    }

    pub fn backup_path(&self, slot: u32) -> PathBuf {
        self.dir.join(format!("slot{slot}.ron.bak"))
    }

    fn io_error(path: &Path, source: std::io::Error) -> SaveError {
        SaveError::Io {
            path: path.display().to_string(),
            source,
        }
    }

    pub fn save(&self, slot: u32, data: &SaveData) -> Result<PathBuf, SaveError> {
        let text = encode(data)?;
        std::fs::create_dir_all(&self.dir).map_err(|e| Self::io_error(&self.dir, e))?;

        let path = self.slot_path(slot);
        let tmp = path.with_extension("ron.tmp");
        std::fs::write(&tmp, text).map_err(|e| Self::io_error(&tmp, e))?;
        if path.exists() {
            let backup = self.backup_path(slot);
            std::fs::copy(&path, &backup).map_err(|e| Self::io_error(&backup, e))?;
        }
        std::fs::rename(&tmp, &path).map_err(|e| Self::io_error(&path, e))?;
        tracing::info!(path = %path.display(), "game saved");
        Ok(path)
    }

    pub fn load(&self, slot: u32) -> Result<SaveData, SaveError> {
        let path = self.slot_path(slot);
        if !path.exists() {
            return Err(SaveError::NotFound(path.display().to_string()));
        }
        let text = std::fs::read_to_string(&path).map_err(|e| Self::io_error(&path, e))?;
        decode(&text)
    }

    /// Loads the backup of a slot (offered to the player when the main file
    /// is corrupted).
    pub fn load_backup(&self, slot: u32) -> Result<SaveData, SaveError> {
        let path = self.backup_path(slot);
        if !path.exists() {
            return Err(SaveError::NotFound(path.display().to_string()));
        }
        let text = std::fs::read_to_string(&path).map_err(|e| Self::io_error(&path, e))?;
        decode(&text)
    }
}
