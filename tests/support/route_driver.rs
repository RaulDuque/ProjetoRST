//! Input-only route driver (V1.7): plays a route through the world by
//! pressing keys, like a player at the keyboard. Shared by
//! `tests/world_route_tests.rs` and `examples/world_main_route.rs`
//! (included with `#[path]`; it is test tooling, not part of the game).
//!
//! What it may do: press and release keys (`ButtonInput<KeyCode>`, right
//! after Bevy's input systems so `just_pressed` is seen) and read the game
//! state (player transform, ground contact, current room, enemies,
//! exploration). It never writes a transform, velocity or resource of the
//! game. On top of that it *audits* the run: every frame it compares the
//! player's position with the previous frame, and a jump larger than any
//! movement the physics allows, in a frame without a room change applied by
//! the game (`RoomChangeStats`), is counted as an artificial repositioning.
//!
//! The driver runs as a `PreUpdate` system, so it works both in headless
//! tests (`app.update()` loops) and in a windowed app (`app.run()`).

#![allow(dead_code)]

use std::fmt::Write as _;

use bevy::input::InputSystems;
use bevy::prelude::*;
use dark_castle_rpg::app::states::GameState;
use dark_castle_rpg::combat::attacks::WeaponStrike;
use dark_castle_rpg::combat::hitboxes::Hitbox;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::economy::systems::{PlayerPurse, ShopStockState};
use dark_castle_rpg::enemies::components::Enemy;
use dark_castle_rpg::inventory::equipment::{Equipment, EquipmentSlot};
use dark_castle_rpg::inventory::inventory::Inventory;
use dark_castle_rpg::inventory::visual::CapeLayer;
use dark_castle_rpg::player::components::{Belongings, CharacterSheet, Player, Vitals};
use dark_castle_rpg::transform::systems::Transformation;
use dark_castle_rpg::world::exploration_systems::Exploration;
use dark_castle_rpg::world::graph::Ability;
use dark_castle_rpg::world::maps::CurrentRoom;
use dark_castle_rpg::world::physics::{FacingDirection, Grounded};
use dark_castle_rpg::world::transitions::RoomChangeStats;

/// Largest legitimate displacement in one frame (fall speed, dodge and
/// knockback are all far below it at 16-33 ms frames).
const MAX_FRAME_MOVE: f32 = 40.0;
/// A step that takes longer than this fails the route.
const STEP_TIMEOUT: u32 = 1800;
/// Frames the jump key stays held for a full jump.
const FULL_JUMP_FRAMES: u32 = 24;
/// Horizontal distance at which `WalkTo` is done.
const ARRIVE_TOLERANCE: f32 = 4.0;
/// Distance at which `Fight` stops walking and swings.
const SWING_RANGE: f32 = 34.0;
/// Frames between two attack presses (the basic attack lasts 0.28 s).
const SWING_EVERY: u32 = 18;

pub const LEFT: KeyCode = KeyCode::ArrowLeft;
pub const RIGHT: KeyCode = KeyCode::ArrowRight;
pub const JUMP: KeyCode = KeyCode::KeyZ;
pub const ATTACK: KeyCode = KeyCode::KeyX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
    Still,
}

impl Dir {
    fn key(self) -> Option<KeyCode> {
        match self {
            Dir::Left => Some(LEFT),
            Dir::Right => Some(RIGHT),
            Dir::Still => None,
        }
    }

