//! Bevy side of NPCs and conversations (V2.1).
//!
//! | System | Set | Does |
//! |---|---|---|
//! | [`spawn_room_npcs`] | Rooms/React | spawns the NPCs of the current room only (room entities) |
//! | [`talk_to_npcs`] | Resolution (before objects) | `D` near an NPC, when no object is usable, opens a conversation |
//! | [`show_talk_prompts`] | Presentation | the "D Talk" mark over the NPC in reach |
//! | [`run_conversation`] | Presentation, `GameState::Dialogue` | lines, choices, shop, end |
//! | [`close_on_room_change`] | Rooms/React | a room change (load) closes dialogue and shop |
//!
//! Conversations and shops are paused game states: no combat, movement or
//! room logic runs while they are open, and closing them returns to
//! `GameState::Playing`. Nothing is read from disk after startup.

use std::collections::BTreeSet;

use bevy::prelude::*;

use super::definitions::DialogueFacts;
use super::dialogue::{Conversation, Step};
use crate::app::states::GameState;
use crate::core::constants::Z_ENEMIES;
use crate::core::events::{Notification, RoomChanged};
use crate::core::resources::GameData;
use crate::input::{InputBindings, PlayerInput};
use crate::player::components::{ActionState, Belongings, Player};
use crate::player::state::PlayerAction;
use crate::world::exploration::Discovery;
use crate::world::exploration_systems::{Exploration, WorldGraph};
use crate::world::interactions::usable_object;
use crate::world::maps::{CurrentRoom, RoomEntity};

/// Horizontal distance (body centres) within which `D` talks to an NPC.
pub const TALK_RANGE: f32 = 56.0;
/// Vertical distance allowed (same floor).
const TALK_HEIGHT: f32 = 40.0;
const PROMPT_COLOR: Color = Color::srgb(0.95, 0.85, 0.5);

/// An NPC in the current room.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct NpcMarker {
    pub id: String,
}

/// The "D Talk" mark over an NPC.
#[derive(Component, Debug, Clone, Copy)]
pub struct TalkPrompt;

/// The conversation being shown (only while `GameState::Dialogue`).
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct ActiveConversation(pub Conversation);

/// The shop being shown (only while `GameState::Shop`).
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct ActiveShop {
    pub shop: String,
    pub npc: String,
}

/// What dialogue conditions and shop entries can check right now.
pub fn dialogue_facts(
    exploration: Option<&Exploration>,
    world: Option<&WorldGraph>,
    belongings: Option<&Belongings>,
) -> DialogueFacts {
    let mut facts = DialogueFacts::default();
    if let Some(exploration) = exploration {
        facts.capabilities = exploration.capabilities();
        for (room, discovery) in &exploration.rooms {
            if *discovery == Discovery::Visited {
                facts.visited_rooms.insert(room.clone());
                if let Some(r) = world.and_then(|w| w.0.room(room)) {
                    facts.visited_regions.insert(r.region.clone());
                }
            }
        }
    }
    if let Some(b) = belongings {
        let items: BTreeSet<String> = b
            .inventory
            .stacks()
            .iter()
            .map(|s| s.id.clone())
            .chain(b.equipment.iter().map(|(_, id)| id.to_owned()))
            .collect();
        facts.items = items;
    }
    facts
}

/// Spawns the NPCs of the current room (as room entities, removed with it).
pub fn spawn_room_npcs(
    mut commands: Commands,
    data: Res<GameData>,
    current: Option<Res<CurrentRoom>>,
    mut prepared: Local<Option<String>>,
) {
    let Some(current) = current else {
        return;
    };
    if prepared.as_deref() == Some(current.0.as_str()) && !current.is_changed() {
        return;
    }
    *prepared = Some(current.0.clone());
    for npc in data.npcs.in_room(&current.0) {
        let (r, g, b) = npc.color;
        let size = Vec2::new(npc.size.0, npc.size.1);
        commands
            .spawn((
                RoomEntity,
                NpcMarker { id: npc.id.clone() },
                Sprite::from_color(Color::srgb(r, g, b), size),
                Transform::from_xyz(npc.position.0, npc.position.1, Z_ENEMIES - 0.5),
            ))
            .with_child((
                TalkPrompt,
                Text2d::new(format!("{}\nD  Talk", npc.name)),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(PROMPT_COLOR),
                Transform::from_xyz(0.0, size.y / 2.0 + 18.0, 0.5),
                Visibility::Hidden,
            ));
    }
}

/// The NPC `D` would talk to: the nearest one in reach (ties: smallest id).
pub fn npc_in_reach<'a>(
    player: Vec2,
    npcs: impl Iterator<Item = (&'a NpcMarker, &'a Transform)>,
) -> Option<&'a NpcMarker> {
    npcs.filter_map(|(npc, t)| {
        let d = t.translation.truncate() - player;
        (d.x.abs() <= TALK_RANGE && d.y.abs() <= TALK_HEIGHT).then_some((d.x.abs(), npc))
    })
    .min_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.id.cmp(&b.1.id)))
    .map(|(_, npc)| npc)
}

