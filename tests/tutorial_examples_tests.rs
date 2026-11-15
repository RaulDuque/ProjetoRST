//! V2.1: the examples of the owner's tutorials are real.
//!
//! Every ```ron block of the tutorials that starts with
//! `// exemplo-verificado: <Kind>` is parsed into the game's own type. The
//! complete example (the Ashen Crypt region, its door, items, enemy, NPC,
//! dialogue and shop) is then added to a **copy** of the shipped data and
//! world (nothing under `assets/` changes), validated like `app::load_world`
//! does, and walked with real keys from the village cottage.
//!
//! V2.2: the official tutorials are `docs/tutorias/` (the owner copies them
//! to `Md-usario/tutorias/` by hand); only `docs/tutorias/` is read.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::input::InputPlugin as BevyInputPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::check_npcs;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::economy::shop::{ShopDefinition, ShopEntry, ShopLibrary};
use dark_castle_rpg::enemies::definitions::EnemyDefinition;
use dark_castle_rpg::inventory::appearance::ItemVisual;
use dark_castle_rpg::inventory::items::{ItemCatalog, ItemDataFile, ItemDefinition};
use dark_castle_rpg::inventory::weapons::WeaponClass;
use dark_castle_rpg::npc::art::NpcLook;
use dark_castle_rpg::npc::definitions::{DialogueNode, NpcDefinition, NpcLibrary};
use dark_castle_rpg::player::visual::sockets::FrameSockets;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::camera_rules::CameraSettings;
use dark_castle_rpg::world::exploration_systems::{activate_world, install_world};
use dark_castle_rpg::world::graph::{
    Door, Grant, Pickup, RegionFile, SecretDef, Veil, WorldDefinition, WorldObject, WorldRoom,
};
use dark_castle_rpg::world::rooms::{EnemySpawn, Platform};
use dark_castle_rpg::world::scenery::{Decor, RegionVisual};
use dark_castle_rpg::world::validation::validate;

#[path = "support/route_driver.rs"]
mod route_driver;

use route_driver::{Dir, Expect, RouteDriver, RouteStatus, Step};

const MARK: &str = "// exemplo-verificado:";

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// The official tutorials (V2.2): `docs/tutorias/`. The owner copies them
/// by hand to `Md-usario/tutorias/`, which is never read or written here
/// (the copy may lag behind until the owner syncs it).
fn tutorial_dir() -> PathBuf {
    root().join("docs/tutorias")
}

/// (file, kind, body) of every verified block.
fn blocks() -> Vec<(String, String, String)> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(tutorial_dir())
        .unwrap_or_else(|e| panic!("{e}"))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .collect();
    files.sort();
    let mut out = Vec::new();
    for file in files {
        let text = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{e}"));
        let name = file
            .file_name()
            .map_or(String::new(), |n| n.to_string_lossy().into_owned());
        let mut rest = text.as_str();
        while let Some(start) = rest.find("```ron\n") {
            let after = &rest[start + 7..];
            let end = after.find("```").unwrap_or(after.len());
            let block = &after[..end];
            rest = &after[end..];
            let first = block.lines().next().unwrap_or("").trim();
            if let Some(kind) = first.strip_prefix(MARK) {
                out.push((name.clone(), kind.trim().to_owned(), block.to_owned()));
            }
        }
    }
    out
}

fn parse<T: for<'de> serde::Deserialize<'de>>(file: &str, kind: &str, body: &str) -> T {
    // One-line entries end with the comma they need inside their list.
    let body = body.trim_end().trim_end_matches(',');
    ron::from_str(body).unwrap_or_else(|e| panic!("{file}: block `{kind}` does not parse: {e}"))
}

/// What the complete example adds to the game.
#[derive(Default)]
struct Example {
    regions: Vec<RegionFile>,
    doors: Vec<(String, Door)>,
    platforms: Vec<(String, Platform)>,
    items: Vec<ItemDataFile>,
    enemies: Vec<EnemyDefinition>,
    npcs: Vec<NpcDefinition>,
    nodes: Vec<DialogueNode>,
    shops: Vec<ShopDefinition>,
}