    fn toward(dx: f32) -> Dir {
        if dx < 0.0 { Dir::Left } else { Dir::Right }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Expect {
    Room(&'static str),
    Secret(&'static str),
    Pickup(&'static str),
    Ability(Ability),
    /// Standing (grounded) with the body centre at this height (±3).
    StandingAt(f32),
    /// The world flag was granted (lever, sigil) (V1.8).
    Flag(&'static str),
    /// The inventory holds at least this many of the item (V1.8).
    HasItem(&'static str, u32),
    /// A transformation is active (true) or the player is human (V1.9).
    FormActive(bool),
    /// MP is at most this value (V1.9: the form's upkeep is being paid).
    MpAtMost(i32),
    /// This item is in this equipment slot (V2.0).
    Equipped(EquipmentSlot, &'static str),
    /// Final ATK / DEF are at least this (V2.0: the sheet's derived stats).
    AttackAtLeast(i32),
    DefenseAtLeast(i32),
    /// A live basic-attack hitbox of the player has this width (V2.0).
    StrikeWidth(f32),
    /// The cape layer is shown (true) or absent (V2.0).
    Cape(bool),
    /// The inventory screen is open (V2.0).
    InventoryOpen(bool),
    /// A conversation / a shop is open (V2.1).
    Talking(bool),
    ShopOpen(bool),
    /// The player's coins are exactly this (V2.1).
    Coins(u32),
    /// Exactly this many of the item in the bag (V2.1.1: no duplication).
    BagCount(&'static str, u32),
    /// Units left of a limited shop entry (V2.1.1).
    StockLeft(&'static str, &'static str, u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    /// Walk to this x of the current room (jumping when blocked).
    WalkTo(f32),
    /// Walk until the game loads this room (jumping when blocked).
    WalkUntilRoom(Dir, &'static str),
    /// A full jump, holding a direction, until landing again.
    Jump(Dir),
    /// Walk to the nearest enemy and attack until the room has none.
    Fight,
    /// Press and release a key once (F5, F9, M...).
    Tap(KeyCode),
    Wait(u32),
    Check(Expect),
    /// A named point of the route (the example takes a screenshot there).
    Mark(&'static str),
}

/// One line of the route log.
#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub frame: u32,
    pub seconds: f32,
    pub room: String,
    pub position: Vec2,
    pub action: String,
    pub result: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RouteStatus {
    Running,
    Passed,
    Failed(String),
}

#[derive(Resource, Debug)]
pub struct RouteDriver {
    steps: Vec<Step>,
    index: usize,
    step_frames: u32,
    frame: u32,
    jump_hold: u32,
    airborne_seen: bool,
    stuck_frames: u32,
    last_x: f32,
    swing_cooldown: u32,
    tap_release: Option<KeyCode>,
    previous: Option<(String, Vec2, u32)>,
    pub status: RouteStatus,
    pub log: Vec<RouteEntry>,
    pub artificial_repositions: u32,
    pub room_changes: u32,
    /// Marks reached, in order (the example screenshots them).
    pub marks: Vec<&'static str>,
}

impl RouteDriver {
    pub fn new(steps: Vec<Step>) -> Self {
        Self {
            steps,
            index: 0,
            step_frames: 0,
            frame: 0,
            jump_hold: 0,
            airborne_seen: false,
            stuck_frames: 0,
            last_x: f32::NAN,
            swing_cooldown: 0,
            tap_release: None,
            previous: None,
            status: RouteStatus::Running,
            log: Vec::new(),
            artificial_repositions: 0,
            room_changes: 0,
            marks: Vec::new(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.status != RouteStatus::Running
    }

    /// The route log as text (one line per step result).
    pub fn report(&self) -> String {
        let mut out = String::new();
        for e in &self.log {
            let _ = writeln!(
                out,
                "{:>6} {:>7.2}s {:<26} ({:>7.1}, {:>7.1})  {:<34} {}",
                e.frame, e.seconds, e.room, e.position.x, e.position.y, e.action, e.result
            );
        }
        let _ = writeln!(
            out,
            "status: {:?}  room changes: {}  artificial repositions: {}",
            self.status, self.room_changes, self.artificial_repositions
        );
        out
    }
}

/// Adds the driver to `app` (keys applied right after Bevy's input systems).
pub fn install(app: &mut App, steps: Vec<Step>) {
    app.insert_resource(RouteDriver::new(steps))
        .add_systems(PreUpdate, drive_route.after(InputSystems));
}

struct View {
    room: String,
    position: Vec2,
    grounded: bool,
    facing: f32,
    seconds: f32,
    inventory: Inventory,
    form_active: bool,
    mp: i32,
    equipment: Equipment,
    attack: i32,
    defense: i32,
    strike_widths: Vec<f32>,
    cape: bool,
    inventory_open: bool,
    game_state: Option<GameState>,
    coins: u32,
    stock_left: Vec<(String, String, u32)>,
}

fn hold(keys: &mut ButtonInput<KeyCode>, dir: Dir) {
    for key in [LEFT, RIGHT] {
        if Some(key) == dir.key() {
            keys.press(key);
        } else {
            keys.release(key);
        }
    }
}

fn release_all(keys: &mut ButtonInput<KeyCode>) {
    for key in [LEFT, RIGHT, JUMP, ATTACK] {
        keys.release(key);
    }
}

pub fn drive_route(
    time: Res<Time>,
    mut driver: ResMut<RouteDriver>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    room: Option<Res<CurrentRoom>>,
    stats: Option<Res<RoomChangeStats>>,
    exploration: Option<Res<Exploration>>,
    player: Query<
        (
            &Transform,
            &Grounded,
            &FacingDirection,
            &Belongings,
            &Vitals,
            Option<&Transformation>,
            &CharacterSheet,
        ),
        With<Player>,
    >,
    enemies: Query<&Transform, (With<Enemy>, Without<Player>)>,
    strikes: Query<&Hitbox, With<WeaponStrike>>,
    capes: Query<&Visibility, With<CapeLayer>>,
    game_state: Option<Res<State<GameState>>>,
    purses: Query<&PlayerPurse, With<Player>>,
    stock: Option<Res<ShopStockState>>,
    data: Option<Res<GameData>>,
) {
    let (Some(room), Ok((transform, grounded, facing, belongings, vitals, form, sheet))) =
        (room, player.single())
    else {
        return;
    };
    let view = View {
        room: room.0.clone(),
        position: transform.translation.truncate(),
        grounded: grounded.on_ground,
        facing: facing.sign(),
        seconds: time.elapsed_secs(),
        inventory: belongings.inventory.clone(),
        form_active: form.is_some_and(|f| f.is_active()),
        mp: vitals.mp.current(),
        equipment: belongings.equipment.clone(),
        attack: sheet.derived().attack,
        defense: sheet.derived().defense,
        strike_widths: strikes.iter().map(|h| h.size.x).collect(),
        cape: capes.iter().any(|v| *v != Visibility::Hidden),
        inventory_open: game_state
            .as_ref()
            .is_some_and(|s| *s.get() == GameState::Inventory),
        game_state: game_state.as_ref().map(|s| *s.get()),
        coins: purses.single().map_or(0, |p| p.coins()),
        stock_left: match (&stock, &data) {
            (Some(stock), Some(data)) => data
                .shops
                .iter()
                .flat_map(|shop| {
                    shop.entries.iter().filter_map(|e| {
                        stock
                            .left(shop, e)
                            .map(|n| (shop.id.clone(), e.item.clone(), n))
                    })
                })
                .collect(),
            _ => Vec::new(),
        },
    };
    let applied = stats.map_or(0, |s| s.applied);
    driver.frame += 1;
    audit(&mut driver, &view, applied);
    if driver.is_done() {
        release_all(&mut keys);
        return;
    }
    if let Some(key) = driver.tap_release.take() {
        keys.release(key);
    }
    let Some(step) = driver.steps.get(driver.index).copied() else {
        release_all(&mut keys);
        driver.status = RouteStatus::Passed;
        let entry = entry(&driver, &view, "end".into(), "PASS".into());
        driver.log.push(entry);
        return;
    };
    driver.step_frames += 1;
    if driver.step_frames > STEP_TIMEOUT {
        release_all(&mut keys);
        let reason = format!("timeout in {step:?}");
        let entry = entry(
            &driver,
            &view,
            format!("{step:?}"),
            format!("FAIL {reason}"),
        );
        driver.log.push(entry);
        driver.status = RouteStatus::Failed(reason);
        return;
    }
    let outcome = advance(&mut driver, &mut keys, &view, step, &exploration, &enemies);
    match outcome {
        Outcome::Continue => {}
        Outcome::Done(result) => {
            let entry = entry(&driver, &view, format!("{step:?}"), result);
            driver.log.push(entry);
            driver.index += 1;
            driver.step_frames = 0;
            driver.jump_hold = 0;
            driver.airborne_seen = false;
            driver.stuck_frames = 0;
        }
        Outcome::Fail(reason) => {
            release_all(&mut keys);
            let entry = entry(
                &driver,
                &view,
                format!("{step:?}"),
                format!("FAIL {reason}"),
            );
            driver.log.push(entry);
            driver.status = RouteStatus::Failed(reason);
        }
    }
}

fn entry(driver: &RouteDriver, view: &View, action: String, result: String) -> RouteEntry {
    RouteEntry {
        frame: driver.frame,
        seconds: view.seconds,
        room: view.room.clone(),
        position: view.position,
        action,
        result,
    }
}

/// Counts room changes made by the game and repositionings made by nobody.
fn audit(driver: &mut RouteDriver, view: &View, applied: u32) {
    if let Some((room, position, last_applied)) = &driver.previous {
        if applied != *last_applied {
            driver.room_changes += applied - last_applied;
        } else if *room == view.room && position.distance(view.position) > MAX_FRAME_MOVE {
            driver.artificial_repositions += 1;
            let line = entry(
                driver,
                view,
                "audit".into(),
                format!("ARTIFICIAL REPOSITION from {position:?}"),
            );
            driver.log.push(line);
        }
    }
    driver.previous = Some((view.room.clone(), view.position, applied));
}

enum Outcome {
    Continue,
    Done(String),
    Fail(String),
}

/// Holds `dir`, jumping over whatever stops the walk.
fn walk(driver: &mut RouteDriver, keys: &mut ButtonInput<KeyCode>, view: &View, dir: Dir) {
    hold(keys, dir);
    if driver.jump_hold > 0 {
        driver.jump_hold -= 1;
        if driver.jump_hold == 0 {
            keys.release(JUMP);
        }
        return;
    }
    let moved = (view.position.x - driver.last_x).abs();
    driver.last_x = view.position.x;
    if view.grounded && moved < 0.3 && dir != Dir::Still {
        driver.stuck_frames += 1;
    } else {
        driver.stuck_frames = 0;
    }
    if driver.stuck_frames >= 4 {
        keys.release(JUMP);
        keys.press(JUMP);
        driver.jump_hold = FULL_JUMP_FRAMES;
        driver.stuck_frames = 0;
    }
}

fn advance(
    driver: &mut RouteDriver,
    keys: &mut ButtonInput<KeyCode>,
    view: &View,
    step: Step,
    exploration: &Option<Res<Exploration>>,
    enemies: &Query<&Transform, (With<Enemy>, Without<Player>)>,
) -> Outcome {
    match step {
        Step::WalkTo(x) => {
            let dx = x - view.position.x;
            if dx.abs() <= ARRIVE_TOLERANCE && view.grounded && driver.jump_hold == 0 {
                hold(keys, Dir::Still);
                return Outcome::Done("arrived".into());
            }
            let dir = if dx.abs() <= ARRIVE_TOLERANCE {
                Dir::Still
            } else {
                Dir::toward(dx)
            };
            walk(driver, keys, view, dir);
            Outcome::Continue
        }
        Step::WalkUntilRoom(dir, target) => {
            if view.room == target {
                hold(keys, Dir::Still);
                keys.release(JUMP);
                return Outcome::Done(format!("entered {target}"));
            }
            walk(driver, keys, view, dir);
            Outcome::Continue
        }
        Step::Jump(dir) => {
            hold(keys, dir);
            if driver.step_frames == 1 {
                keys.release(JUMP);
                keys.press(JUMP);
                return Outcome::Continue;
            }
            if driver.step_frames == FULL_JUMP_FRAMES {
                keys.release(JUMP);
            }
            if !view.grounded {
                driver.airborne_seen = true;
            } else if driver.airborne_seen {
                keys.release(JUMP);
                hold(keys, Dir::Still);
                return Outcome::Done(format!("landed at y={:.1}", view.position.y));
            }
            Outcome::Continue
        }
        Step::Fight => {
            let nearest = enemies
                .iter()
                .map(|t| t.translation.truncate())
                .min_by(|a, b| {
                    a.distance(view.position)
                        .total_cmp(&b.distance(view.position))
                });
            let Some(enemy) = nearest else {
                hold(keys, Dir::Still);
                keys.release(ATTACK);
                return Outcome::Done("no enemy left".into());
            };
            let dx = enemy.x - view.position.x;
            driver.swing_cooldown = driver.swing_cooldown.saturating_sub(1);
            let faces = (dx >= 0.0) == (view.facing > 0.0);
            if dx.abs() > SWING_RANGE || !faces {
                keys.release(ATTACK);
                walk(driver, keys, view, Dir::toward(dx));
            } else {
                hold(keys, Dir::Still);
                if driver.swing_cooldown == 0 {
                    keys.release(ATTACK);
                    keys.press(ATTACK);
                    driver.tap_release = Some(ATTACK);
                    driver.swing_cooldown = SWING_EVERY;
                }
            }
            Outcome::Continue
        }
        Step::Tap(key) => {
            keys.release(key);
            keys.press(key);
            driver.tap_release = Some(key);
            Outcome::Done(format!("tapped {key:?}"))
        }
        Step::Wait(frames) => {
            hold(keys, Dir::Still);
            if driver.step_frames >= frames {
                Outcome::Done("waited".into())
            } else {
                Outcome::Continue
            }
        }
        Step::Mark(name) => {
            driver.marks.push(name);
            Outcome::Done(format!("mark {name}"))
        }
        Step::Check(expect) => check(view, expect, exploration),
    }
}

fn check(view: &View, expect: Expect, exploration: &Option<Res<Exploration>>) -> Outcome {
    let state = exploration.as_ref().map(|e| &e.0);
    let ok = match expect {
        Expect::Room(room) => view.room == room,
        Expect::Secret(id) => state.is_some_and(|s| s.secrets.contains(id)),
        Expect::Pickup(id) => state.is_some_and(|s| s.pickups.contains(id)),
        Expect::Ability(a) => state.is_some_and(|s| s.capabilities().abilities.contains(&a)),
        Expect::StandingAt(y) => view.grounded && (view.position.y - y).abs() <= 3.0,
        Expect::Flag(flag) => state.is_some_and(|s| s.capabilities().flags.contains(flag)),
        Expect::HasItem(item, count) => view.inventory.count(item) >= count,
        Expect::FormActive(active) => view.form_active == active,
        Expect::MpAtMost(mp) => view.mp <= mp,
        Expect::Equipped(slot, item) => view.equipment.get(slot) == Some(item),
        Expect::AttackAtLeast(value) => view.attack >= value,
        Expect::DefenseAtLeast(value) => view.defense >= value,
        Expect::StrikeWidth(width) => view.strike_widths.iter().any(|w| (w - width).abs() < 0.5),
        Expect::Cape(shown) => view.cape == shown,
        Expect::InventoryOpen(open) => view.inventory_open == open,
        Expect::Talking(open) => (view.game_state == Some(GameState::Dialogue)) == open,
        Expect::ShopOpen(open) => (view.game_state == Some(GameState::Shop)) == open,
        Expect::Coins(coins) => view.coins == coins,
        Expect::BagCount(item, count) => view.inventory.count(item) == count,
        Expect::StockLeft(shop, item, left) => view
            .stock_left
            .iter()
            .any(|(s, i, n)| s == shop && i == item && *n == left),
    };
    if ok {
        Outcome::Done("ok".into())
    } else {
        Outcome::Fail(format!("expected {expect:?}"))
    }
}
