//! Local controls file. It is not the character save and it is not in
//! `assets/`. A broken file is left on disk.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::profile::ControlProfile;

#[derive(Debug)]
pub enum LoadError {
    Missing,
    Invalid(String),
}

/// Linux: `$XDG_CONFIG_HOME/dark_castle_rpg/controls.ron`, or
/// `~/.config/dark_castle_rpg/controls.ron`.
/// Windows: `%APPDATA%/dark_castle_rpg/controls.ron`.
pub fn user_path() -> PathBuf {
    if let Some(path) = env_dir("XDG_CONFIG_HOME") {
        return path.join("dark_castle_rpg").join("controls.ron");
    }
    if let Some(path) = env_dir("APPDATA") {
        return path.join("dark_castle_rpg").join("controls.ron");
    }
    if let Some(home) = env_dir("HOME") {
        return home
            .join(".config")
            .join("dark_castle_rpg")
            .join("controls.ron");
    }
    PathBuf::from("dark_castle_rpg").join("controls.ron")
}

fn env_dir(name: &str) -> Option<PathBuf> {
    let value = std::env::var(name).ok()?;
    if value.is_empty() {
        return None;
    }
    Some(PathBuf::from(value))
}

pub fn load(path: &Path) -> Result<ControlProfile, LoadError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(LoadError::Missing);
        }
        Err(error) => return Err(LoadError::Invalid(error.to_string())),
    };
    let profile: ControlProfile = ron::from_str(&text)
        .map_err(|error| LoadError::Invalid(format!("could not read controls: {error}")))?;
    profile.validate().map_err(LoadError::Invalid)?;
    Ok(profile)
}

/// Write to a temporary file in the same directory, then rename it over the
/// destination. A crash mid-write leaves the previous file in place.
pub fn save_atomic(path: &Path, profile: &ControlProfile) -> Result<(), String> {
    profile.validate()?;
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let temporary = path.with_extension("ron.tmp");
    let text = ron::ser::to_string_pretty(profile, ron::ser::PrettyConfig::default())
        .map_err(|error| error.to_string())?;
    {
        let mut file = fs::File::create(&temporary).map_err(|error| error.to_string())?;
        file.write_all(text.as_bytes())
            .map_err(|error| error.to_string())?;
        file.sync_all().map_err(|error| error.to_string())?;
    }
    fs::rename(&temporary, path).map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::presets;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "dark_castle_controls_{name}_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("{e}"));
        dir.join("controls.ron")
    }

    #[test]
    fn a_missing_file_is_reported_and_nothing_is_created() {
        let path = scratch("missing");
        match load(&path) {
            Err(LoadError::Missing) => {}
            other => panic!("expected missing, got {other:?}"),
        }
        assert!(!path.exists());
    }

    #[test]
    fn a_round_trip_keeps_the_hybrid_preset() {
        let path = scratch("round");
        let profile = presets::hybrid();
        save_atomic(&path, &profile).unwrap_or_else(|e| panic!("{e}"));
        let loaded = load(&path).unwrap_or_else(|e| panic!("{e:?}"));
        assert!(loaded.same_bindings(&profile));
    }

    #[test]
    fn a_corrupt_file_is_rejected_and_left_untouched() {
        let path = scratch("corrupt");
        fs::write(&path, "this is not a profile").unwrap_or_else(|e| panic!("{e}"));
        let before = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{e}"));
        assert!(matches!(load(&path), Err(LoadError::Invalid(_))));
        let after = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(before, after);
    }
}
