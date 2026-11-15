# Arquitetura

## Visão geral

```
main.rs ──► app::run() ──► GameData::load(assets/) + load_world(assets/world/) ──► app::build_with_world() ──► GamePlugin
                                                                         ├─ CorePlugin     (mensagens, RNG)
                                                                         ├─ InputPlugin    (teclado → PlayerInput)
                                                                         ├─ WorldPlugin    (sala, física, câmera)
                                                                         ├─ PlayerPlugin   (personagem)
                                                                         ├─ CombatPlugin   (comandos, hitboxes, ataques)
                                                                         ├─ SkillsPlugin   (custos, cooldowns, desbloqueio)
                                                                         ├─ MagicPlugin    (magias, projéteis)
                                                                         ├─ EnemiesPlugin  (spawn, IA, dano)
                                                                         ├─ UiPlugin       (HUD, status, números de dano)
                                                                         ├─ SavePlugin     (F5/F9)
                                                                         ├─ TransformPlugin (V1.9: Forma Espectral)
                                                                         ├─ InventoryPlugin (V2.0: equipar/remover, aparência)
                                                                         ├─ NpcPlugin      (V2.1: NPCs por sala, D para conversar, diálogo)
                                                                         └─ EconomyPlugin  (V2.1: moedas, lojas, compra e venda)
```

O `main.rs` só chama `dark_castle_rpg::app::run()`. Toda a lógica está na
biblioteca (`src/lib.rs`), o que permite testes de integração e o exemplo
`examples/autoplay.rs` reutilizarem o app real.

## Duas camadas em cada módulo

| Módulo | Regras puras (sem Bevy) | Camada Bevy |
|---|---|---|
| `core` | `rng.rs`, `pixel_art.rs` (V2.2: rasterizador da arte provisória) | `constants.rs`, `events.rs`, `resources.rs` (carregador de dados), `time.rs`, `pixel_image.rs` (V2.2) |
| `player` | `attributes.rs`, `progression.rs`, `state.rs`, `visual/{manifest,playback,selection,sockets}.rs` (`sockets`: V2.2), `visual/vfx/rules.rs` | `components.rs`, `movement.rs`, `systems.rs`, `animation.rs`, `visual/{mod,systems}.rs`, `visual/vfx/{mod,systems}.rs` |
| `combat` | `damage.rs`, `command_buffer.rs`, `command_parser.rs` | `command_input.rs`, `hitboxes.rs`, `attacks.rs` |
| `skills` | `definitions.rs`, `unlock.rs`, `cooldowns.rs`, `execution.rs` | `systems.rs` |
| `magic` | `elements.rs` | `casting.rs`, `projectiles.rs` |
| `inventory` | `items.rs`, `equipment.rs`, `inventory.rs`, `weapons.rs`, `appearance.rs`, `loadout.rs`, `compare.rs` (V2.0), `held.rs`, `held_art.rs` (V2.2) | `systems.rs`, `visual.rs` (V2.0), `held_visual.rs` (V2.2: arma e escudo como camadas) |
| `enemies` | `ai.rs`, `definitions.rs` | `components.rs`, `spawning.rs`, `behavior.rs` |
| `world` | `collision.rs`, `rooms.rs`, `progress.rs`, `graph.rs`, `validation.rs`, `exploration.rs`, `restore.rs` (V1.7) | `physics.rs`, `maps.rs`, `camera.rs`, `exploration_systems.rs`, `transitions.rs` (V1.7), `interactions.rs` (V1.8) |
| `save` | `serialization.rs`, `save_manager.rs` (std) | `systems.rs` |
| `ui` | — | `theme.rs`, `hud.rs`, `status_screen.rs`, `damage_numbers.rs`, `world_map.rs` (no `UiPlugin` desde a V1.7), `room_fade.rs` (V1.8), `inventory_screen.rs` (V2.0), `dialogue_box.rs`, `shop_screen.rs` (V2.1) |
| `npc` (V2.1) | `definitions.rs`, `dialogue.rs`, `art.rs` (V2.2: figuras) | `systems.rs`, `mod.rs` |
| `economy` (V2.1) | `shop.rs` | `systems.rs` |
| `transform` (V1.9) | `definitions.rs`, `state.rs`, `crossing.rs` | `systems.rs`, `visual.rs` |
| `input` | — | `mod.rs` (mapeamento de teclas) |
| `app` | — | `mod.rs`, `plugins.rs`, `states.rs` |

