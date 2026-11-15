//! Camera framing rules (V2.3): zoom, dead zones, look-ahead, jump and fall
//! framing and room limits.
//!
//! Pure module: no Bevy types. `assets/data/camera.ron` holds the
//! [`CameraSettings`] (every field has a safe default, so the file and any
//! field may be missing). The Bevy side (`world::camera`) feeds one
//! [`FollowInput`] per frame into a [`CameraRig`] and moves the camera to
//! the returned centre. Zoom is a visible world height; physics, collisions
//! and room sizes never change with it. Details: docs/WORLD_VISUAL_V2_3.md §2.

use serde::Deserialize;
use thiserror::Error;

/// Path of the settings, relative to `assets/`.
pub const CAMERA_SETTINGS_PATH: &str = "data/camera.ron";
/// The reference aspect ratio when no window is known (headless, tests).
pub const REFERENCE_ASPECT: f32 = 16.0 / 9.0;
/// Height of the world the V2.2 camera showed (1:1 on a 720 px window).
pub const UNZOOMED_VIEW_HEIGHT: f32 = 720.0;
/// A move longer than this in one frame is a teleport (respawn, load): the
/// camera snaps instead of sliding across the room.
pub const TELEPORT_DISTANCE: f32 = 200.0;

/// Visual effects that can be switched off for tests and diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EffectToggles {
    pub parallax: bool,
    pub decor: bool,
    pub lights: bool,
    pub fog: bool,
}

impl Default for EffectToggles {
    fn default() -> Self {
        Self {
            parallax: true,
            decor: true,
            lights: true,
            fog: true,
        }
    }
}

impl EffectToggles {
    pub const NONE: EffectToggles = EffectToggles {
        parallax: false,
        decor: false,
        lights: false,
        fog: false,
    };
}

/// `assets/data/camera.ron`. World units unless noted.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CameraSettings {
    /// Visible world height (the zoom): 720 is the unzoomed V2.2 view.
    pub view_height: f32,
    /// Exponential follow rates (1/s), horizontal and vertical.
    pub follow_rate_x: f32,
    pub follow_rate_y: f32,
    /// Half-width of the horizontal dead zone around the focus.
    pub dead_zone_x: f32,
    /// Half-heights of the vertical dead zone while airborne: the focus
    /// only moves up/down when the player leaves it.
    pub dead_zone_up: f32,
    pub dead_zone_down: f32,
    /// Look-ahead distance in the facing direction and how fast it turns.
    pub look_ahead: f32,
    pub look_ahead_rate: f32,
    /// Focus height above the player's centre when standing.
    pub vertical_offset: f32,
    /// Extra look-down while falling faster than `fall_speed`.
    pub fall_look_down: f32,
    pub fall_speed: f32,
    pub effects: EffectToggles,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            view_height: 405.0,
            follow_rate_x: 7.0,
            follow_rate_y: 5.0,
            dead_zone_x: 24.0,
            dead_zone_up: 90.0,
            dead_zone_down: 40.0,
            look_ahead: 70.0,
            look_ahead_rate: 2.5,
            vertical_offset: 40.0,
            fall_look_down: 70.0,
            fall_speed: 420.0,
            effects: EffectToggles::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CameraError {
    #[error("cannot parse the camera settings: {0}")]
    Parse(String),
    #[error("camera setting `{0}` out of range")]
    Range(&'static str),
}

impl CameraSettings {
    pub fn from_ron(text: &str) -> Result<Self, CameraError> {
        let settings: Self = ron::from_str(text).map_err(|e| CameraError::Parse(e.to_string()))?;
        settings.validate()?;
        Ok(settings)
    }

    /// Sane ranges: the zoom never hides more than half the player's room
    /// height nor shows more than the unzoomed view; rates and zones positive.
    pub fn validate(&self) -> Result<(), CameraError> {
        let check = |ok: bool, name| {
            if ok {
                Ok(())
            } else {
                Err(CameraError::Range(name))
            }
        };
        check(
            (240.0..=UNZOOMED_VIEW_HEIGHT).contains(&self.view_height),
            "view_height",
        )?;
        check(
            self.follow_rate_x > 0.0 && self.follow_rate_x <= 60.0,
            "follow_rate_x",
        )?;
        check(
            self.follow_rate_y > 0.0 && self.follow_rate_y <= 60.0,
            "follow_rate_y",
        )?;
        let fits = |v: f32, max: f32| v.is_finite() && (0.0..=max).contains(&v);
        let half_h = self.view_height / 2.0;
        check(fits(self.dead_zone_x, half_h), "dead_zone_x")?;
        check(fits(self.dead_zone_up, half_h), "dead_zone_up")?;
        check(fits(self.dead_zone_down, half_h), "dead_zone_down")?;
        check(fits(self.look_ahead, half_h), "look_ahead")?;
        check(
            self.look_ahead_rate > 0.0 && self.look_ahead_rate <= 60.0,
            "look_ahead_rate",
        )?;
        check(fits(self.vertical_offset, half_h), "vertical_offset")?;
        check(fits(self.fall_look_down, half_h), "fall_look_down")?;
        check(
            self.fall_speed > 0.0 && self.fall_speed.is_finite(),
            "fall_speed",
        )?;
        Ok(())
    }

    /// Zoom factor relative to the V2.2 view (1.0 = unzoomed).
    pub fn zoom(&self) -> f32 {
        UNZOOMED_VIEW_HEIGHT / self.view_height
    }

    /// Visible world size for a window aspect ratio (width / height).
    pub fn view_size(&self, aspect: f32) -> (f32, f32) {
        let aspect = if aspect.is_finite() && aspect > 0.0 {
            aspect
        } else {
            REFERENCE_ASPECT
        };
        (self.view_height * aspect, self.view_height)
    }
}

/// Clamps a camera centre so the view stays inside `[0, room]` on an axis
/// (centred when the room is smaller than the view).
pub fn clamp_axis(target: f32, room: f32, view: f32) -> f32 {
    if room <= view {
        room / 2.0
    } else {
        target.clamp(view / 2.0, room - view / 2.0)
    }
}

/// What the camera knows about the player this frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FollowInput {
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub grounded: bool,
    /// +1 facing right, -1 facing left.
    pub facing: f32,
    pub room: (f32, f32),
    pub view: (f32, f32),
    pub dt: f32,
}

