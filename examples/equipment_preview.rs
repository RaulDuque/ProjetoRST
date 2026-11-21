//! Headless preview of worn equipment (Modular Character Engine 05; a
//! tool, separate from the game).
//!
//! Composes the approved protagonist with any list of designs on every
//! played frame of every clip, through the same pure functions the
//! Equipment Studio shows (`rig::paint::compose_frame`, the settled loop of
//! flexible chains), and writes the 1x composites as raw RGBA:
//!
//! ```text
//! cargo run --example equipment_preview -- NAME [ITEM ...] [--isolated] [--items DIR]
//! # -> target/equipment_preview/NAME_<clip>_f<frame>_<W>x<H>.rgba
//! ```
//!
//! `art-src/equipment-studio/scripts/preview_sheet.py NAME` turns them into a
//! sheet (1x, 2x, 4x). No window, no screenshots: an art iteration loop.
//!
//! `-- --trace ITEM` prints, as JSON, the hem angle of the item's first
//! cloth chain at 60 Hz through idle (1 s) -> run (1.5 s) -> idle (2 s), the
//! body speed easing like the studio's (for the motion checks of
//! `scripts/run_tools_05.py`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, Image, ImageSampler, ImageType};
use dark_castle_rpg::core::resources::project_root;
use dark_castle_rpg::player::visual::playback::loop_frame;
use dark_castle_rpg::player::visual::rig::data::RigLab;
use dark_castle_rpg::player::visual::rig::design::{EquipmentDesign, PartSource};
use dark_castle_rpg::player::visual::rig::flex::{FlexState, angle_of};
use dark_castle_rpg::player::visual::rig::layers::BodyRegion;
use dark_castle_rpg::player::visual::rig::paint::{
    RgbaImage, WornItem, compose_frame, flex_input, paint_on_frame,
};
use dark_castle_rpg::player::visual::rig::skeleton::fit;

const TAG: &str = "[equipment_preview]";
const STUDIO: &str = "art-src/equipment-studio";
const LAB_STAGE: &str = "art-src/modular-character-engine/stage";
const APPROVED: &str = "art-src/n3-8-production-run/stage";
const OUT: &str = "target/equipment_preview";
/// Same easing as the studio's body speed (texels per second squared).
const BODY_ACCEL: f32 = 450.0;
const TRACE_DT: f32 = 1.0 / 60.0;

/// Hem angle of a design's first chain through idle -> run -> idle.
fn trace(rig: &RigLab, d: &EquipmentDesign) -> Result<String, String> {
    let spec = d.flex.first().ok_or("the design has no cloth chain")?;
    let plan = [("idle", 60), ("run", 90), ("idle", 120)];
    let mut state: Option<FlexState> = None;
    let mut speed = 0.0f32;
    let mut rows = Vec::new();
    let mut t = 0.0f32;
    for (clip, steps) in plan {
        let c = rig.clips.get(clip).ok_or("clip")?;
        let mut elapsed = 0.0f32;
        for _ in 0..steps {
            let frame = loop_frame(&c.durations(), elapsed);
            let change = BODY_ACCEL * TRACE_DT;
            speed += (c.ground_speed - speed).clamp(-change, change);
            let mut input = flex_input(rig, spec, clip, frame).ok_or("input")?;
            input.ground_speed = speed;
            let st = state.get_or_insert_with(|| FlexState::rest(spec, input.anchor));
            st.step(spec, input, TRACE_DT);
            let (a, b) = (st.points[0], st.points[st.points.len() - 1]);
            rows.push(format!(
                "{{\"t\": {t:.4}, \"clip\": \"{clip}\", \"speed\": {speed:.2}, \"hem\": {:.3}}}",
                angle_of((b.0 - a.0, b.1 - a.1))
            ));
            elapsed += TRACE_DT;
            t += TRACE_DT;
        }
    }
    Ok(format!("[{}]", rows.join(",\n")))
}

fn decode(path: &Path) -> Result<RgbaImage, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let img = Image::from_buffer(
        &bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        true,
        ImageSampler::Default,
        RenderAssetUsages::default(),
    )
    .map_err(|e| format!("{}: {e}", path.display()))?;
    let (width, height) = (img.width() as usize, img.height() as usize);
    let data = img.data.unwrap_or_default();
    Ok(RgbaImage {
        width,
        height,
        pixels: data.as_chunks::<4>().0.to_vec(),
    })
}

