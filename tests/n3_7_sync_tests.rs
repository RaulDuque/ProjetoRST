//! The N3.7 synchronization study: the approved N3.5/N3.6 art stays byte for
//! byte, the reusable pipeline reproduces the N3.6 run, the real start of
//! the video is a valid clip with provenance, and the scenario and
//! transition data read by the isolated example are coherent.

use std::path::{Path, PathBuf};

use dark_castle_rpg::core::constants::PLAYER_RUN_SPEED;
use dark_castle_rpg::player::visual::manifest::{AnimationManifest, ClipMode};
use serde::Deserialize;

const CANVAS: (usize, usize) = (112, 128);
const PIVOT: (f32, f32) = (56.0, 118.0);
const SOLE_ROW: usize = 118;
/// Keys of `art-src/n3-4-original-protagonist/scripts/n34_palette.py` plus `.`.
const PALETTE_KEYS: &str = ".KWwhHSszeGgqPpoLldM";
const N36_FRAMES: [u32; 13] = [0, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59];
const START_FRAMES: [u32; 6] = [3, 5, 7, 9, 11, 13];

/// FNV-1a 64 of the approved files, recorded at the start of N3.7.
const APPROVED: [(&str, u64); 21] = [
    (
        "art-src/n3-6-animation-proof/generated/run6.png",
        0x0798ef4f59448a15,
    ),
    (
        "art-src/n3-6-animation-proof/generated/run10.png",
        0x6a844e3386b76b53,
    ),
    (
        "art-src/n3-6-animation-proof/generated/stand.png",
        0xfac9351613d5cb45,
    ),
    (
        "art-src/n3-6-animation-proof/generated/run.ron",
        0x8ee6391effd62ec1,
    ),
    (
        "art-src/n3-6-animation-proof/generated/boxes.ron",
        0x1050300a3f4d8cdc,
    ),
    (
        "art-src/n3-5-fidelity/generated/candidate_106.png",
        0x63a38af5ff4fd252,
    ),
    (
        "art-src/n3-5-fidelity/generated/candidate.ron",
        0xc00cf144ef9cb064,
    ),
    (
        "art-src/n3-5-fidelity/source/candidate_106.txt",
        0xa241c1d3ec6b4974,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_000.txt",
        0x1c3b97fdc06a15f7,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_048.txt",
        0x02104e9a11c070e5,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_049.txt",
        0xe909174beaecfbf1,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_050.txt",
        0x6ab9523136c311fa,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_051.txt",
        0xbe6983b34b660b99,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_052.txt",
        0xa1dc128e8da92f09,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_053.txt",
        0x7cf9ac701a693db5,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_054.txt",
        0x50a15bc4f29a8946,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_055.txt",
        0x6b95c3475c258a65,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_056.txt",
        0xf97a506cb2da6a5b,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_057.txt",
        0xf8be06ffcf6b7fae,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_058.txt",
        0xc719843e2998b062,
    ),
    (
        "art-src/n3-6-animation-proof/source/video_059.txt",
        0xe4c05e078dc3ee7b,
    ),
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn here() -> PathBuf {
    root().join("art-src/n3-7-run-sync")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

fn png_size(path: &Path) -> (u32, u32) {
    let bytes = std::fs::read(path).unwrap_or_else(|err| panic!("{}: {err}", path.display()));
    assert!(
        bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
        "{}",
        path.display()
    );
    let word = |at: usize| {
        u32::from_be_bytes(
            bytes[at..at + 4]
                .try_into()
                .unwrap_or_else(|_| panic!("png header")),
        )
    };
    (word(16), word(20))
}

fn manifest(path: &Path) -> AnimationManifest {
    AnimationManifest::from_ron(&read(path))
        .unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

/// A 112x128 grid with palette keys only; returns its lowest opaque row.
fn check_grid(path: &Path) -> usize {
    let text = read(path);
    let rows: Vec<&str> = text.lines().collect();
    assert_eq!(rows.len(), CANVAS.1, "{}: rows", path.display());
    let mut bottom = 0;
    for (y, row) in rows.iter().enumerate() {
        assert_eq!(row.chars().count(), CANVAS.0, "{}: row {y}", path.display());
        assert!(
            row.chars().all(|key| PALETTE_KEYS.contains(key)),
            "{}: row {y} has a key outside the palette",
            path.display()
        );
        if row.chars().any(|key| key != '.') {
            bottom = y;
        }
    }
    bottom
}

#[derive(Deserialize)]
struct Scenario {
    id: String,
    name: String,
    rate: f32,
    speed: f32,
}

#[derive(Deserialize)]
struct SyncPlan {
    reference_speed: f32,
    game_speed: f32,
    base_fps: f32,
    scenarios: Vec<Scenario>,
    run6_phase: Vec<String>,
}

#[derive(Deserialize)]
struct TransitionPlan {
    idle_hold: f32,
    run_frames: usize,
    exit_frame: usize,
    start_clip: String,
}

fn sync_plan() -> SyncPlan {
    ron::from_str(&read(&here().join("generated/sync.ron")))
        .unwrap_or_else(|err| panic!("sync.ron: {err}"))
}

#[test]
fn the_approved_n35_and_n36_art_is_byte_identical() {
    for (file, expected) in APPROVED {
        let bytes = std::fs::read(root().join(file)).unwrap_or_else(|err| panic!("{file}: {err}"));
        assert_eq!(fnv1a(&bytes), expected, "{file} changed");
    }
}

#[test]
fn the_pipeline_reproduces_the_n36_run_texel_for_texel() {
    for frame in N36_FRAMES {
        let name = format!("video_{frame:03}.txt");
        let approved = read(
            &root()
                .join("art-src/n3-6-animation-proof/source")
                .join(&name),
        );
        let rebuilt = read(&here().join("pipeline/run_n36/source").join(&name));
        assert_eq!(approved, rebuilt, "frame {frame}");
    }
    let approved = manifest(&root().join("art-src/n3-6-animation-proof/generated/run.ron"));
    let rebuilt = manifest(&here().join("pipeline/run_n36/generated/run_n36.ron"));
    for name in ["run6", "run10"] {
        let a = approved.clips.get(name).unwrap_or_else(|| panic!("{name}"));
        let b = rebuilt.clips.get(name).unwrap_or_else(|| panic!("{name}"));
        assert_eq!(a.count, b.count, "{name}");
        assert_eq!(a.pivot, b.pivot, "{name}");
        for (x, y) in a.durations.iter().zip(&b.durations) {
            assert!((x - y).abs() < 1e-4, "{name}: {x} vs {y}");
        }
    }
}

#[test]
fn the_real_start_is_a_valid_once_clip_on_the_run_canvas() {
    let dir = here().join("pipeline/run_start/generated");
    let manifest = manifest(&dir.join("run_start.ron"));
    assert!(!manifest.weapon_in_body);
    let clip = manifest
        .clips
        .get("run_start")
        .unwrap_or_else(|| panic!("run_start clip"));
    assert_eq!(clip.count as usize, START_FRAMES.len());
    assert_eq!(clip.pivot, PIVOT);
    assert!(matches!(clip.mode, ClipMode::Once { .. }));
    assert_eq!(clip.fallback.as_deref(), Some("idle"));
    assert!((clip.total_duration() - 0.5).abs() < 0.001);
    let atlas = manifest
        .atlases
        .get(&clip.atlas)
        .unwrap_or_else(|| panic!("run_start atlas"));
    assert_eq!(atlas.cell, (CANVAS.0 as u32, CANVAS.1 as u32));
    assert_eq!(atlas.density, 2);
    assert_eq!(png_size(&dir.join(&atlas.texture)), atlas.image_size());
    for frame in START_FRAMES {
        let bottom = check_grid(
            &here()
                .join("pipeline/run_start/source")
                .join(format!("video_{frame:03}.txt")),
        );
        assert!(
            bottom <= SOLE_ROW,
            "start frame {frame} sinks below the ground"
        );
    }
}

#[test]
fn every_pipeline_frame_records_its_provenance() {
    for (name, frames) in [
        ("run_n36", N36_FRAMES.len()),
        ("run_start", 14), // frames 0..=13 are extracted, six are used
    ] {
        let text = read(&here().join(format!("pipeline/{name}/generated/provenance.json")));
        assert!(
            text.contains("\"pipeline_version\": \"n3.7-pipeline/1\""),
            "{name}"
        );
        assert!(
            text.contains("29cdbcb8594f94f38b7668919d18482af241a0f5fc3f449b7dfc96bec702cc57"),
            "{name}: source hash"
        );
        assert!(text.contains("NOT APPROVED"), "{name}");
        for key in [
            "\"video_index\"",
            "\"timestamp_s\"",
            "\"manual_edit\": false",
            "\"agent_generated\": false",
            "\"source_ai_generated\": true",
        ] {
            assert_eq!(text.matches(key).count(), frames, "{name}: {key}");
        }
    }
}

#[test]
fn the_speed_scenarios_follow_the_measured_stride() {
    let plan = sync_plan();
    assert!((plan.game_speed - PLAYER_RUN_SPEED).abs() < f32::EPSILON);
    assert!((plan.base_fps - 12.0).abs() < f32::EPSILON);
    // Measured from the video: well under the game's speed.
    assert!(plan.reference_speed > 100.0 && plan.reference_speed < 200.0);
    let metrics = read(&here().join("analysis/foot_metrics.json"));
    assert!(metrics.contains(&format!(
        "\"reference_speed_units_s\": {}",
        plan.reference_speed
    )));
    let ids: Vec<&str> = plan.scenarios.iter().map(|s| s.id.as_str()).collect();
    assert_eq!(ids, ["A", "B", "C", "D"]);
    let slide = |s: &Scenario| (s.speed - s.rate * plan.reference_speed).abs();
    let [a, b, c, d] = &plan.scenarios[..] else {
        panic!("four scenarios");
    };
    assert!(!a.name.is_empty());
    // A: authored timing at the game's speed slides; B, C, D do not.
    assert!((a.rate - 1.0).abs() < f32::EPSILON && (a.speed - PLAYER_RUN_SPEED).abs() < 0.1);
    assert!(slide(a) > 100.0);
    assert!((b.speed - PLAYER_RUN_SPEED).abs() < 0.1 && slide(b) < 1.0);
    assert!((c.rate - 1.0).abs() < f32::EPSILON && slide(c) < 1.0);
    assert!(d.rate > 1.0 && d.rate < b.rate && slide(d) < 1.0);
    assert_eq!(plan.run6_phase.len(), 6);
    assert!(plan.run6_phase.iter().any(|phase| phase == "air"));
}

#[test]
fn the_transition_and_contrast_data_fit_the_approved_run() {
    let plan: TransitionPlan = ron::from_str(&read(&here().join("generated/transition.ron")))
        .unwrap_or_else(|err| panic!("transition.ron: {err}"));
    assert!(plan.idle_hold > 0.0);
    assert!(plan.exit_frame < 6);
    assert!(plan.run_frames >= 6);
    assert_eq!(plan.start_clip, "run_start");
    let run6 = png_size(&root().join("art-src/n3-6-animation-proof/generated/run6.png"));
    for variant in ["run6_rim.png", "run6_boots.png"] {
        assert_eq!(
            png_size(&here().join("generated").join(variant)),
            run6,
            "{variant}"
        );
    }
    assert_eq!(
        png_size(&here().join("generated/backlight.png")),
        (CANVAS.0 as u32, CANVAS.1 as u32)
    );
}

#[test]
fn the_game_camera_and_run_speed_are_untouched() {
    let camera = read(&root().join("assets/data/camera.ron"));
    assert!(camera.contains("view_height: 405.0"));
    assert!((PLAYER_RUN_SPEED - 310.0).abs() < f32::EPSILON);
}