/// Follow state kept between frames.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CameraRig {
    /// Point the camera wants to centre (before look-ahead and limits).
    pub focus: (f32, f32),
    /// Current horizontal look-ahead offset (eases towards the facing).
    pub ahead: f32,
    /// Current extra look-down while falling.
    pub down: f32,
    /// Camera centre.
    pub centre: (f32, f32),
    pub initialised: bool,
}

fn ease(current: f32, target: f32, rate: f32, dt: f32) -> f32 {
    current + (target - current) * (1.0 - (-rate * dt.max(0.0)).exp())
}

impl CameraRig {
    /// Puts the camera on the player at once (start, room change, load,
    /// respawn): no slide across the room.
    pub fn snap(&mut self, s: &CameraSettings, input: &FollowInput) {
        let (x, y) = input.position;
        self.focus = (x, y + s.vertical_offset);
        self.ahead = input.facing.signum() * s.look_ahead;
        self.down = 0.0;
        self.centre = (
            clamp_axis(self.focus.0 + self.ahead, input.room.0, input.view.0),
            clamp_axis(self.focus.1, input.room.1, input.view.1),
        );
        self.initialised = true;
    }

    /// One frame of follow; returns the camera centre.
    pub fn step(&mut self, s: &CameraSettings, input: &FollowInput) -> (f32, f32) {
        let (x, y) = input.position;
        let jumped = (x - self.focus.0).abs() > TELEPORT_DISTANCE + s.dead_zone_x
            || (y + s.vertical_offset - self.focus.1).abs() > TELEPORT_DISTANCE + s.dead_zone_up;
        if !self.initialised || jumped {
            self.snap(s, input);
            return self.centre;
        }
        // Horizontal dead zone: the focus only follows past its edges.
        let dx = x - self.focus.0;
        if dx.abs() > s.dead_zone_x {
            self.focus.0 = x - s.dead_zone_x * dx.signum();
        }
        // Vertical: re-centre on the ground; in the air only past the zone,
        // so a jump does not drag the view up and down.
        let wanted_y = y + s.vertical_offset;
        if input.grounded {
            self.focus.1 = wanted_y;
        } else {
            let dy = wanted_y - self.focus.1;
            if dy > s.dead_zone_up {
                self.focus.1 = wanted_y - s.dead_zone_up;
            } else if dy < -s.dead_zone_down {
                self.focus.1 = wanted_y + s.dead_zone_down;
            }
        }
        // Look-ahead eases towards the facing (no snap on a turn).
        self.ahead = ease(
            self.ahead,
            input.facing.signum() * s.look_ahead,
            s.look_ahead_rate,
            input.dt,
        );
        // Falling fast: look down to see the landing.
        let falling = !input.grounded && input.velocity.1 < -s.fall_speed;
        self.down = ease(
            self.down,
            if falling { s.fall_look_down } else { 0.0 },
            s.follow_rate_y,
            input.dt,
        );
        let target = (
            clamp_axis(self.focus.0 + self.ahead, input.room.0, input.view.0),
            clamp_axis(self.focus.1 - self.down, input.room.1, input.view.1),
        );
        self.centre = (
            ease(self.centre.0, target.0, s.follow_rate_x, input.dt),
            ease(self.centre.1, target.1, s.follow_rate_y, input.dt),
        );
        // The easing never leaves the room (both ends are inside it).
        self.centre = (
            clamp_axis(self.centre.0, input.room.0, input.view.0),
            clamp_axis(self.centre.1, input.room.1, input.view.1),
        );
        self.centre
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(x: f32, y: f32) -> FollowInput {
        FollowInput {
            position: (x, y),
            velocity: (0.0, 0.0),
            grounded: true,
            facing: 1.0,
            room: (2560.0, 720.0),
            view: (720.0, 405.0),
            dt: 1.0 / 60.0,
        }
    }

    fn settle(
        rig: &mut CameraRig,
        s: &CameraSettings,
        i: &FollowInput,
        frames: usize,
    ) -> (f32, f32) {
        let mut c = rig.centre;
        for _ in 0..frames {
            c = rig.step(s, i);
        }
        c
    }

    #[test]
    fn defaults_are_valid_and_a_missing_file_field_keeps_them() {
        let s = CameraSettings::default();
        assert!(s.validate().is_ok());
        assert!((s.zoom() - 720.0 / 405.0).abs() < 1e-6);
        let partial =
            CameraSettings::from_ron("(view_height: 480.0)").unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(partial.view_height, 480.0);
        assert_eq!(partial.follow_rate_x, s.follow_rate_x);
        assert_eq!(CameraSettings::from_ron("()"), Ok(s));
    }

    #[test]
    fn absurd_zoom_and_unknown_fields_are_refused() {
        assert_eq!(
            CameraSettings::from_ron("(view_height: 100.0)"),
            Err(CameraError::Range("view_height"))
        );
        assert_eq!(
            CameraSettings::from_ron("(view_height: 2000.0)"),
            Err(CameraError::Range("view_height"))
        );
        assert!(matches!(
            CameraSettings::from_ron("(zoom: 2.0)"),
            Err(CameraError::Parse(_))
        ));
    }

    #[test]
    fn view_size_follows_the_window_aspect_with_a_fixed_height() {
        let s = CameraSettings::default();
        assert_eq!(s.view_size(16.0 / 9.0), (720.0, 405.0));
        assert_eq!(s.view_size(4.0 / 3.0), (540.0, 405.0));
        assert_eq!(s.view_size(f32::NAN), (720.0, 405.0), "unknown aspect");
    }

    #[test]
    fn limits_keep_the_view_inside_small_large_and_vertical_rooms() {
        assert_eq!(clamp_axis(0.0, 2560.0, 720.0), 360.0);
        assert_eq!(clamp_axis(9999.0, 2560.0, 720.0), 2200.0);
        assert_eq!(
            clamp_axis(50.0, 600.0, 720.0),
            300.0,
            "room smaller than view"
        );
        let s = CameraSettings::default();
        let mut rig = CameraRig::default();
        // A 1x3 vertical room: at the top, the view stays inside.
        let mut i = input(640.0, 2100.0);
        i.room = (1280.0, 2160.0);
        let c = settle(&mut rig, &s, &i, 200);
        assert!(c.1 <= 2160.0 - 405.0 / 2.0 + 1e-3 && c.1 > 1500.0, "{c:?}");
        // At the bottom.
        let mut i = input(640.0, 63.0);
        i.room = (1280.0, 2160.0);
        let c = settle(&mut rig, &s, &i, 400);
        assert!((c.1 - 202.5).abs() < 1e-3, "{c:?}");
    }

    #[test]
    fn the_horizontal_dead_zone_ignores_small_moves() {
        let s = CameraSettings::default();
        let mut rig = CameraRig::default();
        rig.snap(&s, &input(1000.0, 63.0));
        let focus = rig.focus.0;
        rig.step(&s, &input(1000.0 + s.dead_zone_x - 1.0, 63.0));
        assert_eq!(rig.focus.0, focus, "inside the zone: no move");
        rig.step(&s, &input(1000.0 + s.dead_zone_x + 10.0, 63.0));
        assert!(
            (rig.focus.0 - (focus + 10.0)).abs() < 1e-3,
            "past the edge: follows"
        );
    }

    #[test]
    fn a_jump_inside_the_vertical_zone_does_not_move_the_view() {
        let s = CameraSettings::default();
        let mut rig = CameraRig::default();
        // Mid-height of a tall room: no room limit interferes.
        let floor = 900.0;
        let mut ground = input(1000.0, floor);
        ground.room = (2560.0, 2160.0);
        settle(&mut rig, &s, &ground, 120);
        let before = rig.centre.1;
        let mut air = ground;
        air.position.1 = floor + s.dead_zone_up - 5.0;
        air.grounded = false;
        let c = settle(&mut rig, &s, &air, 30);
        assert!((c.1 - before).abs() < 1e-3, "{before} -> {}", c.1);
        // A higher climb (a ledge) moves the focus.
        air.position.1 = floor + s.dead_zone_up + 60.0;
        let c = settle(&mut rig, &s, &air, 60);
        assert!(c.1 > before + 30.0);
    }

    #[test]
    fn falling_fast_looks_down_and_landing_recentres() {
        let s = CameraSettings::default();
        let mut rig = CameraRig::default();
        let mut i = input(1000.0, 400.0);
        i.room = (1280.0, 2160.0);
        settle(&mut rig, &s, &i, 120);
        let standing = rig.centre.1;
        i.grounded = false;
        i.velocity.1 = -800.0;
        settle(&mut rig, &s, &i, 60);
        assert!(rig.down > s.fall_look_down * 0.9);
        assert!(rig.centre.1 < standing - 30.0, "looks down");
        i.grounded = true;
        i.velocity.1 = 0.0;
        settle(&mut rig, &s, &i, 240);
        assert!(rig.down < 1.0 && (rig.centre.1 - standing).abs() < 1.0);
    }

    #[test]
    fn look_ahead_eases_when_the_player_turns() {
        let s = CameraSettings::default();
        let mut rig = CameraRig::default();
        let right = input(1280.0, 63.0);
        settle(&mut rig, &s, &right, 200);
        assert!((rig.ahead - s.look_ahead).abs() < 0.5);
        let mut left = right;
        left.facing = -1.0;
        rig.step(&s, &left);
        assert!(rig.ahead > s.look_ahead * 0.8, "no snap on the first frame");
        settle(&mut rig, &s, &left, 300);
        assert!((rig.ahead + s.look_ahead).abs() < 0.5);
    }

    #[test]
    fn a_teleport_snaps_instead_of_sliding() {
        let s = CameraSettings::default();
        let mut rig = CameraRig::default();
        settle(&mut rig, &s, &input(400.0, 63.0), 60);
        let far = input(2300.0, 63.0);
        let c = rig.step(&s, &far);
        assert_eq!(c.0, clamp_axis(2300.0 + s.look_ahead, 2560.0, 720.0));
    }
}
