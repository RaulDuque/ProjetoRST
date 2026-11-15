//! V2.1: NPCs, dialogues, shops, coins and the two-handed rule without the
//! engine (real data and small fixtures). ECS flows are in
//! `town_app_tests.rs`; the input-only route in `world_town_route_tests.rs`.

use std::collections::BTreeSet;
use std::path::Path;

use dark_castle_rpg::app::{check_npcs, load_world};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::economy::shop::{
    MAX_COINS, Purse, ShopDefinition, ShopError, ShopLibrary, ShopStock, Stock, TradeError, buy,
    sell,
};
use dark_castle_rpg::inventory::equipment::{Equipment, EquipmentSlot};
use dark_castle_rpg::inventory::inventory::Inventory;
use dark_castle_rpg::inventory::loadout::{EquipGate, LoadoutError, equip_from_bag};
use dark_castle_rpg::npc::definitions::{
    DialogueCondition, DialogueFacts, DialogueNode, NpcDefinition, NpcError, NpcLibrary,
};
use dark_castle_rpg::npc::dialogue::{Conversation, Step};
use dark_castle_rpg::player::attributes::BaseAttributes;
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::skills::unlock::RequirementContext;
use dark_castle_rpg::world::graph::{Ability, WorldDefinition};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn data() -> GameData {
    GameData::load(root()).unwrap_or_else(|e| panic!("{e}"))
}

fn no_facts() -> DialogueFacts {
    DialogueFacts::default()
}

fn shop(data: &GameData) -> &ShopDefinition {
    data.shops
        .get("hessa_outfitters")
        .unwrap_or_else(|| panic!("no shop"))
}

// ---- NPCs ------------------------------------------------------------------

#[test]
fn the_village_npcs_load_and_stand_in_their_rooms() {
    let mut data = data();
    let npcs: Vec<_> = data.npcs.npcs().collect();
    assert!((4..=6).contains(&npcs.len()), "{} NPCs", npcs.len());
    let world = load_world(root(), &mut data).unwrap_or_else(|e| panic!("{e}"));
    assert!(check_npcs(&world, &data).is_ok());
    for npc in data.npcs.npcs() {
        let room = world
            .room(&npc.room)
            .unwrap_or_else(|| panic!("{}", npc.id));
        assert_eq!(room.region, "ash_village", "{}", npc.id);
        assert!(
            data.npcs.greeting(&npc.id, &no_facts()).is_some(),
            "{}",
            npc.id
        );
    }
    // Exactly one merchant, with its shop.
    let merchants: Vec<_> = data.npcs.npcs().filter(|n| n.shop.is_some()).collect();
    assert_eq!(merchants.len(), 1);
}

fn npc(id: &str, greet: &str) -> NpcDefinition {
    ron::from_str(&format!(
        r#"(id: "{id}", name: "N", kind: Villager, room: "av_square", position: (500.0, 63.0),
            size: (26.0, 46.0), color: (1.0, 1.0, 1.0), greetings: [(node: "{greet}")])"#
    ))
    .unwrap_or_else(|e| panic!("{e}"))
}

