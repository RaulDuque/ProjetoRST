//! Headless tests of the player's visual effects (V1.5).
//!
//! Same setup as `player_visual_tests.rs`: the real game app with an
//! `AssetServer` but no renderer, the real manifest, and an image put under
//! every atlas handle to mark the textures as loaded. What is checked: which
//! effect appears, when, facing which way, for how long, that it is removed
//! (no entity left behind), and that gameplay is bit-for-bit identical with
//! and without effects (no damage, no heal, no motion).

use std::path::Path;
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::combat::damage::DamageOutcome;
use dark_castle_rpg::combat::hitboxes::Hitbox;
use dark_castle_rpg::core::events::{DamageDealt, SkillActivated};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::player::components::{ActionState, Player, SkillBook, Vitals};
use dark_castle_rpg::player::state::{Facing, PlayerState};
use dark_castle_rpg::player::visual::manifest::anchor_for;
use dark_castle_rpg::player::visual::vfx::PlayerVfx;
use dark_castle_rpg::player::visual::{PlayerSpriteSet, PlayerVisual};
use dark_castle_rpg::world::physics::FacingDirection;

const FRAME: Duration = Duration::from_millis(16);
const DT: f32 = 0.016;

#[derive(Resource, Default)]
struct PendingKeys {
    press: Vec<KeyCode>,
    release: Vec<KeyCode>,
}

fn apply_pending_keys(mut pending: ResMut<PendingKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.release.drain(..) {
        keys.release(key);
    }
    for key in pending.press.drain(..) {
        keys.press(key);
    }
}

#[derive(Resource, Default)]
struct PendingActivations(Vec<String>);

/// Hits to deliver to the player on the next frame, as `detect_hits` would.
#[derive(Resource, Default)]
struct PendingHits(Vec<f32>);

fn emit_pending(
    mut activations: ResMut<PendingActivations>,
    mut hits: ResMut<PendingHits>,
    player: Query<Entity, With<Player>>,
    mut activated: MessageWriter<SkillActivated>,
    mut damage: MessageWriter<DamageDealt>,
) {
    let Ok(player) = player.single() else {
        return;
    };
    for skill_id in activations.0.drain(..) {
        activated.write(SkillActivated {
            caster: player,
            skill_id,
        });
    }
    for direction in hits.0.drain(..) {
        damage.write(DamageDealt {
            attacker: player,
            target: player,
            outcome: DamageOutcome::Hit {
                amount: 10,
                critical: false,
                element: Element::Physical,
            },
            direction,
            knockback: Default::default(),
        });
    }
}

fn app(assets: bool) -> App {
    let data = GameData::load(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("game data failed to load: {e}"));
    let mut app = App::new();
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin));
    if assets {
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>()
            .init_asset::<TextureAtlasLayout>();
    }
    app.insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .init_resource::<PendingKeys>()
        .init_resource::<PendingActivations>()
        .init_resource::<PendingHits>()
        .add_systems(Update, emit_pending.in_set(GameSet::Commands))
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_plugins(GamePlugin);
    app
}

fn mark_textures_loaded(app: &mut App) {
    let handles: Vec<Handle<Image>> = app
        .world()
        .resource::<PlayerSpriteSet>()
        .atlases
        .values()
        .map(|a| a.image.clone())
        .collect();
    let mut images = app.world_mut().resource_mut::<Assets<Image>>();
    for handle in handles {
        images
            .insert(&handle, Image::default())
            .unwrap_or_else(|e| panic!("cannot insert test image: {e}"));
    }
}

fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

fn loaded_app() -> App {
    let mut app = app(true);
    app.update();
    mark_textures_loaded(&mut app);
    run(&mut app, 30);
    grant_all_skills(&mut app);
    app
}

