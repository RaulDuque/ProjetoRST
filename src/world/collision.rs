//! Axis-aligned collision for 2D platforming.
//!
//! Pure module: no Bevy types. Movement is resolved one axis at a time
//! (X then Y) against static solids. One-way platforms only block bodies
//! falling onto them from above.

use serde::{Deserialize, Serialize};

/// Tolerance used when comparing edges.
pub const COLLISION_EPSILON: f32 = 0.01;

/// Axis-aligned box given by its centre and half extents.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub center: (f32, f32),
    pub half: (f32, f32),
}

impl Aabb {
    pub fn new(center: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            center,
            half: (size.0 / 2.0, size.1 / 2.0),
        }
    }

    pub fn left(&self) -> f32 {
        self.center.0 - self.half.0
    }
    pub fn right(&self) -> f32 {
        self.center.0 + self.half.0
    }
    pub fn bottom(&self) -> f32 {
        self.center.1 - self.half.1
    }
    pub fn top(&self) -> f32 {
        self.center.1 + self.half.1
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        self.left() < other.right() - COLLISION_EPSILON
            && self.right() > other.left() + COLLISION_EPSILON
            && self.bottom() < other.top() - COLLISION_EPSILON
            && self.top() > other.bottom() + COLLISION_EPSILON
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Solid {
    pub aabb: Aabb,
    /// Only collides from above (jump-through platform).
    #[serde(default)]
    pub one_way: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MoveResult {
    pub center: (f32, f32),
    pub velocity: (f32, f32),
    pub grounded: bool,
    pub hit_ceiling: bool,
    pub hit_wall: bool,
}

/// Moves `body` by `velocity * dt`, stopping at solids.
pub fn move_and_collide(body: Aabb, velocity: (f32, f32), dt: f32, solids: &[Solid]) -> MoveResult {
    let mut result = MoveResult {
        center: body.center,
        velocity,
        ..Default::default()
    };

    // Horizontal pass (one-way platforms never block sideways).
    let mut moved = Aabb {
        center: (body.center.0 + velocity.0 * dt, body.center.1),
        half: body.half,
    };
    for solid in solids.iter().filter(|s| !s.one_way) {
        if moved.intersects(&solid.aabb) {
            if velocity.0 > 0.0 {
                moved.center.0 = solid.aabb.left() - body.half.0;
            } else if velocity.0 < 0.0 {
                moved.center.0 = solid.aabb.right() + body.half.0;
            }
            result.velocity.0 = 0.0;
            result.hit_wall = true;
        }
    }

    // Vertical pass.
    let previous_bottom = body.bottom();
    moved.center.1 += velocity.1 * dt;
    for solid in solids {
        if !moved.intersects(&solid.aabb) {
            continue;
        }
        if solid.one_way {
            let was_above = previous_bottom >= solid.aabb.top() - COLLISION_EPSILON;
            if velocity.1 > 0.0 || !was_above {
                continue;
            }
        }
        if velocity.1 <= 0.0 {
            moved.center.1 = solid.aabb.top() + body.half.1;
            result.grounded = true;
        } else {
            moved.center.1 = solid.aabb.bottom() - body.half.1;
            result.hit_ceiling = true;
        }
        result.velocity.1 = 0.0;
    }

    result.center = moved.center;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floor() -> Solid {
        Solid {
            aabb: Aabb::new((0.0, -10.0), (1000.0, 20.0)),
            one_way: false,
        }
    }

    #[test]
    fn lands_on_floor() {
        let body = Aabb::new((0.0, 30.0), (20.0, 40.0));
        let r = move_and_collide(body, (0.0, -600.0), 0.1, &[floor()]);
        assert!(r.grounded);
        assert!((r.center.1 - 20.0).abs() < 1e-3);
        assert_eq!(r.velocity.1, 0.0);
    }

    #[test]
    fn wall_stops_horizontal_motion() {
        let wall = Solid {
            aabb: Aabb::new((50.0, 20.0), (20.0, 100.0)),
            one_way: false,
        };
        let body = Aabb::new((20.0, 20.0), (20.0, 40.0));
        let r = move_and_collide(body, (400.0, 0.0), 0.1, &[wall]);
        assert!(r.hit_wall);
        assert!((r.center.0 - 30.0).abs() < 1e-3);
    }

    #[test]
    fn one_way_platform_lets_jumps_through() {
        let platform = Solid {
            aabb: Aabb::new((0.0, 100.0), (200.0, 10.0)),
            one_way: true,
        };
        let below = Aabb::new((0.0, 80.0), (20.0, 40.0));
        let up = move_and_collide(below, (0.0, 300.0), 0.05, &[platform]);
        assert!(!up.hit_ceiling);

        let above = Aabb::new((0.0, 126.0), (20.0, 40.0));
        let down = move_and_collide(above, (0.0, -300.0), 0.05, &[platform]);
        assert!(down.grounded);
    }
}
