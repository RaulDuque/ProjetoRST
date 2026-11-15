//! Equipment sockets of the player's sprite frames (V2.2).
//!
//! Pure module: no Bevy types. `assets/animations/player_sockets.ron` is
//! measured in Blender by `art-src/player/scripts/bpy/build_game_sprites_v2_2.py`
//! (the same poses as the atlases) and staged next to the manifest by
//! `post/game_sprites_v2_2.py`: for every frame of every clip, where the
//! right hand (weapon grip), the left hand, the head, the chest and the back
//! (cape collar) are.
//!
//! Units: world units (1 u = 1 contract pixel) relative to the clip's
//! sole-line pivot, `x` towards where the art faces (right), `y` up. Angles
//! are degrees counter-clockwise from `+x`: the direction from the pommel to
//! the guard of the held weapon. Facing left mirrors `x` and the angle
//! ([`place`]). Positions are derived from the art, so they are never
//! saved. Details: docs/VISUAL_EQUIPMENT_V2_2.md §5.

use std::collections::BTreeMap;

use serde::Deserialize;
use thiserror::Error;

use super::manifest::AnimationManifest;

/// Path of the socket table, relative to `assets/`.
pub const SOCKETS_PATH: &str = "animations/player_sockets.ron";
/// Only version understood.
pub const SOCKETS_VERSION: u32 = 1;
/// A socket further than this from the pivot is a measuring error (the
/// combat cell is 128x96 u).
pub const MAX_SOCKET_DISTANCE: f32 = 128.0;

/// A point on the body.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// A hand: where it is and whether it is in front of the chest (towards
/// the camera, so what it holds draws over the body).
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Hand {
    pub x: f32,
    pub y: f32,
    pub front: bool,
}

/// The weapon hand: grip centre, blade direction and depth.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Grip {
    pub x: f32,
    pub y: f32,
    /// Degrees counter-clockwise from +x (pommel -> guard).
    pub angle: f32,
    pub front: bool,
}

