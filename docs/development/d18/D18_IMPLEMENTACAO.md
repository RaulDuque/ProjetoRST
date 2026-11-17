# D18 — Implementação

## Arquivos novos

| Arquivo | Papel |
|---|---|
| `src/inventory/consume.rs` | Regras puras + testes unitários |
| `tests/consumable_app_tests.rs` | Integração headless (UI, MP, save) |
| `docs/development/d18/*` | Documentação da entrega |

## Arquivos alterados (D18)

| Arquivo | Mudança |
|---|---|
| `src/inventory/mod.rs` | `pub mod consume` |
| `src/inventory/systems.rs` | `apply_use_item_requests` + textos |
| `src/core/events.rs` | `UseItemRequest`, `ItemUsed` |
| `src/core/mod.rs` | registro das mensagens |
| `src/ui/inventory_screen.rs` | linhas selecionáveis; Enter → use |
| `src/ui/mod.rs` | navigate antes de apply_use |
| `assets/data/items/starter_gear.ron` | `minor_ether` |
| `assets/data/shops/ash_village.ron` | vende ether |
| `assets/data/character.ron` | start: 3× tonic, 1× ether |
| `tests/equipment_app_tests.rs` | seleciona pike por id |
| `tests/support/town_route.rs` | cursor da loja (+1 por ether) |

## Integração ECS

`InventoryPlugin`: `apply_equip_requests` → `apply_use_item_requests` →
`sync_appearance` (Presentation, também com inventário pausado).

## Persistência

Sem mudança de formato: quantidades e HP/MP já estão no Save V3.
