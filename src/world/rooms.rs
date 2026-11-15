//! Room layouts loaded from `assets/maps/*.ron`.
//!
//! Pure module: no Bevy types. Each room is self-contained (geometry,
//! spawns, exits) so that only the current room needs to be in memory.

use serde::{Deserialize, Serialize};

use super::collision::{Aabb, BarrierKind, Solid};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnemySpawn {
    pub enemy: String,
    pub position: (f32, f32),
    /// Patrol half-width around the spawn point.
    #[serde(default = "default_patrol")]
    pub patrol_half_width: f32,
}

fn default_patrol() -> f32 {
    120.0
}

/// A rectangle of level geometry as written in map files.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Platform {
    /// Bottom-left corner.
    pub min: (f32, f32),
    pub size: (f32, f32),
    #[serde(default)]
    pub one_way: bool,
    /// A barrier some transformation passes (V1.9: spectral veils). Written
    /// by the world graph, never by map files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barrier: Option<BarrierKind>,
}

impl Platform {
    pub fn solid(&self) -> Solid {
        Solid {
            aabb: Aabb::new(
                (
                    self.min.0 + self.size.0 / 2.0,
                    self.min.1 + self.size.1 / 2.0,
                ),
                self.size,
            ),
            one_way: self.one_way,
            barrier: self.barrier,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomDefinition {
    pub id: String,
    pub name: String,
    /// Room extent from (0, 0); the camera is clamped to it.
    pub size: (f32, f32),
    pub background: (f32, f32, f32),
    pub platforms: Vec<Platform>,
    pub player_spawn: (f32, f32),
    #[serde(default)]
    pub enemies: Vec<EnemySpawn>,
}

impl RoomDefinition {
    pub fn solids(&self) -> Vec<Solid> {
        self.platforms.iter().map(Platform::solid).collect()
    }
}