fn node(id: &str, speaker: &str, rest: &str) -> DialogueNode {
    ron::from_str(&format!(
        r#"(id: "{id}", speaker: "{speaker}", lines: ["Hello."] {rest})"#
    ))
    .unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn invalid_npcs_and_dialogues_are_rejected() {
    // Duplicate NPC id.
    assert!(matches!(
        NpcLibrary::new([npc("a", "n"), npc("a", "n")], [node("n", "a", "")]),
        Err(NpcError::DuplicateId(_))
    ));
    // Duplicate node id.
    assert!(matches!(
        NpcLibrary::new([npc("a", "n")], [node("n", "a", ""), node("n", "a", "")]),
        Err(NpcError::DuplicateId(_))
    ));
    // Unknown greeting node, unknown Goto, OpenShop without shop, node of
    // another speaker, unknown speaker.
    let bad: Vec<(Vec<NpcDefinition>, Vec<DialogueNode>)> = vec![
        (vec![npc("a", "missing")], vec![node("n", "a", "")]),
        (
            vec![npc("a", "n")],
            vec![node(
                "n",
                "a",
                r#", choices: [(text: "x", action: Goto("nowhere"))]"#,
            )],
        ),
        (
            vec![npc("a", "n")],
            vec![node(
                "n",
                "a",
                r#", choices: [(text: "x", action: OpenShop)]"#,
            )],
        ),
        (vec![npc("a", "n"), npc("b", "n")], vec![node("n", "a", "")]),
        (
            vec![npc("a", "n")],
            vec![node("n", "a", ""), node("m", "ghost", "")],
        ),
    ];
    for (npcs, nodes) in bad {
        assert!(NpcLibrary::new(npcs, nodes).is_err());
    }
    // A choice list where every choice is conditional could trap the player.
    let trapped = node(
        "n",
        "a",
        r#", choices: [(text: "x", when: [Flag("f")], action: End)]"#,
    );
    assert!(NpcLibrary::new([npc("a", "n")], [trapped]).is_err());
}

#[test]
fn an_npc_in_an_unknown_room_or_outside_the_walls_is_refused() {
    let mut data = data();
    let world = load_world(root(), &mut data).unwrap_or_else(|e| panic!("{e}"));
    let make = |room: &str, x: f32| {
        let def: NpcDefinition = ron::from_str(&format!(
            r#"(id: "z", name: "Z", kind: Villager, room: "{room}", position: ({x}, 63.0),
                size: (26.0, 46.0), color: (1.0, 1.0, 1.0), greetings: [(node: "zn")])"#
        ))
        .unwrap_or_else(|e| panic!("{e}"));
        def
    };
    for (room, x) in [("no_such_room", 500.0), ("av_square", 5000.0)] {
        let mut copy = data.clone();
        copy.npcs = NpcLibrary::new([make(room, x)], [node("zn", "z", "")])
            .unwrap_or_else(|e| panic!("{e}"));
        assert!(check_npcs(&world, &copy).is_err(), "{room} {x}");
    }
    let _: &WorldDefinition = &world;
}

// ---- Dialogues -------------------------------------------------------------

#[test]
fn greetings_follow_progress_and_conversations_walk_their_nodes() {
    let data = data();
    let lib = &data.npcs;
    // Bram: before and after the Relic of the Veil.
    let mut facts = no_facts();
    assert_eq!(
        lib.greeting("bram_road_warden", &facts)
            .map(|n| n.id.as_str()),
        Some("bram_intro")
    );
    facts.capabilities.abilities.insert(Ability::VeilStep);
    assert_eq!(
        lib.greeting("bram_road_warden", &facts)
            .map(|n| n.id.as_str()),
        Some("bram_after_relic")
    );
    // Wenna: region visited; the pilgrim: room visited.
    let mut facts = no_facts();
    facts.visited_regions.insert("forgotten_galleries".into());
    facts.visited_rooms.insert("vc_veiled_well".into());
    assert_eq!(
        lib.greeting("old_wenna", &facts).map(|n| n.id.as_str()),
        Some("wenna_galleries")
    );
    assert_eq!(
        lib.greeting("hooded_pilgrim", &facts)
            .map(|n| n.id.as_str()),
        Some("pilgrim_well_seen")
    );
    // Bram's intro: two lines, then two choices; "ask" goes to a node whose
    // `next` returns to the intro; "Farewell" ends.
    let facts = no_facts();
    let mut c = Conversation::start("bram_road_warden", lib, &facts)
        .unwrap_or_else(|| panic!("no conversation"));
    assert!(
        c.choices(lib, &facts).is_empty(),
        "no choice before the last line"
    );
    assert_eq!(c.confirm(lib, &facts), Step::Continue);
    assert_eq!(c.choices(lib, &facts).len(), 2);
    assert_eq!(c.confirm(lib, &facts), Step::Continue);
    assert_eq!(c.node, "bram_castle");
    assert_eq!(c.confirm(lib, &facts), Step::Continue);
    assert_eq!(c.confirm(lib, &facts), Step::Continue);
    assert_eq!(c.node, "bram_intro");
    c.confirm(lib, &facts);
    c.move_cursor(1, lib, &facts);
    c.move_cursor(5, lib, &facts);
    assert_eq!(c.cursor, 1, "the cursor stays inside the choices");
    assert_eq!(c.confirm(lib, &facts), Step::End);
    // Hessa: the first choice opens the shop.
    let mut h = Conversation::start("hessa_outfitter", lib, &facts)
        .unwrap_or_else(|| panic!("no conversation"));
    assert_eq!(h.confirm(lib, &facts), Step::OpenShop);
}

