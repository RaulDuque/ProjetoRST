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
| `player` | `attributes.rs`, `progression.rs`, `state.rs`, `action_rules.rs` (V2.4), `visual/{manifest,playback,selection,sockets}.rs` (`sockets`: V2.2), `visual/art_profile.rs` (N3.8), `visual/composition.rs` (N4-A), `visual/equipment_visual.rs` (N4-1-A), `visual/vfx/rules.rs` | `components.rs`, `movement.rs`, `systems.rs`, `animation.rs`, `visual/{mod,systems,composite}.rs` (`composite`: N4-A), `visual/vfx/{mod,systems}.rs` |
| `combat` | `damage.rs`, `command_buffer.rs`, `command_parser.rs`, `strike_timeline.rs`, `reaction.rs`, `simultaneous.rs` (V2.4) | `command_input.rs`, `hitboxes.rs`, `attacks.rs` |
| `skills` | `definitions.rs`, `unlock.rs`, `cooldowns.rs`, `execution.rs` | `systems.rs` |
| `magic` | `elements.rs` | `casting.rs`, `projectiles.rs` |
| `inventory` | `items.rs`, `equipment.rs`, `inventory.rs`, `weapons.rs`, `appearance.rs`, `loadout.rs`, `compare.rs` (V2.0), `held.rs`, `held_art.rs` (V2.2) | `systems.rs`, `visual.rs` (V2.0), `held_visual.rs` (V2.2: arma e escudo como camadas) |
| `enemies` | `ai.rs`, `definitions.rs` | `components.rs`, `spawning.rs`, `behavior.rs` |
| `world` | `collision.rs`, `rooms.rs`, `progress.rs`, `graph.rs`, `validation.rs`, `exploration.rs`, `restore.rs` (V1.7), `camera_rules.rs`, `scenery.rs`, `scenery_art.rs` (V2.3) | `physics.rs`, `maps.rs`, `camera.rs`, `exploration_systems.rs`, `transitions.rs` (V1.7), `interactions.rs` (V1.8), `scenery_systems.rs` (V2.3) |
| `save` | `serialization.rs`, `save_manager.rs` (std) | `systems.rs` |
| `ui` | — | `theme.rs`, `hud.rs`, `status_screen.rs`, `damage_numbers.rs`, `world_map.rs` (no `UiPlugin` desde a V1.7), `room_fade.rs` (V1.8), `inventory_screen.rs` (V2.0), `dialogue_box.rs`, `shop_screen.rs` (V2.1) |
| `npc` (V2.1) | `definitions.rs`, `dialogue.rs`, `art.rs` (V2.2: figuras) | `systems.rs`, `mod.rs` |
| `economy` (V2.1) | `shop.rs` | `systems.rs` |
| `transform` (V1.9) | `definitions.rs`, `state.rs`, `crossing.rs` | `systems.rs`, `visual.rs` |
| `input` | — | `mod.rs` (mapeamento de teclas) |
| `app` | — | `mod.rs`, `plugins.rs`, `states.rs` |

As regras puras são testadas sem inicializar a engine (`tests/*.rs`).

**V2.3 — câmera e cenários:** a câmera segue regras puras configuradas em
`assets/data/camera.ron` (zoom por altura de mundo visível, zonas mortas,
olhar à frente, queda, limites) e o cenário (fundo em parallax, decoração,
brilhos, névoa) vem de `visual` das regiões e `decor` das salas, só como
apresentação. Detalhes: [`WORLD_VISUAL_V2_3.md`](WORLD_VISUAL_V2_3.md).

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

**N3.8 — protagonista experimental:** `player::visual::art_profile` (puro)
escolhe a arte pelo `DARK_CASTLE_PROTAGONIST` (padrão: produção V2.2).
Com `n3`, `app::build_for_protagonist` lê a arte de
`art-src/n3-8-production-run/stage/` pelo mesmo caminho de staging, insere
`ExperimentalProtagonist` (selo `ui::art_badge`) e `NearestPlayerArt`
(atlas com amostragem *nearest*). Estados sem clipe mostram o idle, e
`VisualPlayback::placeholder_for` guarda o clipe que faltou. Detalhes:
`docs/development/n3_8/N3_8_CORRIDA_PRODUCAO.md`.