fn run() -> Result<(), String> {
    let root = project_root();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let isolated = args.iter().any(|a| a == "--isolated");
    let tracing = args.iter().any(|a| a == "--trace");
    let items_dir = args
        .iter()
        .position(|a| a == "--items")
        .and_then(|i| args.get(i + 1))
        .map_or_else(|| root.join(STUDIO).join("items"), PathBuf::from);
    let mut plain = args.iter().filter(|a| !a.starts_with("--")).cloned();
    let skip_dir = args
        .iter()
        .position(|a| a == "--items")
        .map(|i| args[i + 1].clone());
    let name = if tracing {
        "trace".to_owned()
    } else {
        plain
            .next()
            .ok_or("usage: equipment_preview NAME [ITEM ...]")?
    };
    let ids: Vec<String> = plain.filter(|a| Some(a) != skip_dir.as_ref()).collect();
    let rig = RigLab::from_ron(
        &std::fs::read_to_string(root.join(LAB_STAGE).join("rig_lab.ron"))
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let rest = fit(&rig
        .rest()
        .and_then(|c| c.frame_joints(0))
        .ok_or("rest pose missing")?);
    let designs: Vec<EquipmentDesign> = ids
        .iter()
        .map(|id| {
            let p = items_dir.join(format!("{id}.ron"));
            let text = std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?;
            EquipmentDesign::from_ron(&text).map_err(|e| format!("{id}: {e}"))
        })
        .collect::<Result<_, _>>()?;
    if tracing {
        let d = designs.first().ok_or("--trace needs an item")?;
        println!("{}", trace(&rig, d)?);
        return Ok(());
    }
    let mut images: HashMap<String, RgbaImage> = HashMap::new();
    for d in &designs {
        for p in &d.parts {
            if let PartSource::Image { path, .. } = &p.source
                && !images.contains_key(path)
            {
                images.insert(path.clone(), decode(&root.join(STUDIO).join(path))?);
            }
        }
    }
    let lookup = |p: &str| images.get(p);
    let out = root.join(OUT);
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let stage = root.join(LAB_STAGE);
    for (key, clip) in &rig.clips {
        let rel = clip.texture.trim_start_matches("approved://");
        let strip = decode(&root.join(APPROVED).join(rel))?;
        let bytes: Vec<u8> = strip.pixels.iter().flatten().copied().collect();
        let cut = clip
            .cut_layers(&bytes, strip.width, |p| {
                std::fs::read_to_string(stage.join(p)).ok()
            })
            .map_err(|e| e.to_string())?;
        let (w, h) = clip.cell;
        let n = w as usize * h as usize;
        let empty: Vec<(BodyRegion, Vec<u8>)> = BodyRegion::ALL
            .iter()
            .map(|r| (*r, vec![0u8; n * 4]))
            .collect();
        for (frame, f) in clip.frames.iter().enumerate() {
            let painted: Vec<_> = designs
                .iter()
                .map(|d| paint_on_frame(&rig, &rest, d, key, frame, &lookup))
                .collect::<Option<Vec<_>>>()
                .ok_or(format!("{key} #{frame}: a design does not fit this pose"))?;
            let items: Vec<WornItem<'_>> = designs
                .iter()
                .zip(&painted)
                .map(|(d, (p, depth))| WornItem {
                    design: d,
                    painted: p,
                    depth: *depth,
                })
                .collect();
            let body = if isolated {
                &empty
            } else {
                cut.iter()
                    .find(|s| s.source == f.source)
                    .map_or(&empty, |s| &s.layers)
            };
            let drop = if isolated { 0 } else { f.drop as usize };
            let px = compose_frame(body, drop, w as usize, &items);
            let file = out.join(format!("{name}_{key}_f{frame}_{w}x{h}.rgba"));
            let raw: Vec<u8> = px.iter().flatten().copied().collect();
            std::fs::write(&file, raw).map_err(|e| e.to_string())?;
        }
    }
    println!("{TAG} {name}: {} items -> {}", designs.len(), out.display());
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{TAG} {e}");
        std::process::exit(1);
    }
}
