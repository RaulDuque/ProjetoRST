//! Modular Character Engine 03 — Equipment Studio: designs as data.
//!
//! - the designs of `art-src/equipment-studio/items/` load, validate and
//!   round-trip through RON (saved independently of the character);
//! - each one is painted by the generic painter on every pose of the idle
//!   and of the six approved run frames: palette colours of its own
//!   materials or of its imported PNGs, binary alpha, on the cell grid;
//! - the two breastplates differ in structure (parts, silhouette), not only
//!   in colour;
//! - the sword keeps the imported PNG's pixels and sits on the fist;
//! - the 02 `ChestModel` paints through the adapter;
//! - editing a vertex in data changes the painted silhouette.
//!
//! `STUDIO_DUMP=1` also writes the 1x rasters to `target/equipment_studio/
//! preview/` (raw RGBA, for offline previews).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, Image, ImageSampler, ImageType};
use dark_castle_rpg::player::visual::rig::data::RigLab;
use dark_castle_rpg::player::visual::rig::design::{EquipmentDesign, PartSource, from_chest_model};
use dark_castle_rpg::player::visual::rig::paint::{PaintJob, Painted, RgbaImage, paint, pose_map};
use dark_castle_rpg::player::visual::rig::skeleton::{Joint, LocalPose, fit};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn studio() -> PathBuf {
    root().join("art-src/equipment-studio")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn lab() -> (RigLab, LocalPose) {
    let rig = RigLab::from_ron(&read(
        &root().join("art-src/modular-character-engine/stage/rig_lab.ron"),
    ))
    .unwrap_or_else(|e| panic!("{e}"));
    let rest = fit(&rig
        .rest()
        .and_then(|c| c.frame_joints(0))
        .unwrap_or_else(|| panic!("rest")));
    (rig, rest)
}

fn design(id: &str) -> EquipmentDesign {
    EquipmentDesign::from_ron(&read(&studio().join(format!("items/{id}.ron"))))
        .unwrap_or_else(|e| panic!("{id}: {e}"))
}

fn decode(path: &Path) -> RgbaImage {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let img = Image::from_buffer(
        &bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        true,
        ImageSampler::Default,
        RenderAssetUsages::default(),
    )
    .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let (w, h) = (img.width() as usize, img.height() as usize);
    let data = img.data.unwrap_or_default();
    RgbaImage {
        width: w,
        height: h,
        pixels: data.as_chunks::<4>().0.to_vec(),
    }
}

fn images_of(d: &EquipmentDesign) -> HashMap<String, RgbaImage> {
    d.parts
        .iter()
        .filter_map(|p| match &p.source {
            PartSource::Image { path, .. } => Some((path.clone(), decode(&studio().join(path)))),
            PartSource::Polygon { .. } => None,
        })
        .collect()
}

/// Paints a design on every frame of every clip.
fn paint_all(rig: &RigLab, rest: &LocalPose, d: &EquipmentDesign) -> Vec<(String, usize, Painted)> {
    let images = images_of(d);
    let lookup = |p: &str| images.get(p);
    let mut out = Vec::new();
    for (key, clip) in &rig.clips {
        for frame in 0..clip.frames.len() {
            let (map, _, turn) =
                pose_map(rig, rest, d, key, frame).unwrap_or_else(|| panic!("{key} {frame}"));
            let job = PaintJob {
                design: d,
                map,
                cell: clip.geometry(),
                clip: key,
                turn,
                images: &lookup,
            };
            let p = paint(&job);
            if std::env::var_os("STUDIO_DUMP").is_some() {
                let dir = root().join("target/equipment_studio/preview");
                let _ = std::fs::create_dir_all(&dir);
                let g = clip.geometry();
                let bytes: Vec<u8> = p.rgba.iter().flatten().copied().collect();
                let _ = std::fs::write(
                    dir.join(format!(
                        "{}_{key}_f{frame}_{}x{}.rgba",
                        d.id, g.width, g.height
                    )),
                    bytes,
                );
            }
            out.push((key.clone(), frame, p));
        }
    }
    out
}

fn palette(d: &EquipmentDesign) -> Vec<[u8; 4]> {
    let mut p: Vec<[u8; 4]> = d
        .materials
        .iter()
        .flat_map(|m| m.ramp)
        .map(|c| [c[0], c[1], c[2], 255])
        .collect();
    for img in images_of(d).values() {
        p.extend(img.pixels.iter().filter(|px| px[3] > 0));
    }
    p
}

#[test]
fn every_design_loads_validates_and_round_trips() {
    for id in ["gothic_cuirass", "leather_jerkin", "arming_sword"] {
        let d = design(id);
        let text = d.to_ron().unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(
            EquipmentDesign::from_ron(&text).unwrap_or_else(|e| panic!("{e}")),
            d,
            "{id}"
        );
        // Stored independently of the character: no reference to its art.
        let raw = read(&studio().join(format!("items/{id}.ron")));
        assert!(
            !raw.contains("n35_idle") && !raw.contains("n36_run6"),
            "{id}"
        );
    }
}

/// Every design of the folder, including the ones made in the studio.
fn all_ids() -> Vec<String> {
    let mut ids: Vec<String> = std::fs::read_dir(studio().join("items"))
        .unwrap_or_else(|e| panic!("{e}"))
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .filter_map(|n| n.strip_suffix(".ron").map(str::to_owned))
        .collect();
    ids.sort();
    ids
}

#[test]
fn the_folder_holds_the_two_breastplates_the_sword_and_a_studio_item() {
    let ids = all_ids();
    for id in ["arming_sword", "gothic_cuirass", "leather_jerkin"] {
        assert!(ids.iter().any(|i| i == id), "{id}");
    }
    // The third item was made with the studio's keys and saved by it.
    let made: Vec<&String> = ids
        .iter()
        .filter(|i| i.starts_with("studio_item_"))
        .collect();
    assert!(!made.is_empty(), "no item saved by the studio");
    for id in made {
        let d = design(id);
        assert_eq!(d.source, "created in the Equipment Studio");
        assert!(d.parts.len() >= 2, "{id}: polygon + imported buckle");
    }
}

#[test]
fn the_same_source_paints_every_pose_on_the_cell_grid() {
    let (rig, rest) = lab();
    for id in all_ids() {
        let id = id.as_str();
        let d = design(id);
        let pal = palette(&d);
        let painted = paint_all(&rig, &rest, &d);
        assert_eq!(painted.len(), 8, "{id}: idle 2 + run 6");
        for (clip, frame, p) in &painted {
            // A belt in the side-view idle is a 4-texel band: about 40 texels.
            assert!(p.opaque() > 30, "{id} {clip} #{frame}: {}", p.opaque());
            for px in p.rgba.iter().filter(|px| px[3] > 0) {
                assert_eq!(px[3], 255, "{id}: partial alpha");
                assert!(pal.contains(px), "{id} {clip} #{frame}: {px:?} off palette");
            }
        }
    }
}

#[test]
fn the_breastplates_differ_in_structure_not_only_colour() {
    let (rig, rest) = lab();
    let a = design("gothic_cuirass");
    let b = design("leather_jerkin");
    let ids = |d: &EquipmentDesign| d.parts.iter().map(|p| p.id.clone()).collect::<Vec<_>>();
    assert_ne!(ids(&a), ids(&b));
    assert!(a.parts.len() >= 6 && b.parts.len() >= 6);
    let pa = paint_all(&rig, &rest, &a);
    let pb = paint_all(&rig, &rest, &b);
    let mut ious = Vec::new();
    let mut seams = Vec::new();
    for ((clip, frame, x), (_, _, y)) in pa.iter().zip(&pb) {
        let w = rig.clips[clip].cell.0 as usize;
        let iou = |a: &[bool], b: &[bool]| {
            let inter = a.iter().zip(b).filter(|(a, b)| **a && **b).count();
            let union = a.iter().zip(b).filter(|(a, b)| **a || **b).count().max(1);
            inter as f32 / union as f32
        };
        let silhouette = |p: &Painted| p.rgba.iter().map(|px| px[3] > 0).collect::<Vec<_>>();
        // Internal structure: texels where two parts meet.
        let structure = |p: &Painted| {
            (0..p.owner.len())
                .map(|t| {
                    let Some(o) = p.owner[t] else { return false };
                    [t.wrapping_sub(1), t + 1, t.wrapping_sub(w), t + w]
                        .iter()
                        .any(|n| matches!(p.owner.get(*n), Some(Some(m)) if *m != o))
                })
                .collect::<Vec<_>>()
        };
        ious.push(iou(&silhouette(x), &silhouette(y)));
        let s = iou(&structure(x), &structure(y));
        seams.push(s);
        assert!(
            s < 0.5,
            "{clip} #{frame}: internal structures too alike ({s})"
        );
        let parts = |p: &Painted| {
            let mut v: Vec<usize> = p.owner.iter().flatten().copied().collect();
            v.sort_unstable();
            v.dedup();
            v.len()
        };
        assert!(
            parts(x) >= 5 && parts(y) >= 5,
            "{clip} #{frame}: visible parts"
        );
    }
    println!("A x B per pose: silhouette IoU {ious:.2?}; internal structure IoU {seams:.2?}");
    let mean = ious.iter().sum::<f32>() / ious.len() as f32;
    assert!(
        mean < 0.85,
        "mean silhouette IoU {mean}: the outlines are too alike"
    );
}

#[test]
fn the_imported_sword_keeps_its_pixels_on_the_fist() {
    let (rig, rest) = lab();
    let d = design("arming_sword");
    let images = images_of(&d);
    let blade: Vec<[u8; 4]> = images
        .values()
        .flat_map(|i| i.pixels.clone())
        .filter(|p| p[3] > 0)
        .collect();
    for (clip, frame, p) in paint_all(&rig, &rest, &d) {
        let shown = p.rgba.iter().filter(|px| blade.contains(px)).count();
        assert!(shown > 150, "{clip} #{frame}: {shown} blade texels");
        let c = &rig.clips[&clip];
        let fist = c
            .frame_joints(frame)
            .unwrap_or_else(|| panic!("j"))
            .get(Joint::HandNear);
        let (x, y) = c.geometry().to_texel(fist);
        let near = (-3..=3)
            .flat_map(|dy| (-3..=3).map(move |dx| (dx, dy)))
            .any(|(dx, dy)| {
                let (tx, ty) = (x as i64 + dx, y as i64 + dy);
                tx >= 0 && ty >= 0 && p.rgba[ty as usize * c.cell.0 as usize + tx as usize][3] > 0
            });
        assert!(near, "{clip} #{frame}: no sword at the fist");
    }
}

#[test]
fn the_02_breastplate_paints_through_the_adapter() {
    let (rig, rest) = lab();
    let d = from_chest_model("plate_cuirass_02", &rig.models["plate_cuirass"]);
    for (clip, frame, p) in paint_all(&rig, &rest, &d) {
        assert!(p.opaque() > 100, "{clip} #{frame}");
    }
}

#[test]
fn editing_a_vertex_in_data_changes_the_silhouette() {
    let (rig, rest) = lab();
    let mut d = design("leather_jerkin");
    let before = paint_all(&rig, &rest, &d)[3].2.opaque();
    if let PartSource::Polygon { points, .. } = &mut d.parts[0].source {
        points[3].0 += 0.4;
    }
    let after = paint_all(&rig, &rest, &d)[3].2.opaque();
    assert!(after > before, "{before} -> {after}");
}
