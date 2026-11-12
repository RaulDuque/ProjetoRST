//! Command notation parsing, sequence recognition and the full
//! input → outcome pipeline (the seven outcomes required by the design).

use dark_castle_rpg::combat::command_buffer::{CommandBuffer, CommandButton, Motion, TimedInput};
use dark_castle_rpg::combat::command_parser::{
    CommandPattern, CommandSequence, CommandTiming, NotationError, Recognition, SequenceMatch,
    match_sequence, recognize,
};
use dark_castle_rpg::combat::damage::DamageKind;
use dark_castle_rpg::player::state::{ActionPhase, Facing};
use dark_castle_rpg::skills::cooldowns::Cooldowns;
use dark_castle_rpg::skills::definitions::{
    SkillCategory, SkillDefinition, SkillEffect, SkillLibrary,
};
use dark_castle_rpg::skills::execution::{
    CasterSnapshot, CommandOutcome, ResourceKind, resolve_command,
};
use dark_castle_rpg::skills::unlock::LearnedSkills;

use Motion::{Back, Down, DownForward, Forward, Neutral, Up};

// ---------------------------------------------------------------- helpers

fn seq(notation: &str) -> CommandSequence {
    CommandSequence::parse(notation).unwrap_or_else(|e| panic!("bad notation {notation}: {e}"))
}

/// Builds a buffer from motions spaced `step` seconds apart, then a button.
fn inputs(motions: &[Motion], button: CommandButton, step: f64) -> Vec<TimedInput> {
    let mut buffer = CommandBuffer::default();
    let mut t = 0.0;
    for m in motions {
        buffer.record_motion(*m, t);
        t += step;
    }
    buffer.record_button(button, t);
    buffer.entries().copied().collect()
}

fn skill(id: &str, notation: &str, mp: i32, hearts: i32, cooldown: f32) -> SkillDefinition {
    SkillDefinition {
        id: id.to_owned(),
        name: id.to_owned(),
        description: String::new(),
        category: SkillCategory::SwordTechnique,
        command: Some(seq(notation)),
        timing: CommandTiming::default(),
        priority: 0,
        mp_cost: mp,
        hearts_cost: hearts,
        cooldown,
        damage_kind: DamageKind::Physical,
        element: dark_castle_rpg::magic::elements::Element::Physical,
        base_damage: 0,
        damage_multiplier: 1.0,
        range: 0.0,
        execution_time: 0.3,
        animation: String::new(),
        vfx: String::new(),
        effect: SkillEffect::None,
        inflicts: None,
        requirements: Default::default(),
        usable_in: vec![ActionPhase::Grounded],
        acquisition: Default::default(),
        command_revealed_on_learn: true,
    }
}

fn library() -> SkillLibrary {
    SkillLibrary::from_definitions([
        skill("crescent_slash", "↓ ↘ → + ATTACK", 0, 0, 1.0),
        skill("ember_bolt", "↓ ↘ → + MAGIC", 8, 0, 0.0),
        skill("rising_edge", "↓ ↑ + ATTACK", 0, 2, 0.0),
        skill("backstep_cut", "→ ← → + ATTACK", 0, 0, 0.0),
    ])
    .unwrap_or_else(|e| panic!("library: {e}"))
}

fn learned(ids: &[&str], lib: &SkillLibrary) -> LearnedSkills {
    let mut l = LearnedSkills::default();
    for id in ids {
        if let Some(def) = lib.get(id) {
            l.grant(def);
        }
    }
    l
}

fn caster<'a>(
    learned: &'a LearnedSkills,
    cds: &'a Cooldowns,
    mp: i32,
    hearts: i32,
) -> CasterSnapshot<'a> {
    CasterSnapshot {
        learned,
        cooldowns: cds,
        mp,
        hearts,
        phase: Some(ActionPhase::Grounded),
    }
}

// ---------------------------------------------------------------- notation

#[test]
fn parses_arrows_and_letters_identically() {
    assert_eq!(seq("↓ ↘ → + ATTACK"), seq("D DF F ATK"));
    assert_eq!(
        seq("d,df,f+attack").motions(),
        &[Down, DownForward, Forward]
    );
    assert_eq!(seq("← → + MAGIC").button(), CommandButton::Magic);
}