As regras puras são testadas sem inicializar a engine (`tests/*.rs`).

**V2.2 — equipamento visível e âncoras:** o corpo do protagonista pode vir sem
arma (`weapon_in_body: false` no manifesto) com uma tabela de âncoras por
quadro (`animations/player_sockets.ron`, medida no Blender). Então
`inventory::held_visual` desenha a arma equipada e o escudo como filhos do
jogador, na âncora do quadro mostrado; a capa pendura da âncora "costas".
Com a arte antiga (sabre no corpo) as camadas ficam desligadas. A arte das
camadas e dos NPCs é gerada em código por `core::pixel_art`. A arte do
jogador pode vir de um staging (`PlayerArtSource` +
`app::build_with_candidate_art`) sem tocar em `assets/`. Detalhes:
[`VISUAL_EQUIPMENT_V2_2.md`](VISUAL_EQUIPMENT_V2_2.md).

## Ordem de um frame

`GameSet` (`src/app/states.rs`), encadeados e executados só em `GameState::Playing`:

| Conjunto | O que acontece |
|---|---|
| `Input` | `read_keyboard` preenche `PlayerInput` |
| `Commands` | `update_facing`; `record_and_resolve_commands` grava o buffer e, ao apertar ATAQUE/MAGIA, classifica o comando: habilidade reconhecida → pedido `SkillActivated`; qualquer outro resultado → `CommandResolved` (+ `BasicAttackRequested` no botão ATAQUE) |
| `Actions` | timers de estado, respawn, guarda/esquiva, locomoção, regeneração; `begin_skill` revalida o pedido com o estado **atual** e confirma (custos, cooldown, `Casting`, `SkillStarted` + `CommandResolved`) ou rejeita (só `CommandResolved` com o motivo); efeitos: `basic_attack`, `melee_techniques` e `cast_spells` (estes dois só para `SkillStarted`); IA dos inimigos |
| `Physics` | gravidade, `move_bodies` (colisão AABB), projéteis |
| `Combat` | invulnerabilidade, hitboxes seguem o dono, `detect_hits` e dano por contato → `DamageDealt` |
| `Resolution` | `receive_damage` do jogador (respeita a invulnerabilidade a cada mensagem) e dos inimigos; morte → `EnemyDefeated`; depois do dano no jogador, `cancel_interrupted_hitboxes` remove as hitboxes presas a um dono em `Hurt`/`Dead` |
| `Progression` | EXP/level up/drops, sincronização de atributos derivados, desbloqueio automático de habilidades; reações a `RoomChanged` (`enemies::enter_new_room`, `magic::clear_on_room_change`), que só ocorre com o grafo de mundo da V1.6 |
| `Presentation` | salvar/carregar, sprite do jogador (`player::visual`, depois de `quick_load`; lê `SkillStarted` para saber a habilidade; ver `PLAYER_VISUAL_ANIMATION_V1_3.md` e `PLAYER_ANIMATION_EXPANSION_V1_4.md`; desde a V1.5 também os VFX `player::visual::vfx`, que leem `SkillStarted` e `DamageDealt` sem alterar o gameplay, ver `PLAYER_ART_POLISH_V1_5.md`), visual temporário (depois do sprite), câmera, HUD, tela de status, números de dano, salvar/carregar (sempre roda) |

## Comunicação

