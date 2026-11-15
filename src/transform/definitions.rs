//! Transformation definitions (V1.9), loaded from
//! `assets/data/transformations.ron`. Pure module: no Bevy types.
//!
//! A definition is plain data, validated when it is loaded: identity, the
//! unlock requirement (the same typed [`Requirement`] doors use), costs,
//! movement and collision profiles, the actions the form keeps, cancel
//! rules, visual parameters and what a load does with it. Nothing in the
//! file is executed: every field is matched by the systems of
//! `transform::systems`. Details: docs/TRANSFORMATIONS_V1_9.md.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::world::collision::BarrierKind;
use crate::world::graph::Requirement;

/// Stable identity of a form. One variant per implemented form: new forms
/// are added with their behaviour, never as empty placeholders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FormId {
    Spectral,
}

/// What the player may do. A form lists the actions it keeps; every other
/// one is blocked while it is active (the human form keeps them all).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FormAction {
    Move,
    Jump,
    Run,
    Crouch,
    /// Basic strikes and sword techniques (the attack button).
    Attack,
    /// Spells (the magic button).
    Spell,
    Guard,
    Dodge,
    /// Chests, levers and switches (the interact key).
    Interact,
}

impl FormAction {
    pub const ALL: [FormAction; 9] = [
        FormAction::Move,
        FormAction::Jump,
        FormAction::Run,
        FormAction::Crouch,
        FormAction::Attack,
        FormAction::Spell,
        FormAction::Guard,
        FormAction::Dodge,
        FormAction::Interact,
    ];
}

/// How the body moves while the form is active.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MovementProfile {
    /// Horizontal speed (world units per second).
    pub speed: f32,
    /// Multiplies the global gravity (0 < scale <= 1).
    pub gravity_scale: f32,
    /// Fall speed cap (negative, world units per second).
    pub max_fall_speed: f32,
}

/// What the body passes while the form is active.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollisionProfile {
    pub passes: Vec<BarrierKind>,
}

/// MP paid to enter and to stay in the form.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FormCosts {
    pub activation_mp: i32,
    pub mp_per_second: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivationRules {
    /// Only while standing on the ground.
    pub grounded_only: bool,
}

/// When the form ends besides death (always) and a load (see [`LoadRule`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelRules {
    /// The transformation key ends it.
    pub voluntary: bool,
    /// It ends when the upkeep cannot be paid.
    pub on_mp_empty: bool,
}

/// Presentation parameters (no asset: the form recolours the existing
/// protagonist sprite, draws an outline and particles).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FormVisual {
    pub tint: (f32, f32, f32),
    pub alpha: f32,
    pub outline: (f32, f32, f32),
    pub outline_alpha: f32,
    /// Seconds between two particles.
    pub mote_interval: f32,
    /// Seconds of upkeep left under which the form pulses (low MP).
    pub low_mp_seconds: f32,
}

/// What a load does with an active form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadRule {
    /// The save never stores the form: the player loads human (the save
    /// refuses positions where a human would not fit).
    HumanForm,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationDefinition {
    pub id: FormId,
    pub name: String,
    pub description: String,
    /// What unlocks the form (e.g. the ability a relic grants).
    pub unlock: Requirement,
    /// Name of what unlocks it, for the HUD ("needs the Relic of the Veil").
    pub unlock_hint: String,
    pub costs: FormCosts,
    pub movement: MovementProfile,
    pub collision: CollisionProfile,
    /// Actions kept while active; every other action is blocked.
    pub allowed: Vec<FormAction>,
    pub activation: ActivationRules,
    pub cancel: CancelRules,
    pub visual: FormVisual,
    pub on_load: LoadRule,
}

impl TransformationDefinition {
    pub fn allows(&self, action: FormAction) -> bool {
        self.allowed.contains(&action)
    }

    /// The actions this form blocks.
    pub fn blocked(&self) -> Vec<FormAction> {
        FormAction::ALL
            .into_iter()
            .filter(|a| !self.allows(*a))
            .collect()
    }

