---
paths:
  - "src/**/*.rs"
---

# Arquitetura

## Camadas por módulo

| Tipo de arquivo | Pode importar Bevy? | Exemplos |
|---|---|---|
| Regra pura | **Não** (só std, serde, ron, thiserror, tracing) | `player/attributes.rs`, `player/progression.rs`, `player/state.rs`, `player/visual/{manifest,playback,selection}.rs`, `world/{graph,validation,exploration}.rs`, `combat/damage.rs`, `combat/command_buffer.rs`, `combat/command_parser.rs`, `skills/{definitions,unlock,cooldowns,execution}.rs`, `inventory/{items,equipment,inventory}.rs`, `magic/elements.rs`, `enemies/{ai,definitions}.rs`, `world/{collision,progress,rooms}.rs`, `save/{serialization,save_manager}.rs`, `core/rng.rs`, `transform/{definitions,state,crossing}.rs`, `inventory/{weapons,appearance,loadout,compare}.rs`, `npc/{definitions,dialogue}.rs`, `economy/shop.rs`, `core/pixel_art.rs`, `player/visual/sockets.rs`, `player/visual/art_profile.rs` (N3.8), `player/visual/composition.rs` (N4-A), `player/visual/equipment_visual.rs` (N4-1-A), `player/visual/rig/*` (Modular Character Engine 01–05, incl. `design`, `paint`, `history`, `flex`), `inventory/{held,held_art}.rs`, `npc/art.rs` (V2.2), `world/{camera_rules,scenery,scenery_art}.rs` (V2.3) |
| Sistema/componente Bevy | Sim | `*/components.rs`, `*/systems.rs`, `player/movement.rs`, `player/visual/mod.rs`, `combat/{hitboxes,attacks,command_input}.rs`, `magic/{casting,projectiles}.rs`, `enemies/{behavior,spawning}.rs`, `world/{physics,maps,camera,exploration_systems,transitions,interactions,scenery_systems}.rs`, `transform/{systems,visual}.rs`, `inventory/{systems,visual,held_visual}.rs`, `npc/{mod,systems}.rs`, `economy/systems.rs`, `core/pixel_image.rs`, `ui/*` |

Antes de adicionar `use bevy` num arquivo puro, crie um arquivo Bevy separado.

## Comunicação entre módulos

- Mensagens (`#[derive(Message)]`) em `src/core/events.rs`: `CommandResolved`,
  `SkillActivated` (pedido), `SkillStarted` (ativação confirmada por `begin_skill`;
  os executores de efeito só reagem a ela), `BasicAttackRequested`, `DamageDealt`,
  `EnemyDefeated`, `Notification`, `RoomChanged` (V1.6: troca de sala no grafo de mundo),
  `RoomRequest` (V1.7), `PickupCollected`, `ProjectileBlocked` (V1.8: projétil parou na geometria),
  `FormChanged` (V1.9: forma iniciada, terminando, terminada ou recusada),
  `EquipRequest` (V2.0: pedido de equipar/remover; só `apply_equip_requests` lê)
  e `EquipmentChanged` (V2.0: troca aplicada), `TradeRequest` (V2.1: compra/venda;
  só `economy::systems::apply_trades` lê).
- NPCs e economia (V2.1): `D` decide objeto **antes** de NPC (`usable_object`);
  diálogo e loja são `GameState`s pausados; moedas só mudam em `apply_trades`.
  Moedas e estoque vão no Save v3 (`SaveData.economy`, V2.1.1); qualquer
  campo novo no save exige um v4 com struct congelado e autorização.
- Equipamento (V2.0): o golpe básico vem do perfil da arma
  (`ItemCatalog::strike_profile`); não mude as constantes de combate para
  diferenciar armas. Bônus sempre recalculados dos slots, nunca somados.
- Transformações (V1.9) aplicam perfis só por componentes opcionais
  (`LocomotionProfile`, `GravityProfile`, `Phasing`); sem eles, o código
  humano não muda. Não mexa nas constantes de movimento humano para uma forma.
- Um módulo **não** altera componentes de outro módulo para aplicar efeitos:
  ele emite uma mensagem e o dono reage (ex.: combate emite `DamageDealt`;
  player e enemies aplicam o dano cada um no seu `receive_damage`).
- O parser de comandos nunca aplica efeitos; ele só classifica a entrada.

## Ordem do frame (`GameSet`, `src/app/states.rs`)

`Input → Commands → Actions → Physics → Combat → Resolution → Progression`
(só em `GameState::Playing`), depois `Presentation` (sempre).
Todo sistema novo deve entrar em um `GameSet`.

- Equipamento visível (V2.2): arma/escudo são camadas (`inventory::held_visual`)
  posicionadas pelas âncoras do quadro (`player/visual/sockets.rs`); ficam
  desligadas enquanto o manifesto tiver `weapon_in_body: true`. Nunca desenhe
  uma arma sobre um corpo que já tem uma.

- Cenário (V2.3): decoração, fundos, brilhos e névoa são só apresentação
  (`RoomEntity` sem colisão); nunca altere `platforms` para acomodar decoração.
  Câmera e efeitos: `assets/data/camera.ron`.

## Dados

Balanceamento e conteúdo ficam em `assets/data/*.ron` e `assets/maps/*.ron`,
carregados e validados por `GameData::load` (`src/core/resources.rs`).
Novo conteúdo não deve exigir mudança de código.
