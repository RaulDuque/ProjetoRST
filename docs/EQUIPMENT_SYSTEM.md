# Equipamentos e inventário

Código: `src/inventory/` (puro) — `items.rs`, `equipment.rs`, `inventory.rs`.
Dados: `assets/data/items/*.ron` (`rarities` + `items`). Testes: `tests/inventory_tests.rs`.

## Itens

`ItemDefinition`: `id`, `name`, `description`, `category`, `rarity`, `modifiers`
(`AttributeModifiers`: STR/CON/INT/LCK, HP/MP, ATK/DEF/MATK/MDEF, HIT, crítico,
velocidades, regeneração, esquiva, resistências), `consumable`, `special_effects`,
`requirements`, `weight`, `sell_value`, `max_stack`.

Categorias: Sword, Dagger, Axe, Spear, HeavyWeapon, Armor, Helmet, Gloves, Boots,
Shield, Ring, Amulet, Consumable, Material, QuestItem.

## Raridades configuráveis

Definidas em dados (`id`, `name`, `color`, `tier`). Itens com raridade inexistente
são rejeitados no carregamento. Protótipo: common, uncommon, rare, legendary.

## Slots

| Slot | Categorias aceitas |
|---|---|
| Weapon | Sword, Dagger, Axe, Spear, HeavyWeapon |
| Offhand | Shield |
| Head / Body / Gloves / Boots | Helmet / Armor / Gloves / Boots |
| Accessory1, Accessory2 | Ring, Amulet |

`Equipment::equip` valida slot e requisitos e devolve o item anterior;
`total_modifiers` soma os bônus, que entram na `CharacterSheet` (atributos recalculados).

## Inventário

`add` (empilha até `max_stack`), `remove`, `discard` (itens de missão não podem
ser descartados), `count`, `sort` (categoria → raridade → nome). Consumir e
inspecionar dependem da tela de inventário (roadmap, etapa 8).

## Drops

`drop_chance(base, LCK) = clamp(base · (1 + LCK · 0.02), 0, 1)`.
Ex.: base 15%, LCK 4 → 16,2%; LCK 50 → 30%.

## Equipamento inicial

`assets/data/character.ron`: Iron Sword (+8 ATK), Leather Jerkin (+4 DEF), 3× Minor Tonic.

## V2.0

- **Equipar e remover em jogo:** tela `I` → `EquipRequest` →
  `inventory::systems::apply_equip_requests` (único escritor). Regras puras
  em `inventory/loadout.rs`: o item sai da bolsa ao equipar e o anterior
  volta; recusas não mudam nada. Trava `EquipGate`: só em locomoção, nunca
  caído, em ação, com golpe vivo, transformado ou em transição.
- **Armas:** `WeaponClass` por categoria (`weapon_classes.ron`) e
  `weapon: (element, reach_bonus)` por item; o ataque básico usa o perfil.
- **Aparência:** `visual: Trail | Cape | Tint` (`inventory/appearance.rs`).
- **Raridade Epic** (tier 3); Legendary passou a tier 4.
- **Catálogo:** `equipment_v2.ron` (22 itens novos).
- **Formato do save inalterado.**
- Tudo em [`EQUIPMENT_EXPANSION_V2_0.md`](EQUIPMENT_EXPANSION_V2_0.md).