fn player(app: &mut App) -> Entity {
    let world = app.world_mut();
    world
        .query_filtered::<Entity, With<Player>>()
        .single(world)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn state(app: &mut App) -> PlayerState {
    let world = app.world_mut();
    world
        .query_filtered::<&ActionState, With<Player>>()
        .single(world)
        .map(|s| s.state())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn with_player<T: Component<Mutability = bevy::ecs::component::Mutable>>(
    app: &mut App,
    f: impl FnOnce(&mut T),
) {
    let entity = player(app);
    let mut c = app
        .world_mut()
        .get_mut::<T>(entity)
        .unwrap_or_else(|| panic!("player component missing"));
    f(&mut c);
}

fn grant_all_skills(app: &mut App) {
    let defs: Vec<_> = app
        .world()
        .resource::<GameData>()
        .skills
        .iter()
        .cloned()
        .collect();
    with_player::<SkillBook>(app, |book| {
        for def in &defs {
            book.learned.grant(def);
        }
    });
}

fn cast(app: &mut App, skill: &str) {
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .push(skill.to_owned());
}

#[derive(Debug, Clone, PartialEq)]
struct SeenVfx {
    clip: String,
    parent: Entity,
    index: usize,
    flip: bool,
    anchor: Vec2,
}

fn effects(app: &mut App) -> Vec<SeenVfx> {
    let world = app.world_mut();
    let mut q = world.query::<(&PlayerVfx, &ChildOf, &Sprite, &Anchor)>();
    q.iter(world)
        .map(|(vfx, parent, sprite, anchor)| SeenVfx {
            clip: vfx.clip.clone(),
            parent: parent.parent(),
            index: sprite
                .texture_atlas
                .as_ref()
                .map_or(usize::MAX, |a| a.index),
            flip: sprite.flip_x,
            anchor: anchor.0,
        })
        .collect()
}

fn clip_range(app: &App, clip: &str) -> (usize, usize, f32) {
    let c = app
        .world()
        .resource::<PlayerSpriteSet>()
        .manifest
        .clips
        .get(clip)
        .unwrap_or_else(|| panic!("manifest without {clip}"));
    (c.first as usize, c.count as usize, c.total_duration())
}

/// Frames during which `clip` is on screen, checking it plays forward.
fn lifetime(app: &mut App, clip: &str, max: usize) -> usize {
    let (first, count, _) = clip_range(app, clip);
    let mut frames = 0;
    let mut last = first;
    for _ in 0..max {
        let seen: Vec<_> = effects(app)
            .into_iter()
            .filter(|e| e.clip == clip)
            .collect();
        assert!(seen.len() <= 1, "{clip} duplicated: {seen:?}");
        let Some(e) = seen.first() else {
            break;
        };
        assert!(
            e.index >= last && e.index < first + count,
            "{clip} frame {e:?}"
        );
        last = e.index;
        frames += 1;
        run(app, 1);
    }
    frames
}

#[test]
fn the_manifest_has_an_effect_for_every_skill_vfx_key_but_the_projectile() {
    let app = loaded_app();
    let set = app.world().resource::<PlayerSpriteSet>();
    for def in app.world().resource::<GameData>().skills.iter() {
        let clip = format!("vfx_{}", def.vfx);
        let has = set.manifest.clips.contains_key(&clip);
        // The Ember Bolt keeps the projectile of the game as its visual.
        assert_eq!(has, def.vfx != "fire_bolt", "{}: {clip}", def.id);
    }
    for clip in ["vfx_guard_spark", "vfx_revive"] {
        assert!(set.manifest.clips.contains_key(clip), "{clip}");
    }
}

#[test]
fn crescent_slash_arc_starts_with_the_strike_and_lasts_the_skill() {
    let mut app = loaded_app();
    cast(&mut app, "crescent_slash");
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Casting);
    let (first, _, _) = clip_range(&app, "vfx_arc_white");
    let seen = effects(&mut app);
    assert_eq!(seen.len(), 1, "one effect: {seen:?}");
    let owner = player(&mut app);
    assert_eq!(seen[0].clip, "vfx_arc_white");
    assert_eq!(seen[0].parent, owner, "a child of the player");
    assert_eq!(seen[0].index, first, "frame 0 together with the hitbox");
    assert!(!seen[0].flip);
    // 0.40 s = 25 frames of 16 ms (the cast frame already counted).
    let frames = 1 + lifetime(&mut app, "vfx_arc_white", 60);
    let expected = (0.40 / DT).ceil() as usize;
    assert!(
        frames.abs_diff(expected) <= 1,
        "{frames} frames, expected ~{expected}"
    );
    run(&mut app, 2);
    assert!(effects(&mut app).is_empty(), "nothing stays on screen");
}

#[test]
fn rising_edge_and_mending_light_have_their_own_effect_and_length() {
    let mut app = loaded_app();
    for (skill, clip, seconds) in [
        ("rising_edge", "vfx_arc_gold", 0.45),
        ("mending_light", "vfx_light_motes", 0.60),
    ] {
        cast(&mut app, skill);
        run(&mut app, 1);
        let seen = effects(&mut app);
        assert_eq!(seen.len(), 1, "{skill}: {seen:?}");
        assert_eq!(seen[0].clip, clip);
        let frames = 1 + lifetime(&mut app, clip, 80);
        // Frames of 16 ms that cover the action (0.45 s -> 29, 0.60 s -> 38).
        let expected = (seconds / DT).ceil() as usize;
        assert!(
            frames.abs_diff(expected) <= 1,
            "{skill}: {frames} frames, ~{expected}"
        );
        run(&mut app, 120);
    }
}

#[test]
fn ember_bolt_keeps_its_projectile_and_adds_no_effect() {
    let mut app = loaded_app();
    cast(&mut app, "ember_bolt");
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Casting);
    assert!(effects(&mut app).is_empty());
}

