# Arquitetura

## Visão geral

```
main.rs ──► app::run() ──► GameData::load(assets/) ──► app::build() ──► GamePlugin
                                                                         ├─ CorePlugin     (mensagens, RNG)
                                                                         ├─ InputPlugin    (teclado → PlayerInput)
                                                                         ├─ WorldPlugin    (sala, física, câmera)
                                                                         ├─ PlayerPlugin   (personagem)
                                                                         ├─ CombatPlugin   (comandos, hitboxes, ataques)
                                                                         ├─ SkillsPlugin   (custos, cooldowns, desbloqueio)
                                                                         ├─ MagicPlugin    (magias, projéteis)
                                                                         ├─ EnemiesPlugin  (spawn, IA, dano)
                                                                         ├─ UiPlugin       (HUD, status, números de dano)
                                                                         └─ SavePlugin     (F5/F9)
```

O `main.rs` só chama `dark_castle_rpg::app::run()`. Toda a lógica está na
biblioteca (`src/lib.rs`), o que permite testes de integração e o exemplo
`examples/autoplay.rs` reutilizarem o app real.

## Duas camadas em cada módulo

| Módulo | Regras puras (sem Bevy) | Camada Bevy |
|---|---|---|
| `core` | `rng.rs` | `constants.rs`, `events.rs`, `resources.rs` (carregador de dados), `time.rs` |
| `player` | `attributes.rs`, `progression.rs`, `state.rs` | `components.rs`, `movement.rs`, `systems.rs`, `animation.rs` |
| `combat` | `damage.rs`, `command_buffer.rs`, `command_parser.rs` | `command_input.rs`, `hitboxes.rs`, `attacks.rs` |
| `skills` | `definitions.rs`, `unlock.rs`, `cooldowns.rs`, `execution.rs` | `systems.rs` |
| `magic` | `elements.rs` | `casting.rs`, `projectiles.rs` |
| `inventory` | `items.rs`, `equipment.rs`, `inventory.rs` | — (ainda sem tela) |
| `enemies` | `ai.rs`, `definitions.rs` | `components.rs`, `spawning.rs`, `behavior.rs` |
| `world` | `collision.rs`, `rooms.rs`, `progress.rs` | `physics.rs`, `maps.rs`, `camera.rs` |
| `save` | `serialization.rs`, `save_manager.rs` (std) | `systems.rs` |
| `ui` | — | `theme.rs`, `hud.rs`, `status_screen.rs`, `damage_numbers.rs` |
| `input` | — | `mod.rs` (mapeamento de teclas) |
| `app` | — | `mod.rs`, `plugins.rs`, `states.rs` |

As regras puras são testadas sem inicializar a engine (`tests/*.rs`).

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
| `Progression` | EXP/level up/drops, sincronização de atributos derivados, desbloqueio automático de habilidades |
| `Presentation` | visual temporário, câmera, HUD, tela de status, números de dano, salvar/carregar (sempre roda) |

## Comunicação

Mensagens em `src/core/events.rs`. Um módulo nunca aplica efeitos em
componentes de outro módulo: ele emite uma mensagem e o dono reage.
Exemplo: `detect_hits` (combat) emite `DamageDealt`; `player::systems::receive_damage`
e `enemies::behavior::receive_damage` aplicam o dano em seus próprios componentes.

## Estados

- `GameState::Playing` — simulação ativa.
- `GameState::StatusScreen` — tela de status aberta (Tab); simulação pausada
  porque todos os `GameSet` exceto `Presentation` exigem `Playing`.
- `PlayerState` (máquina de estados do personagem, `player/state.rs`) — ver `COMBAT_SYSTEM.md`.

## Dados

`GameData` (recurso) é carregado **antes** do app abrir a janela e validado
(sala inicial existe, inimigos e itens referenciados existem, comandos válidos).
Um erro de dados encerra com mensagem clara e código de saída ≠ 0.
Raiz dos assets: `BEVY_ASSET_ROOT` → `CARGO_MANIFEST_DIR` → pasta do executável.

## Mundo e carregamento

Cada sala é um `RoomDefinition` independente (`assets/maps/*.ron`). Só a sala
atual é instanciada (entidades com `RoomEntity`), e a geometria fica em
`RoomGeometry`. Isso permite mapas grandes sem carregar tudo; as transições
entre salas estão no roadmap (etapa 11).

## Ajustes em relação à estrutura sugerida no briefing

Ver `DECISIONS.md` (D-007).
