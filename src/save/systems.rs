//! Quick save (F5) and quick load (F9) of the single prototype slot.
//!
//! The load (V1.7) never moves the player itself: it validates the save,
//! resolves where it resumes (`world::restore`), replaces the persistent
//! state (player, world flags, exploration) and asks the room lifecycle for
//! the saved room (`RoomRequest` with `Arrival::Position`), which loads it
//! and places the player in the same frame. Nothing is changed when the
//! save cannot be read or resolved.

use bevy::prelude::*;

use super::save_manager::SaveStorage;
use super::serialization::{PlayerSave, SaveData};
use crate::app::states::GameState;
use crate::combat::hitboxes::Invulnerable;
use crate::core::constants::{DEFAULT_SAVE_SLOT, PLAYER_INVULNERABILITY, SAVE_DIR};
use crate::core::events::{
    Arrival, FastTravelRequest, Notification, RoomRequest, SavePointActivated,
};
use crate::core::resources::{GameData, project_root};
use crate::input::ActionEdges;
use crate::input::action::Action;
use crate::player::attributes::{RegenAccumulator, ResourcePool};
use crate::player::components::{
    ActionState, Belongings, CharacterSheet, CommandInput, Player, SkillBook, Vitals,
};
use crate::player::state::PlayerState;
use crate::ui::fast_travel_screen::FastTravelSession;
use crate::world::WorldState;
use crate::world::exploration::ExplorationState;
use crate::world::exploration_systems::{Exploration, WorldGraph};
use crate::world::fast_travel::{self, FastTravelError};
use crate::world::maps::CurrentRoom;
use crate::world::physics::{Body, Phasing, RoomGeometry, Velocity};
use crate::world::progress::WorldProgress;
use crate::world::restore::resolve_placement;
use crate::world::transitions::TransitionLock;

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
    &'a Body,
    Option<&'a Phasing>,
    Option<&'a crate::economy::systems::PlayerPurse>,
);

fn snapshot_save(
    room: &CurrentRoom,
    world: &WorldProgress,
    exploration: Option<&Exploration>,
    play_time: f64,
    transform: &Transform,
    sheet: &CharacterSheet,
    vitals: &Vitals,
    book: &SkillBook,
    belongings: &Belongings,
    purse: Option<&crate::economy::systems::PlayerPurse>,
    stock: Option<&crate::economy::systems::ShopStockState>,
) -> SaveData {
    SaveData {
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
        world: world.clone(),
        exploration: exploration.map(|e| e.record()).unwrap_or_default(),
        play_time,
        // V2.1.1 (save v3): coins and shop stock, saved with the bag in the
        // same file, so a purchase and its payment are always saved together.
        economy: Some(crate::economy::shop::EconomyRecord {
            coins: purse.map_or(0, |p| p.coins()),
            stock: stock.map(|s| s.0.clone()).unwrap_or_default(),
        }),
    }
}

fn write_slot(
    slots: &SaveSlots,
    data: &SaveData,
    notifications: &mut MessageWriter<Notification>,
    ok_message: &str,
) {
    match slots.0.save(DEFAULT_SAVE_SLOT, data) {
        Ok(path) => {
            info!(path = %path.display(), "saved");
            notifications.write(Notification(ok_message.into()));
        }
        Err(error) => {
            error!(%error, "save failed");
            notifications.write(Notification(format!("Save failed: {error}")));
        }
    }
}

pub fn quick_save(
    edges: Res<ActionEdges>,
    slots: Res<SaveSlots>,
    room: Res<CurrentRoom>,
    world: Res<WorldState>,
    exploration: Option<Res<Exploration>>,
    play_time: Res<PlayTime>,
    geometry: Res<RoomGeometry>,
    player: Single<PlayerSaveQuery, With<Player>>,
    stock: Option<Res<crate::economy::systems::ShopStockState>>,
    mut notifications: MessageWriter<Notification>,
) {
    if !edges.just_pressed(Action::QuickSave) {
        return;
    }
    let (transform, sheet, vitals, book, belongings, state, body, phasing, purse) =
        player.into_inner();
    if state.state() == PlayerState::Dead {
        notifications.write(Notification("Cannot save while fallen".into()));
        return;
    }
    // V1.9: a save loads in human form, so it is refused where a human
    // would not fit (inside a veil). Nothing is lost: the player saves a
    // step further, and the file on disk is untouched.
    if phasing.is_some() && geometry.blocks(&body.aabb_at(transform.translation.truncate())) {
        notifications.write(Notification("Cannot save inside a veil".into()));
        return;
    }
    let data = snapshot_save(
        &room,
        &world.0,
        exploration.as_deref(),
        play_time.0,
        transform,
        sheet,
        vitals,
        book,
        belongings,
        purse,
        stock.as_deref(),
    );
    write_slot(&slots, &data, &mut notifications, "Game saved");
}