#[test]
fn effects_face_the_player_direction() {
    let mut app = loaded_app();
    with_player::<FacingDirection>(&mut app, |f| f.0 = Facing::Left);
    cast(&mut app, "crescent_slash");
    run(&mut app, 1);
    let seen = effects(&mut app);
    assert_eq!(seen.len(), 1);
    assert!(seen[0].flip, "mirrored when facing left");
    let set = app.world().resource::<PlayerSpriteSet>();
    let clip = &set.manifest.clips["vfx_arc_white"];
    let cell = set.atlases[&clip.atlas].cell;
    let (ax, ay) = anchor_for(clip.pivot, cell, true);
    assert_eq!(seen[0].anchor, Vec2::new(ax, ay), "pivot mirrored too");
}

#[test]
fn a_technique_effect_stops_when_the_strike_is_interrupted() {
    let mut app = loaded_app();
    cast(&mut app, "crescent_slash");
    run(&mut app, 4);
    assert_eq!(effects(&mut app).len(), 1);
    with_player::<ActionState>(&mut app, |s| {
        s.hurt(0.35);
    });
    run(&mut app, 2);
    assert!(effects(&mut app).is_empty(), "no arc left after the hit");
    // A spell effect plays on: the heal already happened when it started.
    run(&mut app, 60);
    cast(&mut app, "mending_light");
    run(&mut app, 3);
    with_player::<ActionState>(&mut app, |s| {
        s.hurt(0.35);
    });
    run(&mut app, 2);
    assert_eq!(effects(&mut app).len(), 1);
    run(&mut app, 60);
    assert!(effects(&mut app).is_empty());
}

#[test]
fn a_blocked_hit_sparks_on_the_guard_only_from_the_front() {
    let mut app = loaded_app();
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .press
        .push(KeyCode::KeyA);
    run(&mut app, 10);
    assert_eq!(state(&mut app), PlayerState::Guarding);
    // Facing right: a hit travelling left (direction −1) comes from the front.
    app.world_mut()
        .resource_mut::<PendingHits>()
        .0
        .extend([-1.0, -1.0]);
    run(&mut app, 1);
    let seen = effects(&mut app);
    assert_eq!(seen.len(), 1, "one spark for two hits in a frame: {seen:?}");
    assert_eq!(seen[0].clip, "vfx_guard_spark");
    assert_eq!(state(&mut app), PlayerState::Guarding, "the guard holds");
    run(&mut app, 70); // the spark ends and the invulnerability wears off
    assert!(effects(&mut app).is_empty());
    app.world_mut().resource_mut::<PendingHits>().0.push(1.0); // from behind
    run(&mut app, 1);
    assert!(
        effects(&mut app).is_empty(),
        "a hit in the back is not blocked"
    );
}

#[test]
fn revive_shows_one_light_and_keeps_one_visual() {
    let mut app = loaded_app();
    with_player::<ActionState>(&mut app, |s| s.die());
    run(&mut app, 20);
    assert!(effects(&mut app).is_empty());
    with_player::<ActionState>(&mut app, |s| s.revive());
    run(&mut app, 1);
    let seen = effects(&mut app);
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].clip, "vfx_revive");
    let world = app.world_mut();
    let visuals = world
        .query_filtered::<(), With<PlayerVisual>>()
        .iter(world)
        .count();
    assert_eq!(visuals, 1, "the revive does not duplicate the sprite");
    run(&mut app, 60);
    assert!(effects(&mut app).is_empty());
}