#[test]
fn rejects_malformed_notation() {
    assert_eq!(CommandSequence::parse(""), Err(NotationError::Empty));
    assert_eq!(
        CommandSequence::parse("↓ →"),
        Err(NotationError::MissingButton)
    );
    assert!(matches!(
        CommandSequence::parse("↓ X ATTACK"),
        Err(NotationError::UnknownToken(t)) if t == "X"
    ));
    assert!(matches!(
        CommandSequence::parse("ATTACK ↓"),
        Err(NotationError::ButtonNotLast(_))
    ));
}

#[test]
fn notation_round_trips_through_display() {
    let original = seq("D DF F + ATTACK");
    assert_eq!(seq(&original.to_string()), original);
}

// ---------------------------------------------------------------- matching

#[test]
fn facing_left_mirrors_forward_and_back() {
    // Holding LEFT while facing left is "forward".
    assert_eq!(Motion::from_axes(-1, 0, Facing::Left), Forward);
    assert_eq!(Motion::from_axes(-1, -1, Facing::Left), DownForward);
    assert_eq!(Motion::from_axes(1, 0, Facing::Left), Back);
    assert_eq!(Motion::from_axes(1, 0, Facing::Right), Forward);
}

#[test]
fn diagonals_are_recognised() {
    let timing = CommandTiming::default();
    let buf = inputs(&[Down, DownForward, Forward], CommandButton::Attack, 0.05);
    assert_eq!(
        match_sequence(&buf, &seq("↓ ↘ → ATK"), &timing),
        SequenceMatch::Complete
    );
}

#[test]
fn tolerates_one_accidental_input() {
    let timing = CommandTiming::default();
    // An accidental Up between Down and DownForward.
    let buf = inputs(
        &[Down, Up, DownForward, Forward],
        CommandButton::Attack,
        0.05,
    );
    assert_eq!(
        match_sequence(&buf, &seq("↓ ↘ → ATK"), &timing),
        SequenceMatch::Complete
    );

    // Two accidental inputs exceed the default tolerance.
    let buf = inputs(
        &[Down, Up, Back, DownForward, Forward],
        CommandButton::Attack,
        0.05,
    );
    assert_ne!(
        match_sequence(&buf, &seq("↓ ↘ → ATK"), &timing),
        SequenceMatch::Complete
    );
}

#[test]
fn neutral_is_ignored_unless_required() {
    let timing = CommandTiming::default();
    let buf = inputs(&[Down, Neutral, Forward], CommandButton::Attack, 0.05);
    assert_eq!(
        match_sequence(&buf, &seq("↓ → ATK"), &timing),
        SequenceMatch::Complete
    );
}

#[test]
fn wrong_button_never_matches() {
    let timing = CommandTiming::default();
    let buf = inputs(&[Down, DownForward, Forward], CommandButton::Jump, 0.05);
    assert_eq!(
        match_sequence(&buf, &seq("↓ ↘ → ATK"), &timing),
        SequenceMatch::NoMatch
    );
}

#[test]
fn step_window_is_configurable() {
    let strict = CommandTiming {
        step_window: 0.05,
        ..CommandTiming::default()
    };
    let buf = inputs(&[Down, DownForward, Forward], CommandButton::Attack, 0.1);
    assert_eq!(
        match_sequence(&buf, &seq("↓ ↘ → ATK"), &strict),
        SequenceMatch::TooSlow
    );
    assert_eq!(
        match_sequence(&buf, &seq("↓ ↘ → ATK"), &CommandTiming::default()),
        SequenceMatch::Complete
    );
}

#[test]
fn priority_breaks_ties_between_similar_commands() {
    let short = CommandPattern {
        id: "short".into(),
        sequence: seq("↘ → ATK"),
        timing: CommandTiming::default(),
        priority: 0,
    };
    let long = CommandPattern {
        id: "long".into(),
        sequence: seq("↓ ↘ → ATK"),
        timing: CommandTiming::default(),
        priority: 0,
    };
    let buf = inputs(&[Down, DownForward, Forward], CommandButton::Attack, 0.05);
    // Same priority: the longer (more specific) command wins.
    assert_eq!(
        recognize(&buf, &[short.clone(), long.clone()]),
        Recognition::Matched(vec!["long".into(), "short".into()])
    );
    // Explicit priority overrides length.
    let boosted = CommandPattern {
        priority: 5,
        ..short
    };
    assert_eq!(
        recognize(&buf, &[long, boosted]),
        Recognition::Matched(vec!["short".into(), "long".into()])
    );
}