    /// Seconds to fall `distance` from rest under the form's gravity.
    pub fn fall_time(&self, distance: f32, gravity: f32) -> f32 {
        let accel = (gravity * self.movement.gravity_scale).abs();
        let top = self.movement.max_fall_speed.abs();
        if accel <= 0.0 || top <= 0.0 {
            return f32::INFINITY;
        }
        let accel_distance = top * top / (2.0 * accel);
        if distance <= accel_distance {
            (2.0 * distance / accel).sqrt()
        } else {
            top / accel + (distance - accel_distance) / top
        }
    }

    /// MP spent entering the form and staying in it for `seconds`.
    pub fn mp_for(&self, seconds: f32) -> f32 {
        self.costs.activation_mp as f32 + self.costs.mp_per_second * seconds
    }

    fn check(&self) -> Result<(), &'static str> {
        let m = &self.movement;
        if !(m.speed.is_finite() && m.speed > 0.0) {
            return Err("movement.speed must be positive");
        }
        if !(m.gravity_scale > 0.0 && m.gravity_scale <= 1.0) {
            return Err("movement.gravity_scale must be in (0, 1]");
        }
        if !(m.max_fall_speed.is_finite() && m.max_fall_speed < 0.0) {
            return Err("movement.max_fall_speed must be negative");
        }
        if self.costs.activation_mp < 0
            || !(self.costs.mp_per_second.is_finite() && self.costs.mp_per_second >= 0.0)
        {
            return Err("costs must not be negative");
        }
        if !self.allows(FormAction::Move) {
            return Err("a form must keep Move");
        }
        if !self.cancel.voluntary && !self.cancel.on_mp_empty {
            return Err("a form must end voluntarily or when MP runs out");
        }
        let v = &self.visual;
        let interval_ok = v.mote_interval.is_finite() && v.mote_interval > 0.0;
        if !(v.alpha > 0.0 && v.alpha <= 1.0) || !interval_ok {
            return Err("visual.alpha must be in (0, 1] and mote_interval positive");
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("cannot read {path}: {message}")]
    Io { path: String, message: String },
    #[error("cannot parse {path}: {message}")]
    Parse { path: String, message: String },
    #[error("form {0:?} is defined twice")]
    Duplicate(FormId),
    #[error("form {form:?}: {reason}")]
    Invalid { form: FormId, reason: &'static str },
}

/// Every transformation of the game, validated.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransformLibrary {
    forms: BTreeMap<FormId, TransformationDefinition>,
}

impl TransformLibrary {
    pub fn from_list(list: Vec<TransformationDefinition>) -> Result<Self, TransformError> {
        let mut forms = BTreeMap::new();
        for def in list {
            def.check().map_err(|reason| TransformError::Invalid {
                form: def.id,
                reason,
            })?;
            let id = def.id;
            if forms.insert(id, def).is_some() {
                return Err(TransformError::Duplicate(id));
            }
        }
        Ok(Self { forms })
    }