#[test]
fn an_attack_in_the_air_shows_the_air_variant_and_falls_after() {
    use dark_castle_rpg::player::visual::VisualPlayback;
    let clip = |app: &mut App| {
        let world = app.world_mut();
        world
            .query_filtered::<&VisualPlayback, With<PlayerVisual>>()
            .single(world)
            .map(|p| p.clip.clone().unwrap_or_default())
            .unwrap_or_else(|e| panic!("{e}"))
    };
    let mut app = loaded_app();
    let keys = |app: &mut App, press: &[KeyCode], release: &[KeyCode]| {
        let mut k = app.world_mut().resource_mut::<PendingKeys>();
        k.press.extend_from_slice(press);
        k.release.extend_from_slice(release);
    };
    keys(&mut app, &[KeyCode::KeyZ], &[]);
    run(&mut app, 6);
    keys(&mut app, &[KeyCode::KeyX], &[]);
    run(&mut app, 1);
    keys(&mut app, &[], &[KeyCode::KeyX]);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    assert_eq!(clip(&mut app), "attack_basic_air");
    let mut after = Vec::new();
    for _ in 0..40 {
        run(&mut app, 1);
        after.push((state(&mut app), clip(&mut app)));
    }
    let falling = after
        .iter()
        .find(|(s, _)| *s != PlayerState::Attacking)
        .unwrap_or_else(|| panic!("the attack never ended: {after:?}"));
    assert!(
        matches!(falling.0, PlayerState::Falling | PlayerState::Jumping),
        "{falling:?}"
    );
    assert_ne!(
        falling.1, "attack_basic_air",
        "the air clip ends with the attack"
    );
}

/// Position, velocity-driven state, HP/MP and hitboxes, frame by frame.
fn gameplay_trace(app: &mut App, script: &[(usize, Option<KeyCode>, Option<&str>)]) -> Vec<String> {
    let mut out = Vec::new();
    for frame in 0..260 {
        for (at, key, skill) in script {
            if *at != frame {
                continue;
            }
            if let Some(key) = key {
                app.world_mut()
                    .resource_mut::<PendingKeys>()
                    .press
                    .push(*key);
            }
            if let Some(skill) = skill {
                cast(app, skill);
            }
        }
        // Keys are released two frames later through the same queue.
        for (at, key, _) in script {
            if *at + 2 == frame
                && let Some(key) = key
            {
                app.world_mut()
                    .resource_mut::<PendingKeys>()
                    .release
                    .push(*key);
            }
        }
        run(app, 1);
        let world = app.world_mut();
        let (t, s, v) = world
            .query_filtered::<(&Transform, &ActionState, &Vitals), With<Player>>()
            .single(world)
            .map(|(t, s, v)| (t.translation, s.state(), v.clone()))
            .unwrap_or_else(|e| panic!("player missing: {e}"));
        let hitboxes = world.query::<&Hitbox>().iter(world).count();
        out.push(format!(
            "{frame} {:08x} {:08x} {s:?} hp={:?} mp={:?} hitboxes={hitboxes}",
            t.x.to_bits(),
            t.y.to_bits(),
            v.hp,
            v.mp
        ));
    }
    out
}

#[test]
fn gameplay_is_identical_with_and_without_effects() {
    let script: &[(usize, Option<KeyCode>, Option<&str>)] = &[
        (5, None, Some("crescent_slash")),
        (50, Some(KeyCode::KeyZ), None),
        (56, Some(KeyCode::KeyX), None),
        (100, None, Some("rising_edge")),
        (150, None, Some("mending_light")),
        (200, None, Some("ember_bolt")),
    ];
    let mut plain = app(false);
    plain.update();
    run(&mut plain, 30);
    grant_all_skills(&mut plain);
    let mut with_vfx = loaded_app();
    // HP below the maximum, so the heal is visible in the trace (same on both).
    for app in [&mut plain, &mut with_vfx] {
        with_player::<Vitals>(app, |v| {
            v.hp.deplete(40);
        });
    }
    let a = gameplay_trace(&mut plain, script);
    let b = gameplay_trace(&mut with_vfx, script);
    assert!(
        b.iter().any(|l| l.contains("Casting")),
        "the script performs skills"
    );
    assert_eq!(
        a, b,
        "effects must not change damage, heal, motion or hitboxes"
    );
}

#[test]
fn a_technique_effect_ends_with_its_action() {
    let mut app = loaded_app();
    for skill in ["crescent_slash", "rising_edge"] {
        cast(&mut app, skill);
        let (mut casting, mut shown) = (0, 0);
        for _ in 0..60 {
            run(&mut app, 1);
            let is_casting = state(&mut app) == PlayerState::Casting;
            let has_effect = !effects(&mut app).is_empty();
            assert!(
                !has_effect || is_casting,
                "{skill}: effect after the action"
            );
            casting += usize::from(is_casting);
            shown += usize::from(has_effect);
        }
        assert!(casting > 0, "{skill} did not start");
        assert_eq!(shown, casting, "{skill}: effect and action last the same");
        run(&mut app, 90);
    }
}
