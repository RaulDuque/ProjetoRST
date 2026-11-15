//! Reading and writing save slots on disk.
//!
//! Engine-independent (std only). Writing is crash-safe and never destroys
//! the previous save:
//! 1. the new save is written to `<slot>.ron.tmp`;
//! 2. the existing `<slot>.ron` (if any) is copied to `<slot>.ron.bak`;
//! 3. the temporary file is renamed over `<slot>.ron`.
//!
//! A save that fails to load (corrupted or too new) is left untouched.
//!
//! Format upgrades (V1.7): before the first write over a slot made by an
//! older version, the old file is also copied to `<slot>.ron.v<N>.bak`
//! (e.g. `slot1.ron.v1.bak`). That copy is never overwritten afterwards,
//! so the last save of each old format stays recoverable even after the
//! rolling `.bak` has been replaced by newer saves. Loading never writes.

use std::path::{Path, PathBuf};

use super::serialization::{
    CURRENT_SAVE_VERSION, DecodedSave, SaveData, SaveError, decode_versioned, encode, peek_version,
};

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

    /// Permanent copy of the slot as written by format `version`.
    pub fn version_backup_path(&self, slot: u32, version: u32) -> PathBuf {
        self.dir.join(format!("slot{slot}.ron.v{version}.bak"))
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
            self.keep_old_format(slot, &path)?;
            let backup = self.backup_path(slot);
            std::fs::copy(&path, &backup).map_err(|e| Self::io_error(&backup, e))?;
        }
        std::fs::rename(&tmp, &path).map_err(|e| Self::io_error(&path, e))?;
        tracing::info!(path = %path.display(), "game saved");
        Ok(path)
    }

    /// Copies a slot of an older format to its permanent versioned backup,
    /// once. Unreadable files are left alone (the rolling `.bak` still
    /// receives them).
    fn keep_old_format(&self, slot: u32, path: &Path) -> Result<(), SaveError> {
        let Ok(text) = std::fs::read_to_string(path) else {
            return Ok(());
        };
        let Ok(version) = peek_version(&text) else {
            return Ok(());
        };
        if version >= CURRENT_SAVE_VERSION {
            return Ok(());
        }
        let keep = self.version_backup_path(slot, version);
        if !keep.exists() {
            std::fs::copy(path, &keep).map_err(|e| Self::io_error(&keep, e))?;
            tracing::info!(path = %keep.display(), "old save format kept");
        }
        Ok(())
    }

    pub fn load(&self, slot: u32) -> Result<SaveData, SaveError> {
        self.load_versioned(slot).map(|d| d.data)
    }

    /// Loads a slot, telling which format it was read from. Read-only: a
    /// migrated save is converted in memory and the file is not touched.
    pub fn load_versioned(&self, slot: u32) -> Result<DecodedSave, SaveError> {
        let path = self.slot_path(slot);
        if !path.exists() {
            return Err(SaveError::NotFound(path.display().to_string()));
        }
        let text = std::fs::read_to_string(&path).map_err(|e| Self::io_error(&path, e))?;
        decode_versioned(&text)
    }

    /// Loads the backup of a slot (offered to the player when the main file
    /// is corrupted).
    pub fn load_backup(&self, slot: u32) -> Result<SaveData, SaveError> {
        let path = self.backup_path(slot);
        if !path.exists() {
            return Err(SaveError::NotFound(path.display().to_string()));
        }
        let text = std::fs::read_to_string(&path).map_err(|e| Self::io_error(&path, e))?;
        decode_versioned(&text).map(|d| d.data)
    }
}
