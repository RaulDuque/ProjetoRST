//! Status screen (Tab): attributes, derived stats, resistances, skills and
//! attribute point allocation. The simulation is paused while it is open.

use bevy::prelude::*;

use super::theme::{
    FONT_LARGE, FONT_NORMAL, FONT_SMALL, PANEL_BG, PANEL_BORDER, TEXT_DIM, TEXT_GOLD, TEXT_MAIN,
    text,
};
use crate::app::states::GameState;
use crate::core::events::Notification;
use crate::core::resources::GameData;
use crate::input::InputBindings;
use crate::magic::elements::{Element, StatusEffect};
use crate::player::attributes::PrimaryAttribute;
use crate::player::components::{CharacterSheet, Player, SkillBook, Vitals};

#[derive(Component, Debug, Clone, Copy)]
pub struct StatusColumn(usize);

const ALLOCATION_KEYS: [(KeyCode, PrimaryAttribute); 4] = [
    (KeyCode::Digit1, PrimaryAttribute::Strength),
    (KeyCode::Digit2, PrimaryAttribute::Constitution),
    (KeyCode::Digit3, PrimaryAttribute::Intelligence),
    (KeyCode::Digit4, PrimaryAttribute::Luck),
];

pub fn toggle_status_screen(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
) {
    if !keys.just_pressed(bindings.status) {
        return;
    }
    next.set(match state.get() {
        GameState::Playing => GameState::StatusScreen,
        GameState::StatusScreen => GameState::Playing,
        // V2.0: Tab does nothing on the inventory screen (I closes it).
        GameState::Inventory | GameState::Dialogue | GameState::Shop | GameState::FastTravel => {
            return;
        }
    });
}

pub fn spawn_status_screen(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::StatusScreen),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        padding: UiRect::all(px(20)),
                        border: UiRect::all(px(2)),
                        ..default()
                    },
                    BackgroundColor(PANEL_BG),
                    BorderColor::all(PANEL_BORDER),
                ))
                .with_children(|panel| {
                    panel.spawn(text("STATUS", FONT_LARGE, TEXT_GOLD));
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: px(36),
                            ..default()
                        })
                        .with_children(|columns| {
                            for index in 0..3 {
                                columns
                                    .spawn((StatusColumn(index), text("", FONT_NORMAL, TEXT_MAIN)));
                            }
                        });
                    panel.spawn(text(
                        "1-4: spend a point on STR / CON / INT / LCK    Tab: close",
                        FONT_SMALL,
                        TEXT_DIM,
                    ));
                });
        });
}

pub fn allocate_points(
    keys: Res<ButtonInput<KeyCode>>,
    data: Res<GameData>,
    mut sheet: Single<&mut CharacterSheet, With<Player>>,
    mut notifications: MessageWriter<Notification>,
) {
    for (key, attribute) in ALLOCATION_KEYS {
        if keys.just_pressed(key)
            && sheet
                .allocate_point(attribute, &data.character.formulas)
                .is_err()
        {
            notifications.write(Notification("No attribute points left".into()));
        }
    }
}

fn percent_text(value: f32) -> String {
    format!("{:+.0}%", value * 100.0)
}

pub fn update_status_screen(
    data: Res<GameData>,
    player: Single<(&CharacterSheet, &Vitals, &SkillBook), With<Player>>,
    mut columns: Query<(&StatusColumn, &mut Text)>,
) {
    let (sheet, vitals, book) = player.into_inner();
    let d = sheet.derived();
    let p = sheet.progression();
    let base = sheet.base();

    let mut attributes = format!(
        "Level {}   EXP {}\nPoints to spend: {}\n\n",
        p.level(),
        p.total_exp(),
        p.unspent_points()
    );
    for attribute in PrimaryAttribute::ALL {
        attributes += &format!(
            "{}  {:>3}  ({:+})\n",
            attribute.short_name(),
            d.effective.get(attribute),
            d.effective.get(attribute) - base.get(attribute)
        );
    }
    attributes += &format!(
        "\nHP  {}/{}\nMP  {}/{}\nHearts {}/{}",
        vitals.hp.current(),
        vitals.hp.max(),
        vitals.mp.current(),
        vitals.mp.max(),
        vitals.hearts.current(),
        vitals.hearts.max()
    );

    let mut derived = format!(
        "ATK   {}\nDEF   {}\nMATK  {}\nMDEF  {}\nHIT   {}\nCRIT  {:.1}%  x{:.2}\nDODGE {:.1}%\n\
         ATK SPD x{:.2}\nMOVE  x{:.2}\nHP regen {:.2}/s\nMP regen {:.2}/s\n\nResistances\n",
        d.attack,
        d.defense,
        d.magic_attack,
        d.magic_defense,
        d.hit,
        d.critical_chance * 100.0,
        d.critical_damage,
        d.dodge * 100.0,
        d.attack_speed,
        d.movement_speed,
        d.hp_regen,
        d.mp_regen
    );
    for element in Element::ALL.into_iter().filter(|e| *e != Element::Physical) {
        derived += &format!(
            "{:<10}{}\n",
            element.display_name(),
            percent_text(d.resistances.get(element))
        );
    }
    for status in StatusEffect::ALL {
        let value = d.status_resistances.get(status);
        if value != 0.0 {
            derived += &format!("{:<10}{}\n", status.display_name(), percent_text(value));
        }
    }

    let mut skills = String::from("Skills & spells\n\n");
    for def in data.skills.iter() {
        let Some(progress) = book.learned.progress(&def.id) else {
            continue;
        };
        let command = match (&def.command, progress.command_revealed) {
            (Some(command), true) => command.ascii(),
            (Some(_), false) => "???".to_owned(),
            (None, _) => "passive".to_owned(),
        };
        skills += &format!(
            "{}  [{}]\n  MP {}  Hearts {}  CD {:.1}s\n",
            def.name, command, def.mp_cost, def.hearts_cost, def.cooldown
        );
    }
    let locked: Vec<String> = data
        .skills
        .iter()
        .filter(|d| !book.learned.is_learned(&d.id))
        .map(|d| {
            if d.requirements.min_level > 0 {
                format!("{} (level {})", d.name, d.requirements.min_level)
            } else {
                d.name.clone()
            }
        })
        .collect();
    if !locked.is_empty() {
        skills += &format!("\nNot learned:\n  {}", locked.join("\n  "));
    }

    for (column, mut value) in &mut columns {
        value.0 = match column.0 {
            0 => attributes.clone(),
            1 => derived.clone(),
            _ => skills.clone(),
        };
    }
}