fn example() -> (Example, usize) {
    let mut ex = Example::default();
    let all = blocks();
    for (file, kind, body) in &all {
        let (kind_name, room) = kind
            .split_once('@')
            .map_or((kind.as_str(), None), |(k, r)| (k, Some(r.to_owned())));
        match kind_name {
            "Regiao" => ex.regions.push(parse(file, kind, body)),
            "Porta" => ex
                .doors
                .push((room.unwrap_or_default(), parse(file, kind, body))),
            "Plataforma" => ex
                .platforms
                .push((room.unwrap_or_default(), parse(file, kind, body))),
            "Itens" => ex.items.push(parse(file, kind, body)),
            "Inimigos" => ex
                .enemies
                .extend(parse::<Vec<EnemyDefinition>>(file, kind, body)),
            "Npcs" => ex
                .npcs
                .extend(parse::<Vec<NpcDefinition>>(file, kind, body)),
            "Dialogos" => ex
                .nodes
                .extend(parse::<Vec<DialogueNode>>(file, kind, body)),
            "Lojas" => ex
                .shops
                .extend(parse::<Vec<ShopDefinition>>(file, kind, body)),
            // Excerpts: they only have to parse.
            "Sala" => {
                let _: WorldRoom = parse(file, kind, body);
            }
            "Objeto" => {
                let _: WorldObject = parse(file, kind, body);
            }
            "Veu" => {
                let _: Veil = parse(file, kind, body);
            }
            "Coleta" => {
                let _: Pickup = parse(file, kind, body);
            }
            "Segredo" => {
                let _: SecretDef = parse(file, kind, body);
            }
            "SpawnInimigo" => {
                let _: EnemySpawn = parse(file, kind, body);
            }
            "Item" => {
                let _: ItemDefinition = parse(file, kind, body);
            }
            "ClasseArma" => {
                let _: WeaponClass = parse(file, kind, body);
            }
            "Npc" => {
                let _: NpcDefinition = parse(file, kind, body);
            }
            "Dialogo" => {
                let _: DialogueNode = parse(file, kind, body);
            }
            "Loja" => {
                let _: ShopDefinition = parse(file, kind, body);
            }
            "Mercadoria" => {
                let _: ShopEntry = parse(file, kind, body);
            }
            // V2.2 excerpts: item visual, NPC look, one frame of sockets.
            "Visual" => {
                let _: ItemVisual = parse(file, kind, body);
            }
            "AparenciaNpc" => {
                let _: NpcLook = parse(file, kind, body);
            }
            "QuadroAncoras" => {
                let _: FrameSockets = parse(file, kind, body);
            }
            // V2.3 excerpts: camera settings (validated), decoration, region look.
            "Camera" => {
                let body = body.trim_end().trim_end_matches(',');
                CameraSettings::from_ron(body)
                    .unwrap_or_else(|e| panic!("{file}: block `{kind}`: {e}"));
            }
            "Decoracao" => {
                let _: Decor = parse(file, kind, body);
            }
            "VisualRegiao" => {
                let v: RegionVisual = parse(file, kind, body);
                v.validate("tutorial")
                    .unwrap_or_else(|e| panic!("{file}: block `{kind}`: {e}"));
            }
            other => panic!("{file}: unknown block kind `{other}`"),
        }
    }
    (ex, all.len())
}

fn item_files() -> Vec<ItemDataFile> {
    let dir = root().join("assets/data/items");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{e}"))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ron"))
        .collect();
    paths.sort();
    paths
        .iter()
        .map(|p| {
            ron::from_str(&std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{e}")))
                .unwrap_or_else(|e| panic!("{}: {e}", p.display()))
        })
        .collect()
}