#[test]
fn similar_sequences_resolve_to_the_right_command() {
    let lib = library();
    let l = learned(&["crescent_slash", "backstep_cut", "rising_edge"], &lib);
    let cds = Cooldowns::default();
    let c = caster(&l, &cds, 50, 10);

    let a = inputs(&[Down, DownForward, Forward], CommandButton::Attack, 0.05);
    let b = inputs(&[Forward, Back, Forward], CommandButton::Attack, 0.05);
    let d = inputs(&[Down, Up], CommandButton::Attack, 0.05);
    assert_eq!(
        resolve_command(&a, &lib, &c),
        CommandOutcome::Executed {
            id: "crescent_slash".into()
        }
    );
    assert_eq!(
        resolve_command(&b, &lib, &c),
        CommandOutcome::Executed {
            id: "backstep_cut".into()
        }
    );
    assert_eq!(
        resolve_command(&d, &lib, &c),
        CommandOutcome::Executed {
            id: "rising_edge".into()
        }
    );
}

// ---------------------------------------------------------------- the seven outcomes

#[test]
fn outcome_executed() {
    let lib = library();
    let l = learned(&["crescent_slash"], &lib);
    let cds = Cooldowns::default();
    let buf = inputs(&[Down, DownForward, Forward], CommandButton::Attack, 0.05);
    assert_eq!(
        resolve_command(&buf, &lib, &caster(&l, &cds, 0, 0)),
        CommandOutcome::Executed {
            id: "crescent_slash".into()
        }
    );
}

#[test]
fn outcome_incomplete() {
    let lib = library();
    let l = learned(&["crescent_slash"], &lib);
    let cds = Cooldowns::default();
    // Missed the initial Down.
    let buf = inputs(&[DownForward, Forward], CommandButton::Attack, 0.05);
    assert_eq!(
        resolve_command(&buf, &lib, &caster(&l, &cds, 0, 0)),
        CommandOutcome::Incomplete {
            id: "crescent_slash".into(),
            matched: 2,
            required: 3
        }
    );
}

#[test]
fn outcome_invalid() {
    let lib = library();
    let l = learned(&["crescent_slash"], &lib);
    let cds = Cooldowns::default();
    let plain = inputs(&[], CommandButton::Attack, 0.05);
    assert_eq!(
        resolve_command(&plain, &lib, &caster(&l, &cds, 0, 0)),
        CommandOutcome::Invalid
    );
    let jump = inputs(&[Down, DownForward, Forward], CommandButton::Jump, 0.05);
    assert_eq!(
        resolve_command(&jump, &lib, &caster(&l, &cds, 0, 0)),
        CommandOutcome::Invalid
    );
}

#[test]
fn outcome_too_slow() {
    let lib = library();
    let l = learned(&["crescent_slash"], &lib);
    let cds = Cooldowns::default();
    let buf = inputs(&[Down, DownForward, Forward], CommandButton::Attack, 0.4);
    assert_eq!(
        resolve_command(&buf, &lib, &caster(&l, &cds, 0, 0)),
        CommandOutcome::TooSlow {
            id: "crescent_slash".into()
        }
    );
}

#[test]
fn outcome_insufficient_mp_and_hearts() {
    let lib = library();
    let l = learned(&["ember_bolt", "rising_edge"], &lib);
    let cds = Cooldowns::default();
    let spell = inputs(&[Down, DownForward, Forward], CommandButton::Magic, 0.05);
    assert_eq!(
        resolve_command(&spell, &lib, &caster(&l, &cds, 5, 10)),
        CommandOutcome::InsufficientResources {
            id: "ember_bolt".into(),
            resource: ResourceKind::Mp,
            required: 8,
            available: 5
        }
    );
    let edge = inputs(&[Down, Up], CommandButton::Attack, 0.05);
    assert_eq!(
        resolve_command(&edge, &lib, &caster(&l, &cds, 50, 1)),
        CommandOutcome::InsufficientResources {
            id: "rising_edge".into(),
            resource: ResourceKind::Hearts,
            required: 2,
            available: 1
        }
    );
}

#[test]
fn outcome_not_learned() {
    let lib = library();
    let l = LearnedSkills::default();
    let cds = Cooldowns::default();
    let buf = inputs(&[Down, Up], CommandButton::Attack, 0.05);
    assert_eq!(
        resolve_command(&buf, &lib, &caster(&l, &cds, 50, 10)),
        CommandOutcome::NotLearned {
            id: "rising_edge".into()
        }
    );
}