**N4-A — máquina de sprites (prova isolada):** `player::visual::composition`
(puro) lê um personagem em camadas de um RON (peças, camadas, ordem,
ocultação, variantes, exceções de ordem por clipe) e resolve as camadas de
um clipe e quadro. `player::visual::composite` (Bevy) cria um sprite por
slot de camada e aponta todos para o mesmo `VisualPlayback` (`frame` e
`flip`, preenchidos por `animate_player_visual`): um relógio só. Ainda não
é usado pelo jogo; a prova é `cargo run --example n4_sprite_machine`.
Detalhes: `docs/development/n4/N4A_ARQUITETURA.md`.

**N4-1-A — equipamento visual por dados (prova isolada):** a base é o
personagem **vestido** (a partição das camadas da N4-A/C1). Uma
`VisualFamily` (slot, perfil de clipes, HIDE/REPLACE padrão, máscaras de
autoria, ordem por quadro, regra de clipe ausente) e um `EquipmentVisual`
(folhas por clipe, folhas espelhadas, variantes, sobrescritas) são lidos em
RON e juntados à composição por `equipment_visual::equip`, que valida tudo e
transforma cada item em peça comum. `resolve_facing` aplica OVERLAY (o slot),
HIDE (`hide_layers` por clipe) e REPLACE (`replaces`). Itens novos são só
dados; a arte é gerada ou verificada offline por
`art-src/n4-sprite-machine/scripts/build_equipment.py`. Não toca inventário
nem save. Detalhes: `docs/development/n4/N41A_ARQUITETURA.md`.

**N41-B — fábrica de equipamentos (autoria offline):** templates, materiais,
detalhes e perfis de animação em JSON (`art-src/n4-sprite-machine/factory/`)
e uma receita por item; `scripts/factory.py` gera os PNG (mapa de altura,
luz, rampa do material, contorno seletivo, detalhes só no interior), com
seed, proveniência e cache, e escreve os itens de tronco como manifestos
`EquipmentVisual` da família `torso_plate` em `stage/factory/items/`. O jogo
lê esses manifestos com o mesmo `equip` e os desenha com o mesmo compositor e
`VisualPlayback`; nada é gerado em tempo de execução e nenhum código Rust
mudou. Espadas, capas e ombreiras ficam isoladas (sem soquete/slot
validado). Detalhes: `docs/development/n4/N41B_ARQUITETURA.md`.

## Ordem de um frame

`GameSet` (`src/app/states.rs`), encadeados e executados só em `GameState::Playing`:

| Conjunto | O que acontece |
|---|---|
| `Input` | `read_keyboard` preenche `PlayerInput` |
| `Commands` | `update_facing`; `record_and_resolve_commands` grava o buffer e, ao apertar ATAQUE/MAGIA, classifica o comando: habilidade reconhecida → pedido `SkillActivated`; qualquer outro resultado → `CommandResolved` (+ `BasicAttackRequested` no botão ATAQUE) |
| `Actions` | timers de estado, respawn, guarda/esquiva, locomoção, regeneração; `begin_skill` revalida o pedido com o estado **atual** e confirma (custos, cooldown, `Casting`, `SkillStarted` + `CommandResolved`) ou rejeita (só `CommandResolved` com o motivo); efeitos: `basic_attack`, `melee_techniques` e `cast_spells` (estes dois só para `SkillStarted`); depois de `begin_skill`, `apply_strike_cancel_policy` (V2.4, recurso `StrikeCancelPolicy`); IA dos inimigos |
| `Physics` | gravidade, `move_bodies` (colisão AABB), projéteis |
| `Combat` | invulnerabilidade, golpes em antecipação armados (`arm_pending_strikes`, V2.4), hitboxes seguem o dono, `detect_hits` e dano por contato → `DamageDealt` |
| `Resolution` | `receive_damage` do jogador (respeita a invulnerabilidade a cada mensagem) e dos inimigos, ambos filtrando o frame pelo recurso `SimultaneousHitPolicy` (V2.4, `Legacy` = sem filtro) e aplicando o recuo de `DamageDealt::knockback`; morte → `EnemyDefeated`; depois do dano no jogador, `cancel_interrupted_hitboxes` remove as hitboxes presas a um dono em `Hurt`/`Dead` |
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