/// The shipped data and world plus the example, checked like the game does.
fn with_example() -> (GameData, WorldDefinition) {
    let (ex, count) = example();
    assert!(count >= 20, "only {count} verified blocks");
    let mut data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    // Items: the shipped files plus the example's.
    let mut files = item_files();
    files.extend(ex.items);
    data.items = ItemCatalog::from_data(files).unwrap_or_else(|e| panic!("items: {e}"));
    assert!(data.items.weapons_without_class().is_empty());
    for enemy in ex.enemies {
        data.enemies.insert(enemy.id.clone(), enemy);
    }
    // NPCs, nodes and shops: shipped plus the example's.
    let mut npcs: Vec<NpcDefinition> = data.npcs.npcs().cloned().collect();
    let shipped_nodes: Vec<DialogueNode> = shipped_nodes(&data.npcs);
    npcs.extend(ex.npcs);
    let mut nodes = shipped_nodes;
    nodes.extend(ex.nodes);
    data.npcs = NpcLibrary::new(npcs, nodes).unwrap_or_else(|e| panic!("npcs: {e}"));
    let mut shops: Vec<ShopDefinition> = data.shops.iter().cloned().collect();
    shops.extend(ex.shops);
    data.shops = ShopLibrary::new(shops, &data.items).unwrap_or_else(|e| panic!("shops: {e}"));
    for npc in data.npcs.npcs() {
        if let Some(shop) = &npc.shop {
            assert!(
                data.shops.get(shop).is_some_and(|s| s.keeper == npc.id),
                "{} / {shop}",
                npc.id
            );
        }
    }
    // World: shipped plus the example's region and patches.
    let mut world =
        WorldDefinition::load_dir(&root().join("assets/world")).unwrap_or_else(|e| panic!("{e}"));
    for region in ex.regions {
        world
            .regions
            .insert(region.region.id.clone(), region.region.clone());
        for room in region.rooms {
            assert!(!world.rooms.contains_key(&room.id), "{} exists", room.id);
            world.rooms.insert(room.id.clone(), room);
        }
    }
    for (room, door) in ex.doors {
        let doors = &mut world
            .rooms
            .get_mut(&room)
            .unwrap_or_else(|| panic!("no room {room}"))
            .doors;
        // The same door shown in two tutorials is added once.
        if !doors.contains(&door) {
            doors.push(door);
        }
    }
    for (room, platform) in ex.platforms {
        world
            .rooms
            .get_mut(&room)
            .unwrap_or_else(|| panic!("no room {room}"))
            .platforms
            .push(platform);
    }
    let report = validate(&world);
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    for room in world.rooms.values() {
        for spawn in &room.enemies {
            assert!(data.enemies.contains_key(&spawn.enemy), "{}", spawn.enemy);
        }
        let grants = room
            .pickups
            .iter()
            .map(|p| Some(p.grants.clone()))
            .chain(room.objects.iter().map(|o| o.grant()));
        for grant in grants.flatten() {
            if let Grant::Item(item) = grant {
                assert!(data.items.get(&item).is_some(), "{item}");
            }
        }
    }
    check_npcs(&world, &data).unwrap_or_else(|e| panic!("{e}"));
    (data, world)
}

fn shipped_nodes(lib: &NpcLibrary) -> Vec<DialogueNode> {
    let mut out: Vec<DialogueNode> = Vec::new();
    fn walk(lib: &NpcLibrary, id: &str, out: &mut Vec<DialogueNode>) {
        if out.iter().any(|n| n.id == id) {
            return;
        }
        let Some(node) = lib.node(id) else {
            return;
        };
        out.push(node.clone());
        if let Some(next) = &node.next {
            walk(lib, next, out);
        }
        for choice in &node.choices {
            if let dark_castle_rpg::npc::definitions::ChoiceAction::Goto(t) = &choice.action {
                walk(lib, t, out);
            }
        }
    }
    for npc in lib.npcs() {
        for g in &npc.greetings {
            walk(lib, &g.node, &mut out);
        }
    }
    out
}