type PlayerRestQuery<'a> = (
    &'a Transform,
    &'a CharacterSheet,
    &'a mut Vitals,
    &'a SkillBook,
    &'a Belongings,
    &'a ActionState,
    &'a Body,
    Option<&'a Phasing>,
    Option<&'a crate::economy::systems::PlayerPurse>,
);

/// D17: rest at a world save point — heal HP/MP, discover the altar, write
/// the slot. F5 quick-save remains available for development.
pub fn rest_at_save_point(
    mut activated: MessageReader<SavePointActivated>,
    slots: Res<SaveSlots>,
    room: Res<CurrentRoom>,
    mut world: ResMut<WorldState>,
    exploration: Option<Res<Exploration>>,
    play_time: Res<PlayTime>,
    geometry: Res<RoomGeometry>,
    mut player: Query<PlayerRestQuery, With<Player>>,
    stock: Option<Res<crate::economy::systems::ShopStockState>>,
    mut notifications: MessageWriter<Notification>,
) {
    let Ok((transform, sheet, mut vitals, book, belongings, state, body, phasing, purse)) =
        player.single_mut()
    else {
        return;
    };
    for event in activated.read() {
        if state.state() == PlayerState::Dead {
            notifications.write(Notification("Cannot rest while fallen".into()));
            continue;
        }
        if phasing.is_some() && geometry.blocks(&body.aabb_at(transform.translation.truncate())) {
            notifications.write(Notification("Cannot rest inside a veil".into()));
            continue;
        }
        vitals.hp.refill();
        vitals.mp.refill();
        let first = world.0.discover_save_point(&event.id);
        world.0.set_last_save_point(&event.id);
        let data = snapshot_save(
            &room,
            &world.0,
            exploration.as_deref(),
            play_time.0,
            transform,
            sheet,
            &vitals,
            book,
            belongings,
            purse,
            stock.as_deref(),
        );
        let message = if first {
            "Save point discovered — rested and saved"
        } else {
            "Rested and saved"
        };
        write_slot(&slots, &data, &mut notifications, message);
    }
}

/// After a successful rest, open the fast-travel menu when other altars are known.
pub fn offer_fast_travel(
    mut activated: MessageReader<SavePointActivated>,
    graph: Option<Res<WorldGraph>>,
    world: Res<WorldState>,
    player: Query<&ActionState, With<Player>>,
    lock: Option<Res<TransitionLock>>,
    mut session: ResMut<FastTravelSession>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Some(graph) = graph else {
        activated.clear();
        return;
    };
    let Ok(action) = player.single() else {
        activated.clear();
        return;
    };
    if action.state() == PlayerState::Dead {
        activated.clear();
        return;
    }
    if lock.is_some_and(|l| l.is_locked()) {
        activated.clear();
        return;
    }
    for event in activated.read() {
        let dests = fast_travel::destinations(&graph.0, &world.0, &event.id);
        if dests.is_empty() {
            continue;
        }
        session.origin = event.id.clone();
        next.set(GameState::FastTravel);
    }
}

fn travel_refusal_text(error: &FastTravelError) -> String {
    match error {
        FastTravelError::UnknownOrigin(_) | FastTravelError::UnknownDestination(_) => {
            "Cannot travel there".into()
        }
        FastTravelError::DestinationNotDiscovered(_) => "Altar not yet discovered".into(),
        FastTravelError::SameAltar => "Already at this altar".into(),
        FastTravelError::Dead => "Cannot travel while fallen".into(),
        FastTravelError::Busy => "Cannot travel right now".into(),
        FastTravelError::TransitionLocked => "Cannot travel during a passage".into(),
        FastTravelError::NoDestinations => "No other discovered altars".into(),
    }
}

/// Applies a confirmed fast-travel choice via the room lifecycle (D19).
/// Does not heal, does not write a save, does not change `last_save_point`.
pub fn apply_fast_travel_requests(
    mut requests: MessageReader<FastTravelRequest>,
    graph: Option<Res<WorldGraph>>,
    world: Res<WorldState>,
    session: Res<FastTravelSession>,
    lock: Option<Res<TransitionLock>>,
    player: Query<&ActionState, With<Player>>,
    mut room_requests: MessageWriter<RoomRequest>,
    mut next: ResMut<NextState<GameState>>,
    mut notifications: MessageWriter<Notification>,
) {
    if requests.is_empty() {
        return;
    }
    let Some(graph) = graph else {
        requests.clear();
        return;
    };
    let Ok(action) = player.single() else {
        requests.clear();
        return;
    };
    let locked = lock.is_some_and(|l| l.is_locked());
    for request in requests.read() {
        match fast_travel::plan_travel(
            &graph.0,
            &world.0,
            &session.origin,
            &request.destination,
            action.state(),
            locked,
        ) {
            Ok(dest) => {
                notifications.write(Notification(format!("Traveling to {}", dest.room_name)));
                next.set(GameState::Playing);
                room_requests.write(RoomRequest {
                    room: dest.room_id,
                    arrival: Arrival::Position(Vec2::new(dest.arrival.0, dest.arrival.1)),
                });
            }
            Err(error) => {
                notifications.write(Notification(travel_refusal_text(&error)));
            }
        }
    }
}