/// Every socket of one frame.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct FrameSockets {
    pub weapon: Grip,
    pub hand_l: Hand,
    pub head: Point,
    pub chest: Point,
    pub back: Point,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SocketTable {
    pub version: u32,
    /// Clip key (as in the manifest) -> one entry per frame.
    pub clips: BTreeMap<String, Vec<FrameSockets>>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum SocketError {
    #[error("cannot parse the socket table: {0}")]
    Parse(String),
    #[error("unsupported socket table version {0}")]
    Version(u32),
    #[error("clip {0}: not in the animation manifest")]
    UnknownClip(String),
    #[error("clip {clip}: {sockets} socket frames for {frames} sprite frames")]
    FrameCount {
        clip: String,
        sockets: usize,
        frames: u32,
    },
    #[error("clip {clip} frame {frame}: socket value not finite or too far from the pivot")]
    Value { clip: String, frame: usize },
}

impl FrameSockets {
    fn values(&self) -> [f32; 11] {
        [
            self.weapon.x,
            self.weapon.y,
            self.weapon.angle,
            self.hand_l.x,
            self.hand_l.y,
            self.head.x,
            self.head.y,
            self.chest.x,
            self.chest.y,
            self.back.x,
            self.back.y,
        ]
    }

    fn is_sane(&self) -> bool {
        let finite = self.values().iter().all(|v| v.is_finite());
        let points = [
            (self.weapon.x, self.weapon.y),
            (self.hand_l.x, self.hand_l.y),
            (self.head.x, self.head.y),
            (self.chest.x, self.chest.y),
            (self.back.x, self.back.y),
        ];
        finite
            && points
                .iter()
                .all(|(x, y)| x.hypot(*y) <= MAX_SOCKET_DISTANCE)
    }
}

impl SocketTable {
    /// Parses a table and checks it against the manifest it belongs to.
    pub fn from_ron(text: &str, manifest: &AnimationManifest) -> Result<Self, SocketError> {
        let table: Self = ron::from_str(text).map_err(|e| SocketError::Parse(e.to_string()))?;
        table.validate(manifest)?;
        Ok(table)
    }

    /// Every clip must exist with exactly its frame count; values must be
    /// finite and inside the cell. Clips without sockets are allowed (their
    /// layers fall back, see [`SocketTable::frame`]).
    pub fn validate(&self, manifest: &AnimationManifest) -> Result<(), SocketError> {
        if self.version != SOCKETS_VERSION {
            return Err(SocketError::Version(self.version));
        }
        for (key, frames) in &self.clips {
            let clip = manifest
                .clips
                .get(key)
                .ok_or_else(|| SocketError::UnknownClip(key.clone()))?;
            if frames.len() != clip.count as usize {
                return Err(SocketError::FrameCount {
                    clip: key.clone(),
                    sockets: frames.len(),
                    frames: clip.count,
                });
            }
            if let Some(frame) = frames.iter().position(|f| !f.is_sane()) {
                return Err(SocketError::Value {
                    clip: key.clone(),
                    frame,
                });
            }
        }
        Ok(())
    }

    /// Sockets of a frame. A clip without sockets (an effect clip, or a
    /// table older than the manifest) returns `None`: the equipment layers
    /// are then hidden for that frame instead of floating somewhere wrong.
    pub fn frame(&self, clip: &str, frame: usize) -> Option<&FrameSockets> {
        self.clips.get(clip)?.get(frame)
    }
}

/// A socket mirrored for the facing: `(x, y, angle_radians)` in the
/// player's local space (x right on screen). `facing_sign` is +1 facing
/// right, -1 facing left; mirroring negates x and the angle (the sprite is
/// flipped horizontally, so its rotation turns the other way).
pub fn place(x: f32, y: f32, angle_degrees: f32, facing_sign: f32) -> (f32, f32, f32) {
    let sign = if facing_sign < 0.0 { -1.0 } else { 1.0 };
    (x * sign, y, angle_degrees.to_radians() * sign)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> AnimationManifest {
        AnimationManifest::from_ron(
            r#"(
                version: 1,
                facing: Right,
                atlases: {
                    "locomotion": (texture: "a.png", cell: (128, 128), columns: 8, rows: 2, padding: 2, density: 2),
                },
                clips: {
                    "idle": (atlas: "locomotion", first: 0, count: 2, mode: Loop(fps: 12.0), pivot: (64.0, 126.0),
                        fallback: None, durations: [0.1, 0.1], speed_reference: None),
                },
            )"#,
        )
        .unwrap_or_else(|e| panic!("{e}"))
    }

    const FRAME: &str = "(weapon: (x: 10.0, y: 30.0, angle: 45.0, front: true), \
        hand_l: (x: -6.0, y: 28.0, front: false), head: (x: 1.0, y: 46.0), \
        chest: (x: 0.0, y: 36.0), back: (x: -2.0, y: 39.0))";

    fn table(frames: usize) -> String {
        let list = vec![FRAME; frames].join(", ");
        format!("(version: 1, clips: {{ \"idle\": [{list}] }})")
    }

    #[test]
    fn a_complete_table_parses_and_finds_frames() {
        let t = SocketTable::from_ron(&table(2), &manifest()).unwrap_or_else(|e| panic!("{e}"));
        let f = t.frame("idle", 1).unwrap_or_else(|| panic!("frame"));
        assert_eq!(f.weapon.angle, 45.0);
        assert!(f.weapon.front && !f.hand_l.front);
        assert!(t.frame("idle", 2).is_none());
        assert!(t.frame("run", 0).is_none(), "clip without sockets");
    }

    #[test]
    fn frame_count_must_match_the_manifest() {
        let e = SocketTable::from_ron(&table(3), &manifest());
        assert!(matches!(e, Err(SocketError::FrameCount { sockets: 3, .. })));
    }

    #[test]
    fn unknown_clips_and_versions_are_refused() {
        let text = table(2).replace("\"idle\"", "\"swim\"");
        assert!(matches!(
            SocketTable::from_ron(&text, &manifest()),
            Err(SocketError::UnknownClip(_))
        ));
        let text = table(2).replace("version: 1", "version: 9");
        assert_eq!(
            SocketTable::from_ron(&text, &manifest()),
            Err(SocketError::Version(9))
        );
    }

    #[test]
    fn incomplete_or_absurd_data_is_refused() {
        let missing = table(2).replace(", back: (x: -2.0, y: 39.0)", "");
        assert!(matches!(
            SocketTable::from_ron(&missing, &manifest()),
            Err(SocketError::Parse(_))
        ));
        let far = table(2).replace("x: 10.0", "x: 900.0");
        assert!(matches!(
            SocketTable::from_ron(&far, &manifest()),
            Err(SocketError::Value { .. })
        ));
    }

    #[test]
    fn facing_left_mirrors_position_and_angle() {
        let (x, y, a) = place(10.0, 30.0, 45.0, 1.0);
        assert_eq!((x, y), (10.0, 30.0));
        assert!((a - 45f32.to_radians()).abs() < 1e-6);
        let (x, y, a) = place(10.0, 30.0, 45.0, -1.0);
        assert_eq!((x, y), (-10.0, 30.0));
        assert!((a + 45f32.to_radians()).abs() < 1e-6);
    }
}
