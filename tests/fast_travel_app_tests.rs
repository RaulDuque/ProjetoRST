//! D19: fast travel between discovered save altars (headless).

use bevy::prelude::*;
use dark_castle_rpg::app::states::GameState;
use dark_castle_rpg::core::constants::DEFAULT_SAVE_SLOT;
use dark_castle_rpg::core::events::{Arrival, FastTravelRequest, Notification, RoomRequest};
use dark_castle_rpg::player::components::{Belongings, Player, Vitals};
use dark_castle_rpg::ui::fast_travel_screen::FastTravelSession;
use dark_castle_rpg::world::WorldState;
use dark_castle_rpg::world::maps::RoomEntity;

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

const APPROACH_SAVE: &str = "vg_approach_save";
const CAUSEWAY_SAVE: &str = "vg_causeway_save";
const AT_APPROACH: (f32, f32) = (360.0, 63.0);
const AT_CAUSEWAY: (f32, f32) = (1940.0, 63.0);

#[derive(Resource, Default)]
struct Notes(Vec<String>);

fn record(mut messages: MessageReader<Notification>, mut notes: ResMut<Notes>) {
    notes.0.extend(messages.read().map(|n| n.0.clone()));
}

fn app_named(name: &str) -> App {
    let mut app = game(&storage(name));
    app.init_resource::<Notes>().add_systems(PostUpdate, record);
    app
}

fn discover(app: &mut App, at: (f32, f32)) {
    place(app, at);
    run(app, 2);
    tap(app, KeyCode::KeyD);
    run(app, 4);
}

fn bag_tonic(app: &mut App) -> u32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Belongings, With<Player>>()
        .single(world)
        .map(|b| b.inventory.count("minor_tonic"))
        .unwrap_or_else(|e| panic!("{e}"))
}

fn room_entities(app: &mut App) -> usize {
    count::<With<RoomEntity>>(app)
}

#[test]
fn travels_between_discovered_altars_and_persists() {
    let saves = storage("d19_travel");
    let mut app = game(&saves);
    app.init_resource::<Notes>().add_systems(PostUpdate, record);

    assert_eq!(current(&app), APPROACH);
    let tonics = bag_tonic(&mut app);
    discover(&mut app, AT_APPROACH);
    assert!(
        app.world()
            .resource::<WorldState>()
            .0
            .discovered_save_points
            .contains(APPROACH_SAVE)
    );
    // Only one altar known: stay in Playing (no empty menu).
    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::Playing
    );

    app.world_mut().write_message(RoomRequest {
        room: CAUSEWAY.into(),
        arrival: Arrival::Position(Vec2::new(AT_CAUSEWAY.0, AT_CAUSEWAY.1)),
    });
    run(&mut app, 3);
    assert_eq!(current(&app), CAUSEWAY);
    discover(&mut app, AT_CAUSEWAY);
    assert!(
        app.world()
            .resource::<WorldState>()
            .0
            .discovered_save_points
            .contains(CAUSEWAY_SAVE)
    );
    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::FastTravel
    );
    assert_eq!(
        app.world().resource::<FastTravelSession>().origin,
        CAUSEWAY_SAVE
    );

    // Confirm travel to the approach altar (only other destination).
    tap(&mut app, KeyCode::Enter);
    run(&mut app, 5);
    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::Playing
    );
    assert_eq!(current(&app), APPROACH);
    let xy = player_xy(&mut app);
    assert!((xy.x - AT_APPROACH.0).abs() < 8.0, "arrival x={xy:?}");
    assert_eq!(bag_tonic(&mut app), tonics);
    // Travel must not rewrite last_save_point (still the causeway rest).
    assert_eq!(
        app.world()
            .resource::<WorldState>()
            .0
            .last_save_point
            .as_deref(),
        Some(CAUSEWAY_SAVE)
    );

    // Return trip via message (menu already closed).
    app.world_mut().resource_mut::<FastTravelSession>().origin = APPROACH_SAVE.to_owned();
    // Discoveries persist across F5/F9.
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    {
        let mut q = app
            .world_mut()
            .query_filtered::<&mut Vitals, With<Player>>();
        let mut vitals = q
            .single_mut(app.world_mut())
            .unwrap_or_else(|e| panic!("{e}"));
        vitals.hp.deplete(7);
    }
    let hp_before = player_hp(&mut app);
    app.world_mut().write_message(FastTravelRequest {
        destination: CAUSEWAY_SAVE.into(),
    });
    // Must be in FastTravel or Playing with locomotion — force Playing + Idle.
    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::Playing
    );
    run(&mut app, 5);
    assert_eq!(current(&app), CAUSEWAY);
    assert_eq!(player_hp(&mut app), hp_before);
    assert_eq!(bag_tonic(&mut app), tonics);

    tap(&mut app, KeyCode::F9);
    run(&mut app, 5);
    let loaded = saves
        .load(DEFAULT_SAVE_SLOT)
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(loaded.world.discovered_save_points.contains(APPROACH_SAVE));
    assert!(loaded.world.discovered_save_points.contains(CAUSEWAY_SAVE));
}

#[test]
fn refuses_undiscovered_destination_and_cancel_keeps_room() {
    let mut app = app_named("d19_refuse");
    discover(&mut app, AT_APPROACH);
    app.world_mut().resource_mut::<FastTravelSession>().origin = APPROACH_SAVE.to_owned();
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::FastTravel);
    run(&mut app, 2);
    // No other discoveries: navigate closes the screen.
    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::Playing
    );

    // Discover both, open menu, Esc cancels.
    app.world_mut().write_message(RoomRequest {
        room: CAUSEWAY.into(),
        arrival: Arrival::Position(Vec2::new(AT_CAUSEWAY.0, AT_CAUSEWAY.1)),
    });
    run(&mut app, 3);
    discover(&mut app, AT_CAUSEWAY);
    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::FastTravel
    );
    let before = room_entities(&mut app);
    tap(&mut app, KeyCode::Escape);
    run(&mut app, 2);
    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::Playing
    );
    assert_eq!(current(&app), CAUSEWAY);
    assert_eq!(room_entities(&mut app), before);

    app.world_mut().write_message(FastTravelRequest {
        destination: "no_such_altar".into(),
    });
    run(&mut app, 2);
    assert_eq!(current(&app), CAUSEWAY);
}
