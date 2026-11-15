//! Held equipment (V2.2): which weapon and shield the character shows, and
//! where, resolved from the equipped items.
//!
//! Pure module: no Bevy types. It complements [`super::appearance`] (trail,
//! cape, body tint) without changing it: the body frames of V2.2 have no
//! weapon drawn in, so the weapon in the hand is a layer chosen here from
//! the item category, and the shield a layer on the left arm.
//!
//! Two hands (V2.1 rule, visual side): a two-handed weapon (`two_handed` in
//! `weapon_classes.ron`) occupies both hands. The loadout rules already
//! refuse a shield with it; if an older or edited save still holds both, the
//! shield is not hidden (the inconsistency stays visible) but slung on the
//! back, never in a hand that is holding the weapon.
//! Details: docs/VISUAL_EQUIPMENT_V2_2.md §6-7.

use super::appearance::{ItemVisual, Rgb};
use super::equipment::{Equipment, EquipmentSlot};
use super::items::{ItemCatalog, ItemCategory};

/// Accent of a weapon without a `Trail` visual (the starting sword).
pub const DEFAULT_ACCENT: Rgb = Rgb(0.78, 0.62, 0.32);
/// Face colour of a shield without a `Tint` visual.
pub const DEFAULT_SHIELD: Rgb = Rgb(0.55, 0.42, 0.28);

/// The silhouette family of a held weapon (one per weapon category).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WeaponForm {
    Sword,
    Dagger,
    Axe,
    Spear,
    Heavy,
}

impl WeaponForm {
    pub const ALL: [WeaponForm; 5] = [
        WeaponForm::Sword,
        WeaponForm::Dagger,
        WeaponForm::Axe,
        WeaponForm::Spear,
        WeaponForm::Heavy,
    ];

    /// The form of a weapon category (`None` for anything else).
    pub fn of(category: ItemCategory) -> Option<Self> {
        match category {
            ItemCategory::Sword => Some(WeaponForm::Sword),
            ItemCategory::Dagger => Some(WeaponForm::Dagger),
            ItemCategory::Axe => Some(WeaponForm::Axe),
            ItemCategory::Spear => Some(WeaponForm::Spear),
            ItemCategory::HeavyWeapon => Some(WeaponForm::Heavy),
            _ => None,
        }
    }
}

/// The weapon in the right hand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeldWeapon {
    pub form: WeaponForm,
    /// Guard/haft wrap colour: the item's `Trail` colour, else the default.
    pub accent: Rgb,
    pub two_handed: bool,
}

/// Where the shield is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShieldPlacement {
    /// On the left arm (a one-handed weapon or none).
    LeftHand,
    /// Slung on the back: a two-handed weapon holds both hands.
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeldShield {
    pub color: Rgb,
    pub placement: ShieldPlacement,
}

/// What the hands show.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HeldLook {
    pub weapon: Option<HeldWeapon>,
    pub shield: Option<HeldShield>,
}

impl HeldLook {
    /// Reads the `Weapon` and `Offhand` slots. Unknown items and items of
    /// the wrong category show nothing (never a default weapon).
    pub fn resolve(equipment: &Equipment, catalog: &ItemCatalog) -> Self {
        let weapon = equipment
            .get(EquipmentSlot::Weapon)
            .and_then(|id| catalog.get(id).map(|def| (id, def)))
            .and_then(|(id, def)| {
                let form = WeaponForm::of(def.category)?;
                let accent = match def.visual {
                    Some(ItemVisual::Trail { color }) => color,
                    _ => DEFAULT_ACCENT,
                };
                Some(HeldWeapon {
                    form,
                    accent,
                    two_handed: catalog.is_two_handed(id),
                })
            });
        let shield = equipment
            .get(EquipmentSlot::Offhand)
            .and_then(|id| catalog.get(id))
            .filter(|def| def.category == ItemCategory::Shield)
            .map(|def| HeldShield {
                color: match def.visual {
                    Some(ItemVisual::Tint { color }) => color,
                    _ => DEFAULT_SHIELD,
                },
                placement: shield_placement(weapon.as_ref()),
            });
        HeldLook { weapon, shield }
    }

    /// Number of held layers shown (reports).
    pub fn layer_count(&self) -> usize {
        usize::from(self.weapon.is_some()) + usize::from(self.shield.is_some())
    }
}

impl WeaponForm {
    /// Long weapons that would cross the floor if held like the sabre at rest.
    pub fn is_long(self) -> bool {
        matches!(self, WeaponForm::Spear | WeaponForm::Heavy)
    }
}

/// Angle (degrees) the weapon is drawn at. The sockets give the sabre's
/// direction; at rest (any non-strike frame) the sabre points down, which
/// would plant a spear or a greatblade in the floor: those are carried the
/// other way round (point up: a spear held upright, a greatblade on the
/// shoulder). Strike frames always follow the sabre, so a swing never jumps.
pub fn held_angle(form: WeaponForm, socket_angle: f32, striking: bool) -> f32 {
    let pointing_down = socket_angle.to_radians().sin() < 0.0;
    if form.is_long() && !striking && pointing_down {
        socket_angle + 180.0
    } else {
        socket_angle
    }
}

/// The two-hands rule, visual side.
pub fn shield_placement(weapon: Option<&HeldWeapon>) -> ShieldPlacement {
    if weapon.is_some_and(|w| w.two_handed) {
        ShieldPlacement::Back
    } else {
        ShieldPlacement::LeftHand
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_weapon_category_has_its_own_form() {
        let forms: std::collections::BTreeSet<_> = [
            ItemCategory::Sword,
            ItemCategory::Dagger,
            ItemCategory::Axe,
            ItemCategory::Spear,
            ItemCategory::HeavyWeapon,
        ]
        .into_iter()
        .filter_map(WeaponForm::of)
        .collect();
        assert_eq!(forms.len(), WeaponForm::ALL.len());
        assert_eq!(WeaponForm::of(ItemCategory::Shield), None);
        assert_eq!(WeaponForm::of(ItemCategory::Armor), None);
    }

    #[test]
    fn long_weapons_are_carried_point_up_at_rest_only() {
        // Idle sabre angle (-66): sword and axe follow it, spear and
        // greatblade turn point up; during a strike everything follows.
        assert_eq!(held_angle(WeaponForm::Sword, -66.0, false), -66.0);
        assert_eq!(held_angle(WeaponForm::Axe, -66.0, false), -66.0);
        assert_eq!(held_angle(WeaponForm::Spear, -66.0, false), 114.0);
        assert_eq!(held_angle(WeaponForm::Heavy, -66.0, false), 114.0);
        assert_eq!(held_angle(WeaponForm::Spear, -66.0, true), -66.0);
        // Already pointing up (guard): unchanged.
        assert_eq!(held_angle(WeaponForm::Spear, 50.0, false), 50.0);
    }

    #[test]
    fn a_two_handed_weapon_moves_the_shield_to_the_back() {
        let one = HeldWeapon {
            form: WeaponForm::Sword,
            accent: DEFAULT_ACCENT,
            two_handed: false,
        };
        let two = HeldWeapon {
            form: WeaponForm::Spear,
            two_handed: true,
            ..one
        };
        assert_eq!(shield_placement(None), ShieldPlacement::LeftHand);
        assert_eq!(shield_placement(Some(&one)), ShieldPlacement::LeftHand);
        assert_eq!(shield_placement(Some(&two)), ShieldPlacement::Back);
    }
}