Mensagens em `src/core/events.rs`. Um módulo nunca aplica efeitos em
componentes de outro módulo: ele emite uma mensagem e o dono reage.
Exemplo: `detect_hits` (combat) emite `DamageDealt`; `player::systems::receive_damage`
e `enemies::behavior::receive_damage` aplicam o dano em seus próprios componentes.

## Estados

- `GameState::Playing` — simulação ativa.
- `GameState::StatusScreen` — tela de status aberta (Tab); simulação pausada
  porque todos os `GameSet` exceto `Presentation` exigem `Playing`.
- `GameState::Inventory` (V2.0) — tela de equipamento aberta (I); pausa
  como a de status. Equipar acontece em `Presentation`
  (`inventory::systems::apply_equip_requests`, único escritor do
  equipamento, a partir de `EquipRequest`). Como o `quick_load`, ele
  atualiza `CharacterSheet`, `Vitals` e `CombatProfile` do jogador no mesmo
  frame, para a tela pausada mostrar os números finais.
- `GameState::Dialogue` e `GameState::Shop` (V2.1) — conversa e loja,
  pausadas como as outras telas. `npc::systems::close_on_room_change` fecha
  as duas se um load trocar a sala.
- `PlayerState` (máquina de estados do personagem, `player/state.rs`) — ver `COMBAT_SYSTEM.md`.

## Dados

`GameData` (recurso) é carregado **antes** do app abrir a janela e validado
(sala inicial existe, inimigos e itens referenciados existem, comandos válidos).
Um erro de dados encerra com mensagem clara e código de saída ≠ 0.
Raiz dos assets: `BEVY_ASSET_ROOT` → `CARGO_MANIFEST_DIR` → pasta do executável.

## Mundo e carregamento

Cada sala é um `RoomDefinition` independente (`assets/maps/*.ron`). Só a sala
atual é instanciada (entidades com `RoomEntity`), e a geometria fica em
`RoomGeometry`. Isso permite mapas grandes sem carregar tudo.

**V1.6 (protótipo):**
- **Grafo de mundo:** `assets/world/` com regiões, salas, portas e requisitos tipados.
- **Validador e estado de exploração.**
- **Plugin de transições:** `WorldExplorationPlugin` reutiliza `spawn_room`,
  `RoomEntity` e `RoomGeometry` e anuncia `RoomChanged`.
- **Fora do jogo principal:** o plugin é usado pelo exemplo
  `world_vertical_slice` e pelos testes, não pelo `GamePlugin`.
- **Detalhes:** `WORLD_ARCHITECTURE.md`, `ROOM_SYSTEM.md` e `V1_6_VERTICAL_SLICE.md`.

**V1.7 (integração):**
- **Início no mundo:** `app::run` carrega o grafo, valida (erros impedem o
  início) e começa em `vigil_gates_approach`.
- **Plugins:** `WorldExplorationPlugin` e `WorldMapUiPlugin` são registrados
  sempre e agem só com `WorldGraph`.
- **Sala única:** `app::build` sem mundo continua sendo o jogo de sala única,
  usado pelos testes antigos, pelo `autoplay` e pelo `visual_check`.
- **Ciclo de vida:** `GameSet::Rooms` → `RoomStep::{Request, Apply, React}`.
- **Save:** v2 com migração v1.
- **Detalhes:** [`WORLD_INTEGRATION_V1_7.md`](WORLD_INTEGRATION_V1_7.md).

**V1.8 (exploração avançada):**
- **Objetos interativos:** `world::interactions`.
- **Mensagem nova `ProjectileBlocked`:** a magia avisa onde um projétil
  parou na geometria, e o mundo a usa para paredes quebráveis.
- **Fade de transição:** `ui::room_fade`.
- **Tecla de interação:** `D`.
- **Detalhes:** [`WORLD_EXPLORATION_V1_8.md`](WORLD_EXPLORATION_V1_8.md).

## Ajustes em relação à estrutura sugerida no briefing

Ver `DECISIONS.md` (D-007).
