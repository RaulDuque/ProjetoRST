//! Which protagonist art the game shows (N3.8). Pure module: no Bevy types.
//!
//! The production art (V2.2 body, every state animated) is the default and
//! stays untouched. The new protagonist approved in N3.5/N3.6 only has an
//! idle pose and the 15 fps run, so it is an **experimental presentation
//! mode**, chosen with `DARK_CASTLE_PROTAGONIST=n3 cargo run`. Its art is
//! read from a stage laid out like `assets/` through the same loader, the
//! same manifest format and the same animation systems (no second
//! animation system). Promoting it to the default is a pending owner gate.
//! Details: docs/development/n3_8/N3_8_CORRIDA_PRODUCAO.md.

use thiserror::Error;

/// Environment variable that selects the protagonist art.
pub const PROTAGONIST_ENV: &str = "DARK_CASTLE_PROTAGONIST";
/// Stage of the experimental protagonist, relative to the project root.
pub const N3_EXPERIMENTAL_STAGE: &str = "art-src/n3-8-production-run/stage";

/// The protagonist art the game is built with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProtagonistArt {
    /// The published V2.2 art in `assets/` (every state animated).
    #[default]
    Production,
    /// N3.5 idle + N3.6 run at 15 fps; other states hold the idle pose.
    N3Experimental,
}

/// The value of [`PROTAGONIST_ENV`] is not a known art.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("unknown DARK_CASTLE_PROTAGONIST value {0:?} (expected \"production\" or \"n3\")")]
pub struct UnknownArt(pub String);

impl ProtagonistArt {
    /// Reads the setting. Unset or empty means the production art.
    pub fn from_setting(value: Option<&str>) -> Result<Self, UnknownArt> {
        match value.map(str::trim) {
            None | Some("") => Ok(Self::Production),
            Some(v) if v.eq_ignore_ascii_case("production") || v.eq_ignore_ascii_case("v2_2") => {
                Ok(Self::Production)
            }
            Some(v)
                if v.eq_ignore_ascii_case("n3") || v.eq_ignore_ascii_case("n3_experimental") =>
            {
                Ok(Self::N3Experimental)
            }
            Some(other) => Err(UnknownArt(other.to_owned())),
        }
    }

    /// The setting, falling back to the production art when it is unknown
    /// (the error is returned so the caller can report it).
    pub fn from_setting_or_production(value: Option<&str>) -> (Self, Option<UnknownArt>) {
        match Self::from_setting(value) {
            Ok(art) => (art, None),
            Err(error) => (Self::Production, Some(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unset_or_empty_is_the_production_art() {
        assert_eq!(
            ProtagonistArt::from_setting(None),
            Ok(ProtagonistArt::Production)
        );
        assert_eq!(
            ProtagonistArt::from_setting(Some("  ")),
            Ok(ProtagonistArt::Production)
        );
        assert_eq!(ProtagonistArt::default(), ProtagonistArt::Production);
    }

    #[test]
    fn known_values_select_the_art() {
        for v in ["production", "V2_2"] {
            assert_eq!(
                ProtagonistArt::from_setting(Some(v)),
                Ok(ProtagonistArt::Production)
            );
        }
        for v in ["n3", "N3", " n3_experimental "] {
            assert_eq!(
                ProtagonistArt::from_setting(Some(v)),
                Ok(ProtagonistArt::N3Experimental)
            );
        }
    }

    #[test]
    fn an_unknown_value_falls_back_to_production_and_is_reported() {
        let (art, error) = ProtagonistArt::from_setting_or_production(Some("n4"));
        assert_eq!(art, ProtagonistArt::Production);
        let error = error.unwrap_or_else(|| panic!("error expected"));
        assert!(error.to_string().contains("\"n4\""));
    }
}