type PlayerLoadQuery<'a> = (
    &'a mut CharacterSheet,
    &'a mut Vitals,
    &'a mut SkillBook,
    &'a mut Belongings,
    &'a mut ActionState,
    &'a mut Velocity,
    &'a mut CommandInput,
    &'a mut Invulnerable,
    Option<&'a mut crate::economy::systems::PlayerPurse>,
);

pub fn quick_load(
    edges: Res<ActionEdges>,
    slots: Res<SaveSlots>,
    data: Res<GameData>,
    graph: Option<Res<WorldGraph>>,
    exploration: Option<ResMut<Exploration>>,
    mut world: ResMut<WorldState>,
    mut play_time: ResMut<PlayTime>,
    player: Single<PlayerLoadQuery, With<Player>>,
    stock: Option<ResMut<crate::economy::systems::ShopStockState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut requests: MessageWriter<RoomRequest>,
    mut notifications: MessageWriter<Notification>,
) {
    if !edges.just_pressed(Action::QuickLoad) {
        return;
    }
    let save = match slots.0.load_versioned(DEFAULT_SAVE_SLOT) {
        Ok(save) => save,
        Err(error) => {
            warn!(%error, "load failed");
            notifications.write(Notification(format!("Load failed: {error}")));
            return;
        }
    };
    let migrated = save.migrated();
    let source_version = save.source_version;
    let save = save.data;
    // Resolve the room first: an unusable save changes nothing.
    let placement = match resolve_placement(
        graph.as_ref().map(|g| &g.0),
        &data.rooms,
        &save.player.room,
        save.player.position,
    ) {
        Ok(placement) => placement,
        Err(error) => {
            warn!(%error, "load failed");
            notifications.write(Notification(format!("Load failed: {error}")));
            return;
        }
    };

    let (
        mut sheet,
        mut vitals,
        mut book,
        mut belongings,
        mut state,
        mut velocity,
        mut buffer,
        mut invulnerable,
        purse,
    ) = player.into_inner();
    let p = save.player;
    let formulas = &data.character.formulas;
    let bonus = p.equipment.total_modifiers(&data.items);
    *sheet = CharacterSheet::new(p.attributes, p.progression, bonus, formulas);
    vitals.sync_max(sheet.derived());
    vitals.hp = ResourcePool::new(p.hp, sheet.derived().max_hp);
    vitals.mp = ResourcePool::new(p.mp, sheet.derived().max_mp);
    vitals.hearts = ResourcePool::new(p.hearts, sheet.derived().max_hearts);
    vitals.hp_regen = RegenAccumulator::default();
    vitals.mp_regen = RegenAccumulator::default();
    book.learned = p.learned_skills;
    book.cooldowns = Default::default();
    belongings.inventory = p.inventory;
    belongings.equipment = p.equipment;
    // V2.1.1: the economy of the save; a v1/v2 save (no economy) gets the
    // starting coins and full stock, the same at every load of that file.
    let economy = save
        .economy
        .unwrap_or_else(|| crate::economy::shop::EconomyRecord {
            coins: data.character.start.coins,
            stock: Default::default(),
        });
    if let Some(mut purse) = purse {
        purse.0 = crate::economy::shop::Purse::new(economy.coins);
    }
    if let Some(mut stock) = stock {
        stock.0 = economy.stock;
    }
    state.revive();
    velocity.0 = Vec2::ZERO;
    buffer.clear();
    invulnerable.remaining = PLAYER_INVULNERABILITY;
    world.0 = save.world;
    // The saved exploration replaces the current one entirely.
    if let Some(mut exploration) = exploration {
        let restored = ExplorationState::from_record(save.exploration);
        if let Some(graph) = &graph {
            let unknown = restored.unknown_ids(&graph.0);
            if !unknown.is_empty() {
                warn!(
                    ?unknown,
                    "save names content this world does not have; ignored"
                );
            }
        }
        exploration.0 = restored;
    }
    play_time.0 = save.play_time;
    requests.write(RoomRequest {
        room: placement.room.clone(),
        arrival: Arrival::Position(Vec2::new(placement.position.0, placement.position.1)),
    });
    next_state.set(GameState::Playing);
    for note in &placement.notes {
        warn!(?note, "load placement adjusted");
        notifications.write(Notification(note.message()));
    }
    if migrated {
        info!(
            source_version,
            "save migrated in memory; the file is unchanged"
        );
        notifications.write(Notification(format!(
            "Loaded a v{source_version} save (rewritten only when you save)"
        )));
    }
    notifications.write(Notification("Game loaded".into()));
}
