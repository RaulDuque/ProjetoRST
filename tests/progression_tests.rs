//! Experience curve, levelling, attribute points and skill requirements.

use std::collections::{BTreeMap, BTreeSet};

use dark_castle_rpg::player::attributes::{BaseAttributes, PrimaryAttribute};
use dark_castle_rpg::player::progression::{
    AllocationError, ExperienceCurve, LevelGrowth, Progression, ProgressionRules,
};
use dark_castle_rpg::skills::definitions::Requirements;
use dark_castle_rpg::skills::unlock::{RequirementContext, RequirementError};

fn rules() -> ProgressionRules {
    ProgressionRules {
        curve: ExperienceCurve {
            base: 100.0,
            exponent: 1.0,
            max_level: 5,
        },
        growth: LevelGrowth {
            automatic: BaseAttributes {
                strength: 1,
                constitution: 1,
                intelligence: 0,
                luck: 0,
            },
            points_per_level: 3,
        },
    }
}

#[test]
fn curve_is_configurable() {
    let curve = ExperienceCurve {
        base: 10.0,
        exponent: 2.0,
        max_level: 99,
    };
    assert_eq!(curve.exp_to_next(1), Some(10));
    assert_eq!(curve.exp_to_next(3), Some(90));
    assert_eq!(curve.exp_to_next(99), None);
    assert_eq!(curve.exp_to_next(0), None);
}

#[test]
fn gaining_exp_levels_up_and_grows_attributes() {
    let mut p = Progression::default();
    let mut attrs = BaseAttributes::default();
    let before = attrs;

    let summary = p.gain_experience(99, &rules(), &mut attrs);
    assert_eq!(summary.levels_gained, 0);
    assert_eq!(p.level(), 1);

    // 1 more → level 2 (needs 100); then 200 more → level 3.
    let summary = p.gain_experience(201, &rules(), &mut attrs);
    assert_eq!(summary.levels_gained, 2);
    assert_eq!(summary.new_level, 3);
    assert_eq!(p.exp_into_level(), 0);
    assert_eq!(p.total_exp(), 300);
    assert_eq!(p.unspent_points(), 6);
    assert_eq!(attrs.strength, before.strength + 2);
    assert_eq!(attrs.constitution, before.constitution + 2);
    assert_eq!(attrs.intelligence, before.intelligence);
}

#[test]
fn level_is_capped() {
    let mut p = Progression::default();
    let mut attrs = BaseAttributes::default();
    p.gain_experience(1_000_000, &rules(), &mut attrs);
    assert_eq!(p.level(), 5);
    assert_eq!(p.exp_into_level(), 0);
    assert_eq!(p.total_exp(), 1_000_000);
}

#[test]
fn attribute_points_are_distributable() {
    let mut p = Progression::default();
    let mut attrs = BaseAttributes::default();
    assert_eq!(
        p.allocate_point(PrimaryAttribute::Luck, &mut attrs),
        Err(AllocationError::NoPointsAvailable)
    );
    p.gain_experience(100, &rules(), &mut attrs);
    let luck = attrs.luck;
    for _ in 0..3 {
        assert!(p.allocate_point(PrimaryAttribute::Luck, &mut attrs).is_ok());
    }
    assert_eq!(attrs.luck, luck + 3);
    assert!(
        p.allocate_point(PrimaryAttribute::Luck, &mut attrs)
            .is_err()
    );
}

#[test]
fn requirements_check_level_attributes_items_and_progress() {
    let attrs = BaseAttributes::default();
    let items: BTreeSet<String> = ["iron_sword".to_owned()].into();
    let flags: BTreeSet<String> = BTreeSet::new();
    let ctx = RequirementContext {
        level: 3,
        attributes: &attrs,
        owned_items: &items,
        progress_flags: &flags,
    };

    let mut req = Requirements {
        min_level: 4,
        ..Default::default()
    };
    assert_eq!(
        req.check(&ctx),
        Err(RequirementError::Level {
            required: 4,
            current: 3
        })
    );

    req.min_level = 1;
    req.attributes = BTreeMap::from([(PrimaryAttribute::Intelligence, 20)]);
    assert!(matches!(
        req.check(&ctx),
        Err(RequirementError::Attribute { .. })
    ));

    req.attributes.clear();
    req.items = vec!["iron_sword".into()];
    assert!(req.check(&ctx).is_ok());
    req.items.push("moon_key".into());
    assert_eq!(
        req.check(&ctx),
        Err(RequirementError::Item("moon_key".into()))
    );

    req.items.clear();
    req.progress_flags = vec!["met_the_archivist".into()];
    assert!(matches!(
        req.check(&ctx),
        Err(RequirementError::Progress(_))
    ));
}