#[test]
fn every_tutorial_follows_the_template_and_every_block_parses() {
    let dir = tutorial_dir();
    let required = [
        "00_LEIA_PRIMEIRO.md",
        "01_CRIAR_SALAS.md",
        "02_EDITAR_SALAS.md",
        "03_CRIAR_REGIOES_E_FASES.md",
        "04_CONECTAR_SALAS_E_PORTAS.md",
        "05_CRIAR_ITENS.md",
        "06_CRIAR_ARMAS.md",
        "07_CRIAR_ARMADURAS_E_ACESSORIOS.md",
        "08_CRIAR_BAUS.md",
        "09_CRIAR_SALAS_SECRETAS.md",
        "10_CRIAR_PAREDES_QUEBRAVEIS.md",
        "11_CRIAR_ALAVANCAS_E_MECANISMOS.md",
        "12_CRIAR_RELIQUIAS.md",
        "13_CRIAR_BARREIRAS_ESPECTRAIS.md",
        "14_ADICIONAR_INIMIGOS.md",
        "15_EDITAR_MAPA_E_EXPLORACAO.md",
        "16_TESTAR_CONTEUDO.md",
        "17_ERROS_COMUNS.md",
        "18_CRIAR_NPCS.md",
        "19_CRIAR_DIALOGOS.md",
        "20_CRIAR_LOJAS.md",
        "21_CADASTRAR_MERCADORIAS.md",
        "22_CONFIGURAR_PRECOS.md",
        "23_CRIAR_CIDADE.md",
        "24_CRIAR_ARMA_COM_APARENCIA.md",
        "25_CONFIGURAR_POSICAO_DA_ARMA.md",
        "26_CRIAR_SPRITES_DE_NPCS.md",
        "27_ASSOCIAR_SPRITES_A_NPCS.md",
        "28_ADICIONAR_MARCADORES_NO_MAPA.md",
        "29_TESTAR_EQUIPAMENTOS_VISUAIS.md",
        "30_CONFIGURAR_CAMERA.md",
        "31_CONFIGURAR_ZOOM.md",
        "32_DECORAR_UMA_SALA.md",
        "33_CRIAR_FUNDOS_EM_PARALLAX.md",
        "34_CONFIGURAR_ATMOSFERA_DE_REGIAO.md",
        "35_TESTAR_CENARIOS_VISUAIS.md",
    ];
    let sections = [
        "## 1. O que vamos fazer?",
        "## 2. O que você precisa saber antes?",
        "## 3. Quais arquivos vamos utilizar?",
        "## 4. Passo a passo",
        "## 5. Exemplo completo",
        "## 6. Como testar?",
        "## 7. Como editar depois?",
        "## 8. Problemas comuns",
        "## 9. Como desfazer com segurança?",
    ];
    for name in required {
        let text =
            std::fs::read_to_string(dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"));
        for section in sections {
            assert!(text.contains(section), "{name}: missing `{section}`");
        }
        // Never recommend destructive git commands as a way to undo.
        for bad in ["git clean -", "git reset --hard ", "git checkout -- ."] {
            assert!(
                !text.contains(&format!("`{bad}")) || text.contains("Não use"),
                "{name}: recommends {bad}"
            );
        }
        // Every relative link points to an existing tutorial.
        for link in text.split("](").skip(1) {
            let target = link.split(')').next().unwrap_or("");
            if target.ends_with(".md") && !target.contains('/') {
                assert!(dir.join(target).is_file(), "{name}: broken link {target}");
            }
        }
    }
    let (_, count) = example();
    assert!(count >= 20);
}

#[test]
fn the_complete_example_passes_the_game_checks() {
    let (data, world) = with_example();
    assert!(world.room("ac_secret_niche").is_some());
    assert!(data.items.get("crypt_cleaver").is_some());
    assert!(data.npcs.npc("morrow_crypt_keeper").is_some());
}

#[test]
fn the_complete_example_is_playable_with_real_keys() {
    let (mut data, world) = with_example();
    install_world(&mut data, &world);
    let saves = SaveStorage::new(PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("tutorial_saves"));
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            16,
        )))
        .insert_resource(data)
        .add_plugins(GamePlugin)
        .insert_resource(SaveSlots(saves));
    activate_world(&mut app, world);
    for _ in 0..4 {
        app.update();
    }
    // Test setup: start in the village cottage (the way there is the town
    // route's job, tests/world_town_route_tests.rs).
    app.world_mut().write_message(RoomRequest {
        room: "av_cottage".into(),
        arrival: Arrival::Door("to_smithy".into()),
    });
    for _ in 0..4 {
        app.update();
    }
    use Dir::Right;
    use Step::*;
    let d = KeyCode::KeyD;
    let steps = vec![
        WalkUntilRoom(Right, "ac_crypt_stair"),
        Wait(10),
        Fight,
        WalkTo(308.0),
        Tap(d),
        Wait(5),
        Check(Expect::Flag("ac_ossuary_gate_open")),
        WalkTo(540.0),
        Tap(d),
        Wait(3),
        Check(Expect::Talking(true)),
        Tap(KeyCode::Enter),
        Wait(3),
        Check(Expect::ShopOpen(true)),
        Tap(KeyCode::Escape),
        Wait(3),
        Check(Expect::ShopOpen(false)),
        WalkUntilRoom(Right, "ac_bone_ossuary"),
        WalkTo(400.0),
        Wait(5),
        Check(Expect::Ability(
            dark_castle_rpg::world::graph::Ability::WallBreaker,
        )),
        WalkTo(664.0),
        Tap(d),
        Wait(5),
        Check(Expect::HasItem("crypt_cleaver", 1)),
        WalkTo(1180.0),
        Tap(KeyCode::KeyX),
        Wait(15),
        Check(Expect::Secret("ac_cracked_wall")),
        WalkUntilRoom(Right, "ac_secret_niche"),
        WalkTo(624.0),
        Tap(d),
        Wait(5),
        Check(Expect::HasItem("gravecloth_mantle", 1)),
        WalkTo(924.0),
        Tap(d),
        Wait(5),
        Check(Expect::HasItem("bone_charm", 1)),
    ];
    route_driver::install(&mut app, steps);
    for _ in 0..20_000 {
        app.update();
        if app.world().resource::<RouteDriver>().is_done() {
            break;
        }
    }
    let driver = app.world().resource::<RouteDriver>();
    let report = driver.report();
    assert_eq!(driver.status, RouteStatus::Passed, "\n{report}");
    assert_eq!(driver.artificial_repositions, 0, "\n{report}");
}