/// `D` starts a conversation with the NPC in reach, unless an object (chest,
/// lever, switch) is usable there: objects always win, so one press never
/// does both. Not while fallen or in an action (same rule as objects).
pub fn talk_to_npcs(
    mut commands: Commands,
    input: Res<PlayerInput>,
    data: Res<GameData>,
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    current: Option<Res<CurrentRoom>>,
    player: Query<(&Transform, &ActionState, &Belongings), With<Player>>,
    npcs: Query<(&NpcMarker, &Transform)>,
    mut next: ResMut<NextState<GameState>>,
    mut notifications: MessageWriter<Notification>,
) {
    if !input.interact_pressed {
        return;
    }
    let Ok((transform, action, belongings)) = player.single() else {
        return;
    };
    if !action.state().allows(PlayerAction::Interact) {
        return;
    }
    if let (Some(world), Some(exploration), Some(current)) = (&world, &exploration, &current)
        && let Some(room) = world.0.room(&current.0)
        && usable_object(room, exploration, transform).is_some()
    {
        return;
    }
    let Some(npc) = npc_in_reach(transform.translation.truncate(), npcs.iter()) else {
        return;
    };
    let facts = dialogue_facts(exploration.as_deref(), world.as_deref(), Some(belongings));
    match Conversation::start(&npc.id, &data.npcs, &facts) {
        Some(conversation) => {
            commands.insert_resource(ActiveConversation(conversation));
            next.set(GameState::Dialogue);
        }
        None => {
            notifications.write(Notification("They have nothing to say.".into()));
        }
    }
}

/// Shows the prompt of the NPC in reach only (and never while talking).
pub fn show_talk_prompts(
    state: Res<State<GameState>>,
    player: Query<&Transform, With<Player>>,
    npcs: Query<(&NpcMarker, &Transform, &Children)>,
    mut prompts: Query<&mut Visibility, With<TalkPrompt>>,
) {
    let target = match (player.single(), state.get()) {
        (Ok(p), GameState::Playing) => npc_in_reach(
            p.translation.truncate(),
            npcs.iter().map(|(n, t, _)| (n, t)),
        )
        .map(|n| n.id.clone()),
        _ => None,
    };
    for (npc, _, children) in &npcs {
        let shown = target.as_deref() == Some(npc.id.as_str());
        for child in children.iter() {
            if let Ok(mut visibility) = prompts.get_mut(child) {
                visibility.set_if_neq(if shown {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                });
            }
        }
    }
}

/// `↑`/`↓` select a choice; `Enter` or `D` advance or confirm; `Esc` ends.
pub fn run_conversation(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    data: Res<GameData>,
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    player: Query<&Belongings, With<Player>>,
    conversation: Option<ResMut<ActiveConversation>>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Some(mut conversation) = conversation else {
        next.set(GameState::Playing);
        return;
    };
    let facts = dialogue_facts(
        exploration.as_deref(),
        world.as_deref(),
        player.single().ok(),
    );
    if keys.just_pressed(KeyCode::Escape) {
        commands.remove_resource::<ActiveConversation>();
        next.set(GameState::Playing);
        return;
    }
    if keys.just_pressed(bindings.down) {
        conversation.0.move_cursor(1, &data.npcs, &facts);
    }
    if keys.just_pressed(bindings.up) {
        conversation.0.move_cursor(-1, &data.npcs, &facts);
    }
    if !(keys.just_pressed(bindings.confirm) || keys.just_pressed(bindings.interact)) {
        return;
    }
    match conversation.0.confirm(&data.npcs, &facts) {
        Step::Continue => {}
        Step::End => {
            commands.remove_resource::<ActiveConversation>();
            next.set(GameState::Playing);
        }
        Step::OpenShop => {
            let npc = conversation.0.npc.clone();
            commands.remove_resource::<ActiveConversation>();
            match data.npcs.npc(&npc).and_then(|n| n.shop.clone()) {
                Some(shop) => {
                    commands.insert_resource(ActiveShop { shop, npc });
                    next.set(GameState::Shop);
                }
                None => next.set(GameState::Playing),
            }
        }
    }
}

/// A room change (a load from a paused screen) closes any conversation or
/// shop, so the player never stays stuck in one.
pub fn close_on_room_change(
    mut commands: Commands,
    mut changes: MessageReader<RoomChanged>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
) {
    if changes.read().count() == 0 {
        return;
    }
    commands.remove_resource::<ActiveConversation>();
    commands.remove_resource::<ActiveShop>();
    if matches!(state.get(), GameState::Dialogue | GameState::Shop) {
        next.set(GameState::Playing);
    }
}