#[test]
fn conditions_are_a_closed_typed_set() {
    let mut facts = no_facts();
    facts.items.insert("watch_pike".into());
    facts.capabilities.flags.insert("fg_hall_gate_open".into());
    assert!(DialogueCondition::Item("watch_pike".into()).holds(&facts));
    assert!(DialogueCondition::Flag("fg_hall_gate_open".into()).holds(&facts));
    assert!(!DialogueCondition::Ability(Ability::AstralKey).holds(&facts));
    // Anything else is not even parsed: there is no script language.
    assert!(ron::from_str::<DialogueCondition>(r#"Run("rm -rf /")"#).is_err());
}

// ---- Shops and coins -------------------------------------------------------

#[test]
fn shop_data_refuses_coin_loops_and_invalid_entries() {
    let data = data();
    for def in data.shops.iter() {
        for entry in &def.entries {
            let item = data
                .items
                .get(&entry.item)
                .unwrap_or_else(|| panic!("{}", entry.item));
            assert!(entry.price > item.sell_value, "{}", entry.item);
        }
    }
    let bad = |entries: &str| -> Result<ShopLibrary, ShopError> {
        let def: ShopDefinition = ron::from_str(&format!(
            r#"(id: "s", name: "S", keeper: "k", buys: true, entries: [{entries}])"#
        ))
        .unwrap_or_else(|e| panic!("{e}"));
        ShopLibrary::new([def], &data.items)
    };
    for entries in [
        r#"(item: "iron_sword", price: 0, stock: Unlimited)"#,
        r#"(item: "iron_sword", price: 40, stock: Unlimited)"#,
        r#"(item: "iron_sword", price: 999999, stock: Unlimited)"#,
        r#"(item: "iron_sword", price: 90, stock: Limited(0))"#,
        r#"(item: "nothing", price: 90, stock: Unlimited)"#,
        r#"(item: "iron_sword", price: 90, stock: Unlimited), (item: "iron_sword", price: 95, stock: Unlimited)"#,
    ] {
        assert!(bad(entries).is_err(), "{entries}");
    }
    assert!(bad(r#"(item: "iron_sword", price: 41, stock: Unlimited)"#).is_ok());
    // An unknown field is an error, not ignored.
    assert!(
        ron::from_str::<ShopDefinition>(
            r#"(id: "s", name: "S", keeper: "k", entries: [], discount: 2)"#
        )
        .is_err()
    );
}

#[test]
fn purse_never_overflows_or_goes_negative() {
    let purse = Purse::new(u32::MAX);
    assert_eq!(purse.coins(), MAX_COINS);
    assert_eq!(purse.checked_add(1), None);
    assert_eq!(Purse::new(5).checked_spend(6), None);
    assert_eq!(Purse::new(5).checked_spend(5), Some(Purse::new(0)));
}

fn bag(data: &GameData, items: &[&str]) -> Inventory {
    let mut bag = Inventory::default();
    for id in items {
        bag.add(id, 1, &data.items)
            .unwrap_or_else(|e| panic!("{e}"));
    }
    bag
}

#[test]
fn buying_is_all_or_nothing_and_limited_stock_runs_out_once() {
    let data = data();
    let shop = shop(&data);
    let facts = no_facts();
    let mut stock = ShopStock::default();
    let mut purse = Purse::new(300);
    let mut inv = bag(&data, &[]);
    // Limited(1): bought once, then sold out (a second buy in the same frame
    // or later changes nothing).
    assert_eq!(
        buy(
            shop,
            "quiet_soles",
            &mut stock,
            &mut purse,
            &mut inv,
            &data.items,
            &facts
        ),
        Ok(260)
    );
    assert_eq!(purse.coins(), 40);
    let before = (purse, inv.clone(), stock.clone());
    assert_eq!(
        buy(
            shop,
            "quiet_soles",
            &mut stock,
            &mut purse,
            &mut inv,
            &data.items,
            &facts
        ),
        Err(TradeError::SoldOut)
    );
    // Not enough coins.
    assert!(matches!(
        buy(
            shop,
            "gutter_knife",
            &mut stock,
            &mut purse,
            &mut inv,
            &data.items,
            &facts
        ),
        Err(TradeError::NotEnoughCoins { have: 40, need: 80 })
    ));
    // Hidden entry (needs the relic) and items not sold here.
    for item in ["still_water_amulet", "duskwater_saber", "no_item"] {
        assert_eq!(
            buy(
                shop,
                item,
                &mut stock,
                &mut purse,
                &mut inv,
                &data.items,
                &facts
            ),
            Err(TradeError::NotForSale),
            "{item}"
        );
    }
    assert_eq!((purse, inv.clone(), stock.clone()), before);
    assert_eq!(inv.count("quiet_soles"), 1);
    // With the relic the amulet appears.
    let mut relic = no_facts();
    relic.capabilities.abilities.insert(Ability::VeilStep);
    let mut rich = Purse::new(1000);
    assert!(
        buy(
            shop,
            "still_water_amulet",
            &mut stock,
            &mut rich,
            &mut inv,
            &data.items,
            &relic
        )
        .is_ok()
    );
}

#[test]
fn selling_follows_the_rules_and_buy_sell_loops_lose_coins() {
    let data = data();
    let shop = shop(&data);
    let facts = no_facts();
    let mut equipment = Equipment::default();
    let ctx_base = BaseAttributes::default();
    let empty = BTreeSet::new();
    let ctx = RequirementContext {
        level: 1,
        attributes: &ctx_base,
        owned_items: &empty,
        progress_flags: &empty,
    };
    let sword = data
        .items
        .get("iron_sword")
        .unwrap_or_else(|| panic!("sword"));
    equipment
        .equip(EquipmentSlot::Weapon, sword, &ctx)
        .unwrap_or_else(|e| panic!("{e}"));
    let mut inv = bag(&data, &["minor_tonic", "minor_tonic"]);
    let mut purse = Purse::new(0);
    // Equipped items are not in the bag.
    assert_eq!(
        sell(
            shop,
            "iron_sword",
            &mut purse,
            &mut inv,
            &equipment,
            &data.items
        ),
        Err(TradeError::NotOwned)
    );
    // Repeated sales: one unit each, then none left.
    assert_eq!(
        sell(
            shop,
            "minor_tonic",
            &mut purse,
            &mut inv,
            &equipment,
            &data.items
        ),
        Ok(10)
    );
    assert_eq!(
        sell(
            shop,
            "minor_tonic",
            &mut purse,
            &mut inv,
            &equipment,
            &data.items
        ),
        Ok(10)
    );
    assert_eq!(
        sell(
            shop,
            "minor_tonic",
            &mut purse,
            &mut inv,
            &equipment,
            &data.items
        ),
        Err(TradeError::NotOwned)
    );
    assert_eq!(purse.coins(), 20);
    // Last weapon: with nothing equipped, the only weapon cannot be sold.
    let mut bare = Equipment::default();
    let mut one = bag(&data, &["iron_sword"]);
    assert_eq!(
        sell(shop, "iron_sword", &mut purse, &mut one, &bare, &data.items),
        Err(TradeError::LastWeapon)
    );
    bare = equipment.clone();
    assert!(sell(shop, "iron_sword", &mut purse, &mut one, &bare, &data.items).is_ok());
    // Purse full: refused, the item stays.
    let mut full = Purse::new(MAX_COINS);
    let mut tonic = bag(&data, &["minor_tonic"]);
    assert_eq!(
        sell(
            shop,
            "minor_tonic",
            &mut full,
            &mut tonic,
            &equipment,
            &data.items
        ),
        Err(TradeError::PurseFull)
    );
    assert_eq!(tonic.count("minor_tonic"), 1);
    // Buy and sell back the same item 20 times: always fewer coins.
    let mut stock = ShopStock::default();
    let mut coins = Purse::new(10_000);
    let mut inv = bag(&data, &[]);
    for _ in 0..20 {
        buy(
            shop,
            "gutter_knife",
            &mut stock,
            &mut coins,
            &mut inv,
            &data.items,
            &facts,
        )
        .unwrap_or_else(|e| panic!("{e}"));
        sell(
            shop,
            "gutter_knife",
            &mut coins,
            &mut inv,
            &equipment,
            &data.items,
        )
        .unwrap_or_else(|e| panic!("{e}"));
    }
    assert_eq!(coins.coins(), 10_000 - 20 * (80 - 35));
}

// ---- Two hands ---------------------------------------------------------------

#[test]
fn two_handed_weapons_and_off_hand_items_exclude_each_other() {
    let data = data();
    let base = BaseAttributes::default();
    let empty = BTreeSet::new();
    let ctx = RequirementContext {
        level: 1,
        attributes: &base,
        owned_items: &empty,
        progress_flags: &empty,
    };
    let gate = EquipGate {
        state: PlayerState::Idle,
        transformed: false,
        in_transition: false,
        strike_active: false,
    };
    assert!(data.items.is_two_handed("watch_pike"));
    assert!(data.items.is_two_handed("bell_maul"));
    assert!(!data.items.is_two_handed("iron_sword"));
    assert!(!data.items.is_two_handed("gutter_knife"));
    // Shield first: the pike is refused, nothing moves.
    let mut inv = bag(&data, &["watch_buckler", "watch_pike", "bell_maul"]);
    let mut eq = Equipment::default();
    equip_from_bag(
        &mut inv,
        &mut eq,
        EquipmentSlot::Offhand,
        "watch_buckler",
        &data.items,
        &ctx,
        &gate,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let before = (inv.clone(), eq.clone());
    assert_eq!(
        equip_from_bag(
            &mut inv,
            &mut eq,
            EquipmentSlot::Weapon,
            "watch_pike",
            &data.items,
            &ctx,
            &gate
        ),
        Err(LoadoutError::OffhandInUse)
    );
    assert_eq!((inv.clone(), eq.clone()), before);
    // Two-handed first: the shield is refused.
    let mut inv = bag(&data, &["watch_buckler", "bell_maul"]);
    let mut eq = Equipment::default();
    equip_from_bag(
        &mut inv,
        &mut eq,
        EquipmentSlot::Weapon,
        "bell_maul",
        &data.items,
        &ctx,
        &gate,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        equip_from_bag(
            &mut inv,
            &mut eq,
            EquipmentSlot::Offhand,
            "watch_buckler",
            &data.items,
            &ctx,
            &gate
        ),
        Err(LoadoutError::TwoHandedWeapon)
    );
    assert_eq!(eq.get(EquipmentSlot::Offhand), None);
    assert_eq!(inv.count("watch_buckler"), 1);
    // Stock helper: unlimited entries report no count.
    let shop = shop(&data);
    let stock = ShopStock::default();
    let tonic = shop.entries.iter().find(|e| e.item == "minor_tonic");
    assert!(tonic.is_some_and(|e| e.stock == Stock::Unlimited && stock.left(shop, e).is_none()));
}