#[test]
fn outcome_on_cooldown() {
    let lib = library();
    let l = learned(&["crescent_slash"], &lib);
    let mut cds = Cooldowns::default();
    cds.start("crescent_slash", 1.0);
    let buf = inputs(&[Down, DownForward, Forward], CommandButton::Attack, 0.05);
    assert!(matches!(
        resolve_command(&buf, &lib, &caster(&l, &cds, 0, 0)),
        CommandOutcome::OnCooldown { id, remaining } if id == "crescent_slash" && remaining > 0.9
    ));
}

#[test]
fn not_allowed_in_wrong_state() {
    let lib = library();
    let l = learned(&["crescent_slash"], &lib);
    let cds = Cooldowns::default();
    let buf = inputs(&[Down, DownForward, Forward], CommandButton::Attack, 0.05);
    let mut c = caster(&l, &cds, 0, 0);
    c.phase = None; // e.g. hurt
    assert_eq!(
        resolve_command(&buf, &lib, &c),
        CommandOutcome::NotAllowedNow {
            id: "crescent_slash".into()
        }
    );
}

// ---------------------------------------------------------------- buffer behaviour

#[test]
fn clearing_the_buffer_prevents_duplicate_execution() {
    let lib = library();
    let l = learned(&["crescent_slash"], &lib);
    let cds = Cooldowns::default();
    let c = caster(&l, &cds, 0, 0);

    let mut buffer = CommandBuffer::default();
    buffer.record_motion(Down, 0.0);
    buffer.record_motion(DownForward, 0.05);
    buffer.record_motion(Forward, 0.1);
    buffer.record_button(CommandButton::Attack, 0.15);
    let first: Vec<_> = buffer.entries().copied().collect();
    assert!(resolve_command(&first, &lib, &c).is_executed());

    // The game clears the buffer after resolving a button press.
    buffer.clear();
    buffer.record_button(CommandButton::Attack, 0.2);
    let second: Vec<_> = buffer.entries().copied().collect();
    assert_eq!(resolve_command(&second, &lib, &c), CommandOutcome::Invalid);
}

#[test]
fn earlier_button_press_blocks_reuse_of_old_motions() {
    let timing = CommandTiming::default();
    let mut buffer = CommandBuffer::default();
    buffer.record_motion(Down, 0.0);
    buffer.record_motion(DownForward, 0.05);
    buffer.record_motion(Forward, 0.1);
    buffer.record_button(CommandButton::Attack, 0.15);
    buffer.record_button(CommandButton::Attack, 0.2);
    let entries: Vec<_> = buffer.entries().copied().collect();
    assert_eq!(
        match_sequence(&entries, &seq("↓ ↘ → ATK"), &timing),
        SequenceMatch::NoMatch
    );
}

#[test]
fn holding_a_direction_records_it_once_and_old_inputs_expire() {
    let mut buffer = CommandBuffer::new(8, 1.0);
    assert!(buffer.record_motion(Down, 0.0));
    assert!(!buffer.record_motion(Down, 0.1));
    assert!(buffer.record_motion(Forward, 0.2));
    assert_eq!(buffer.len(), 2);
    buffer.prune(1.15);
    assert_eq!(buffer.len(), 1);
}

#[test]
fn library_loads_from_ron() {
    let text = r#"[
        (
            id: "test_slash",
            name: "Test Slash",
            description: "A test.",
            category: SwordTechnique,
            command: Some("↓ → + ATTACK"),
            damage_kind: Physical,
            execution_time: 0.3,
            effect: MeleeStrike(width: 40.0, height: 30.0, lunge_speed: 0.0),
        ),
    ]"#;
    let defs = SkillLibrary::parse_ron(text, "inline").unwrap_or_else(|e| panic!("{e}"));
    let lib = SkillLibrary::from_definitions(defs).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(lib.patterns().len(), 1);
    assert_eq!(lib.patterns()[0].sequence, seq("D F ATK"));

    let bad = r#"[( id: "x", name: "x", description: "", category: SwordTechnique,
        command: Some("↓ ? ATTACK"), damage_kind: Physical, execution_time: 0.1, effect: None )]"#;
    assert!(SkillLibrary::parse_ron(bad, "inline").is_err());
}
