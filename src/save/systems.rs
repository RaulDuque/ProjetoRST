//! Quick save (F5) and quick load (F9) of the single prototype slot.

use bevy::prelude::*;

use super::save_manager::SaveStorage;
use super::serialization::{PlayerSave, SaveData};
use crate::app::states::GameState;
use crate::combat::hitboxes::Invulnerable;
use crate::core::constants::{DEFAULT_SAVE_SLOT, PLAYER_INVULNERABILITY, SAVE_DIR};
use crate::core::events::Notification;
use crate::core::resources::{GameData, project_root};
use crate::input::InputBindings;
use crate::player::attributes::RegenAccumulator;
use crate::player::components::{
    ActionState, Belongings, CharacterSheet, CommandInput, Player, SkillBook, Vitals,
};
use crate::player::state::PlayerState;
use crate::world::WorldState;
use crate::world::camera::{MainCamera, snap_to};
use crate::world::maps::CurrentRoom;
use crate::world::physics::{RoomGeometry, Velocity};

#[derive(Resource, Debug, Clone)]
pub struct SaveSlots(pub SaveStorage);

impl Default for SaveSlots {
    fn default() -> Self {
        Self(SaveStorage::new(project_root().join(SAVE_DIR)))
    }
}

/// Total play time, persisted in saves.
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct PlayTime(pub f64);

pub fn track_play_time(time: Res<Time>, mut play_time: ResMut<PlayTime>) {
    play_time.0 += time.delta_secs_f64();
}

type PlayerSaveQuery<'a> = (
    &'a Transform,
    &'a CharacterSheet,
    &'a Vitals,
    &'a SkillBook,
    &'a Belongings,
    &'a ActionState,
);

pub fn quick_save(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    slots: Res<SaveSlots>,
    room: Res<CurrentRoom>,
    world: Res<WorldState>,
    play_time: Res<PlayTime>,
    player: Single<PlayerSaveQuery, With<Player>>,
    mut notifications: MessageWriter<Notification>,
) {
    if !keys.just_pressed(bindings.quick_save) {
        return;
    }
    let (transform, sheet, vitals, book, belongings, state) = player.into_inner();
    if state.state() == PlayerState::Dead {
        notifications.write(Notification("Cannot save while fallen".into()));
        return;
    }
    let data = SaveData {
        player: PlayerSave {
            room: room.0.clone(),
            position: (transform.translation.x, transform.translation.y),
            attributes: *sheet.base(),
            progression: sheet.progression().clone(),
            hp: vitals.hp.current(),
            mp: vitals.mp.current(),
            hearts: vitals.hearts.current(),
            learned_skills: book.learned.clone(),
            inventory: belongings.inventory.clone(),
            equipment: belongings.equipment.clone(),
        },
        world: world.0.clone(),
        play_time: play_time.0,
    };
    match slots.0.save(DEFAULT_SAVE_SLOT, &data) {
        Ok(path) => {
            info!(path = %path.display(), "saved");
            notifications.write(Notification("Game saved".into()));
        }
        Err(error) => {
            error!(%error, "save failed");
            notifications.write(Notification(format!("Save failed: {error}")));
        }
    }
}

type PlayerLoadQuery<'a> = (
    &'a mut Transform,
    &'a mut CharacterSheet,
    &'a mut Vitals,
    &'a mut SkillBook,
    &'a mut Belongings,
    &'a mut ActionState,
    &'a mut Velocity,
    &'a mut CommandInput,
    &'a mut Invulnerable,
);

pub fn quick_load(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    slots: Res<SaveSlots>,
    data: Res<GameData>,
    geometry: Res<RoomGeometry>,
    mut world: ResMut<WorldState>,
    mut play_time: ResMut<PlayTime>,
    player: Single<PlayerLoadQuery, (With<Player>, Without<MainCamera>)>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut notifications: MessageWriter<Notification>,
) {
    if !keys.just_pressed(bindings.quick_load) {
        return;
    }
    let save = match slots.0.load(DEFAULT_SAVE_SLOT) {
        Ok(save) => save,
        Err(error) => {
            warn!(%error, "load failed");
            notifications.write(Notification(format!("Load failed: {error}")));
            return;
        }
    };
    if !data.rooms.contains_key(&save.player.room) {
        notifications.write(Notification(format!(
            "Load failed: unknown room `{}`",
            save.player.room
        )));
        return;
    }

    let (
        mut transform,
        mut sheet,
        mut vitals,
        mut book,
        mut belongings,
        mut state,
        mut velocity,
        mut buffer,
        mut invulnerable,
    ) = player.into_inner();
    let p = save.player;
    let formulas = &data.character.formulas;
    let bonus = p.equipment.total_modifiers(&data.items);
    *sheet = CharacterSheet::new(p.attributes, p.progression, bonus, formulas);
    vitals.sync_max(sheet.derived());
    vitals.hp = crate::player::attributes::ResourcePool::new(p.hp, sheet.derived().max_hp);
    vitals.mp = crate::player::attributes::ResourcePool::new(p.mp, sheet.derived().max_mp);
    vitals.hearts =
        crate::player::attributes::ResourcePool::new(p.hearts, sheet.derived().max_hearts);
    vitals.hp_regen = RegenAccumulator::default();
    vitals.mp_regen = RegenAccumulator::default();
    book.learned = p.learned_skills;
    book.cooldowns = Default::default();
    belongings.inventory = p.inventory;
    belongings.equipment = p.equipment;
    state.revive();
    velocity.0 = Vec2::ZERO;
    buffer.clear();
    invulnerable.remaining = PLAYER_INVULNERABILITY;
    transform.translation.x = p.position.0;
    transform.translation.y = p.position.1;
    snap_to(
        &mut camera,
        Vec2::new(p.position.0, p.position.1),
        &geometry,
    );
    world.0 = save.world;
    play_time.0 = save.play_time;
    next_state.set(GameState::Playing);
    notifications.write(Notification("Game loaded".into()));
}