    pub fn load(path: &Path) -> Result<Self, TransformError> {
        let text = std::fs::read_to_string(path).map_err(|e| TransformError::Io {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;
        let list: Vec<TransformationDefinition> =
            ron::from_str(&text).map_err(|e| TransformError::Parse {
                path: path.display().to_string(),
                message: e.to_string(),
            })?;
        Self::from_list(list)
    }

    pub fn get(&self, id: FormId) -> Option<&TransformationDefinition> {
        self.forms.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TransformationDefinition> {
        self.forms.values()
    }

    /// The form the transformation key enters (V1.9: the only one).
    pub fn primary(&self) -> Option<&TransformationDefinition> {
        self.forms.values().next()
    }

    /// For each barrier kind, what lets the player through it: the unlock
    /// of the form that passes it (`Any` of them if several do). The world
    /// manifest's `passages` must say the same (`app::load_world`).
    pub fn passages(&self) -> BTreeMap<BarrierKind, Requirement> {
        let mut out: BTreeMap<BarrierKind, Vec<Requirement>> = BTreeMap::new();
        for def in self.forms.values() {
            for kind in &def.collision.passes {
                out.entry(*kind).or_default().push(def.unlock.clone());
            }
        }
        out.into_iter()
            .map(|(kind, mut unlocks)| {
                let requirement = if unlocks.len() == 1 {
                    unlocks.remove(0)
                } else {
                    Requirement::Any(unlocks)
                };
                (kind, requirement)
            })
            .collect()
    }

    /// The form that passes `kind` (the first one, by id).
    pub fn passing(&self, kind: BarrierKind) -> Option<&TransformationDefinition> {
        self.forms
            .values()
            .find(|d| d.collision.passes.contains(&kind))
    }
}

#[cfg(test)]
pub(crate) mod tests_support {
    use super::*;
    use crate::world::graph::Ability;

    pub fn spectral() -> TransformationDefinition {
        TransformationDefinition {
            id: FormId::Spectral,
            name: "Spectral Form".into(),
            description: "test".into(),
            unlock: Requirement::Ability(Ability::VeilStep),
            unlock_hint: "Relic of the Veil".into(),
            costs: FormCosts {
                activation_mp: 8,
                mp_per_second: 4.0,
            },
            movement: MovementProfile {
                speed: 150.0,
                gravity_scale: 0.35,
                max_fall_speed: -220.0,
            },
            collision: CollisionProfile {
                passes: vec![BarrierKind::Spectral],
            },
            allowed: vec![FormAction::Move],
            activation: ActivationRules {
                grounded_only: true,
            },
            cancel: CancelRules {
                voluntary: true,
                on_mp_empty: true,
            },
            visual: FormVisual {
                tint: (0.6, 0.85, 1.0),
                alpha: 0.55,
                outline: (0.7, 0.95, 1.0),
                outline_alpha: 0.4,
                mote_interval: 0.1,
                low_mp_seconds: 3.0,
            },
            on_load: LoadRule::HumanForm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tests_support::spectral;
    use super::*;
    use crate::world::graph::Ability;

    #[test]
    fn a_form_blocks_what_it_does_not_keep() {
        let def = spectral();
        assert!(def.allows(FormAction::Move));
        assert_eq!(def.blocked().len(), FormAction::ALL.len() - 1);
        assert!(def.blocked().contains(&FormAction::Attack));
    }

    #[test]
    fn invalid_definitions_are_refused() {
        let mut no_move = spectral();
        no_move.allowed.clear();
        assert!(matches!(
            TransformLibrary::from_list(vec![no_move]),
            Err(TransformError::Invalid { .. })
        ));
        let mut rising = spectral();
        rising.movement.max_fall_speed = 10.0;
        assert!(TransformLibrary::from_list(vec![rising]).is_err());
        assert!(matches!(
            TransformLibrary::from_list(vec![spectral(), spectral()]),
            Err(TransformError::Duplicate(FormId::Spectral))
        ));
    }

    #[test]
    fn passages_come_from_the_forms_that_pass_each_kind() {
        let lib = TransformLibrary::from_list(vec![spectral()]).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(
            lib.passages().get(&BarrierKind::Spectral),
            Some(&Requirement::Ability(Ability::VeilStep))
        );
        assert_eq!(
            lib.passing(BarrierKind::Spectral).map(|d| d.id),
            Some(FormId::Spectral)
        );
    }

    #[test]
    fn falls_accelerate_then_keep_the_cap() {
        let def = spectral();
        let g = -1900.0;
        let accel: f32 = 1900.0 * 0.35;
        // Short fall: still accelerating.
        assert!((def.fall_time(10.0, g) - (20.0 / accel).sqrt()).abs() < 1e-4);
        // Long fall: the cap dominates.
        let long = def.fall_time(1000.0, g);
        assert!(long > 1000.0 / 220.0 && long < 1000.0 / 220.0 + 0.2);
        assert!((def.mp_for(2.0) - 16.0).abs() < 1e-6);
    }
}
